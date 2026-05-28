#!/bin/bash
set -euo pipefail

# End-to-end deploy of tower-defense to Steam.
#   - Installs steamcmd locally if missing
#   - Mints a namsh build_id + hmac_key for crash reporting (PDB upload)
#   - Builds the Windows target via namui CLI (unless --skip-build),
#     injecting NAMSH_BUILD_ID and NAMSH_HMAC_KEY so the binary can sign
#     crash intake requests
#   - Uploads the resulting PDB to namsh
#   - Syncs the build output into content/ and uploads via SteamPipe
#   - Sets the build live on the specified branch
#
# CD key holders receive the build on whichever branch their package targets
# (by default the "default" branch). Opt-in branches (e.g. "beta") require
# users to enable them in Steam client -> Properties -> Betas.

APP_ID=2793590
STEAM_USER="${STEAM_USER:-skatpgusskat}"
BRANCH="${STEAM_BRANCH:-default}"

NAMSH_URL="${NAMSH_URL:-https://g6vldebf.fn0.dev}"
NAMSH_TOKEN_FILE="${NAMSH_TOKEN_FILE:-$HOME/.config/namsh/token}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_OUTPUT="$REPO_ROOT/target/namui/x86_64-pc-windows-msvc"

STEAMCMD_DIR="$SCRIPT_DIR/steamcmd"
STEAMCMD="$STEAMCMD_DIR/steamcmd.sh"
STEAMCMD_URL="https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz"

CONTENT_DIR="$SCRIPT_DIR/content"
VDF_SRC="$SCRIPT_DIR/app_build_${APP_ID}.vdf"
VDF_GENERATED="$SCRIPT_DIR/app_build_${APP_ID}.generated.vdf"
SOURCE_EXE="namui-runtime-x86_64-pc-windows-msvc.exe"
TARGET_EXE="boxboxdefense.exe"
PDB_FILE="namui_runtime_x86_64_pc_windows_msvc.pdb"

SKIP_BUILD=0
SKIP_STEAM=0
PREVIEW=0
while [[ $# -gt 0 ]]; do
    case "$1" in
        --skip-build) SKIP_BUILD=1; shift ;;
        --skip-steam) SKIP_STEAM=1; shift ;;
        --preview)    PREVIEW=1; shift ;;
        --branch)     BRANCH="$2"; shift 2 ;;
        -h|--help)
            cat <<EOF
Usage: $0 [OPTIONS]

End-to-end Steam deployment for tower-defense.

Options:
  --branch NAME   Branch to set live (default: "$BRANCH"). Use "" to skip setlive.
  --skip-build    Skip the build step (and the namsh PDB upload).
  --skip-steam    Skip the Steam upload (still builds + uploads PDB to namsh).
  --preview       Dry-run: validate VDF without uploading to Steam.
  -h, --help      Show this help.

Environment:
  STEAM_USER       Steamworks username (default: $STEAM_USER)
  STEAM_BRANCH     Same as --branch (--branch wins if both set)
  NAMSH_URL        Override namsh base URL (default: $NAMSH_URL)
  NAMSH_TOKEN      Pre-supplied namsh CLI Bearer token (skips the OAuth flow)
  NAMSH_TOKEN_FILE Where to read/write the namsh CLI token on disk
                   (default: $NAMSH_TOKEN_FILE)
  NAMSH_TOKEN_LABEL Label to attach to a token minted via the OAuth flow
EOF
            exit 0 ;;
        *) echo "Unknown argument: $1"; exit 1 ;;
    esac
done

MISSING=()
for cmd in curl jq python3 openssl; do
    command -v "$cmd" >/dev/null 2>&1 || MISSING+=("$cmd")
done
if [ "${#MISSING[@]}" -gt 0 ]; then
    {
        echo "ERROR: missing required commands: ${MISSING[*]}"
        echo ""
        echo "Install (Ubuntu / WSL Ubuntu):"
        echo "  sudo apt-get install -y ${MISSING[*]}"
    } >&2
    exit 1
fi

