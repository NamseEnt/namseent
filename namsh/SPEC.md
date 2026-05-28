# namsh

namui crash analyzer. Storage and library for native crash dumps + PDBs,
with a tiny gating layer. Optimized for AI-driven analysis: dumps and PDBs
are downloaded by a local CLI/AI and analyzed off-platform.

## 0. Stack

- Forte project (Rust → wasm32-wasip2 backend, React frontend).
- Turso DB (auto-provisioned by `forte deploy`).
- One R2 bucket via `fn0-object-storage` (per-project bucket, presigned PUT/GET).
- Domain: assigned automatically by `forte deploy`.

## 1. Roles

- **Admin**: holds `NAMSH_ADMIN_TOKEN`. Adds/removes users out of band (curl).
- **User**: GitHub-OAuth-authenticated developer. Browses crashes, manages
  builds (PDB upload), issues CLI tokens for AI use.
- **Game client**: anonymous (no user identity). Only allowed call is
  `intake_crash`, gated by per-build HMAC + per-IP rate limit.

## 2. Data Model

`forte_doc` schemas in `rs/src/docs.rs`.

```rust
UserDoc {
  github_id: i64,                  // pk
  github_login: String,
  created_at: DateTime,
  cli_tokens: Vec<CliTokenEntry>,
  web_sessions: Vec<WebSessionEntry>,
}
// UserDoc presence == allowlist. No UserDoc means OAuth login is refused.

BuildDoc {
  build_id: String,                // pk (developer-chosen; git hash, semver, etc.)
  created_at: DateTime,
  uploaded_by: i64,                // github_id of issuer
  hmac_key_hex: String,            // 32 random bytes hex; minted once, never rotated
  pdb_uploaded: bool,              // true once the user has actually PUT a PDB
  pdb_r2_key: Option<String>,
  pdb_size: Option<u64>,
}

StackGroupDoc {
  stack_hash: String,              // pk (hex; client-computed, server does not verify)
  first_seen: DateTime,
  last_seen: DateTime,
  count: u64,                      // strictly monotonic; counted even when no dump is stored
  dump_ids: Vec<String>,           // bounded to MAX_DUMPS_PER_GROUP (= 3)
  latest_context: CrashContext,    // overwritten on each intake
}

DumpDoc {
  dump_id: String,                 // pk (uuid)
  stack_hash: String,
  build_id: String,
  uploaded_at: DateTime,
  r2_key: String,
  context: CrashContext,
  client_ip: String,
}

IpRateLimitDoc {
  ip: String,                      // pk
  recent_requests: Vec<DateTime>,  // pruned to 24h window on each access
}

CliAuthorizationCodeDoc {
  code: String,                    // pk; URL-safe base64, ~43 chars
  github_id: i64,
  code_challenge: String,          // base64url(SHA-256(verifier))
  redirect_uri: String,            // http loopback only
  label: String,                   // label the CLI token will get
  expires_at: DateTime,            // now + 5 min
}
```

`CrashContext` (serialized as JSON, embedded in StackGroupDoc/DumpDoc):

```rust
CrashContext {
  build_id: String,
  install_id: String,              // anonymous, client-generated UUID
  session_uptime_sec: u64,
  log_tail: Option<String>,        // <= 64 KiB; client truncates
}
```

## 3. R2 Layout

- `pdb/<build_id>.pdb`
- `dump/<stack_hash>/<dump_id>.dmp`

## 4. Stack Hash

Computed entirely by the client. namsh does not parse minidumps and does not
verify the hash against the uploaded dump.

The client is responsible for picking a stable hash function (recommended:
top-N exception-thread frames in `module+offset_from_module_base` form,
SHA-256 hex). If the client's algorithm is unstable, dedup falls apart;
namsh has no opinion.

## 5. HMAC (intake)

- Per-build, 32 random bytes (hex), minted once by `request_pdb_upload` and
  stored in `BuildDoc.hmac_key_hex`. Never rotated; if leaked, the developer
  must mint a new `build_id`.
- Payload: `build_id || stack_hash || sha256(canonical_json(context))`,
  concatenated as raw bytes.
- Signature: HMAC-SHA256, sent hex-encoded.
- Headers on `intake_crash`:
  - `X-Namsh-Build-Id: <build_id>`
  - `X-Namsh-Signature: <hex>`
