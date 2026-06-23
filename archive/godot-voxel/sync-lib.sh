#!/usr/bin/env bash
# Build the Rust bridge and copy the cdylib into the Godot project's bin/ so the
# .gdextension can find it via a res:// path. Run from anywhere.
#
#   ./sync-lib.sh            # debug build (default)
#   ./sync-lib.sh release    # release build
set -euo pipefail

profile="${1:-debug}"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"

case "$profile" in
  debug)   cargo build -p vivarium-godot --manifest-path "$repo/Cargo.toml" ;;
  release) cargo build -p vivarium-godot --release --manifest-path "$repo/Cargo.toml" ;;
  *) echo "usage: $0 [debug|release]" >&2; exit 2 ;;
esac

mkdir -p "$here/bin"
case "$(uname -s)" in
  Darwin) ext=dylib ;;
  Linux)  ext=so ;;
  *) echo "unsupported OS: $(uname -s)" >&2; exit 1 ;;
esac
dest="$here/bin/libvivarium_godot.$ext"
# Replace, don't overwrite-in-place: on Apple Silicon, overwriting a loaded/
# code-signed dylib invalidates its signature and the kernel SIGKILLs anything
# that dlopen()s it ("Code Signature Invalid"). Remove then copy, then re-apply
# a fresh ad-hoc signature so Gatekeeper/AMFI is satisfied.
rm -f "$dest"
cp "$repo/target/$profile/libvivarium_godot.$ext" "$dest"
if [ "$(uname -s)" = "Darwin" ]; then
  codesign --force --sign - "$dest" >/dev/null 2>&1 \
    && echo "re-signed (ad-hoc) $dest" \
    || echo "warning: codesign failed on $dest" >&2
fi
echo "synced libvivarium_godot.$ext ($profile) -> $here/bin/"