open_browser() {
    local url="$1"
    local opener
    # Prefer powershell.exe over cmd.exe on WSL: cmd.exe interprets `&` in the
    # OAuth URL as a command separator (even when the URL is bash-quoted, since
    # WSL interop doesn't always re-quote single-token args before handing the
    # line to cmd.exe), which silently drops query params like `code_challenge`.
    if command -v wslview >/dev/null 2>&1; then opener="wslview"
    elif command -v xdg-open >/dev/null 2>&1; then opener="xdg-open"
    elif command -v open >/dev/null 2>&1; then opener="open"
    elif command -v powershell.exe >/dev/null 2>&1; then opener="powershell.exe"
    elif command -v cmd.exe >/dev/null 2>&1; then opener="cmd.exe"
    else return 1; fi
    case "$opener" in
        cmd.exe)
            # Caret-escape `&` so cmd.exe doesn't split the URL into separate
            # commands when invoked via WSL interop.
            local cmd_url="${url//&/^&}"
            ( nohup cmd.exe /c start "" "$cmd_url" </dev/null >/dev/null 2>&1 & ) ;;
        powershell.exe)
            ( nohup powershell.exe -NoProfile -Command "Start-Process '$url'" </dev/null >/dev/null 2>&1 & ) ;;
        *)
            ( nohup "$opener" "$url" </dev/null >/dev/null 2>&1 & ) ;;
    esac
    return 0
}

namsh_urlencode() {
    python3 -c "import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1], safe=''))" "$1"
}

namsh_load_or_authorize_token() {
    if [ -n "${NAMSH_TOKEN:-}" ]; then return 0; fi
    if [ -s "$NAMSH_TOKEN_FILE" ]; then
        NAMSH_TOKEN="$(cat "$NAMSH_TOKEN_FILE")"
        if [ -n "$NAMSH_TOKEN" ]; then return 0; fi
    fi
    namsh_oauth_flow
}

namsh_oauth_flow() {
    local code_verifier code_challenge state label
    code_verifier="$(openssl rand 32 | base64 | tr -d '=\n' | tr '/+' '_-')"
    code_challenge="$(printf '%s' "$code_verifier" | openssl dgst -sha256 -binary \
        | base64 | tr -d '=\n' | tr '/+' '_-')"
    state="$(openssl rand -hex 16)"
    label="${NAMSH_TOKEN_LABEL:-td-$(hostname)}"

    local code_file port_file
    code_file="$(mktemp)"
    port_file="$(mktemp)"

    python3 - "$code_file" "$port_file" <<'PYEOF' &
import sys
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs

code_path, port_path = sys.argv[1], sys.argv[2]

class H(BaseHTTPRequestHandler):
    def log_message(self, *a, **k): pass
    def do_GET(self):
        q = parse_qs(urlparse(self.path).query)
        with open(code_path, "w") as f:
            f.write(q.get("code", [""])[0])
        self.send_response(200)
        self.send_header("Content-Type", "text/plain; charset=utf-8")
        self.end_headers()
        self.wfile.write("Authorized. You can close this tab.".encode())

s = HTTPServer(("127.0.0.1", 0), H)
with open(port_path, "w") as f:
    f.write(str(s.server_address[1]))
s.handle_request()
PYEOF
    local listener_pid=$!

    local i=0
    while [ ! -s "$port_file" ] && [ $i -lt 50 ]; do sleep 0.1; i=$((i+1)); done
    if [ ! -s "$port_file" ]; then
        kill "$listener_pid" 2>/dev/null || true
        echo "ERROR: namsh OAuth listener failed to start"
        exit 1
    fi
    local port redirect_uri
    port="$(cat "$port_file")"
    redirect_uri="http://127.0.0.1:${port}/"

    local url
    url="${NAMSH_URL}/oauth/cli/authorize?redirect_uri=$(namsh_urlencode "$redirect_uri")"
    url="${url}&code_challenge=${code_challenge}&code_challenge_method=S256"
    url="${url}&state=${state}&label=$(namsh_urlencode "$label")"

    echo ""
    echo "=== namsh CLI OAuth ==="
    echo "Open in your browser to authorize:"
    echo "  $url"
    echo ""
    if open_browser "$url"; then
        echo "(Attempted to open the browser automatically. If nothing happened, click the URL above.)"
    else
        echo "(No supported browser opener found. Click the URL above manually.)"
    fi

    wait "$listener_pid" 2>/dev/null || true

    if [ ! -s "$code_file" ]; then
        echo "ERROR: did not receive authorization code"
        exit 1
    fi
    local code
    code="$(cat "$code_file")"
    rm -f "$code_file" "$port_file"

    local payload resp t
    payload="$(jq -nc \
        --arg c "$code" \
        --arg v "$code_verifier" \
        --arg r "$redirect_uri" \
        '{code:$c, code_verifier:$v, redirect_uri:$r}')"
    resp="$(curl -fsS -X POST "${NAMSH_URL}/__forte_action/oauth_cli_exchange" \
        -H "Content-Type: application/json" \
        -d "$payload")"
    t="$(jq -r '.t' <<<"$resp")"
    if [ "$t" != "Ok" ]; then
        echo "ERROR: namsh oauth_cli_exchange returned $resp"
        exit 1
    fi
    NAMSH_TOKEN="$(jq -r '.token' <<<"$resp")"
    mkdir -p "$(dirname "$NAMSH_TOKEN_FILE")"
    ( umask 077 && printf '%s' "$NAMSH_TOKEN" > "$NAMSH_TOKEN_FILE" )
    chmod 600 "$NAMSH_TOKEN_FILE"
    echo "namsh token saved to $NAMSH_TOKEN_FILE"
}

