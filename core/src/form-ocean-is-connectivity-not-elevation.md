---
slug: form-ocean-is-connectivity-not-elevation
type: formulation
status: conditional
stage: draft
depends:
  - form-derived-sea-level
  - form-declared-boundary-contract
  - obs-connectivity-fills-the-basins-the-threshold-drained
  - norm-probes-before-claims
---

# Being under the datum makes a cell submerged; the ocean has to reach it to make it sea

Sea level is a height, and "the sea" is not the set of cells below it. A cell is **submerged** when its bed lies under the derived waterline ( #form-derived-sea-level ); it is **ocean** only when the ocean can get there. The two are different sets, and the difference is exactly the world's endorheic basins — every closed depression whose floor dips below the datum.

The distinction is load-bearing because *ocean* is a **drain**. Routing seeds its Priority-Flood from the ocean set, so a cell mislabelled sea is told it is already an outlet: it holds no water, and no downstream repair can give it any. A settle of any length, a halo of any depth, a volume-limited fill and a full water balance all operate below this classification and are powerless against an error in it.

## Formal Expression

1. **The law.** Given the derived waterline $z_\text{sea}$ ( #form-derived-sea-level ) and a domain $\Omega$, let $S = \{i \in \Omega : z_i \le z_\text{sea}\}$ be the **submerged** set. The **ocean** is the subset of $S$ connected to the domain boundary through $S$:
   $$O = \{i \in S : \exists \text{ a path } i \to \partial\Omega \text{ within } S\}.$$
   Outlets are $O$ together with whatever the boundary contract declares ( #form-declared-boundary-contract ). The complement $S \setminus O$ is **standing water that is not sea** — landlocked, and free to hold a level surface at its own spill point.

2. **Why the boundary is the right seed.** Past $\partial\Omega$ lies the rest of a planet that is ~95% submerged ( #obs-connectivity-fills-the-basins-the-threshold-drained FE(4)), so a submerged region reaching the edge of any domain is continuous with the world ocean. Nothing is decreed to *be* the ocean: the ocean is where the water already is, and connectivity is computed. Prior art frames it identically — the ocean as *"a designated sink region or the map edge"* (Barnes, Callaghan & Wickert 2021, Fill–Spill–Merge).

3. **Connectivity is eight-connected**, matching the neighbourhood the flow router itself uses. A strait one diagonal step wide is one water can cross, so it is one this agrees is open. The mask is a boolean reachability set and is therefore independent of visit order — determinism costs nothing here.

4. **Declared scope: enclosure is only as real as the domain is wide.** An enclosed sea larger than $\Omega$ touches $\partial\Omega$ and is read as ocean. A tile-local reader is therefore biased toward *calling lakes sea*, in the same direction and for the same structural reason that a tile-local outlet ring is biased toward draining basins ( #obs-tile-outlets-grade-away-the-basins ). A **whole cube face is the domain that adjudicates the planet's real basins**; smaller windows are honest only about basins smaller than themselves.

5. **What this does not settle.** Whether a landlocked basin actually *stands* full to its spill point is a water-balance question this formulation does not answer — see #obs-connectivity-fills-the-basins-the-threshold-drained FE(5) for the wet limit and what it assumes. This clause fixes only *what may hold water at all*.

6. **A residual seed defect, named and unrepaired.** When a domain contains no ocean under a no-flux wall, Priority-Flood still needs a seed, and the current rule makes the single **lowest** cell an outlet — which is always inside the deepest basin, so the deepest basin in a coastless walled window can never hold water. Views set a wall per drawn unit, so this reaches the paint on any drawn tile with no submerged cell touching its rim. Distinct from the classification error this segment repairs, and it is the next thing in this area to fix.

## Epistemic Status

**Max attainable: `exact`** for FE(1)–(3) — the mask is a stated construction over stated inputs, and `examples/lake_surface_probe` can falsify it in both directions (an enclosed crater that holds nothing refutes it; connected ocean reported as standing water refutes it the other way). Both arms are measured.

**Currently `conditional`,** and the condition is FE(4): the identification of the computed mask with the planet's real ocean holds only where the domain contains the enclosing land ring. That is a genuine local assumption, not a hedge — at tile scale the mask is provably biased toward sea, and the bias direction is known.

The formulation is `formulation` rather than `derived` because a defensible alternative exists: a *designated* ocean region (a stored mask produced once at whole-planet scale and consumed by every reader) would remove FE(4)'s window dependence entirely. That is a better end state and it is not what is built. Stage `draft`.

## Discussion

The threshold reading was not a shortcut anyone chose; it is what "sea level" means when the only thing you have is a height. Its cost stayed hidden because it fails *silently and in the plausible direction* — a below-datum basin painted as ocean looks like ocean, and the picture is only wrong if you already know the basin is enclosed.

The reason it is worth stating as law rather than fixing quietly in one function is that **five** separate places used to ask "is this cell ocean?" with independent thresholds. As of 2026-07-31 they share `sea_level::ocean_mask` (and `ErodedRegion::ocean_mask` for region-scoped readers). The ocean set is a **world object** other nomos consume; plate work will still want a store-memoized whole-planet mask (FE below), but the classification law no longer forks per consumer.

## Working Notes

- **Shared object — landed 2026-07-31.** `sea_level::ocean_mask` / `is_submerged` are the pure functions; `Fluvial::outlets`, water initial fill + rim hold, terminal `globe`, explorer surface/water/provenance paints + mesh freeboard, and `column_at` / `column_from_surface_at_sea_classified` all consume them. Dual-arm unit test: `ocean_mask_is_connectivity_not_elevation`. **Still owed for full world-object status:** a store-memoized face/planet mask under a complete key (today the mask is recomputed from heights; plate work wants one durable article). Legacy `column_from_surface_at_sea` still thresholds for pour-style tests — production paths use the classified form.
- **Owed, and deliberately not guessed at:** the water nomos in `nomotheke.rs` still declares `"SEA_LEVEL_M"` among its `assumptions`, which stopped being true when `water_tile` moved to the derived waterline — the built path no longer assumes the decree (the compat constructor still does, for probes). The declaration is therefore false as it stands. It was left rather than edited because whether those strings are a *checked* contract against `ASSUMPTIONS.md` row names was not verified, and inventing a name that silently matches nothing would replace a stale declaration with an unenforced one. Cost of the repair is one `SRC_HASH` move and a keys-only rebuild (payloads unchanged), which is the cohort shape entry `2026-07-29-02` already established.
- **Owed: FE(6)**, the coastless-wall seed. A candidate: give such a domain *no* outlet and let the fill raise everything to the rim, which is what a genuinely sealed basin does — but D8 needs a receiver, so this wants design rather than a one-line change.
- **Not yet measured:** how much of the FE(4) window bias survives at region scale with halo exchange ( #form-same-level-halo-exchange ). A region is wider than a tile and narrower than a face; the honest answer is a sweep nobody has run.
- **Prior art to adopt, not reinvent:** Fill–Spill–Merge (Barnes et al. 2021, ESurf 9:105–121) is a strict superset of the wet-limit fill, $O(N \log N)$, with a closed-form lake level for the partially-filled endorheic case; CHONK 1.0 (Gailleton et al. 2024, GMD 17:71–90) carries lakes coupled to erosion including an evaporation term in the level balance. Dossier: `msc/research-lem-sota/lake-and-settle-sota-2026-07-29.md`.
