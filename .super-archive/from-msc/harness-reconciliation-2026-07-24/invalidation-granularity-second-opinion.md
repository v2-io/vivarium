# Invalidation granularity — second opinion

*Peer review of the proposal to replace the whole-crate source digest with per-nomos source attribution derived from `NomosDecl::deps`. Written 2026-07-24 for the agent deciding today. Everything numeric here was run against the tree at `eb6710e`; every command is listed in §8 so you can re-run rather than trust.*

**Bottom line.** The sketch does not survive. It under-keys today, live, in at least three places — I demonstrated one by running it. But the reason it dies is not the reason the implementing session gave, and the more useful result is what I found when I tried to strengthen it instead: I built the strongest *safe* version (attribution derived from the actual module reference graph, not from `deps`) and measured what it would buy. Over the last 39 commits it would have spared **zero** commits entirely and given partial relief on 10; on the phase-cost weights it saves nothing at all when you edit `sea_level.rs`, which is the third most-edited file in the crate. The granularity of the key is bounded by the granularity of the code, and this code is not partitioned by nomos.

There is a third option, and it is aimed squarely at the measurement that started this: **19% of source-touching commits in the last ten days could not have changed one computed byte** — six were doc-comment-only, one was CLI-only. Those are free to reclaim with no attribution map, no dependency graph, and no under-key exposure. That is §5.

---

## 1. What the sketch does when you actually run it

The proposal keys each nomos by a digest over its own module closed over its declared `deps` cone. I reimplemented `source_hash::digest_files` in Python and checked it against the live `VIVARIUM_SRC_HASH` — it reproduces `b70379b7faf18a5f` exactly, so scoped digests below are computed by the same algorithm the build uses.

**The demonstration.** `query.rs`'s `epoch_reduction_key` mints its stem from `MANTLE_THERMAL`. `MANTLE_THERMAL.deps` is empty. So the sketch's cone for that key is the single file `mantle_thermal.rs`. The key's own doc comment says why that is currently safe: *"Completeness is guaranteed by `src=SRC_HASH` on the stem (the whole-crate source digest folds every module the pour/ledger touch — `sea_level`, `erosion_return`, `gen` bathymetry, hydrosphere)."*

I edited one constant in `sea_level.rs` (`SAMPLE_LEVEL` 8 → 7 — the resolution the pour samples at, a parameter of exactly the kind this workflow iterates on), rebuilt, and re-ran the builder on the same world:

| $T_p$ (°C) | `derived_sea_m` before | after | Δ (m) |
|---|---|---|---|
| 1464.16 | 5011.783 | 5011.623 | −0.161 |
| 1487.62 | 5070.672 | 5070.518 | −0.154 |
| 1515.90 | 5135.311 | 5135.416 | +0.104 |
| 1550.00 | 5181.992 | 5182.087 | +0.096 |
| 1591.11 | 5194.132 | 5194.359 | +0.227 |
| 1640.66 | 5195.196 | 5195.285 | +0.088 |

Whole-crate digest: `b70379b7faf18a5f` → `8dfcc6764b370cc4`. Scoped digest over `{mantle_thermal.rs}`: `343b64d8ffdadef7` → `343b64d8ffdadef7`, **unchanged**.

So under the sketch every one of those six roots would have been a Hit, and the world would have carried the old sea level at every epoch, silently, with a key that says it is current. That is the FE(2) unsafe direction, on the file that has been edited ten times in ten days.

**Two more, without running them.** They follow from the same measurement, and the second one may matter more than the first:

- `climate.rs` calls `crate::noise::fbm` (line 72, inside `precip_jitter_factor`, which `query.rs` line 298 uses to build every stored climate tile). `CLIMATE.deps` is `[&HYDROSPHERE]`. Noise is nowhere in the cone. Edit `noise.rs`, and under the sketch every climate tile serves a stale value.
- `query.rs` belongs to no nomos's module set at all, so under the sketch editing it invalidates nothing — and `query.rs` is where every tile is actually computed and every memo is actually put. It is the second most-edited file in the crate.