namsh_request_pdb_upload() {
    local payload resp t
    payload="$(jq -nc --arg b "$BUILD_ID" '{build_id:$b}')"
    resp="$(curl -fsS -X POST "${NAMSH_URL}/__forte_action/request_pdb_upload" \
        -H "Authorization: Bearer $NAMSH_TOKEN" \
        -H "Content-Type: application/json" \
        -d "$payload")"
    t="$(jq -r '.t' <<<"$resp")"
    if [ "$t" != "Ok" ]; then
        echo "ERROR: namsh request_pdb_upload returned $resp"
        exit 1
    fi
    NAMSH_HMAC_KEY_HEX="$(jq -r '.hmacKeyHex' <<<"$resp")"
    NAMSH_PDB_PUT_URL="$(jq -r '.pdbPresignedPutUrl' <<<"$resp")"
}

namsh_confirm_pdb_uploaded() {
    local payload resp t
    payload="$(jq -nc --arg b "$BUILD_ID" '{build_id:$b}')"
    resp="$(curl -fsS -X POST "${NAMSH_URL}/__forte_action/confirm_pdb_uploaded" \
        -H "Authorization: Bearer $NAMSH_TOKEN" \
        -H "Content-Type: application/json" \
        -d "$payload")"
    t="$(jq -r '.t' <<<"$resp")"
    if [ "$t" != "Ok" ]; then
        echo "WARNING: namsh confirm_pdb_uploaded returned $resp"
        return 1
    fi
    local size
    size="$(jq -r '.size' <<<"$resp")"
    echo "namsh recorded PDB size: $size bytes"
}

BUILD_ID="$(date -u +%Y-%m-%d_%H-%M-%SZ)"

NAMSH_HMAC_KEY_HEX=""
NAMSH_PDB_PUT_URL=""

if [ "$SKIP_BUILD" -eq 0 ]; then
    namsh_load_or_authorize_token
    echo "=== Requesting namsh PDB upload for build_id=$BUILD_ID ==="
    namsh_request_pdb_upload
fi

# 1. Install steamcmd locally if missing
if [ "$SKIP_STEAM" -eq 0 ] && [ ! -x "$STEAMCMD" ]; then
    echo "=== Installing steamcmd -> $STEAMCMD_DIR ==="
    mkdir -p "$STEAMCMD_DIR"
    curl -sqL "$STEAMCMD_URL" | tar -xz -C "$STEAMCMD_DIR"
    echo "=== Bootstrapping steamcmd (self-update) ==="
    "$STEAMCMD" +quit > /dev/null 2>&1 || true
fi

