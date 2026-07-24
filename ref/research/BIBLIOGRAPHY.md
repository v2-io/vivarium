# vivarium — verified bibliography (relata-backed)

*(Built 2026-07-03; PDF column updated 2026-07-09. Every entry below was verified against its publisher or archive page (title/authors/venue/year/DOI) and lives in relata with a `bib-fields` verification event by
`claude-fable-5-vivarium-bibliography`. Use `relata show <key>` for full metadata;
PDFs are registered in relata's external store where we could legitimately obtain them. Only publisher-OA, arXiv, HAL/institutional repositories, and authors' own pages were used.)*

> **Local reading copies (2026-07-09).** Seven of the manual-chase items were
> obtained (author/HAL/eprints copies) and registered in relata. Git-ignored local
> reading copies live under `ref/research/pdfs/` (methodology: `gear-1984-multirate`,
> `berger-1984-adaptive`, `galin-2019-review`), `ref/geology/pdfs/` (terrain-erosion
> lineage: `cordonnier-2016-large`, `schott-2023-large`), and `ref/hydrology/pdfs/`
> (`almeida-2012-improving`, `almeida-2013-applicability`). We hold no redistribution
> rights — these dirs are git-ignored; relata's external store is the canonical
> registration. PDF filenames are the relata keys.

> [!warning]
> **Relata is under heavy construction (2026-07-10) — new registrations DEFERRED until after the refactor.** Do not add new relata-backed rows below until relata stabilizes; track pending items here and revisit then. **Pending registrations:** **`cordonnier-2018-sculpting`** (CCB\*18 — *Sculpting Mountains: Interactive terrain modeling based on subsurface geology*, IEEE TVCG 24(5):1756–1769, 2018; metadata from `galin-2019-review`'s reference list; already *cited* in `.super-archive/from-design/DESIGN-SYSTEMS.md` but unregistered), plus any ⭐ items from `ref/research/frontier-2026-07-10/frontier-scan-2026-07-10.md` promoted to citable use (e.g. Tzathas–Cordonnier 2024 analytical erosion, Braun–Deal 2023 threshold-SPIM, L'Ecuyer 2021 splittable RNG, Doehring 2024 AMR-multirate, Kang–Masud 2025 two-way coupling).

## Multiscale & heterogeneous methods (graduated teaching zoo: `.super-archive/from-theory/multiscale-methods.md`)

