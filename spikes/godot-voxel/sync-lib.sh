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
cp "$repo/target/$profile/libvivarium_godot.$ext" "$here/bin/"
echo "synced libvivarium_godot.$ext ($profile) -> $here/bin/"
