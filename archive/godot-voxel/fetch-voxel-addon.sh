#!/usr/bin/env bash
# Fetch the godot_voxel (Zylann Voxel Tools) GDExtension addon into addons/.
# The ~105 MB of platform binaries are git-ignored and re-fetched by this
# script, pinned by sha256 so a future run gets the exact build this spike used.
#
# Source: https://github.com/Zylann/godot_voxel/releases (v1.6x, "Voxel Tools
# 1.6 GDExtension for Godot 4.5+"). Loads on Godot 4.7 via GDExtension
# forward-compat.
set -euo pipefail

url="https://github.com/Zylann/godot_voxel/releases/download/v1.6x/GodotVoxelExtension.zip"
sha="dfee985a0cff7059a31ada665e88a634fdcc3eab51f83fe5f6dd48939dd5372a"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
zip="$(mktemp -t godotvoxel).zip"

echo "downloading godot_voxel addon..."
curl -sL -o "$zip" "$url"
got="$(shasum -a 256 "$zip" | awk '{print $1}')"
if [ "$got" != "$sha" ]; then
  echo "sha256 mismatch: expected $sha, got $got" >&2
  exit 1
fi

unzip -oq "$zip" -d "$here"
rm -f "$zip"
echo "godot_voxel addon installed -> $here/addons/zylann.voxel/"
