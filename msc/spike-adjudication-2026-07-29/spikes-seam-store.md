# Seam/store family adjudication — msc spikes vs core (2026-07-29)

Adjudicator: seam/store slice agent. Method: read every prose artifact in the four
directories in full (PREDICTIONS.md, RUN.txt, NOTE.md, lit-notes.md, the two .out
files, probe-output.txt header; src/ judged by what the prose claims of it), then
verified each candidate claim against the absorbing segments' *content*, DECISIONS
full text, ASSUMPTIONS.md, and live code state (`chunk.rs`, `bin/check`,
`examples/`). DECISIONS treated as first-class absorbing surface per the router
slice's finding.

## Verdicts, one line each

| Directory | Verdict |
|---|---|
| `msc/spike-cross-face-seam/` | **Keep-live until residue lands** — 5 of its 6 probe findings have **no carrier anywhere** (no segment, no DECISIONS row); it is the least-absorbed spike in this family, and one finding constrains the fresh 2026-07-28 halo design |
| `msc/spike-wavelet-store/` | **Keep-live, deliberately** — all 8 probes absorbed or superseded-by-stronger, but it is the *cited primary instrument* of three `exact`-status segments, a live parts-shelf (edge-length/arm metrics awaiting the staggered-FV router), and the store-build decision it informs is still open. Delete `target/` (build artifacts, ~junk) |
| `msc/spike-null-space/` | **Graduate-after-repointing** — findings fully absorbed (DECISIONS council-accepted + ASSUMPTIONS row 68 + two segments); but two DECISIONS `|ref` lines cite its files as carriers, incl. `probe-output.txt §"SECOND MECHANISM"` for the Jarrett entry — re-point refs to the .super-archive path (or accept as history-layer provenance) when moving |
| `msc/spike-null-space-probe/` | **Graduate-after-landing-X** — the instrument survives in-crate (`examples/null_space_gate.rs`), but NOTE.md's two action items are unlanded and one segment now states a falsehood it corrects (below) |

## Residue — Tier 1 (real: measurement or constraint whose only carrier is the spike)

### R1. Cross-face halo depth cap — and it collides with the halo-exchange design
`spike-cross-face-seam/RUN.txt` PROBE 2: the two cross-face grids co-align **only on
the shared edge**. Depth-1 correspondence exact (0/256 mismatch); depth 2 breaks on
**200/256 edge cells (78%)**, depth 3 on 222/256, depth 4 on 232/256, worst at cube
corners. This is a hard constraint on cross-face stencil/halo radius. **No segment
or DECISIONS row owns it.** It directly bears on `#form-same-level-halo-exchange`
(built 2026-07-28), whose mechanism carves on an $(n+2d)^2$ window with measured
budgets at **d=16** — but whose backing probe (`#obs-exchange-repairs-...` FE(1))
ran entirely **within one face** (L13, origin (640,5376), 4×4 tiles interior to a
face). Neither halo segment mentions that a d≥2 halo crossing a cube edge has no
exact cell correspondence — so the exchange design as stated silently doesn't
transfer to tiles at cube edges without an interpolation choice that nobody has
declared. Land: a clause on `#form-same-level-halo-exchange` (scope: measured
within-face; cross-face d≥2 needs a declared resampling, constraint measured in the
spike) and/or on `#form-cellid-chunk-patch` FE(4). This is the "wish we had
integrated sooner" item.

### R2. The face axis transform is latent — no adjacency table needed
PROBE 1: the `to_unit`/`from_unit` round-trip **is** the cross-face transform —
involution bit-exact on 64/64, 256/256, 1024/1024 edge cells (L4/L6/L8), genuine
adjacency geometry (ratio ∈ [0.87, 1.41]), known-bad clamp discriminated.
`#form-cellid-chunk-patch` FE(4) still says only "Designed, unbuilt: cross-cube-face
halo fill with the face axis transform" (file line 21) and Known-incomplete repeats
"designed, not built" (line 29) — the segment doesn't know the design question is
*answered and measured* (transform = the projection round-trip, checkable by
involution; the real hole is depth, R1). The spike commit (22b7604) promised
"verdicts in the landing report"; **no landing report exists in the tree** and no
DECISIONS row was ever written. Land: upgrade FE(4)'s sentence from
designed-unbuilt to spiked-measured-unbuilt with the involution result.

