# vivarium

*An enclosed living world kept for observation.*

Deterministic constructed worlds (cube-sphere planet) for play and for *in vivia* study — ground truth by construction for quantities ASF/AAT needs. Member of [Archema](../README.md).

## Source of truth

**Claim segments** under [`core/src/`](core/src/), ordered by [`core/OUTLINE.md`](core/OUTLINE.md), written under [`FORMAT.md`](FORMAT.md). The slug is the identity. Agents and humans: start at [`CLAUDE.md`](CLAUDE.md).

| Authority | File |
|---|---|
| Project claims | `core/src/*.md` (ordered by [`core/OUTLINE.md`](core/OUTLINE.md)) |
| Term dictionary | [`LEXICON.udon`](LEXICON.udon) |
| Who decided | [`DECISIONS.decision-log.udon`](DECISIONS.decision-log.udon) |
| Ethics / moratorium | [`ETHICS.md`](ETHICS.md) · `#scope-moratorium-endogenous-emergence` |
| Assumptions ledger | [`ASSUMPTIONS.md`](ASSUMPTIONS.md) |
| ASF reading gates | `#scope-asf-reading-gates` (Level C hard-gates agent-seam work) |

[`ASF.md`](ASF.md) is a thin historical router only — **not** claim canon; do not cite its old section numbers as law.

Big-picture residual / ice vs segment intuition (not law): [`CONSOLIDATION-STATUS.md`](CONSOLIDATION-STATUS.md).

## Run (instruments, not canon)

**Install once — then they are just commands, from any directory:**

```bash
bin/install          # vivarium, vivarium-globe, vivarium-worldview → ~/.cargo/bin
                     # (bin/install vivarium for just the CLI — no bevy, seconds)
```

```bash
vivarium build &                 # build in the background
vivarium-globe                   # store-backed planet (observe-only); T = deep-time playback
vivarium-worldview               # first-person store surface (observe-only)
vivarium status                  # what exists, at what fidelity
vivarium -h                      # everything else
```

Re-run `bin/install` after changing the code. Working *on* the CLI, `bin/vivarium <cmd>` runs it straight from the tree without installing (`VIVARIUM_DEBUG=1` for a debug build), and `cargo run -p vivarium-world --bin vivarium -- <cmd>` is the long form older notes use.

```bash
cargo test -p vivarium-world --lib
```

**Which world?** Every command prints the world directory it resolved and *why* — an explicit argument, `$VIVARIUM_WORLD`, or the shared default `~/.cache/vivarium/globe-world` (the same world the globe and worldview open). The directory argument is optional everywhere; the announce line is what keeps "optional" from meaning "unknowable."

**What is it building?** The **manifest** carries this vivium's demand — `order`, `target_phase`, `level`, `frames`, `erosion_epochs`, `water_steps` ( #form-manifest-prescribes-vivium FE(2) ). `vivarium demand` shows it; `vivarium demand frames=60 level=9` sets it; build flags set it too and stick. Demand is never folded into a key, so changing it invalidates nothing and every view — the globe included — sees the same numbers.

**Build a world, and watch it happen:**

```bash
export VIVARIUM_WORLD="${VIVARIUM_WORLD:-$HOME/.cache/vivarium/globe-world}"
vivarium new "$VIVARIUM_WORLD" first-light
# emerged-land flux is still unmet — waive for provisional materialization:
vivarium build --level 6 --epochs 20 --allow-unmet
vivarium status                   # pyramid + provisional column + this world's demand
vivarium-globe                     # spin the built surface
```

**Watch it build, or watch it again** — one reader, two ends ( #form-time-indexed-stage-chains FE(5) ). Run the build in one terminal and this in another:

```bash
vivarium watch                # follows a running builder; globe repaints as roots land
vivarium watch --replay       # walks the store's landing history instead
```

Replay orders by root **landing** time — build history, not world-time, because root files carry no world-time. The reader says so every run, and its interior column counts how much world-time interior exists to replay at all.

Views load builder `erosion-tile` roots via store census — they do not cold-run fluvial evolution.

## Standing law

No endogenous frontier / emergence-capable minds in a vivium. See `ETHICS.md` §0 and `#scope-moratorium-endogenous-emergence`.
