#!/usr/bin/env bash
# Shared helpers for orientation gate. Source only — not run alone.
# shellcheck shell=bash

ORIENT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ORIENT_DIR="$ORIENT_ROOT/.orient"
STUDY_FILE="$ORIENT_DIR/study"
PASS_FILE="$ORIENT_DIR/pass"
QUIZ_FILE="$ORIENT_ROOT/bin/orient-quiz.sha"
# Required Level-A corpus (full-file digests). Edit carefully — agents must re-study.
ORIENT_REQUIRED=(
  "Claude.md"
  "FORMAT.md"
  "ETHICS.md"
  "core/OUTLINE.md"
  "core/src/scope-moratorium-endogenous-emergence.md"
  "core/src/norm-no-depiction-without-referent.md"
  "core/src/norm-decision-authority.md"
  "core/src/norm-probes-before-claims.md"
  "core/src/scope-segment-canon.md"
  "core/src/disc-known-active-hotspots.md"
  "core/src/form-time-indexed-stage-chains.md"
  "core/src/form-core-view-wall.md"
)

STUDY_TTL_SEC="${ORIENT_STUDY_TTL_SEC:-7200}"   # 2h
PASS_TTL_SEC="${ORIENT_PASS_TTL_SEC:-3600}"     # 1h

orient_die() { echo "orient: $*" >&2; exit 1; }

orient_norm() {
  # stdin → normalized answer for hashing
  tr '[:upper:]' '[:lower:]' | tr -s '[:space:]' ' ' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//'
}

orient_sha_file() {
  # portable sha256 of file contents
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

orient_corpus_hash() {
  # ordered digest of path=filehash lines
  local acc="" p
  for p in "${ORIENT_REQUIRED[@]}"; do
    local f="$ORIENT_ROOT/$p"
    [[ -f "$f" ]] || orient_die "missing required path: $p"
    acc+="${p}=$(orient_sha_file "$f")"$'\n'
  done
  orient_sha_str "$acc"
}

orient_seal() {
  local p f
  mkdir -p "$ORIENT_DIR"
  : >"$ORIENT_DIR/sealed.$$"
  for p in "${ORIENT_REQUIRED[@]}"; do
    f="$ORIENT_ROOT/$p"
    [[ -f "$f" ]] || continue
    echo "$f" >>"$ORIENT_DIR/sealed.$$"
    chmod a-r "$f" || orient_die "failed to seal $p"
  done
}

orient_unseal() {
  local list="$ORIENT_DIR/sealed.$$"
  # Also recover a leaked seal from a crashed prove (any sealed.* leftover).
  local f
  if [[ -f "$list" ]]; then
    while IFS= read -r f; do
      [[ -e "$f" ]] || continue
      chmod 644 "$f" 2>/dev/null || chmod u+r "$f" 2>/dev/null || true
    done <"$list"
    rm -f "$list"
  fi
  # Emergency: if Claude.md is unreadable, restore all required paths.
  if [[ -f "$ORIENT_ROOT/Claude.md" ]] && [[ ! -r "$ORIENT_ROOT/Claude.md" ]]; then
    for p in "${ORIENT_REQUIRED[@]}"; do
      f="$ORIENT_ROOT/$p"
      [[ -e "$f" ]] || continue
      chmod 644 "$f" 2>/dev/null || true
    done
  fi
}
