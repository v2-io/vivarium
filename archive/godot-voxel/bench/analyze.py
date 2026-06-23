#!/usr/bin/env python3
"""Aggregate Godot-voxel benchmark CSVs and compare configs.

Usage:  analyze.py <config-dir> [<config-dir> ...]
Each <config-dir> holds run_*.csv from bench/run.sh. Prints, per config, the
median over the steady-state window (t >= 14 s, after the scripted 180° whip)
across its runs, plus the run-to-run spread — so a real effect can be told from
this machine's measured jitter.

Trust order (from the 2026-06-23 noise-floor calibration): fps and data_blocks
are reproducible (~±7-8%); mesh_blocks / q_total / draw_calls / vmem swing
±20-37% between identical runs — treat as signal only if large or replicated.
"""
import csv, glob, os, statistics as st, sys

STEADY_T = 14.0
METRICS = ["fps", "worst_ms", "mesh_blocks", "data_blocks", "q_total", "draw_calls", "vmem_mb"]
TRUSTED = {"fps", "data_blocks"}


def run_medians(path):
    rows = []
    with open(path) as f:
        for r in csv.DictReader(f):
            row = {k: float(v) for k, v in r.items()}
            if row["t"] >= STEADY_T:
                rows.append(row)
    return {m: st.median(r[m] for r in rows) for m in METRICS} if rows else None


def config_summary(d):
    """Median-of-run-medians and spread for one config dir."""
    runs = [run_medians(p) for p in sorted(glob.glob(os.path.join(d, "run_*.csv")))]
    runs = [r for r in runs if r]
    out = {}
    for m in METRICS:
        vals = [r[m] for r in runs]
        med = st.median(vals)
        spread = (max(vals) - min(vals)) / med * 100 if med else 0.0
        out[m] = (med, spread, len(vals))
    return out


def main(dirs):
    summaries = {os.path.basename(d.rstrip("/")): config_summary(d) for d in dirs}
    names = list(summaries)
    base = names[0]
    print(f"steady-state (t>={STEADY_T:.0f}s) median per config; n runs in (); ± = run-to-run spread")
    print(f"{'metric':12} " + "  ".join(f"{n:>16}" for n in names))
    for m in METRICS:
        cells = []
        for n in names:
            med, spread, k = summaries[n][m]
            cells.append(f"{med:8.0f} ±{spread:2.0f}% (n{k})")
        star = " *" if m in TRUSTED else "  "
        line = f"{m:12} " + "  ".join(f"{c:>16}" for c in cells)
        # delta vs first config for trusted metrics
        if m in TRUSTED and len(names) > 1:
            b = summaries[base][m][0]
            deltas = []
            for n in names[1:]:
                v = summaries[n][m][0]
                deltas.append(f"{n}:{(v-b)/b*100:+.0f}%" if b else "n/a")
            line += "   Δ " + " ".join(deltas)
        print(line + star)
    print("\n* = trusted metric (reproducible). Others are noisy; need >~20% or replication to believe.")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    main(sys.argv[1:])
