# vivarium

*An enclosed living world kept for observation.*

Deterministic constructed worlds (cube-sphere planet) for play and for *in vivia* study — ground truth by construction for quantities ASF/AAT needs. Member of [Archema](../README.md).

## Source of truth

**Claim segments** under [`core/src/`](core/src/), ordered by [`core/OUTLINE.md`](core/OUTLINE.md), written under [`FORMAT.md`](FORMAT.md). The slug is the identity. Agents and humans: start at [`CLAUDE.md`](CLAUDE.md).

| Authority | File |
|---|---|
| Project claims | `core/src/*.md` |
| Term dictionary | [`LEXICON.udon`](LEXICON.udon) |
| Who decided | [`DECISIONS.decision-log.udon`](DECISIONS.decision-log.udon) |
| Ethics / moratorium | [`ETHICS.md`](ETHICS.md) |
| Assumptions ledger | [`ASSUMPTIONS.md`](ASSUMPTIONS.md) |

## Run (instruments, not canon)

```bash
cargo test -p vivarium-world --lib
cargo run -p vivarium-world --bin vivarium -- status
cargo run -p vivarium-world --bin vivarium -- info --width 80
cargo run --release -p vivarium-globe   # store-backed planet view
```

Default world dir: `$VIVARIUM_WORLD` or `~/.cache/vivarium/globe-world`.

## Standing law

No endogenous frontier / emergence-capable minds in a vivium. See `ETHICS.md` §0 and `#scope-moratorium-endogenous-emergence`.
