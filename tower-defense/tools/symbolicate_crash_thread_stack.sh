#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage:
    tools/symbolicate_crash_thread_stack.sh --exe <path/to/app.exe> --pdb <path/to/app.pdb> --dmp <path/to/crash.dmp> [--work-dir <dir>] [--full-out <path>]

What it does:
  1) Runs dump_syms with EXE + PDB (required for Windows x64 CFI)
  2) Builds a Breakpad symbol store for minidump-stackwalk
  3) Runs minidump-stackwalk
  4) Prints a concise crash-thread stack (human-readable)

Requirements:
  - dump_syms
  - minidump-stackwalk

Example:
    tools/symbolicate_crash_thread_stack.sh \
    --exe target/namui/x86_64-pc-windows-msvc/namui-runtime-x86_64-pc-windows-msvc.exe \
    --pdb target/namui/x86_64-pc-windows-msvc/namui_runtime_x86_64_pc_windows_msvc.pdb \
    --dmp target/namui/x86_64-pc-windows-msvc/crash.dmp
EOF
}

need_cmd() {
    local cmd="$1"
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "ERROR: required command not found: $cmd" >&2
        exit 1
    fi
}

EXE=""
PDB=""
DMP=""
WORK_DIR=""
FULL_OUT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --exe)
            EXE="$2"
            shift 2
            ;;
        --pdb)
            PDB="$2"
            shift 2
            ;;
        --dmp)
            DMP="$2"
            shift 2
            ;;
        --work-dir)
            WORK_DIR="$2"
            shift 2
            ;;
        --full-out)
            FULL_OUT="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "ERROR: unknown argument: $1" >&2
            usage
            exit 1
            ;;
    esac
done

if [[ -z "$EXE" || -z "$PDB" || -z "$DMP" ]]; then
    echo "ERROR: --exe, --pdb, --dmp are required" >&2
    usage
    exit 1
fi

need_cmd dump_syms
need_cmd minidump-stackwalk
need_cmd awk
need_cmd sed
need_cmd head
need_cmd mkdir
need_cmd cp
need_cmd basename
need_cmd mktemp

if [[ ! -f "$EXE" ]]; then
    echo "ERROR: exe not found: $EXE" >&2
    exit 1
fi
if [[ ! -f "$PDB" ]]; then
    echo "ERROR: pdb not found: $PDB" >&2
    exit 1
fi
if [[ ! -f "$DMP" ]]; then
    echo "ERROR: dmp not found: $DMP" >&2
    exit 1
fi

cleanup=0
if [[ -z "$WORK_DIR" ]]; then
    WORK_DIR="$(mktemp -d)"
    cleanup=1
fi

mkdir -p "$WORK_DIR"
RAW_SYM="$WORK_DIR/raw.sym"
STACK_OUT="$WORK_DIR/stackwalk.txt"
SYMBOLS_ROOT="$WORK_DIR/symbols"

if [[ -z "$FULL_OUT" ]]; then
    FULL_OUT="$STACK_OUT"
fi

PDB_BASENAME="$(basename "$PDB")"
PDB_STEM="${PDB_BASENAME%.pdb}"

# EXE + PDB are both required for robust Windows x64 unwind info (STACK CFI).
dump_syms "$EXE" "$PDB" > "$RAW_SYM"

MODULE_LINE="$(head -n 1 "$RAW_SYM")"
DEBUG_ID="$(awk 'NR==1 && $1=="MODULE" {print $4}' "$RAW_SYM")"
MODULE_FILE="$(awk 'NR==1 && $1=="MODULE" {print $5}' "$RAW_SYM")"

if [[ -z "$DEBUG_ID" || -z "$MODULE_FILE" ]]; then
    echo "ERROR: failed to parse MODULE header from symbol file" >&2
    echo "First line: $MODULE_LINE" >&2
    exit 1
fi

SYMBOL_DIR="$SYMBOLS_ROOT/$PDB_BASENAME/$DEBUG_ID"
mkdir -p "$SYMBOL_DIR"

# Keep both names to avoid filename expectation mismatches across tools.
cp "$RAW_SYM" "$SYMBOL_DIR/${PDB_STEM}.sym"
cp "$RAW_SYM" "$SYMBOL_DIR/${PDB_BASENAME}.sym"

minidump-stackwalk --symbols-path "$SYMBOLS_ROOT" "$DMP" > "$FULL_OUT"

echo "=== Crash Summary ==="
awk '/^Crash reason:|^Crash address:|^Process uptime:/{print}' "$FULL_OUT"

echo ""
echo "=== Crash Thread (concise) ==="
awk '
BEGIN { in_crashed=0; seen=0; expect_found=0 }
/^Thread [0-9]+ .*\(crashed\)/ {
    if (seen == 0) {
        print $0
        in_crashed=1
        seen=1
    }
    next
}
{
    if (in_crashed == 0) next
    if ($0 ~ /^Thread [0-9]+ / || $0 ~ /^Loaded modules:/ || $0 ~ /^Unloaded modules:/) exit
    if ($0 ~ /^[[:space:]]*[0-9]+[[:space:]][[:space:]]/) {
        print $0
        expect_found=1
    } else if ($0 ~ /^    Found by:/) {
        if (expect_found == 1) {
            print $0
            expect_found=0
        }
    }
}
END {
    if (seen == 0) {
        print "(No crashed thread section found; showing first 80 lines of full output.)"
    }
}
' "$FULL_OUT"

if ! grep -qE '^Thread [0-9]+ .*\(crashed\)' "$FULL_OUT"; then
    sed -n '1,80p' "$FULL_OUT"
fi

echo ""
echo "=== Paths ==="
echo "MODULE debug_id: $DEBUG_ID"
echo "MODULE file: $MODULE_FILE"
echo "Full stackwalk: $FULL_OUT"
echo "Symbol root: $SYMBOLS_ROOT"

if [[ "$cleanup" -eq 1 ]]; then
    echo ""
    echo "NOTE: temporary work-dir used: $WORK_DIR"
    echo "Keep files by passing: --work-dir <dir>"
fi
