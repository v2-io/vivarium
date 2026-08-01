#!/usr/bin/env bash
# Shared helpers for orientation gate. Source only — not run alone.
# shellcheck shell=bash

ORIENT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ORIENT_DIR="$ORIENT_ROOT/.orient"
TEST_FILE="$ORIENT_DIR/test.jsonl"
PASS_FILE="$ORIENT_DIR/pass"
BURN_FILE="$ORIENT_DIR/burned"
SEAL_LIST="$ORIENT_DIR/sealed_paths"

# Seal scope: all markdown under core/src (NOT core/OUTLINE.md).
# Outline alone is not enough; the trap is confident outline-only "orientation."
ORIENT_SEAL_GLOB='core/src/**/*.md'

STUDY_TTL_SEC="${ORIENT_STUDY_TTL_SEC:-7200}"
PASS_TTL_SEC="${ORIENT_PASS_TTL_SEC:-1800}"   # 30m — short; one-shot commit
TEST_TTL_SEC="${ORIENT_TEST_TTL_SEC:-300}"   # 5m to complete prove after try-me
SAMPLE_N="${ORIENT_SAMPLE_N:-12}"

orient_die() { echo "orient: $*" >&2; exit 1; }

orient_norm() {
  tr '[:upper:]' '[:lower:]' | tr -s '[:space:]' ' ' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//' \
    | tr -d '[:punct:]'
}

orient_sha_file() {
  local f="$1"
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$f" | awk '{print $1}'
  else
    sha256sum "$f" | awk '{print $1}'
  fi
}

orient_sha_str() {
  local s="$1"
  if command -v shasum >/dev/null 2>&1; then
    printf '%s' "$s" | shasum -a 256 | awk '{print $1}'
  else
    printf '%s' "$s" | sha256sum | awk '{print $1}'
  fi
}

orient_list_segment_md() {
  # All claim segments; exclude nothing under core/src that is .md
  find "$ORIENT_ROOT/core/src" -type f -name '*.md' | sort
}

orient_seal_core_src() {
  mkdir -p "$ORIENT_DIR"
  : >"$SEAL_LIST"
  local f
  while IFS= read -r f; do
    [[ -f "$f" ]] || continue
    echo "$f" >>"$SEAL_LIST"
    chmod a-r "$f" || orient_die "failed to seal $f"
  done < <(orient_list_segment_md)
  echo "orient: sealed $(wc -l <"$SEAL_LIST" | tr -d ' ') files under core/src/" >&2
}

orient_unseal_core_src() {
  local f
  if [[ -f "$SEAL_LIST" ]]; then
    while IFS= read -r f; do
      [[ -e "$f" ]] || continue
      chmod 644 "$f" 2>/dev/null || chmod u+r "$f" 2>/dev/null || true
    done <"$SEAL_LIST"
    rm -f "$SEAL_LIST"
  fi
  # emergency restore if any core/src md still unreadable
  while IFS= read -r f; do
    if [[ -e "$f" && ! -r "$f" ]]; then
      chmod 644 "$f" 2>/dev/null || true
    fi
  done < <(orient_list_segment_md 2>/dev/null || true)
}

orient_git_head() {
  git -C "$ORIENT_ROOT" rev-parse --short HEAD 2>/dev/null || echo "nogit"
}