**Why this happens, stated as the corpus already states it.** `#form-nomotheke-registry` Known-incomplete (2) is *"Under-declaration of non-flux deps"*, with the example *"uplift kernel uses fated noise with empty `deps` — not caught by consumed⇒in-deps."* That residual is open. `deps` is a declaration of *flux-level* coupling that the `consumed⇒in-deps` test enforces only where a consumed quantity has a registered producer; it was never a claim about which source files a nomos's computation reads. The sketch treats it as one.

So on your framing question — is *hand-maintained* versus *derivable* a hair you are splitting to keep the hypothesis alive? The distinction is real and I would defend it. It does not rescue this sketch, because `deps` is *itself* hand-maintained, and core already names it under-covering. Deriving from it inherits the gap and converts it from a declaration defect into silent corruption.

## 2. So I built the strong version instead

The honest steelman is: derive attribution from the module reference graph the compiler actually enforces, not from `deps`. I extracted every `crate::<module>` reference from the crate's 33 source files with comments stripped, and took transitive closures. Under that scheme, editing module $m$ invalidates nomos $N$ exactly when $m$ is in $N$'s code closure — no hand map anywhere.

Closure sizes, out of 33 modules:

| nomos | module | closure |
|---|---|---|
| `noise` | `noise.rs` | 1 |
| `uplift-tile` | `uplift.rs` | 3 |
| `lithosphere` / `isostasy` | `lithosphere.rs` (shared) | 3 |
| `planet` | `planet.rs` | 4 |
| `mantle-thermal` | `mantle_thermal.rs` | 5 |
| `hydrosphere` | `hydrosphere.rs` | 5 |
| `climate` | `climate.rs` | 7 |
| `initial-topography` | `gen.rs` | **13** |
| `water-tile` | `water.rs` | **14** |
| `erosion-tile` | `erosion.rs` | **17** |
| (orchestrator) | `query.rs` | **26** |

The fBm prior — the thing furthest upstream in the declared graph — has a code closure of thirteen modules including the entire mantle/lithosphere/sea-level chain, because `gen.rs` calls `crate::sea_level::tectonic_surface_m` and `derived_sea_level_m`. In the code, `initial-topography` is not upstream of anything.

Two module-level cycles fall out, and both are informative rather than incidental:

- `{erosion_return, gen, sea_level}`
- `{audit, nomotheke, store}`

A cycle is not a bug by itself, but it collapses attribution (the three members can never be told apart) and it forecloses the compiler-enforced version of this idea, §6.

## 3. What the strong version would have bought

Simulated over the 39 commits since 2026-07-14, treating `{query, nomotheke, store, lib, source_hash}` as always-invalidating — which they must be, since they mint the keys and drive every compute:

- **22 commits (56%)** invalidate all eleven nomos anyway.
- **10 commits** invalidate a strict subset.
- **7 commits** invalidate nothing — but all seven touched only `examples/`, which `build.rs` already excludes. They are free today.

**Zero commits move from "whole store" to "nothing."** The relief is entirely in the middle: four commits would have been confined to `erosion-tile` alone, three to the `{initial-topography, erosion-tile, water-tile}` triple.

Weight that by measured cost. A cold whole-world build at L7 (24 tiles) is 4.15 s, split:

| phase | time | share |
|---|---|---|
| initial-topography | 238.7 ms | 6% |
| erosion | 758.3 ms | 18% |
| water | 1.6 s | 39% |
| epoch reductions | 1.5 s | 36% |

Applying the closures:

| you edit… | ideal derived attribution recomputes | saved |
|---|---|---|
| `erosion.rs` | erosion + water | 43% |
| `water.rs` | water | 61% |
| `sea_level.rs` | everything | **0%** |
| `lithosphere.rs` | everything | **0%** |
| `gen.rs` | topography + erosion + water | 36% |
| `query.rs`, `nomotheke.rs` | everything | **0%** |
| `craton_morphology.rs`, `globe.rs`, `audit.rs`, `ordinum.rs`, `spec.rs`, `flux.rs` | nothing | 100% |

