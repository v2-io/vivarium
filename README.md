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

```bash
cargo test -p vivarium-world --lib
cargo run -p vivarium-world --bin vivarium -- status
cargo run -p vivarium-world --bin vivarium -- info --width 80
cargo run --release -p vivarium-globe      # store-backed planet (observe-only)
cargo run --release -p vivarium-worldview  # first-person store surface (observe-only)
```

**See a built world (builder + views share one store):**

```bash
export VIVARIUM_WORLD="${VIVARIUM_WORLD:-$HOME/.cache/vivarium/globe-world}"
cargo run -p vivarium-world --bin vivarium -- new "$VIVARIUM_WORLD" first-light
# emerged-land flux is still unmet — waive for provisional materialization:
cargo run -p vivarium-world --bin vivarium -- build "$VIVARIUM_WORLD" --level 6 --epochs 20 --allow-unmet
cargo run -p vivarium-world --bin vivarium -- status "$VIVARIUM_WORLD"   # pyramid + provisional column
cargo run --release -p vivarium-globe                                    # spin the built surface
# optional: first-person over the same store (no VIVARIUM_ALLOW_VIEW_EVOLUTION)
cargo run --release -p vivarium-worldview
```

Default world dir: `$VIVARIUM_WORLD` or `~/.cache/vivarium/globe-world`. Views load builder `erosion-tile` roots via store census — they do not cold-run fluvial evolution.

## Standing law

No endogenous frontier / emergence-capable minds in a vivium. See `ETHICS.md` §0 and `#scope-moratorium-endogenous-emergence`.