| relata key | what it is | PDF |
|---|---|---|
| `brandt-1977-multilevel` | multigrid, the founding paper (Math. Comp. 31) | ✅ (AMS OA) |
| `berger-1984-adaptive` | AMR, the founding paper (JCP 53) | ✅ (local; relata) |
| `gear-1984-multirate` | multirate time integration (BIT 24) | ✅ (local; relata) |
| `e-2003-heterogeneous` | HMM (Commun. Math. Sci. 1(1)) | ✅ |
| `kevrekidis-2003-equation-free` | equation-free / coarse projective integration (CMS 1(4)) | ✅ |
| `grabowski-2001-coupling` | superparameterization (CRCP), JAS 58 | ✅ (AMS free archive) |
| `khairoutdinov-2001-cloud` | superparameterization in a real GCM, GRL 28 | ❌ chase |
| `vandenbulcke-2019-upscaling` | upscaling child→parent via assimilation — closest prior art to detail→abstract (Ocean Sci. 15) | ✅ (Copernicus OA) |
| `e-2011-principles` | *Principles of Multiscale Modeling* (CUP book) | ✅ (author's own posting, princeton.edu) |
| `mokhov-2018-build` | *Build Systems à la Carte* (PACMPL 2/ICFP) — the §11 lazy-memoized-graph frame | ✅ |
| `gustafson-2017-beating` | posit arithmetic (SFI 4(2)) — §9 exactness thread | ✅ (journal OA) |

## Shallow water, sediment, channel physics (what the water sim stands on)

| relata key | what it is | PDF |
|---|---|---|
| `bates-2010-simple` | local-inertial shallow water, the ancestor scheme (J. Hydrol. 387) | ❌ chase |
| `almeida-2012-improving` | **the $\theta$ flux-smoothing scheme** (WRR 48, W05528) | ✅ (local; relata) |
| `almeida-2013-applicability` | where local-inertial is valid (WRR 49) | ✅ (local; relata) |
| `dietrich-1989-sediment` | armor expresses sediment-supply *deficit* (Nature 340) | ❌ chase |
| `grant-1997-critical` | Froude ≈ 1 self-organization in steep streams (WRR 33) | ❌ chase |
| `jarrett-1984-hydraulics` | slope-dependent roughness in high-gradient streams (JHE 110) | ❌ chase |
| `fischer-1979-mixing` | *Mixing in Inland and Coastal Waters* (book) — transverse dispersion `K = α·d·u*` | ❌ chase (book) |
| `rouse-1937-modern` | the Rouse suspended-sediment profile (Trans. ASCE 102) | ❌ chase |

## Landscape evolution & erosion (what the erosion pipeline stands on)

| relata key | what it is | PDF |
|---|---|---|
| `davy-2009-fluvial` | the erosion/deposition (ξ–q) model our deposition follows (JGR-ES 114) | ❌ chase |
| `whipple-1999-dynamics` | stream-power incision dynamics (JGR 104) | ✅ (uchicago mirror) |
| `lague-2014-stream` | SPIM: evidence, theory, beyond — the honest review (ESPL 39) | ✅ (OSU mirror) |
| `braun-2013-very` | O(n) implicit stream-power solver (Geomorphology 180–181) | ✅ (pre-existing) |
| `yuan-2019-new` | O(n) implicit solver *with deposition* — our scheme's basis (JGR-ES 124) | ✅ (GFZ repository) |
| `barnes-2014-priority` | Priority-Flood (C&G 62) | ✅ (arXiv preprint) |
| `montgomery-1994-physically` | topographic control on shallow landsliding (WRR 30) — the slumping frontier | ❌ chase |

## Terrain graphics lineage (context for the erosion ports)

| relata key | what it is | PDF |
|---|---|---|
| `cordonnier-2016-large` | uplift + stream-power terrain generation (CGF 35) | ✅ (local; relata) |
| `galin-2019-review` | digital terrain modeling STAR (CGF 38) | ✅ (local; relata) |
| `schott-2023-large` | interactive large-scale erosion authoring (ToG 42(5)) | ✅ (local; relata) |
| `mei-2007-fast` | GPU virtual-pipes hydraulic erosion (PG'07) — our pipes ancestor | ✅ (INRIA lab page) |
| `cortial-2019-procedural` | procedural tectonic planets (CGF 38) — pre-existing entry | (pre-existing) |
| `perlin-2002-improving` | improved Perlin noise — pre-existing entry | (pre-existing) |

## The manual-chase list (verified real; we could not download legitimately by script)

Most of these are one browser-click for a human — the blockers are bot-walls,
not paywalls, for at least the HAL and AGU items.

**Obtained 2026-07-09** (7 — author/HAL/eprints copies, registered in relata,
git-ignored local copies per the note at top): `berger-1984-adaptive`,
`gear-1984-multirate`, `almeida-2012-improving`, `almeida-2013-applicability`,
`cordonnier-2016-large`, `galin-2019-review`, `schott-2023-large`.

**Still to chase** (9):

- `khairoutdinov-2001-cloud` — AGU/Wiley, likely free in browser: https://agupubs.onlinelibrary.wiley.com/doi/pdf/10.1029/2001GL013552
- `bates-2010-simple` — Elsevier, paywalled (Bristol PURE may have AM): https://doi.org/10.1016/j.jhydrol.2010.03.027
- `davy-2009-fluvial` — AGU/Wiley pdfdirect: https://agupubs.onlinelibrary.wiley.com/doi/pdfdirect/10.1029/2008JF001146 (also HAL: insu-00424874)
- `grant-1997-critical` — AGU/Wiley: https://agupubs.onlinelibrary.wiley.com/doi/10.1029/96WR03134 (Grant is USFS — treesearch may have it)
- `montgomery-1994-physically` — AGU/Wiley: https://agupubs.onlinelibrary.wiley.com/doi/10.1029/93WR02979
- `dietrich-1989-sediment` — Nature, paywalled: https://doi.org/10.1038/340215a0
- `jarrett-1984-hydraulics` — ASCE, paywalled: https://ascelibrary.org/doi/10.1061/(ASCE)0733-9429(1984)110:11(1519)
- `rouse-1937-modern` — ASCE archive, paywalled: https://cedb.asce.org/CEDBsearch/record.jsp?dockey=0288088
- `fischer-1979-mixing` — book (Academic Press, ISBN 9780122581502); Caltech authors' library has a record: https://authors.library.caltech.edu/records/zmf3v-0dx76

## Corrections found during verification

- **`water.rs:350` misattributes the $\theta$-scheme.** The comment says "$\theta$ flux smoothing (de Almeida & Bates 2013)"; the $\theta$-weighted flux scheme is
  **de Almeida, Bates, Freer & Souvignet 2012** (WRR 48, W05528,
  `almeida-2012-improving`). The 2013 paper (`almeida-2013-applicability`)
  is the *applicability* study. One-line comment fix, deliberately not applied here (crates/ was under concurrent integration work at the time).
- **"Schott 2023" disambiguated**: it is *Large-scale terrain authoring through interactive erosion simulation*, ToG 42(5), DOI 10.1145/3592787.
  The adjacent 10.1145/3618350 is the same group's meandering-rivers paper —
  a nice future source for the bank-mechanics rung, but a different paper.
- **All attributions in `.super-archive/from-theory/multiscale-methods.md` checked out** (Berger &
  Oliger 1984; Brandt 1977; E & Engquist 2003; Kevrekidis et al. 2003; Gear
  & Wells 1984; Grabowski 2001 / Khairoutdinov & Randall 2001) — its
  "unverified training memory" caveat is now discharged and the doc updated with exact years.
- Two title typos exist *on Project Euclid's own pages* ("Heterognous",
  "Mocroscopic") — trust the DOIs, not their HTML titles, if scraping.

## Not ingested, deliberately

Web resources cited in `ref/geology/NOTES.md` that are not papers (Azgaar's Fantasy Map Generator notes, Experilous planet-generation blog, Gumin's WaveFunctionCollapse repo, Summerville/Aokana posts): real influences, wrong shape for a citation manager — they stay as URLs in the NOTES that use them.
Deeper rungs we haven't built yet (bank-mechanics/meandering sources, glacial and aeolian process papers, Whittaker's biome classification) belong to a second sweep when those systems are actually being designed.