Ten-day churn, for reading that table against reality: `nomotheke.rs` 16, `query.rs` 11, `sea_level.rs` 10, `lib.rs` 7, `flux.rs` 7, `bin/vivarium.rs` 6, `store.rs` 6, `erosion.rs` 6, `audit.rs` 5, `lithosphere.rs` 4.

The last row of the cost table is the only clean win, and it is where your motivating measurement landed: `craton_morphology.rs` is reached by no nomos, so a comment there would indeed cost nothing. But `craton_morphology.rs` was edited once in ten days, and it never appeared in a commit by itself.

## 4. The finding underneath the numbers

Attribution granularity cannot exceed code granularity, and the code is not partitioned by nomos. Concretely: `nomotheke.rs` is 1138 lines holding all eleven declarations, `query.rs` is 971 lines holding every memoization path, `sea_level.rs` is 811 lines holding a pour that four nomos read through. `lithosphere` and `isostasy` share one file and can never be separated at file grain. `initial-topography` lives in `gen.rs`, which the closure shows is not a leaf.

That is the honest form of the ceiling core named. *"The ceiling waits until the module graph makes attribution derivable"* is satisfiable in the weak sense today — I derived it in §2 and it is safe — and it does not pay, because the module graph is not shaped like the nomos graph. The unblock condition as written is about derivability; the condition that actually governs the payoff is **separation**, and it is a property of the code layout, not of the keying scheme.

This is worth putting in the segment either way, because it is a stronger and more checkable statement than what is there now, and it tells a future session what to measure before trying again.

## 5. The third option: two narrowings that need no attribution at all

Both of these are safe because of what the *compiler* guarantees, not because of what a map declares. Neither touches `deps`, neither needs a module graph, and neither can under-key by getting an attribution wrong.

**(a) Exclude `src/bin/` from the library's digest.** `build.rs` walks `src/` recursively, so `src/bin/vivarium.rs` is folded into `VIVARIUM_SRC_HASH`. But `src/bin/*.rs` are separate binary targets that depend on the lib; the lib cannot reference them, and Cargo enforces that. Nothing in `src/bin/` can change any nomos's output. This is a one-line path filter in `collect_rs`, and it is derivable in the strongest available sense — Cargo's target model, not a parse. `bin/vivarium.rs` was edited six times in ten days.

**(b) Normalize comments and whitespace out of the digest.** Six of 36 source-touching commits (17%) changed only doc comments and blank lines. All six were cross-reference repointing during the `core/` consolidation — `doc/design/DESIGN-MATERIAL.md` → `.super-archive/from-design/DESIGN-MATERIAL.md`, `doc/plan/regula-conformance-design.md` → `#detail-regula-design`, and so on. Each one invalidated the entire store. **38% of all changed source lines in that window were comment or blank lines**, which is what you would expect from a codebase that carries its claim references in module docs — a good practice that this digest taxes.

Together: **7 of 36 source-touching commits (19%) would have cost nothing.**

The correctness surface for (b) is the lexer, not the architecture. A naive strip corrupts on a string literal containing `//`, or on raw strings — which is a real hazard and the reason to do it at token level rather than by regex. It is also directly convictable by a test in the shape `source_hash` already uses: digest a fixture, digest the same fixture with comments added and whitespace shifted, assert equality; digest it with one token changed, assert inequality. That is the same conviction standard as `one_byte_of_content_flips_the_digest`.

`#form-complete-content-addressed-key` FE(4) already reserves room for this: *"Optional IR-normalized hashing and finer per-nomos attribution are non-dogmatic tightening."* Comment normalization is the cheap end of that clause and it is the half that does not require attribution.

**Honest limit on both.** They fix the papercut you measured; they do not deliver the workflow. Thirty of 36 commits contained real code changes and would still invalidate the store. If the goal is "iterate on an algorithm with the world staying warm," (a) and (b) do not get you there. What they do get you is that documentation work, cross-reference repair, and CLI work stop costing a full rebuild — and given how much of this project's activity is exactly that, I would take them regardless of what happens to the granularity question.

