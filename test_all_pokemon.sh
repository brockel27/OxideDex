#!/usr/bin/env bash
set -euo pipefail

# Comprehensive OxideDex test: all base species (IDs 1-1025) + all alternate forms.
# Usage: bash test_all_pokemon.sh [--no-build]
#
# Output: test_results_<timestamp>.log in the project root.
# Failures are flagged in the log and summarized at the end.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"  # must run from project root so rustemon cache is reused

BINARY="$SCRIPT_DIR/target/release/OxideDex"
TIMESTAMP="$(date +%Y%m%d_%H%M%S)"
LOG_FILE="$SCRIPT_DIR/test_results_${TIMESTAMP}.log"
TIMEOUT_SECS=20
TMP_STDERR="$(mktemp /tmp/oxidedex_test_XXXXXX)"
SKIP_BUILD=0
[[ "${1:-}" == "--no-build" ]] && SKIP_BUILD=1

trap 'rm -f "$TMP_STDERR"' EXIT

log() { echo "$@" | tee -a "$LOG_FILE"; }

# ── Phase 0: Header ──────────────────────────────────────────────────────────
log "=== OxideDex Full Test Suite ==="
log "Started:  $(date)"
log "Log file: $LOG_FILE"
log ""

# ── Phase 1: Build ───────────────────────────────────────────────────────────
if [ "$SKIP_BUILD" -eq 0 ]; then
    log "--- Building release binary ---"
    if cargo build --release 2>&1 | tee -a "$LOG_FILE"; then
        log "Build succeeded."
    else
        log "ERROR: Build failed. Aborting."
        exit 1
    fi
    log ""
else
    log "--- Skipping build (--no-build) ---"
    log ""
fi

if [ ! -x "$BINARY" ]; then
    log "ERROR: Binary not found at $BINARY. Run without --no-build."
    exit 1
fi

# ── Phase 2: Fetch Pokémon list ──────────────────────────────────────────────
log "--- Fetching Pokémon list from PokeAPI ---"

POKEMON_LIST="$(
    curl --silent --fail --max-time 30 \
        "https://pokeapi.co/api/v2/pokemon?limit=2000" \
    | python3 -c "
import json, sys
data = json.load(sys.stdin)
def get_id(url):
    return int(url.rstrip('/').split('/')[-1])
# Sort by numeric ID: base species (1-1025) first, then forms (10001+)
for r in sorted(data['results'], key=lambda r: get_id(r['url'])):
    print(r['name'])
"
)"

TOTAL="$(echo "$POKEMON_LIST" | wc -l | tr -d ' ')"
log "Fetched $TOTAL Pokémon entries to test."
log ""

# ── Phase 3: Test loop ───────────────────────────────────────────────────────
log "--- Running tests ---"
printf "%-5s  %-45s  %-6s  %s\n" "IDX" "SLUG" "RESULT" "ERROR" >> "$LOG_FILE"
printf "%s\n" "$(printf '%.0s-' {1..80})" >> "$LOG_FILE"

PASS_COUNT=0
FAIL_COUNT=0
FAIL_LIST=()
IDX=0

while IFS= read -r slug; do
    IDX=$((IDX + 1))

    set +e
    timeout "$TIMEOUT_SECS" "$BINARY" "$slug" > /dev/null 2> "$TMP_STDERR"
    EXIT_CODE=$?
    set -e

    if [ $EXIT_CODE -eq 124 ]; then
        RESULT="FAIL"
        ERROR_MSG="TIMEOUT after ${TIMEOUT_SECS}s"
    elif [ $EXIT_CODE -eq 0 ]; then
        RESULT="PASS"
        ERROR_MSG=""
    else
        ERROR_MSG="$(tr '\n' ' ' < "$TMP_STDERR" | sed 's/[[:space:]]*$//')"
        RESULT="FAIL"
    fi

    printf "%-5s  %-45s  %-6s  %s\n" "$IDX" "$slug" "$RESULT" "$ERROR_MSG" >> "$LOG_FILE"

    if [ "$RESULT" = "FAIL" ]; then
        FAIL_COUNT=$((FAIL_COUNT + 1))
        FAIL_LIST+=("$IDX|$slug|$ERROR_MSG")
    else
        PASS_COUNT=$((PASS_COUNT + 1))
    fi

    # Overwriting progress line on terminal (stderr only, not in log)
    printf "\r  [%d/%d] %-45s %s   " "$IDX" "$TOTAL" "$slug" "$RESULT" >&2
done <<< "$POKEMON_LIST"

printf "\n" >&2

# ── Phase 4: Summary ─────────────────────────────────────────────────────────
log ""
log "$(printf '%.0s=' {1..80})"
log "=== TEST SUMMARY ==="
log "$(printf '%.0s=' {1..80})"
log "Completed:    $(date)"
log "Total tested: $TOTAL"
log "PASSED:       $PASS_COUNT"
log "FAILED:       $FAIL_COUNT"
log ""

if [ "${#FAIL_LIST[@]}" -gt 0 ]; then
    log "--- FAILURES ---"
    for entry in "${FAIL_LIST[@]}"; do
        IFS='|' read -r idx fslug errmsg <<< "$entry"
        log "  [$idx] $fslug"
        if [ -n "$errmsg" ]; then
            log "        $errmsg"
        fi
    done
else
    log "All tests passed!"
fi

log ""
log "Full results saved to: $LOG_FILE"
