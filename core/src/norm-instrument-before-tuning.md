---
slug: norm-instrument-before-tuning
type: normative
status: exact
stage: draft
depends:
  - norm-probes-before-claims
---

# Build the instrument before tuning by feel

Before optimizing or tuning any behavior by feel, build the instrument: a repeatable deterministic benchmark, a measured noise floor, and one variable changed at a time.

## Formal Expression

1. **Instrument first.** No tuning or optimization proceeds on felt impressions of speed or quality. The first deliverable of a tuning effort is a **repeatable, deterministic benchmark** that anyone can re-run.
2. **Noise floor before attribution.** Measure the benchmark's own run-to-run variance **before** attributing any effect to a change. An improvement inside the noise floor is not an improvement; it is a coin flip wearing one.
3. **One variable at a time.** A sweep that changes two things and improves has learned nothing attributable. Where a joint change is forced, say so and mark the attribution as confounded.
4. **Why this is architecture, not preference.** The developer is itself an adaptive agent whose only observation channel onto the system is its instruments — tuning by feel is acting on an observation channel that does not exist. (The AAT framing lived in graduated ARCHITECTURE §7; the norm survives the file.)

## Epistemic Status

**Max attainable: exact** as process law. **Currently `exact` as adopted practice** — carried in the graduated `ARCHITECTURE.md` §7 ("build the instrument before tuning by feel is architecture, not preference", Joseph-era doctrine) and practiced since (probe-first sessions throughout July 2026). **No DECISIONS ratification record exists**; authority is practiced doctrine with claude segment packaging (2026-07-29), not a `:by joseph` entry — say so if this norm is ever contested rather than defending it as ratified.

The founding episode (Godot view-performance benchmark, 2026-06-23: a repeatable LOD/streaming benchmark replacing feel-tuning, with the measured cost knob and numbers) is history-layer: `archive/godot-voxel/bench/README.md`. The numbers are archaeology; the procedure is the durable part.

Stage `draft`.

## Discussion

The norm is `#norm-probes-before-claims` specialized to performance and quality tuning, where the temptation to skip the probe is strongest because the feedback *feels* immediate — a build that feels faster is the exact analogue of a claim that feels verified. `#norm-probe-sensitivity` governs the discrimination side (can the instrument tell the difference at all); this norm governs the order of operations (the instrument exists before the tuning starts).

## Working Notes

- The noise-floor clause has no mechanized guard (nothing fails a build when a tuning lands without a benchmark). If this discipline is ever caught failing, `#norm-caught-disciplines-become-mechanisms` applies.
