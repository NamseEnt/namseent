#!/bin/bash
set -euo pipefail

# End-to-end deploy of tower-defense to Steam.
#   - Installs steamcmd locally if missing
#   - Builds the Windows target via namui CLI (unless --skip-build)
#   - Syncs the build output into content/ and uploads via SteamPipe
#   - Sets the build live on the specified branch
#
# CD key holders receive the build on whichever branch their package targets
# (by default the "default" branch). Opt-in branches (e.g. "beta") require
# users to enable them in Steam client -> Properties -> Betas.

APP_ID=2793590
STEAM_USER="${STEAM_USER:-skatpgusskat}"
BRANCH="${STEAM_BRANCH:-default}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_OUTPUT="$REPO_ROOT/target/namui/x86_64-pc-windows-msvc"

STEAMCMD_DIR="$SCRIPT_DIR/steamcmd"
STEAMCMD="$STEAMCMD_DIR/steamcmd.sh"
STEAMCMD_URL="https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz"

CONTENT_DIR="$SCRIPT_DIR/content"
VDF_SRC="$SCRIPT_DIR/app_build_${APP_ID}.vdf"
VDF_GENERATED="$SCRIPT_DIR/app_build_${APP_ID}.generated.vdf"

SKIP_BUILD=0
PREVIEW=0
while [[ $# -gt 0 ]]; do
    case "$1" in
        --skip-build) SKIP_BUILD=1; shift ;;
        --preview)    PREVIEW=1; shift ;;
        --branch)     BRANCH="$2"; shift 2 ;;
        -h|--help)
            cat <<EOF
Usage: $0 [OPTIONS]

End-to-end Steam deployment for tower-defense.

Options:
  --branch NAME   Branch to set live (default: "$BRANCH"). Use "" to skip setlive.
  --skip-build    Skip the build step; reuse existing build output.
  --preview       Dry-run: validate VDF without uploading.
  -h, --help      Show this help.

Environment:
  STEAM_USER      Steamworks username (default: $STEAM_USER)
  STEAM_BRANCH    Same as --branch (--branch wins if both set)
EOF
            exit 0 ;;
        *) echo "Unknown argument: $1"; exit 1 ;;
    esac
done

if ! command -v curl &> /dev/null; then
    echo "ERROR: 'curl' is not installed. sudo apt-get install -y curl"
    exit 1
fi

# 1. Install steamcmd locally if missing
if [ ! -x "$STEAMCMD" ]; then
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
    namui build x86_64-pc-windows-msvc \
        --manifest-path "$REPO_ROOT/Cargo.toml" \
        --release
fi

if [ ! -d "$BUILD_OUTPUT" ]; then
    echo "ERROR: build output not found at $BUILD_OUTPUT"
    exit 1
fi

# 3. Sync build output into content/
echo "=== Syncing build output -> $CONTENT_DIR ==="
rm -rf "$CONTENT_DIR"
mkdir -p "$CONTENT_DIR"
cp -r "$BUILD_OUTPUT"/. "$CONTENT_DIR"/

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
echo "=== Deployment complete ==="
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
