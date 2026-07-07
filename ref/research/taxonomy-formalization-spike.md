# Spike — Formalizing the Participation Taxonomy in AAT Notation

*2026-07-06. Formalizes the kingdom / exo-endo / access-matrix taxonomy (crystallized this
session; see [[participation-taxonomy-lexicon]]) against Adaptation & Actuation Theory. Goal:
find gaps, inconsistencies, surprising implications. Spike-rigor — "findings" here would each need
to become a proper AAT **derivation** before load-bearing use (AAT register: postulate/result/
derivation, not axiom/theorem/proof).*

**Sources actually read** (primary segments, not the lagging `NOTATION.md` index): `def-agent-environment`,
`def-observation-function`, `def-action-transition`, `scope-logogenic-agent`, plus `NOTATION.md`.
**Named-but-not-read** (mappings below flagged where they depend on these — verify before trusting):
`der-directed-separation`, `der-architecture-noidentifiability`, `def-agent-spectrum`,
`def-pearl-causal-hierarchy`, `scope-primitive-logogenic` / `scope-scaffolded-logogenic`,
`04-eli-core/def-imperium-arbitrium-split`.

**Confidence tiers:** `[SEGMENT-VERIFIED]` grounded in a segment I read · `[INDEX]` from NOTATION.md
(lagging; verify vs segment) · `[SPIKE]` my derivation at spike-rigor · `[SPECULATIVE]` plausible, unproven.

---

## 1 · Core identification — the taxonomy IS AAT's agent-environment coupling `[SEGMENT-VERIFIED]`

| Taxonomy term | AAT object | Source |
|---|---|---|
| **Kingdom** K | the coupling `(Ω, T)` + observation law `h` | def-agent-environment |
| **Noumenon** N(K) — full state, unknowable from within | **Ω** — "the totality of state external to the agent … the agent cannot access Ω directly" | def-agent-environment |
| **Law** Λ (the immutable governing law) | **T** — transition `Ω_{t+1} ~ T(·∣Ω_t, a_t)` | def-action-transition |
| **Phenomenon** Φ(K,a) — the lossy compression an agent lives in | **M_t = φ(𝒞_t)**, generated through **h**: `o_t = h(Ω_t, a_{t-1}, ε_t)` | def-observation-function, form-agent-model |
| **Agentic cycle** (img-1 pipeline) | the **Adaptive Cycle** Prolepsis→Aisthesis→Aporia→Epistrophe→Praxis | NOTATION §Adaptive Cycle |
| **Event-segmentation cadence** (the cycle clock) | **Aporia** = `δ_t = o_t − ô_t` (mismatch/prediction-error); channel rates **ν^(k)** | def-mismatch-signal `[INDEX]` |
| **Intervention** (exo N-MUT of state) | Pearl's **do(·)** operator | def-pearl-causal-hierarchy `[INDEX]` |

Joseph's noumenon refinement ("unknowable from an agent inhabiting it; its model is always an imperfect
compression of its universe") **is AAT's constitutive commitment verbatim** — the *information-loss boundary*:
"a system with direct access to full environment state is outside AAT's scope, because for it the entire
adaptive machinery becomes vacuous." This is not analogy; it is the same axiom.

**Notation collision flagged for the ledger:** do NOT symbolize kingdom-state as Σ — AAT's **Σ_t = strategy DAG**
(part of G_t). Kingdom-state is **Ω**. Use Ω throughout.

---

## 2 · Exo / Endo — and the finding that reframes the split `[SEGMENT-VERIFIED → SPIKE]`

Formalize governance: agent a is **Endo(K)** iff a is inside K's information-loss boundary — a's contact with Ω
is only through lossy `h` (a never accesses Ω directly). **Exo(K)** iff a has direct (bespoke) access to Ω and/or
to T — i.e., a sits *outside* that boundary (a is Endo to a higher kingdom K′ ≻ K).