# 2. Build Windows target via namui CLI
if [ "$SKIP_BUILD" -eq 0 ]; then
    if ! command -v namui &> /dev/null; then
        echo "ERROR: 'namui' CLI not found on PATH."
        echo "Install it from namui/namui-cli, or run with --skip-build if the"
        echo "build output already exists at $BUILD_OUTPUT."
        exit 1
    fi
    echo "=== Building Windows target ==="
    NAMSH_BUILD_ID="$BUILD_ID" \
    NAMSH_HMAC_KEY="$NAMSH_HMAC_KEY_HEX" \
    NAMSH_URL="$NAMSH_URL" \
    namui build x86_64-pc-windows-msvc \
        --manifest-path "$REPO_ROOT/Cargo.toml" \
        --release
fi

if [ ! -d "$BUILD_OUTPUT" ]; then
    echo "ERROR: build output not found at $BUILD_OUTPUT"
    exit 1
fi

# 2b. Upload PDB to namsh (presigned PUT URLs expire in 10 minutes, so we
# re-request after the build to get a fresh one).
if [ "$SKIP_BUILD" -eq 0 ]; then
    if [ ! -f "$BUILD_OUTPUT/$PDB_FILE" ]; then
        echo "ERROR: PDB not found at $BUILD_OUTPUT/$PDB_FILE"
        exit 1
    fi
    echo "=== Refreshing namsh PDB upload URL ==="
    namsh_request_pdb_upload
    echo "=== Uploading PDB ($(du -h "$BUILD_OUTPUT/$PDB_FILE" | cut -f1)) -> namsh ==="
    curl -fsS -X PUT \
        -H "Content-Type: application/octet-stream" \
        --data-binary "@$BUILD_OUTPUT/$PDB_FILE" \
        "$NAMSH_PDB_PUT_URL" >/dev/null
    namsh_confirm_pdb_uploaded
fi

if [ "$SKIP_STEAM" -eq 1 ]; then
    echo ""
    echo "=== Skipping Steam upload (--skip-steam). build_id=$BUILD_ID ==="
    exit 0
fi

# 3. Sync build output into content/
echo "=== Syncing build output -> $CONTENT_DIR ==="
rm -rf "$CONTENT_DIR"
mkdir -p "$CONTENT_DIR"
cp -r "$BUILD_OUTPUT"/. "$CONTENT_DIR"/

if [ ! -f "$CONTENT_DIR/$SOURCE_EXE" ]; then
    echo "ERROR: expected executable not found: $CONTENT_DIR/$SOURCE_EXE"
    exit 1
fi
mv -f "$CONTENT_DIR/$SOURCE_EXE" "$CONTENT_DIR/$TARGET_EXE"

# 4. Generate VDF with the requested branch / preview flag
sed -E "s/(\"setlive\"[[:space:]]+)\"[^\"]*\"/\1\"$BRANCH\"/" "$VDF_SRC" > "$VDF_GENERATED"
if [ "$PREVIEW" -eq 1 ]; then
    sed -i -E 's/("preview"[[:space:]]+)"0"/\1"1"/' "$VDF_GENERATED"
    echo "=== PREVIEW mode: dry-run, no upload ==="
fi

# 5. Upload via steamcmd
echo "=== Uploading (setlive: \"$BRANCH\") ==="
set +e
"$STEAMCMD" +login "$STEAM_USER" +run_app_build "$VDF_GENERATED" +quit
EXIT=$?
set -e

rm -f "$VDF_GENERATED"

if [ "$EXIT" -ne 0 ]; then
    echo "=== steamcmd failed with exit code $EXIT ==="
    exit "$EXIT"
fi

echo ""
echo "=== Deployment complete (build_id=$BUILD_ID) ==="
if [ "$PREVIEW" -eq 0 ]; then
    if [ -z "$BRANCH" ]; then
        echo "Build uploaded. Assign it to a branch manually at:"
        echo "  https://partner.steamgames.com/apps/builds/$APP_ID"
    else
        echo "Build is now live on branch \"$BRANCH\"."
        if [ "$BRANCH" = "default" ]; then
            echo "CD key holders will receive this update automatically."
        else
            echo "Note: users must opt into \"$BRANCH\" via Steam client -> Properties -> Betas."
        fi
    fi
fi