### R3. Default-0 out-of-face halo convicted as physical defect (sink + cliff)
PROBE 4a: today's `chunk.rs` default-0 halo is a Dirichlet-0 sink — mass drift
**linear-growing bias** (−4.07e-14 → −4.07e-11 over 1→1000 steps). PROBE 5: it is a
manufactured **~121 m cliff that does not shrink with the arc** (plateau at L6/L8/L10
vs filled halo tracking the within-face step, 0.507→0.032 m). `chunk.rs:74` still
ships this behavior ("at default — real cross-face halo fill is the loader's job").
Only carrier: the spike. Note `#norm-no-depiction-without-referent` already treats
the cross-face discontinuity as "a real defect" the explorer's mesh smooths over and
paints magenta — the *world-side* measurement of that defect lives only here. Land:
one measured sentence wherever FE(4) ends up (probably `#form-cellid-chunk-patch`),
so the gap is a convicted physical defect, not a cosmetic TODO.

### R4. Null-space gate: segment now states a falsehood; wiring recommendation unlanded
`#detail-fvm-control-volume` FE(6) (file line 31): "Null-space probe (instrument,
**not yet built as gate**)" — stale since 2026-07-24. The can-fail gate exists at
`crates/vivarium-world/examples/null_space_gate.rs` (green run captured in
`spike-null-space-probe/gate-2026-07-24.out`), reusing the full instrument's core by
`#[path]`, with the two tolerance lessons baked into source comments (f32 kernels
need 1e-5·λmax cut, not 1e-9; cleanliness judged by ≥4 decades of daylight, not a
fixed gap floor — gate source lines 77–105, 147–149). Verified: **`bin/check` does
NOT run it** (no null_space line), so NOTE.md's "wire into bin/check now (one
line)" recommendation is unlanded — a guard nobody runs is back to narration.
Land before graduating the dir: (a) fix FE(6) to "gate built as example; bin/check
wiring open; library-test lift = move ~200-line core to `src/probe/`" (NOTE.md §3 is
the only carrier of that graduation-cost analysis); (b) either add the bin/check
line or record the decision not to. `#form-declared-structure-tradeoff` WN line 42
"Instruments owed: null-space/eigenvalue probe on each field nomos" is half-stale
the same way (instrument exists + gate; the per-nomos one-closure habit is what's
still owed).

## Residue — Tier 2 (arguable: small, judgment call whether to land)