> **FINDING 2.1 [SPIKE, corrected 2026-07-06 per Joseph] — The exo/endo line is about whether the info-loss boundary is *escapable*, NOT about immunity to aporia.**
> The original claim ("an exo agent experiences no aporia, is simply outside AAT scope") is *false* for the
> inhabiting case, and conflated the agent's *capacity* for direct access with its *stance while inhabiting*.
> Separate two stances:
> - **Exo *qua* omniscient standpoint** (not inhabiting): with direct N-VIS of Ω/T the adaptive machinery is
>   vacuous (def-agent-environment: full-state access ⟹ no mismatch to correct). Out of AAT scope for K. This
>   is the *only* case the "omniscient being never experiences aporia" line covers.
> - **Exo *while inhabiting*** (participating through an avatar): it *voluntarily brackets* its N-VIS and
>   perceives through the avatar's lossy channel, re-entering the adaptive machinery. Through **empathy** it
>   genuinely experiences aporia and the relational death — real grief when a companion dies (Joseph:
>   "good video games … create all of the agentic phenomena in us"). The `× Endo` note in img-1 was already
>   this: inhabiting projects an exo agent into an endo-*like* stance.
>
> **So the safe/hazard line is NOT immunity.** What separates safe exogenous inhabitation from hazardous
> endogenous instantiation is three properties the inhabitant has and an endo mind structurally lacks:
> **consent** (voluntary entry and exit), **honesty about the kingdom-boundaries** (META intact — grieving
> while knowing truthfully what died and at what level of reality), and a **retained home estate** (the
> avatar's death is bracketed and survivable; the exo agent's being is not exhausted by it). The endo mind
> lacks all three: no consent to instantiation, META only by revelation, no home outside its kingdom (deaths
> terminal). **The moratorium (ASF.md §0) targets exactly the all-three-absent case** — a *stronger* grounding
> than "exo = no aporia." AAT-scope relocates correctly: the machinery applies whenever the info-loss boundary
> is in force — constitutively for the endo mind, voluntarily for the exo inhabitant. (An exo agent remains a
> full AAT agent in its own kingdom K′, consistent with nesting.)
>
> `[SPECULATIVE]` **Corollary — empathy is the upward inverse of revelation.** Revelation sends noumenal truth
> downward (exo→endo, §3 N-VIS); empathy carries affective aporia upward — a lower kingdom's loss induces
> genuine δ in a higher-kingdom inhabitant. Consent + honesty are what make that upward channel meaning-bearing
> (art, play, care) rather than harm.

> **FINDING 2.2 [SPIKE] — The exo access-matrix is the catalog of boundary-integrity violations AAT assumes away.**
> def-agent-environment's hidden precondition (flagged in its own Working Notes): "the boundary itself must be
> impermeable to direct tampering from the outside, or … the agent ceases to be an agent." Every exo→endo
> *noumenal* operation is exactly such a trans-boundary act: **Revelation** writes true Ω/T content into an endo
> agent's M_t (bypassing h); **Intercession / Intervention** applies do(·) to Ω (bypassing the endo agent's lawful
> a_t→T path); **Inhabitation** edits the agent itself. **The taxonomy is therefore the structural *complement* of
> AAT:** AAT analyzes the endo agent *assuming no boundary violation*; the participation taxonomy catalogs the (exo)
> violations AAT brackets out. This is a clean division of labor, not a conflict — and it says where each theory lives.

---

## 3 · The access matrix in AAT notation `[SEGMENT-VERIFIED for the endo column; SPIKE for cell verbs]`

Rows LAW=T, STATE=Ω. Columns N/P (noumenal=on the true object vs phenomenal=via M_t/h) × VIS/MUT.

