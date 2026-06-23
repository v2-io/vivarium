#!/usr/bin/env bash
# Run the deterministic Godot benchmark N times for a named config and archive the
# telemetry CSVs into bench/<name>/ in the repo — so results are reproducible and
# version-controlled, not stranded in Godot's ephemeral user:// dir.
#
#   ./run.sh <config-name> <repeats> [ENV=VAL ...]
#   ./run.sh baseline 3
#   ./run.sh vratio03 3 VIVARIUM_VRATIO=0.3
#
# Each run writes run_<i>.csv (telemetry) and run_<i>_t24.png (final frame) into
# bench/<config-name>/. PNGs are git-ignored (bulky, regenerable); the CSVs are the
# durable record. Bench duration overridable via BENCH_SECS (default 24).
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
proj="$(cd "$here/.." && pwd)"
ud="$HOME/Library/Application Support/Godot/app_userdata/vivarium — godot voxel spike"

name="${1:?usage: run.sh <config-name> <repeats> [ENV=VAL ...]}"
repeats="${2:-3}"
shift 2 || true   # remaining args are ENV=VAL overrides for the config under test

outdir="$here/$name"
mkdir -p "$outdir"
for i in $(seq 1 "$repeats"); do
  tag="_${name}_${i}"
  rm -f "$ud/telemetry${tag}.csv" "$ud/bench${tag}_"*.png
  echo "[$name] run $i/$repeats ..."
  env "$@" VIVARIUM_BENCH=1 VIVARIUM_TAG="$tag" \
      VIVARIUM_BENCH_SECS="${BENCH_SECS:-24}" VIVARIUM_SHOT_SECS=24 \
      godot --path "$proj" >/dev/null 2>&1 || true
  cp "$ud/telemetry${tag}.csv" "$outdir/run_${i}.csv"
  [ -f "$ud/bench${tag}_t24.png" ] && cp "$ud/bench${tag}_t24.png" "$outdir/run_${i}_t24.png" || true
done
echo "[$name] -> $outdir"
