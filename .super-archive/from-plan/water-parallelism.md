# Water parallelism — CPU-parallel and GPU compute (plan, 2026-07-03)

> **Graduated 2026-07-24** to `.super-archive/from-plan/`. Build/reasoning trail only — claim homes in `core/`. Do not re-mine as claim source.


*Intent and constraints for whoever takes water performance on — deliberately
not a task list. Written the day the async mesher landed (6→121 fps) and the
question "can water go to the GPU?" got a real look.*

## Why

The pipes kernel is the frame's hot loop: ~100 steps/s single-threaded at
1024² (L21 over ~4.9 km). Two things want orders of magnitude more: **fine
nested water near the pawn** (L22–L23 — Joseph's original fine-water ask, the
deepest item on the 07-02 list) and fast-forward experimentation (#11's time
controls). The kernel is near-ideal parallel work; the machine running it has
a dozen idle cores and a very large idle GPU.

## The one structural precondition: gather, not scatter

Sediment advection currently SCATTERS (each cell pushes load to neighbours).
Every parallel backend — rayon or GPU — wants the GATHER dual (each cell pulls
from its neighbours' fluxes). The rewrite is small, determinism-preserving,
and shared prep for both backends. Do it once, on the CPU reference, first.

## Staging that seemed right from here

1. **Gather rewrite + rayon row-parallelism** on the existing CPU kernel:
   hours of work, likely 5–8× on the M4 Max, determinism trivially intact
   (fixed partitioning, no atomics). Worth doing regardless of GPU plans.
2. **wgpu compute backend**: the virtual-pipes family was *born* as a GPU
   algorithm (Mei et al. 2007 ran in shaders). All stages are regular-grid
   stencils; ping-pong storage buffers; WGSL ports directly from the
   gather-form CPU code. It is MEMORY-bound: ~100 MB traffic/step at 1024²
   against ~400 GB/s unified bandwidth → thousands of steps/s, call it
   20–40×. Apple unified memory makes per-burst readback (~tens of MB)
   essentially free. A solid day of careful work, plus validation.

## The determinism policy (load-bearing — see DESIGN-REDUX §12)

Determinism-as-ontology is not negotiable, and GPUs complicate it: kernels are
bit-reproducible on one device+driver but NOT across devices. Policy:

- The **CPU kernel is the reference implementation** — canon for tests,
  replay, and probes. It never goes away.
- A GPU backend is a **rung on the implementation ladder** behind the same
  flux interface (§12): swappable, and its identity is part of the nomos key
  (FILL_ALGO_VERSION-style), so caches and saves never silently mix backends.
- Validation is by probe, not vibes: conservation totals, channel_profile
  regime invariants, and velocity_histogram fingerprints must agree with the
  CPU reference within stated tolerance; the tolerance is written down, and
  divergence beyond it is a bug in the rung, full stop.

## What stays CPU

Erosion: Priority-Flood and the elevation-ordered stream-power pass are
sequential by nature, run rarely, and cache — not worth GPU heroics. Sampling
/ coordinate-noise for far LOD rings is a plausible future GPU candidate, but
async meshing already bought the frame budget back; don't optimize ahead of a
measured need.

## Placement constraint

`vivarium-world` is deliberately dependency-free (the pure reference). A wgpu
backend lives in a separate crate (or the worldview spike while it's
spike-grade) — the core's purity is a stated property, not an accident.