- Verification: `BuildDoc { build_id }` lookup → HMAC verify. No build →
  `UnknownBuild`. Bad signature → `InvalidSignature`. Neither bumps any
  counters.

## 6. IP Rate Limit (intake)

- 1 request per minute AND 5 requests per 24h, per source IP.
- Limit applies to all `intake_crash` calls regardless of outcome (including
  ones that would only bump a counter). Reason: an attacker who learned an
  HMAC key could otherwise pin counters at max and starve real dumps.
- Limit is enforced **after** HMAC verification, so unsigned/invalid traffic
  cannot waste a real client's rate budget.
- Storage: `IpRateLimitDoc { ip, recent_requests }`. Read-modify-write per
  request; entries older than 24h are pruned in place.

## 7. Authentication

Two surfaces, both implemented in `rs/src/common/auth.rs` (ported from
`fn0-control`):

- **Web session**: GitHub OAuth → HMAC-signed cookie (`namsh_session`)
  containing `{ github_id, token }`. `token` is also stored in
  `UserDoc.web_sessions`; logout deletes both.
- **CLI token** (Bearer): `namsh_<base64url(github_id ++ uuid)>.<base64url(hmac)>`,
  signed with `NAMSH_TOKEN_HMAC_KEY`. Verified by `verify_cli_token` and
  `bearer_user`. No expiry. No scopes. Revocable. Two issuance paths:
  - **Interactive**: user mints from `/tokens` (UI) and copies the value once.
  - **CLI OAuth (loopback PKCE)**: a CLI on the user's box runs the flow in
    §7.1 and never has to see the token by hand. namsh's `/tokens` page is
    still the source of truth for listing/revocation.

### 7.1 CLI OAuth flow (loopback PKCE)

