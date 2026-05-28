# namsh

namui crash analyzer — minimal storage for native crash dumps and PDBs,
gated by GitHub OAuth + admin-managed allowlist.

See [SPEC.md](./SPEC.md) for the full design.

## Layout

```
namsh/
├── Forte.toml          forte project marker (populated by forte deploy)
├── env.yaml            encrypted env (see "Bootstrap")
├── SPEC.md             design document
├── rs/                 Rust backend (wasm32-wasip2)
│   ├── Cargo.toml
│   ├── build.rs
│   └── src/
│       ├── lib.rs
│       ├── docs.rs            forte_doc schemas
│       ├── common/            auth, admin, github helpers
│       ├── actions/           RPC actions
│       └── pages/             page handlers (props)
└── fe/                 React frontend
    └── src/
        ├── app.tsx
        └── pages/             page components
```

## Required environment variables

All required; backend panics at startup if any is missing.

| Var | Form | Notes |
| --- | --- | --- |
| `__dek` | encrypted | KMS-wrapped DEK that decrypts the rest |
| `GITHUB_CLIENT_ID` | plain | GitHub OAuth app |
| `GITHUB_CLIENT_SECRET` | secret | GitHub OAuth app |
| `NAMSH_TOKEN_HMAC_KEY` | secret | 32 random bytes, hex; HMAC for CLI tokens |
| `COOKIE_SECRET` | secret | 32 random bytes, hex; signs the session cookie |
| `NAMSH_ADMIN_TOKEN` | secret | Bearer that unlocks `add_user`/`remove_user`/`list_users` |

Auto-injected by the Forte runtime:
- `TURSO_URL`, `TURSO_AUTH_TOKEN` — per-project Turso DB
- `FN0_OBJECT_STORAGE_URL` — per-project R2 bucket via `fn0-object-storage`

## Bootstrap

1. **Provision infra and deploy once empty.**
   `forte deploy` — this provisions the Turso DB and R2 bucket and assigns
   a domain like `namsh.fn0.dev`.

2. **Mint the DEK + secrets.**
   namsh does not ship its own `secrets_init`/`secrets_encrypt` actions.
   Reuse `fn0-control`'s tooling (or add Pulumi resources to compose
   `namshBootstrapEnvYaml` the same way fn0 produces `controlBootstrapEnvYaml`).
   Outputs go into `env.yaml`.

3. **Register the GitHub OAuth app.**
   Callback URL: `https://<your-namsh-domain>/oauth/github/callback`.
   Scope: `read:user`.

4. **Redeploy with secrets populated.**
   `forte deploy`.

5. **Add yourself to the allowlist.**
   ```sh
   curl -X POST https://<your-namsh-domain>/actions/add_user \
     -H "Authorization: Bearer $NAMSH_ADMIN_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{"github_id": <your gh id>, "github_login": "<your login>"}'
   ```

6. **Sign in.** Visit `https://<your-namsh-domain>/login`.

7. **Register your first build.** Visit `/builds`, enter a `build_id` (your
   git hash works), and copy:
   - the HMAC key — bake into the game build
   - the presigned PUT URL — upload your PDB to it
   - hit "Refresh PDB status" so namsh records the upload

## Client integration (game side)

To send a crash from your game:

1. Compute `stack_hash` (hex) by your own deterministic algorithm. namsh
   does not parse minidumps; identical hashes are merged, divergent ones
   are not.
2. Serialize a `CrashContext` (see SPEC §2) as JSON. Use `serde_json` (or an
   equivalent that produces the same byte stream).
3. Compute the signature:
   `HMAC-SHA256(key = hex_decode(hmac_key_hex), message = build_id || stack_hash || sha256(context_json))`
4. POST to `https://<your-namsh-domain>/actions/intake_crash` with:
   - body: `{ build_id, stack_hash, context }`
   - headers: `X-Namsh-Build-Id`, `X-Namsh-Signature` (hex)
5. If the response contains `upload.presigned_put_url`, do a single PUT of
   the raw `.dmp` to that URL. If not, the count was bumped but namsh
   already holds 3 dumps for this hash — drop the file locally.

Limits: 1 intake/minute, 5 intakes/24h per source IP (after HMAC check).

## CLI / AI access

Visit `/tokens` to mint a long-lived Bearer token. Use it on any authed
endpoint:

```sh
curl https://<your-namsh-domain>/actions/list_stack_groups \
  -H "Authorization: Bearer namsh_..."
```