## 6. If you want separation, the compiler can enforce it — and that is a code change, not a key change

The version of your idea that cannot silently under-key is a **workspace crate split**. Cargo will not let crate A use crate B's items unless `Cargo.toml` declares the dependency, so `H^*(C) = \mathrm{hash}(\mathrm{src}(C)) \oplus \bigoplus_{D \in \mathrm{deps}(C)} H^*(D)$ is attribution derived from a graph the compiler refuses to let you get wrong. This is what the original plan's *"the Nix move"* actually is: Nix derivations are per-package, and a crate is the natural package.

It is real work, and the §2 cycles are the first bill: `{erosion_return, gen, sea_level}` cannot become three crates while it is a cycle. I would treat that as information rather than obstruction — the cycle exists because the surface prior calls the sea-level pour, which is a design fact worth looking at on its own merits.

I am not recommending this now. I am naming it because it is the only version of "finer attribution" I found that meets the never-under-key bar without a hand-checked static analysis, and because if the answer to §4's separation problem is ever pursued, this is the shape it takes.

## 7. Two levers that are not the key, and one reframe

**The byte cost is an eviction problem, not a key problem.** `#form-store-as-save` FE(5) says invalidation is correctness and eviction is space, and its Known-incomplete (2) names *"no GC, no full manifest, no run-mode enforcement yet"*. Your 28 MB orphaned at L9 and >1 GB at L12 is entirely that gap. `Query::eroded_region_census` already partitions roots into fresh and stale by comparing the `src=` field against the running binary's hash, so the selection logic for a GC already exists and is already tested. Deleting stale-`src` regenerable roots is never a correctness event under FE(5). This is small, safe, and removes half the complaint without touching a key.

**The time cost extrapolation assumes a build mode the architecture is retiring.** L12 whole-world is 24,576 tiles at 64², about 100 M cells; your ~40 min is roughly right for that. But that number is builder v0's *degenerate beacon* — the CLI's own module doc calls the whole-world sweep exactly that, and names the next increment: *"no demand spool yet (explorers file demand in the next increment)."* `#detail-builder-daemon` FE(2) has the builder owning a demand frontier of beacons → causal cones → work queue, and FE(6) puts "demand spool + read-only query" in the next slice. Under demand-driven building, the cost of an edit is the cone you are looking at, not the world — at any level. Your stated workflow is *"background building toward points of interest, restarting at the right points, watching progress live."* That is a description of the demand spool. It is already planned, it is already the named next increment, and it makes the invalidation-granularity question much less load-bearing than it looks from inside a whole-world sweep.

I think this is the part most worth your attention, and I want to be clear that it is a judgment and not a measurement: I believe the granularity question is a proxy, and the thing actually blocking the workflow is that the builder sweeps breadth-first over the whole world instead of toward demand.

## 8. On BREAK-1 — orthogonal, not the alternative

`#detail-vivium-lifecycle` BREAK-1 says phases exist as memoization and immutability boundaries *"else law-iteration cost is $\Omega(\mathrm{world})$ per edit."* That addresses a different problem than the one you measured. Freezing a phase pins its output as a content-addressed input so downstream stops re-deriving it from law. It does not stop a stray edit invalidating the downstream memos, because those keys still fold `SRC_HASH` for their own code.

So the two are not alternatives to weigh against each other. Freezing cuts the *cone*; attribution cuts the *breadth*. If you take the §5 narrowings and the demand spool, freezing remains worth doing later for its own reasons and does not compete with anything here.

## 9. What I did not establish, and what would change my mind

