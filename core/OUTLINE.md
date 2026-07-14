# Vivarium Methodology Core
## *(Instructions)*

The specification of what vivarium is and must do, written as **segments**, one per file in `core/src/`, each stating one thing that is defined, follows, or is restricted, and so forth. This outline organizes them and gives them a logical ordering generally, although there is not as much of an intrinsic dependency chain as in ASF volumes. Nevertheless, **this file carries the ordering; the slug carries the identity; [`FORMAT`](../FORMAT.md) carries the rules.**

> [!danger] **Read [`FORMAT`](../FORMAT.md)** before writing or reviewing a segment
>
> It holds the frontmatter (`type` / `status` / `stage` / `depends`), the promotion gates, the epistemic triage, and the document cadence — and, in its second half, the cross-reference scheme, the prose voice, and the math rules, **which bind every file in this repository and not only segments**.

This methodology assumes prior exposure to and understanding of [CLAUDE](../CLAUDE.md), [README](../README.md), the [LEXICON](../LEXICON.udon), and [ETHICS](../ETHICS.md). The lexicon in particular will be referenced throughout the segments. While the outline here and the segments (done and proposed) may start to form a bit of a narrative, work to keep them as granular as possible with regular audits and checks for flow and cross-segment coherence etc.

### Aspect

*(We'll move this to FORMAT once it solidifies a bit further. Some of these listed aspects are so likely to overlap every time that they may turn out to not be very useful. In other words, it's probably not close to being MECE yet or even useful, but it's a start and it is helping me get this to diverge from ASF just enough to work for Vivarium)*

| Aspect       | Description                                                                                                 |
| ------------ | ----------------------------------------------------------------------------------------------------------- |
| Math         | Mathematical findings & tools                                                                               |
| Physics      | Physical sciences; the natural world & universe's known behavior                                            |
| Model        | Algorithm & low-fidelity approximation models, constraints, & tradeoffs                                     |
| Machine      | Algorithmic implementation-concerns that optimize performance for specific architectures & coding realities |
| Project      | High-level project concerns -- especially conceptual and principles                                         |
| Ops          | SOPs & conventions adopted for regularity & predictability & not necessarily for underlying intrinsics      |
| Critical (?) | Provisional additional flag: Anything that is non-negotiable for this project                               |

## *(Transitional)* Existing corpus

As the segments for this outline are only beginning to be fleshed out, the question arises: where are the first 20 days worth of findings and process and progress? It does not live in one place, and that is the problem this directory exists to solve. It is distributed across the corpus — `doc/`, `ref/`, `LEXICON.udon`, `DECISIONS.decision-log.udon`, `tabularium/` and tons of stuff in archival now— and no single artifact contains it. The front doors that used to summarize that corpus are in `.archive/`, deliberately: Whenever possible, the assembly is to be made from the direct sources, as the summaries have important points that exist nowhere else but are also almost all incomplete and non-authoritative.

## I. Guiding Principles, Expectations, & Obligations
*(All of these sections and tables are very likely to change. We need to start somewhere. This outline is what makes it easy to reorder and organize as it evolves while keeping each segment self-contained and independently auditable given its dependencies.)*

| Type       | Aspect           | Tag                                                    | Claim or Topic                                          | Stage   |
| ---------- | ---------------- | ------------------------------------------------------ | ------------------------------------------------------- | ------- |
| Discussion | Project          | [#disc-vivarium-history](src/disc-vivarium-history.md) | Vivarium timeline & background                          | missing |
|            |                  |                                                        | (#gap) Epistemology                                     |         |
|            |                  |                                                        | (#gap) Voice & Conduct Discipline                       |         |
|            |                  |                                                        | (#gap) General Methodology Principles (one per segment) |         |
| Discussion | Project Critical | [#ethics](../ETHICS.md)                                | Crucial moratorium on endo-logogenic participation etc. | drafted |
|            |                  |                                                        | (#gap) ...                                              |         |

## II. Principle Concepts & Project Layout


## III. Runtime, Environment, & CLI

*(Including world generation vs exploration vs inhabitation vs participation decouplings)*

## IV. Kingdoms, Orders, and Ordinum


## V. Physical, Mathematical, & Algorithmic Toolsets

*(Including:  The "grid", FVM, flux-web, multiscale, coarse-fine-coarse-fine traversal rhythms, etc.)*

## VI. Active Contribution & Development

*(Although I think a lot of the more critical things are covered above... but this could be the remaining procedural or SOP-type conventions that aren't necessarily principles so much as consistency checks and workflow / role descriptions etc...)*

## VII. Tabularium & Existing Phases & Nomos

### **Terrestris Ordinum** - Our current best effort earth-like world engine

***(Right now these are essentially identical to the phases in the [Terrestris Ordinum](../tabularium/terrestris.ordinum.udon). This is crucial because a lot of the effort is going into fleshing out these nomos and the findings need an organized home. If anything, this outline is the authoritative launchpad and current canon state, and it will directly link to and validate from here...)***

#### 1. **Ante-Mundane**

#### 2. **Protogenic**

#### 3. **Primordial**, e.g., Water-Cycle & Atmospheric systems

#### 4. **Abyssal**, e.g., Geological-intensive systems

#### 5. **Primeval**, e.g., Early-life, Evolution, & Ecological systems

#### 6. **Archaic**, e.g., Complex-life, Ecology, & **Primitive Endogenous Agents**

*(Also a potential initial target for Exogenous participation -- i.e., a target state for interventionary simulation)*

#### 7. **Aboriginal**, e.g., Language & **Complex Endogenous Agents**

#### 8. **Prehistoric**, e.g., **Hominid** & Holocene-like dynamics

#### 9. **Modern**, e.g., Writing & Game-world-like Targets