A CLI script (e.g. tower-defense's `steam/deploy.sh`) bootstraps a token
without manual copy/paste:

1. CLI generates a random 32-byte `code_verifier` and
   `code_challenge = base64url(SHA-256(code_verifier))`; binds a loopback
   socket on `http://127.0.0.1:<rand-port>/`; opens the user's browser at
   `https://<namsh>/oauth/cli/authorize?redirect_uri=...&code_challenge=...&code_challenge_method=S256&state=...&label=...`.
2. namsh's authorize page enforces: `code_challenge_method == "S256"`,
   non-empty `state` and `code_challenge`, and `redirect_uri` is a *loopback*
   URI (`127.0.0.1`, `localhost`, `[::1]`). If the user is not logged in,
   namsh stashes the full authorize URL in a short-lived
   `namsh_pending_cli_consent` cookie and redirects to `/login`; the OAuth
   callback consumes that cookie and resumes the authorize page.
3. User clicks "Approve". `approve_cli_authorization` mints a
   `CliAuthorizationCodeDoc` (5-minute TTL), then returns a
   `redirect_to = <redirect_uri>?code=...&state=...` URL; the browser
   redirects, hitting the CLI's loopback server.
4. CLI POSTs `/actions/oauth_cli_exchange { code, code_verifier, redirect_uri }`.
   namsh verifies the code (single-use, unexpired, `redirect_uri` match,
   `SHA-256(code_verifier) == code_challenge`), deletes the code, mints a CLI
   token (same shape as `issue_token`), records it under
   `UserDoc.cli_tokens`, and returns it.

The token is identical to one issued from `/tokens`; revocation works the
same way. namsh does not store the `code_verifier` (PKCE check is one-shot).

OAuth callback gating: after exchanging the GitHub code, if
`UserDocGet { github_id }` returns `None` we do not create a session; we
redirect to `/login?error=not_authorized`. On success, if a
`namsh_pending_cli_consent` cookie holds a `/oauth/cli/authorize`-prefixed
URL, we redirect there instead of `/`.

## 8. Admin

Out-of-band only, modeled on `fn0-control`'s `common::admin::verify`:
`Authorization: Bearer <NAMSH_ADMIN_TOKEN>`, constant-time compare.

Admin actions:
- `add_user { github_id, github_login }` → create empty `UserDoc`.
- `remove_user { github_id }` → delete `UserDoc` (also revokes all sessions
  and CLI tokens by virtue of deletion).
- `list_users` → enumerate `UserDoc`s.

## 9. Endpoints

### Public

- `POST /actions/intake_crash`
  - Input: `{ build_id, stack_hash, context: CrashContext }`
  - Headers: `X-Namsh-Build-Id`, `X-Namsh-Signature`
  - Flow: HMAC verify → IP rate-limit check → `StackGroupDoc` upsert →
    if `count < 3`: mint `dump_id`, presigned PUT URL, append to `dump_ids`,
    create `DumpDoc` → always `count += 1`, `last_seen = now`,
    `latest_context = context`.
  - Output: `Ok { upload: Some { dump_id, presigned_put_url } | None }`,
    `RateLimited`, `InvalidSignature`, `UnknownBuild`.

- `GET /login` (page) — sign-in button + error display.
- `GET /oauth/github` — redirect to GitHub.
- `GET /oauth/github/callback` — exchange code, gate on UserDoc, create
  session OR redirect with error; honors `namsh_pending_cli_consent` (§7.1).
- `GET /oauth/cli/authorize` — see §7.1.
- `POST /actions/oauth_cli_exchange` — see §7.1.

### Authenticated (session OR Bearer)

- `request_pdb_upload { build_id }` →
  `Ok { pdb_presigned_put_url, hmac_key_hex, build_id }`
  - First call for a `build_id`: creates `BuildDoc`, mints `hmac_key_hex`.
  - Subsequent calls: returns the existing `hmac_key_hex` and a fresh
    presigned PUT URL (PDB overwrite allowed).
- `request_pdb_download { build_id }` → `Ok { presigned_get_url }` /
  `NotFound`.
- `request_dump_download { dump_id }` → `Ok { presigned_get_url }`.
- `list_stack_groups` → recent N groups, summary fields.
- `get_stack_group { stack_hash }` → group + dump summaries.
- `list_builds` → builds + PDB upload status.
- `issue_token { label }` / `list_tokens` / `revoke_token { id }` — same
  shape as `fn0-control`.
- `approve_cli_authorization { redirect_uri, code_challenge, code_challenge_method, state, label }`
  — session-only, see §7.1.

### Admin (Bearer NAMSH_ADMIN_TOKEN)

- `add_user`, `remove_user`, `list_users` (see §8).

## 10. Pages

- `/` — stack groups list (count, last_seen, build_id distribution).
- `/issues/:stack_hash` — group detail + per-dump download buttons + context.
- `/builds` — per-build summary (PDB present? size? upload button hint).
- `/tokens` — CLI token issue/list/revoke (mirrors fn0-control).
- `/oauth/cli/authorize` — consent screen for the CLI OAuth flow (§7.1).
- `/login`, `/oauth/github/callback`.

## 11. Environment Variables

Sourced from `env.yaml` (Forte's encrypted env mechanism). All are mandatory;
the binary `expect`s them at startup with no fallback.

- `NAMSH_ADMIN_TOKEN` — Bearer for admin actions.
- `NAMSH_TOKEN_HMAC_KEY` — HMAC key for CLI tokens.
- `COOKIE_SECRET` — HMAC key for signed cookies.
- `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET` — GitHub OAuth app.

## 12. Bootstrap

1. `forte deploy` (creates Turso DB + assigns domain).
2. Generate secrets, populate `env.yaml`, redeploy.
3. Register the GitHub OAuth app with the assigned domain as callback URL.
4. Add yourself: `curl -X POST https://<domain>/actions/add_user \
   -H "Authorization: Bearer $NAMSH_ADMIN_TOKEN" \
   -d '{"github_id": ..., "github_login": "..."}'`
5. Visit `/login`, sign in.
6. `/builds` → register your first build → grab `hmac_key_hex` + PDB
   presigned URL. Bake the HMAC key into your game build.

## 13. Non-goals

- No server-side minidump parsing, no symbolication, no grouping heuristics.
  Dumps are stored verbatim; analysis lives in the local AI/CLI.
- No notifications, no diff-by-release, no regression detection.
- No multi-tenancy beyond the user allowlist.
- No PDB versioning per `build_id`; the latest upload wins.
- No client-supplied attachments beyond the structured `CrashContext`
  (log_tail is the escape hatch).

## 14. Operational Limits

- `MAX_DUMPS_PER_GROUP = 3` (hard).
- `RATE_LIMIT_PER_MINUTE = 1`, `RATE_LIMIT_PER_24H = 5` (per IP).
- `LOG_TAIL_MAX_BYTES = 64 * 1024` (client must truncate; server rejects
  larger).
- R2 retention: forever. No GC.