- **A1. Single-valued conservation extended across a genuine cube edge.**
  `#form-face-flux-register`'s measured table is the *same-face* PROBE 7
  (L19|L20). Cross-face PROBE 4c reproduces it on a real cube edge (single-valued
  ≤3.2e-15 at 10k steps; two-sided grows to 4.9e-13), and PROBE 3 verified the
  segment's `#form-grid-equiangular-staggered` FE(2) ownership rule (lower CellId,
  global Ord) is cross-face-safe as a real `store.rs` citizen surviving reopen,
  with the per-face-local (i,j) rule convicted as ambiguous. One sentence on
  `#form-face-flux-register` ("holds across cube faces; ownership rule verified
  cross-face-safe, spike measured") would close it.
- **A2. Matched-seam null result** (PROBE 4b): on a matching symmetric cross-face
  seam two-sided *also* conserves to epsilon — the register's value on matching
  seams is the data-structure guarantee, not a numeric win. Useful sensitivity
  caveat for `#form-face-flux-register` FE(5); currently only in the spike.
- **A3. Prediction adjudication never happened.** PREDICTIONS.md was pre-registered
  per `#norm-probe-sensitivity`; the promised verdict pass doesn't exist. Notably
  P4a predicted rel. loss 1e-2..1e-1 and measured **4e-11** — 9 orders off (right
  sign/shape, wildly wrong magnitude: the probe's diffusion flux is tiny relative
  to total mass). If the changelog/pre-registration discipline wants specimens,
  this is one; otherwise fold into R3's landing sentence.
- **A4. Literature novelty gap** (`spike-wavelet-store/lit-notes.md` Q2 [∅], lines
  207–212): no conservative cell-average MRA on the *equiangular cube-sphere*
  exists in the literature; the Schröder–Sweldens Bio-Haar transfer is "a genuine
  (small, honest) novelty surface." Nowhere in core. Only matters if the store is
  ever built/published — defensible to leave in the spike, but worth remembering
  it exists. (The other lit-notes meat is absorbed: non-local-flux gap →
  `#sketch-detail-abstract-reversion` FE(5); Harten never-project lesson → same;
  three-condition refluxing shape → `#form-face-flux-register` FE(4).)
- **A5. 11-day bit-stability datum** (NOTE.md §1): full instrument re-run on
  2026-07-24 matched the 07-13 DECISIONS figures to every printed digit across 11
  days of kernel evolution — "a small determinism datum." No carrier after
  graduation; probably fine to lose, mentioned for completeness.

## What is absorbed / superseded-by-stronger (verified, not filename-matched)

- **Wavelet PROBE 1** (area additivity + uniform-area control screams) →
  `#obs-cube-locked-kernel-bias` FE(3) (+17.810% figure verbatim, L5–L13
  no-convergence) + `DECISIONS[drainage-area-uses-a-uniform-cell-area]`; the fix is
  live (`measure::cell_area_m2`, promoted from the spike's `area.rs` — measure.rs:5
  credits it).
- **PROBE 2/3/4** (perfect reconstruction; exactly-conservative lossy compression;
  O(log N) bit-exact edit) → `#sketch-detail-abstract-reversion` FE(4) +
  `DECISIONS[wavelet-store-spiked-the-seam-is-not-the-details]` (council-accepted
  for the findings; build decision open).
- **PROBE 5** (Jensen commutator) → superseded-by-stronger: FE(5) carries a
  re-measured +7.17 m on a fresh verified-land footprint plus the depth-2 trunk
  carrier result and `#obs-coarse-only-closure-nogo` — strictly beyond the spike's
  +15.30 m/no-predictor finding. RUN.txt's magnitudes are two re-basings stale;
  segments say so.
- **PROBE 6** (mean-pin) → `#obs-mean-pin-manufactures-seam` (re-based numbers,
  retirement executed, block-const convicted) — superseded-by-stronger.
- **PROBE 7** (refluxing ceases; face MRA = cell MRA one design) →
  `#form-face-flux-register` FE(3)/(4) + four council-accepted DECISIONS entries
  (incl. the grid STAGGER-IT entry carrying the numbers at line 509).
- **PROBE 8** (cell_size_m bias, Jacobian determinant/shear split) →
  `#obs-cube-locked-kernel-bias` FE(3)/(4), fixed live.
- **Cross-face PROBE 0** (guard) — no residue by design.
- **spike-null-space (07-13)** → `DECISIONS[our-kernels-have-no-null-space-the-solitons-were-roll-waves]`
  (council-accepted, carries controls, pins, Brillouin-zone map),
  ASSUMPTIONS.md line 68 (staggering row), `#detail-nomos-defect-anatomy` line 42,
  `#form-declared-structure-tradeoff` Discussion. The instrument itself lives
  in-crate (`examples/null_space/`). Nothing unabsorbed except the ref-pointing
  caveat in the verdict table.

## Adjacent findings (asked-for extras)

1. **Three sweep breadcrumbs to delete** once the above lands (their stated
   done-signal): `#form-seam-flux-exchange` WN line 69 (cross-face spike — note its
   "likely primary absorbing segment: this one" guess was **wrong**; the residue
   belongs to `#form-cellid-chunk-patch` / `#form-same-level-halo-exchange` /
   `#form-face-flux-register`), `#form-store-as-save` WN line 58 (wavelet spike —
   also the wrong primary; adjudication: *still-open alternative*, keep-live),
   `#detail-fvm-control-volume` WN line 50 (null-space pair — right segment).
2. **Dangling pointer:** `spike-wavelet-store/RUN.txt` last line says "verdicts in
   the spike's FINDINGS.md" — that file never existed in git; verdicts actually
   landed in DECISIONS + segments. Harmless once the dir's disposition is recorded.
3. **Same species:** cross-face commit 22b7604 says "verdicts in the landing
   report" — also nonexistent (see R2).
4. **Number drift, deliberate but worth knowing:** `#form-face-flux-register`'s
   PROBE 7 table (1.41e-8→2.58e-5) differs from RUN.txt's current print
   (3.63e-8→3.77e-5) — different re-based runs of the same probe; qualitative
   verdict identical. Not an error, but the segment cites "PROBE 7 + RUN.txt" as
   instrument while carrying numbers RUN.txt no longer prints.
5. **`msc/spike-wavelet-store/target/`** — cargo build output, safe to delete
   regardless of the dir's fate.
6. If wavelet-store is ever graduated later: three `exact` segments cite
   `msc/spike-wavelet-store/ PROBE N + RUN.txt` as *primary instrument*
   (`#obs-mean-pin` line 53, `#form-face-flux-register` line 55,
   `#obs-cube-locked` line 41) and `.super-archive/` is "treat as deleted for
   tactical dependence" — instrument citations would need re-homing, which is a
   second reason keep-live is the honest verdict now.