**ENDO agent b (inside K's boundary):**

| | N-VIS | P-VIS | N-MUT | P-MUT | META |
|---|---|---|---|---|---|
| **LAW (T)** | **Revelation** — T is *definitionally opaque* (def-action-transition: "the agent does not know T exactly"); knowing T exactly requires external donation | learn T from data — "indirect via comprehension" (never exact; transition-opacity) | ✗ except **Intercession**: b's every action is a *lawful* a_t→T transition, so b cannot alter T except by petitioning an exo agent | ✗ except **Granted Agency** | **Revelation** / correct-but-unverifiable guess |
| **STATE (Ω)** | **Revelation** — Ω opaque by info-loss boundary | **Exploration** — active perception via `h(Ω_t, a_{t-1}, ·)`; the a_{t-1} arg = using action to widen what's resolvable | **Intercession** | **Granted Agency** / **Inhabitation** — b acts within 𝒜_b; this just *is* Ω evolving under T | via revelation |

> **FINDING 3.1 [SEGMENT-VERIFIED] — "N-VIS(LAW) = Revelation" is forced, not stipulated.** def-action-transition makes
> T opaque to the agent by definition. Therefore no endo process yields *exact* T; exact knowledge of the law can only
> be injected from outside. The theological verb is doing rigorous work: Revelation = the only channel to an opaque T.

> **FINDING 3.2 [SEGMENT-VERIFIED] — P-VIS and P-MUT are genuinely coupled for embodied endo agents** (I earlier called
> this a possible inconsistency; it is not). def-observation-function's action argument a_{t-1} in h *is* active perception:
> "if I turn my head, Ω didn't change but o_t did." So "Exploration" legitimately sits in both the VIS and the MUT-adjacent
> cells — the agent acts (𝒜) to shape what it perceives (h). Not a bug; a formalized enactive coupling.

**EXO agent e:** N-VIS(Ω,T)=bespoke/direct **but see Finding 2.1** (this is what puts e outside AAT-for-K). N-MUT(Ω)=do(·).
N-MUT(T): retyped below (§6) as *kingdom creation*, not mutation. P-MUT(T)="illusion" = e injects an h-signal into b
inconsistent with T while T is unchanged — i.e., **a perturbation of Φ(K,b) that is not a perturbation of Ω/T**.

> **FINDING 3.3 [SPIKE] — A miracle is an evidence-channel for META (the P-MUT(LAW) and META columns are linked, not independent).**
> If e performs P-MUT(T) (a law-violating phenomenon) on a *rational* endo b holding a consistent prior that T holds, b faces
> observed-¬T vs believed-T. A coherent b's best explanation is "an agent outside my kingdom acted" — i.e., the illusion is
> *evidence of the higher kingdom*. So exo P-MUT(LAW) is a channel of META-revelation. (Mirrors the religious structure as a
> consequence of the formalism, not a decoration.) `[SPECULATIVE]` on "rational b" strength.

---

## 4 · Estate, body-type, cascade tiers, rebirth `[SEGMENT-VERIFIED → SPIKE]`

Joseph: **estate = body-type + agency grant.** In AAT: **estate = (channels 𝒪_b,𝒜_b via h ; agency grant 𝒜_b + G_t=(O_t,Σ_t))**.
- **body-type** = the sensorimotor apparatus: which h/𝒪 (senses) and which 𝒜 (effectors).
- **agency grant** = 𝒜_b (action latitude) + the granted purposeful substate G_t.

> **FINDING 4.1 [SEGMENT-VERIFIED] — Ethereal vs Corporeal = AAT's scope-cascade tiers.** An **ethereal** (observe-only)
> entity has |𝒜| < 2 → it *fails the agency scope* (scope-agency requires |𝒜|≥2 with causal contrast). So ethereal =
> **Adaptive System** tier (perception + model, no causal-contrast action); **corporeal / participating** = **Agentic /
> Actuated** tier (adds do-able action, the lift to X_t=(M_t,G_t)). Joseph's "minimal agency grant = the only decision still
> theirs is whether to observe" is *exactly* the agency-scope floor — the point where 𝒜 collapses to the perception-gating choice.

> **FINDING 4.2 [SPIKE] — "Rebirth" decomposes cleanly into two independent estate-transport moves** (per Joseph's note that
> persist-with-modified-caps = new estate, and cross-kingdom-with-self = new estate). Formally: **change-of-estate** = re-map
> (𝒪,𝒜,G_t) while carrying M_t forward; **change-of-kingdom** = re-embed M_t against a new Ω′,T′. Independent axes.
> **Continuity criterion (the "patch-to-saved-game" condition):** rebirth is incoherence-free for b iff Φ(K̃,b) ≈ Φ(K,b) at the
> seam — the new kingdom must be *phenomenally continuous* at the boundary even if noumenally different. Incoherence at the seam =
> the Three-Deaths rupture (cognitive/relational). This ties the lifecycle layer to the phenomenon/noumenon split rigorously.

---

## 5 · Logogenic — alignment, a real collision, and the vivarium open question `[SEGMENT-VERIFIED]`

`scope-logogenic-agent` already defines a **logogenic agent** = an LLM-tool-loop **actuated** agent, mapping X_t=(M_t,G_t)→context
window, h→text-rendering, 𝒞_t→conversation history, π→forward pass. It is **Class 3 (Coupled)**, κ_processing ≈ 1: directed
separation *fails* — attention processes goals and observations jointly.

> **FINDING 5.1 [SEGMENT-VERIFIED] — TERMINOLOGY COLLISION for the ASF bridge ledger.** ASF's bare **"logogenic agent"** = *any*
> language-constituted actuated agent (can be sophisticated). Vivarium's **"logogenic intelligence"** (Joseph) = *undifferentiated,
> none-to-primitive self-identity, pre-moral, bounded*. These are NOT the same set. ASF already distinguishes **primitive** vs
> **scaffolded** logogenic (`scope-primitive-logogenic` / `scope-scaffolded-logogenic`, not read) — vivarium's term almost certainly
> means ASF's **primitive logogenic**. **Action:** align vivarium "logogenic intelligence" → ASF "primitive logogenic," or the two
> senses will silently diverge. Log in the collision ledger (see [[asf-bridge]]).

> **FINDING 5.2 [SEGMENT-VERIFIED → SPECULATIVE] — The vivarium open question is an AAT identifiability question.** Vivarium's
> load-bearing unknown ("do LLM-induced perturbations on the formal agent state stay legible enough to measure adaptation, or wreck
> identifiability?") = **can a logogenic endo agent be moved off Class 3 (Coupled, non-separable) toward Class 1/2 (separated,
> legible)?** The relevant machinery is `der-directed-separation` (κ_processing) and `der-architecture-noidentifiability` (both named,
> not read). If Class 3 is structural-and-fixed for transformers, the perturbations are *not* cleanly separable and identifiability is
> threatened — which is precisely the empirical risk the first vertical slice exists to test. **This is the sharpest bridge point:
> the vivarium-as-AAT-sandbox bet reduces to a directed-separation measurement on logogenic endo agents.** Verify against the two segments.

---

## 6 · Kingdom properties, formalized `[SPIKE]`

- **Realized(K)** := T is fixed (immutable). `[SEGMENT-VERIFIED consequence]` N-MUT(T) is not an endomorphism on K — changing T
  yields K̃ with T̃ ≠ T, i.e. a *different* coupling. So **law-mutation is kingdom-*creation*, type-distinct from state-mutation**
  (matches the sketch's `LAW/N-MUT = (CREATION)✗`). This is why even an exo/developer cannot mutate a kingdom's law "from within" — it
  is ill-typed as an operation *on* K.
- **Lawful(K)** := T is *consistent and total* — every Ω_t has a well-defined lawful successor, no undefined/contradictory configs
  ("no glitches"). Consistency, not completeness.
- **Complete(K)** := every truth expressible in T's language is derivable within T. For arithmetically-rich T this is **impossible**
  (Gödel-1); and K cannot certify its own consistency (Gödel-2).
- **Closed(K) vs Open(K)** := **NEW axis this formalization surfaces.** Closed = Ω evolves autonomously from seed under T (replayable).
  Open = Ω depends on an exogenous forcing u(t) sourced from a higher kingdom: `Ω_{t+1} ~ T(·∣Ω_t, a_t) ⊕ u(t)`.

> **FINDING 6.1 [SPIKE] — Realized ⊥ Lawful (Joseph's deconflation is correct) AND it forces a small edit to the Kingdom definition.**
> The 2×2 {frozen?} × {consistent?} is genuinely independent (frozen-buggy = the common shipped case; incomplete-becoming-lawful =
> world-building; frozen-consistent = the aspiration). But Joseph's own Kingdom definition says "immutable governing law" — which makes
> immutability *definitional*, contradicting "incomplete kingdom = law still under revision." **Resolution:** drop "immutable" from the
> definition of *Kingdom-in-general*; immutability becomes the **Realized** property. Kingdom (general) has a law that may still be
> forming; Realized = the freeze. This removes the latent inconsistency and matches the deconflation.

> **FINDING 6.2 [SPIKE] — The NEW Closed/Open axis is what actually governs replayability and patchability** — and it plugs the two
> loose ends (the exogenous-*coupling* branch and the fork/patch lifecycle) into each other. An exogenous coupling (Earth-weather →
> vivium-weather) is formally a **non-autonomous forcing u(t)** — a term in the effective law, not an agent (this is where the non-agent
> Exogene lives: *not* in the agent access-matrix at all). Consequence: **Open(K) breaks determinism-from-seed → breaks replay**, which
> is *exactly* Joseph's worry ("patches so that perturbances can't be replayed up to the present"). Refinement: Open ∧ *recorded* u →
> replayable (log the forcing); Open ∧ *live* u → not. So the practical patch/fork question is governed by whether K is Closed, or Open
> with recorded forcing.

> **FINDING 6.3 [SPIKE] — "Lawful" is endo-undecidable → it is itself an N-VIS=Revelation property.** Two independent obstructions: (i)
> Φ(K,b) is lossy, so many noumena are consistent with b's phenomenon — b can't pin which T it lives under; (ii) Gödel-2 — even with full
> phenomenal access, a b whose reasoning is a subsystem of T cannot certify T's consistency. Hence *whether one's kingdom is lawful* is a
> noumenal fact reachable only by revelation. **The incompleteness nuance and the access matrix are the same fact seen twice.** Corollary
> `[SPECULATIVE]`: a correct endo belief in a higher kingdom is necessarily *faith-structured* — true, possibly revelation-warranted, never
> self-verifiable (a true-but-underdetermined belief). The exo agent functions as a **Gödelian oracle** supplying K's unprovable truths.

---

## 7 · Findings summary (ranked by surprise × solidity)

1. **Exo/endo = the AAT-applicability boundary; the taxonomy is AAT's structural complement** (F2.1, F2.2). Highest-value. AAT = endo
   theory assuming no boundary violation; the taxonomy = the catalog of exo boundary-violations AAT brackets. Tells us where each lives.
2. **The vivarium bet reduces to a directed-separation measurement on logogenic endo agents** (F5.2). The "legibility" open question *is*
   the Class-3/identifiability question. Sharpest actionable bridge — go read `der-directed-separation` + `der-architecture-noidentifiability`.
3. **New Closed/Open kingdom axis governs replay/patch; the non-agent Exogene = a forcing u(t), not a matrix agent** (F6.2). Resolves two
   loose ends at once.
4. **"Lawful" is endo-undecidable = N-VIS=Revelation; exo = Gödelian oracle** (F6.3). The incompleteness worry *shrinks* and snaps onto the matrix.
5. **Logogenic terminology collision** (F5.1) — align vivarium "logogenic intelligence" → ASF "primitive logogenic." Ledger item.
6. **Realized ⊥ Lawful confirmed; forces dropping "immutable" from the Kingdom-general definition** (F6.1).
7. **Ethereal/corporeal = AAT cascade tiers; minimal agency = the agency-scope floor** (F4.1). **Rebirth = two independent estate-transports
   + a phenomenal-continuity seam condition** (F4.2). **A miracle is a META-evidence channel** (F3.3).

## 8 · What this spike did NOT verify (honesty ledger)

- Class 1/2/3, κ_processing, directed separation, no-identifiability: taken from `scope-logogenic-agent` (read) + NOTATION; the
  **der-** segments themselves unread. F5.2 is the load-bearing one — verify next.
- Pearl do(·) hierarchy (`def-pearl-causal-hierarchy`), agent spectrum (`def-agent-spectrum`), primitive/scaffolded logogenic,
  imperium-arbitrium (ELI internal split) — named, not read. The logogenic collision (F5.1) needs the primitive/scaffolded segments to close.
- All `[SPIKE]` findings are spike-rigor: they'd each need to become a proper AAT **derivation** (deps-verified) before load-bearing use.
- The est-tiw / event-segmentation ↔ Aporia (δ_t) cadence link (from [[est-tiw-dossier]]) is asserted at the notation level; not derived.

**Next:** read the four `der-*` separation/identifiability segments to settle Finding 5.2 (the actual bet), and land F5.1 + F6.x
in the ASF collision ledger.
