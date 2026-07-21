---
slug: form-in-vivia-citation
type: formulation
status: conditional
stage: draft
depends:
  - def-in-vivia
  - form-complete-content-addressed-key
  - post-determinism-as-ontology
---

# Operational *in vivia* citation

A citable *in vivia* result names a complete world-artifact key: seed, generator/nomos versions, phase memo, and intervention script — not a prose footnote to "the sim."

## Formal Expression

1. **Citation bundle.** An empirical or simulation claim that is *in vivia* ( #def-in-vivia) cites a **specific vivium artifact** by the following operational keys, jointly:
   - **seed** — the world seed that fixes fated draws ( #post-determinism-as-ontology);
   - **generator / nomos versions** — identity and version of every producing nomos (and generator stack) whose output the claim depends on;
   - **phase memo** — which phase ladder state / Promise set the run was under (enough to recover the law box);
   - **intervention script** — the ordered exogenous interventions (or their content-addressed encoding) applied to produce the cited state or trajectory.
2. **Exact citation needs complete keys.** Content-addressed storage makes the citation **exact** only when every byte-affecting input participates in the memo key ( #form-complete-content-addressed-key). An incomplete key is silent corruption of the citation, not a weaker citation.
3. **Register strength is conditional.** Meeting the *in vivia* register for a study still requires #def-in-vivia's conditions (real dynamics, ground truth for claimed quantities, reproducibility in principle). This segment is the **operational recipe** for the citation half; it does not grant laboratory-grade status to an unprobed system.

## Epistemic Status

**Max attainable: exact** as a formulation of what a complete citation must include, **once** generator/nomos pinning and complete-key practice are fully mechanized.

**Currently `conditional`.** The register definition is settled ( #def-in-vivia; Joseph 2026-07-04 on the *in vivia* idea). The four-part recipe is the operational meaning carried from the ASF bridge integration program. **Honest gaps that block claiming bit-perfect citation infrastructure today:**

1. **Generator-pinning** is open as a dictionary term and practice ( #lexicon/term/generator-pinning).
2. **Hand-stamped nomos versions** remain present practice; source-derived versions are the target remedy ( #form-complete-content-addressed-key known incomplete surfaces).
3. **Enforcement layer** for cone-fold / regula-style citation was sketched (`doc/plan/regula-conformance-design.md`) and is not this segment's claim that it is built.

Stage `draft`. Do not assert that every `cargo run` example is already a citable vivium.

## Discussion

This is how *in vivia* cashes out for ASF and for external papers: a third empirical register between toy model and field study, with transfer assumptions stated and the substrate named by complete keys. Without the bundle, "we saw it in vivarium" is marketing. With incomplete keys, the citation looks exact and is not.

## Working Notes

- Pair any tooling work (manifest fields, `vivarium status` citation printout, store root discipline) to this segment; do not invent a second citation schema in plans.
- Historical home: former `ASF.md` §7 item 5 (sometimes cited as §7.5).
- Upstream consumers (charter concept-matrix *in vivia* row) should cite `#form-in-vivia-citation` + `#def-in-vivia`, not section numbers in the demoted bridge file.
