---
slug: sketch-nomos-declaration-boxes
type: sketch
status: sketch
stage: draft
depends:
  - form-flux-web
  - def-nomos
  - disc-prime-question
  - norm-bias-vs-noise
  - form-kernel-imperative-boundary
  - form-declared-structure-tradeoff
---

# Nomos declaration boxes (schema landed; procedures open)

*Sketch of the NOMOS-CONTRACT five-box anatomy. Box ① quantities is `#form-flux-web`; box ⑤'s law side is `#disc-prime-question` + `#norm-bias-vs-noise`. As of 2026-07-24 all five boxes have **fields on `NomosDecl`** and structural tests; what remains sketch-grade is the box *set*'s completeness and the per-box earning procedures.*

## Formal Expression (sketch)

1. **Five boxes (anatomy from Jul-13 defects).** ① quantities · ② geometry (what algorithm assumes vs grid/regime delivers) · ③ semantics (what the number means; statistic + exactness) · ④ structure (preserved-exact / preserved-approx / sacrificed) · ⑤ claim (unphysical term added — parity + bias/noise verdict).
2. **Generalization.** ②–⑤ are the same *declare and match* move as ①, applied to geometry, meaning, invariants, and the lie.
3. **Data-model state (live).** `nomotheke.rs` carries: ② `Assumes{assumption, delivered, note}` (Holds/Violated/Unexamined); ③ `Statistic` + `Exactness` on every promise and consume, with `Statistic::Undeclared` as the honest carrier of the open column fork, and `audit::statistic_match_across_registry` as the match instrument; ④ `StructureDecl` over the closed `STRUCTURES` vocabulary (three disjoint lists by test); ⑤ `UnphysicalTerm{parity, verdict}` with sign-definite ⇒ Bias enforced by test — plus `Family`, `ExecutionClass`, `Timescale{band, z}`. Current entries carry the measured 07-13/07-23 findings by citation (fan, phantom flux, $\theta$, clips, hydrostatic/Fr envelopes, undeclared water assets).
4. **Adjudication.** Bias vs noise ( #norm-bias-vs-noise ) cuts across all boxes — not a sixth box.
5. **Not closed.** The box *set* was abstracted from one audit day's sample; completeness unproved. Declaration *content* convicts only where a test or probe reads it — the structural tests convict shape (vocabulary closure, disjointness, sign-definite⇒Bias, no Unsound statistic edge); they do not convict a wrong `Delivered::Holds` (that still takes an audit or probe).

## Epistemic Status

**Currently `sketch`** for the box set and procedures; the fields and structural tests are **live code** (`nomotheke.rs`, `audit.rs`; 2026-07-24). Source anatomy: `#detail-nomos-defect-anatomy` (its "the data model cannot hold this" blocker is retired; its per-box procedure files remain unwritten intent). Stage `draft`.

## Working Notes

- Earning-procedure shape + full defect map + failure-gallery seeds: #detail-nomos-defect-anatomy (not iced NOMOS-CONTRACT).
- Promote (or split per-box) toward `formulation` when procedures exist as core peels. Fields without procedures invite confident wrong content.
- Open: "conflicts-with" surface on box ④; ordinum reachability (keeper range vs promise predicate).
