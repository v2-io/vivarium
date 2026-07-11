# tabularium — the library of instituted artifacts

*The Roman **tabularium** was the state archive where the bronze tablets of law were kept. Here it holds vivarium's **structured-but-not-Rust** artifacts — the ones that are more than prose and less than code: law-data with skeletal structure and conformance, authored in **udon** and consumed by Rust through libudon. Established 2026-07-11 with its first tablet, the Tellus ordinum.*

## What lives here

Artifacts whose content is *data the machine reads*, not narrative and not implementation:

- **ordina** (`*.ordinum.udon`) — codified **phase floors**: per-phase charges, promises, and defeasances for a world-*kind*. The reportatio (prose working text) each was compiled from lives in `doc/` or `.archive/`; the pinned `:reportatio` / `:reportatio-pin` records the exact source. *Present:* [`tellus.ordinum.udon`](tellus.ordinum.udon) — the Earth-world-kind (`:manifold cube-sphere-3d-voxel`).
- **regulae** (`*.regula.udon`) — world-level **conformance profiles**: which slots at what minimum rigor, which absences are permitted, the epistemological posture. Each pins an `ordinum@version`. *Coming:* `terrestris.regula.udon` (Regula Terrestris pins Tellus), spec at [`../doc/plan/regula-conformance-design.md`](../doc/plan/regula-conformance-design.md).
- and, over time, the other structured law-data the design calls for (declarations, slot registries, …) — each a single-root udon document.

The **lexicon** (`../LEXICON.udon`) and the **process norms** (`../doc/PROCESS.udon`) are also udon, but they are project-governance front-doors, so they stay at their load-bearing locations rather than in the archive of world-law.

## Conventions

- **Filename = `<name>.<root-element-type>.udon`.** The root element type is the schema, so the file self-describes and tools filter by it (`ls *.ordinum.udon`). The schema-suffix makes per-type subdirectories redundant — the tabularium is **flat**.
- **Version lives in `:version`, not the filename.** Git versions the file; a version-in-filename is minted only when a *fork* forces two lineages to coexist.
- **One ordinum, many regulae.** An ordinum is a phase floor for a world-*kind* (Tellus is Earth-lineage; a cellular-automata / 2-D testbed is the anticipated second). Its *schema* is world-kind-agnostic; only its *content* is domain-specific. Multiple regulae may pin the same ordinum at different rigor/target-phase.
- **Rust consumes these via libudon** (`~/src/udon/core`), so the structure is a real interface, not decoration — validate any edit with `cargo run --example stdin_parse < <file>` from the udon core until a udon-cli lints it. Full norms: [`../doc/PROCESS.udon`](../doc/PROCESS.udon) (`udon-for-structure`, `udon-safe-subset`, `udon-filenames`). Vocabulary: [`../LEXICON.udon`](../LEXICON.udon).
