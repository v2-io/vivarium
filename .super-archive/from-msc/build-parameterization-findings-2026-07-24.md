# Build/CLI parameterization findings — harness upgrade note

*2026-07-24, from a live dialogue with Joseph (he: "why in the world is there a subsystem-specific flag in the global command?" → "I need to upgrade the harness"). Findings, not an audit; verified against code where noted, and one plausible-but-wrong near-miss recorded so it is not re-walked. Instrument/harness residue — **not** claim canon; the claims it leans on are cited by slug. Home for the actionable version until a harness owner picks it up.*

## TL;DR

`vivarium build`'s `--epochs` flag is **erosion's convergence knob hoisted to the world-level CLI**, and it is a symptom, not the disease. The disease is that **build parameterization has no proper home yet**: per-vivium "how far / at what fidelity" belongs on the **manifest** (`#form-manifest-prescribes-vivium`, "designed; partial implementation"), per-nomos convergence params belong **on the nomos declaration folded into its key** (`#form-add-system-contract` + `#form-complete-content-addressed-key`), and the demand→materialization mapping is the **builder daemon**'s job (`#detail-builder-daemon`, unbuilt). The same gap makes you type `build` by hand, hardcodes the playback frame count, and strands erosion's knob on the global CLI — three faces of "demand has no home."

## Verified current state (code, 2026-07-24)

- `bin/vivarium.rs:238` — `let epochs = flag(rest, "--epochs").unwrap_or(40);` — one integer, default 40.
- `bin/vivarium.rs:344–345` — the same `epochs` feeds both subsystems:
  - `world.erosion_tile(face, level, oi, oj, TILE_NX, epochs)` — erosion carving iterations.
  - `world.water_tile(face, level, oi, oj, TILE_NX, epochs, 200)` — water's parameter is named **`erosion_epochs`** (`query.rs:556`), plus a **hardcoded `steps = 200`** (water's own relaxation count).
- `mantle_thermal.rs:92` — `abyssal_epochs()` returns a **hardcoded 6-element vector** `[3.6, 3.4, 3.2(present), 3.0, 2.8, 2.6]` Ga. No flag controls it. This is the deep-time **playback frame count** (globe **T** key) and the "epoch reductions: 6 materialized" line — **unrelated to `--epochs`** despite the shared word.

## Two things called "epochs" (terminology collision — a real defect)

1. **`--epochs E`** = erosion iterations on the present-day tile (a fidelity/convergence knob).
2. **cooling epochs** = the `abyssal_epochs()` deep-time snapshots the playback steps through.

They share only the word. This collision cost real confusion in the dialogue ("I asked for 60 epochs and got 6 frames"). Disambiguating the vocabulary (e.g. *erosion-iterations* vs *cooling-snapshots* / *frames*) is part of the harness upgrade, not cosmetic.

## Near-miss recorded (do not re-walk)

It is tempting to read "one `--epochs` number driving both erosion (slow/geological) and water (fast/hydrological)" as a **multirate-law violation** (`#form-scale-separation-directional`: never one shared timestep). **That reading is wrong** — checked. Water does not iterate `epochs` times; its parameter is `erosion_epochs`, i.e. it names *which eroded bed to settle onto* (dependency-by-key, `#form-depend-by-key-never-latest`), and its own step count is the separate hardcoded `200`. The coupling is correct. The defect is **layering**, not multirate. (This is a clean specimen of a structurally-plausible story that the parameter name `erosion_epochs` falsifies on inspection.)

## The actual defect: parameterization has no home

`build` is explicitly **"builder v0"** — a hand-cranked scaffold. Its flags are expedients for infrastructure the segments already design but that isn't built:

- **World-level demand** (target phase, region, fidelity/level) → the CLI *should* express this, and the manifest should pin per-vivium prescription: `#form-manifest-prescribes-vivium` FE(2) (target phase, permits, demand posture). FE(5): live `spec`/roots carry only a "thin subset (name, seed, format)"; the full field set is "designed; partial implementation."
- **Per-nomos convergence params** (erosion iterations, water steps, …) → belong **on each `NomosDecl`**, declared and **folded into the complete key** (`#form-add-system-contract` clause 4; `#form-complete-content-addressed-key`), so they participate in invalidation and are not global CLI integers. Today erosion's is a CLI flag and water's is a hardcoded literal — both wrong-shaped, differently.
- **Demand → materialization** → the **builder daemon** (`#detail-builder-daemon`: beacons, demand spool, cones) is the designed owner. `build`-with-flags is the stand-in until it exists.

So `--epochs` on the global command is **compliance debt against `#form-manifest-prescribes-vivium`**: subsystem tuning at the world-build seam because the manifest+demand layer that should own it is unbuilt. Joseph, in effect, rediscovered *why the builder daemon is the highest-leverage unbuilt runtime piece*.

## Work items (un-conflated)

1. **[small, standalone] Playback frame density.** Make `abyssal_epochs()` produce N snapshots along the same 3.6→2.6 Ga curve from a demand parameter. **Design care:** sample **nested** (the N-set contains the current 6 Ga values) so existing epoch-reduction memos stay valid instead of all re-computing — each snapshot's T_p is in its store key, so non-nested densification silently invalidates the lot. This is the "60 frames" Joseph asked for; it is not `--epochs`.
2. **[principled] Move convergence params off the global CLI.** Erosion iterations and water steps become `NomosDecl` fields, folded into their keys; the manifest carries per-vivium fidelity/target-phase; `build` takes **world-level demand**, not subsystem knobs. A real step toward the daemon.
3. **[cheap, high-value] Disambiguate "epochs".** Rename so erosion-iterations and cooling-snapshots/frames never again share a word across the CLI, `mantle_thermal`, and the build log.
4. **[watch] Water's `steps = 200`** is a hardcoded literal in the CLI call site — same class as erosion's flag (a nomos param living in the wrong place), just failing quietly instead of loudly. Move it with (2).

## Epistemic status

Findings from a dialogue; code facts verified 2026-07-24 at the lines cited. The target shape is the segments' own architecture (manifest / add-system-contract / complete-key / builder-daemon), not a new proposal — this note only observes that the CLI does not yet conform and names the conforming shape. No claim segment changes here; if a harness upgrade lands, its parameter-home decisions may warrant a DECISIONS row and possibly a `build`/manifest ops segment. Companion: `DECISIONS[build-cli-parameterization-is-manifest-and-nomos-debt]`.