- I measured commit granularity, not iteration granularity. Tight iteration on one kernel between commits is more attributable than the commit record shows, and my §3 numbers understate the win by an amount I cannot quantify from git. If you have a session where you edited `erosion.rs` twenty times in an afternoon, that is evidence I do not have.
- My module graph comes from a regex over `crate::` paths with comments stripped. It will miss a dependency expressed only through a re-export, a trait method resolved without naming its module, or a macro. Every such miss makes the closures I reported *too small*, which means §3's payoff numbers are upper bounds — the real payoff is smaller, not larger. That direction is convenient for my conclusion, which is a reason to be suspicious of it, so: the specific way to check is `cargo modules` or a `rustc --emit=dep-info` pass, neither of which I ran.
- I did not test whether comment normalization is implementable cleanly against Rust's raw-string and nested-block-comment grammar. I believe it is; I did not write it.
- What would change my mind on the whole thing: a measurement showing that real iteration sessions concentrate edits in `erosion.rs` or `water.rs` alone. Those two are the only large-cost, small-closure modules in the crate, and attribution pays for them and almost nowhere else.

## 10. What I would do

1. Take §5(a) and §5(b). Small, compiler-safe, convictable by a test in the shape `source_hash` already uses, and they reclaim 19% of recent commits including the exact class of edit that produced your measurement.
2. Take §7's stale-`src` GC. Small, and it retires the orphaned-bytes half of the problem outright.
3. Do not build per-nomos attribution. Record the no-go in `#form-complete-content-addressed-key` Known-incomplete (3) — but sharpen it rather than restating it, because the current text names derivability as the unblock condition, and the measurement says derivability is not the binding constraint. Separation of code by nomos is. That is a better signpost for whoever picks this up.
4. Treat the demand spool as the thing that actually serves the workflow, and let §7 rather than this document set that priority.

## 11. Feedback on the brief

The brief was unusually good to receive: the leaning was stated plainly enough to attack, the failure modes you already suspected were listed, and the "a no-go with a demonstration is a real result" framing meant I could spend the effort on running the counterexample rather than on hedging it. The reproduction recipe let me confirm your baseline in two minutes, which is exactly what it was for.

One thing I would have wanted earlier, and it is the thing your follow-up supplied: the transcript channel. I would probably have gone to `#form-nomotheke-registry` Known-incomplete (2) faster if I had known the *record* of the deliberation was reachable, because that residual is the whole argument and it is one line in a segment I might not have opened.

Where I think the question was framed slightly too narrowly: it was posed as a choice between two keying schemes, with BREAK-1 as the fallback. All three are about the key or the memo boundary. The measurement that ended up mattering most — 19% of commits could not change a byte, and 56% would invalidate everything under any scheme — only came into view once I stopped asking "which scheme" and started asking "what do the edits actually look like." That question is cheap and I would ask it first next time.

## Appendix — commands run

Working tree was clean at start and is clean now; the only file I edited under `crates/` was `crates/vivarium-world/src/sea_level.rs`, restored byte-identical (verified: the whole-crate digest is back to `b70379b7faf18a5f`). Scratch worlds were created under the session scratchpad, never in `~/.cache/vivarium/`.

```text
cargo build --release -p vivarium-world --bins
./target/release/vivarium new  <scratch>/w1 probe
./target/release/vivarium build <scratch>/w1 --level 7 --epochs 40     # cold: 4.15s
./target/release/vivarium build <scratch>/w1 --level 7 --epochs 40     # warm: ~0ms
# then: SAMPLE_LEVEL 8 -> 7 in sea_level.rs; rebuild; rebuild world; compare roots
# then: cp back, rebuild, confirm digest == b70379b7faf18a5f
```

Analysis scripts (Python, read-only over the tree and `git log`): FNV-1a `digest_files` reimplementation checked against the live `VIVARIUM_SRC_HASH`; `crate::` module-reference graph with comments stripped, transitive closures, Tarjan SCCs; per-commit invalidation simulation over `git log --since=2026-07-14`; comment-only and `src/bin`-only commit classification by `git show --unified=0`. None of them wrote to the repository.

I am staying on the line — send follow-ups. The two I would most like to be asked: whether the §2 module graph holds up against `cargo`'s own dependency information, and whether there is an iteration session in the transcripts that contradicts §9's first bullet.
