# Event Segmentation, Temporal Integration Windows, and Cross-Species Timing — Research Dossier

*Compiled 2026-07-06 for vivarium (ASF/AAT simulation proving-ground). Citation-grounded; every claim tiered.*

**Confidence tiers used throughout:** `[WELL-ESTABLISHED]` = reported consistently across primary sources/reviews; `[CONTESTED]` = real debate or mixed evidence in peer-reviewed literature; `[SPECULATIVE]` = an author's own hypothesis presented as such, or a synthesis *we* would be making that no single source asserts. All citations were retrieved and cross-checked against publisher/PubMed/arXiv metadata during research; the few unverified ones are flagged inline.

---

## Bottom line (read this first)

1. **Both halves of the anchoring contrast are real and citable.** Sensory *temporal resolution* (critical flicker-fusion, CFF) robustly **scales with body size / metabolic rate** — small fast animals resolve faster (Healy et al. 2013). A *cognitive/behavioral segmentation unit* of roughly **2–3 s** has been claimed to be **approximately size-independent** across mammals (Gerstner & Fazio 1995; Pöppel's ~3 s "subjective present") — but on a thin, dated, small-N evidence base, never tested with CFF-style phylogenetic rigor.
2. **Event Segmentation Theory (EST) is a mature, prediction-error-based framework** (Zacks & Tversky 2001; Zacks et al. 2007; Zacks 2020) with real computational models (Reynolds/Zacks/Braver 2007; Franklin et al. 2020 SEM; Baldassano et al. 2017).
3. **EST is already being applied to LLM memory** — the standout is **EM-LLM (ICLR 2025)**, which segments an LLM token stream at Bayesian-surprise (prediction-error) boundaries and retrieves episodically over ~10M tokens. This is the highest-value item for vivarium's context-architecture question.
4. **The Amichay et al. (2026) PLOS Biology citation is VERIFIED** and essentially as described — with one framing correction: the *observed signal cluster* is 0.5–4 Hz peaking near ~3 Hz (median 3.45 Hz); the **~2 Hz is the neural-resonance / Arnold-tongue center**, not the observed-tempo peak. Don't conflate the two.
5. **The key open question (#3) is a confirmed gap:** *no published work connects the EST / specious-present / temporal-integration-window literature to the ~2 Hz animal-communication-tempo literature.* Any synthesis linking them is novel and must be marked `[SPECULATIVE]`.

**Tiered-confidence note on the key open question:** that temporal *resolution* scales with size is `[WELL-ESTABLISHED]`; that a ~2–3 s segmentation unit is size-*independent* is `[CONTESTED]` (asserted + locally supported, not rigorously tested); that no one has bridged the two literatures is a **confirmed negative** (verified by reading the 2026 paper's references and by failed bidirectional searches).

---

## 1. Event Segmentation Theory (EST)

### 1.1 Canonical sources — `[WELL-ESTABLISHED]`

- **Zacks, J. M., & Tversky, B. (2001).** Event structure in perception and conception. *Psychological Bulletin*, 127(1), 3–21. DOI: 10.1037/0033-2909.127.1.3. — Conceptual foundation: events, like objects, have taxonomies and partonomies; segmentation is hierarchical.
- **Zacks, J. M., Speer, N. K., Swallow, K. M., Braver, T. S., & Reynolds, J. R. (2007).** Event perception: A mind-brain perspective. *Psychological Bulletin*, 133(2), 273–293. DOI: 10.1037/0033-2909.133.2.273. — **The canonical statement of EST proper.**
- **Zacks, J. M., & Swallow, K. M. (2007).** Event segmentation. *Current Directions in Psychological Science*, 16(2), 80–84. DOI: 10.1111/j.1467-8721.2007.00480.x. — Compact entry point.
- **Kurby, C. A., & Zacks, J. M. (2008).** Segmentation in the perception and memory of events. *Trends in Cognitive Sciences*, 12(2), 72–79. DOI: 10.1016/j.tics.2007.11.004. — Best short review.
- **Zacks, J. M. (2020).** Event perception and memory. *Annual Review of Psychology*, 71, 165–191. DOI: 10.1146/annurev-psych-010419-051101. — **Cite this for "current state of the theory."**

### 1.2 The prediction-error / "event model in working memory" mechanism — `[WELL-ESTABLISHED]` (core claim); driver `[CONTESTED]`

Per Zacks et al. 2007: perceptual systems **continuously predict** the near future using an **event model** held in working memory. The model is held stable ("perceptual anchoring") while predictions succeed; when prediction error **transiently spikes**, an event boundary is perceived and the working-memory model is **updated**. The *transience* is load-bearing — Reynolds/Zacks/Braver 2007 found boundaries are better recovered from the *change* in prediction error (normalized to a running average) than from instantaneous error.

- Empirical grounding: **Zacks, Kurby, Eisenberg, & Haroutunian (2011).** Prediction error associated with the perceptual segmentation of naturalistic events. *J. Cognitive Neuroscience*, 23(12), 4057–4066. DOI: 10.1162/jocn_a_00078.
- `[CONTESTED]` **Clewett et al. (2025).** Prediction error is out of context: the dominance of contextual stability in structuring episodic memories. *Psychonomic Bulletin & Review*. DOI: 10.3758/s13423-025-02723-4. — Argues *contextual stability/change*, not prediction error per se, dominates. Treat "prediction error causes segmentation" as leading-but-debated (see also §5 latent-cause vs. Bayesian-surprise debate).

### 1.3 Event boundaries and memory

- **Boundary-enhanced encoding / attentional gating — `[WELL-ESTABLISHED]`:** **Swallow, K. M., Zacks, J. M., & Abrams, R. A. (2009).** Event boundaries in perception affect memory encoding and updating. *J. Experimental Psychology: General*, 138(2), 236–257. DOI: 10.1037/a0015631. (A published correction fixed an eccentricity-covariate issue; authors state major conclusions unaffected.)
- **Neural boundary-locked activity — `[WELL-ESTABLISHED]`:** **Speer, N. K., Zacks, J. M., & Reynolds, J. R. (2007).** Human brain activity time-locked to narrative event boundaries. *Psychological Science*, 18(5), 449–455. DOI: 10.1111/j.1467-9280.2007.01920.x.
- **Doorway / location-updating effect — phenomenon `[WELL-ESTABLISHED]`, robustness `[CONTESTED]`:**
  - **Radvansky, G. A., & Copeland, D. E. (2006).** Walking through doorways causes forgetting: Situation models and experienced space. *Memory & Cognition*, 34(5), 1150–1156. DOI: 10.3758/BF03193261.
  - **Radvansky, G. A., Tamplin, A. K., & Krawietz, S. A. (2010).** Walking through doorways causes forgetting: Environmental integration. *Psychonomic Bulletin & Review*, 17(6), 900–904. DOI: 10.3758/PBR.17.6.900.
  - **Radvansky, G. A., Krawietz, S. A., & Tamplin, A. K. (2011).** Walking through doorways causes forgetting: Further explorations. *Quarterly J. Experimental Psychology*, 64(8), 1632–1645. DOI: 10.1080/17470218.2011.571267.
  - *Honest caveat:* large-scale replications of the doorway *memory decrement* have been mixed; the phenomenon and its location-updating interpretation are canonical, but treat the **effect size/robustness as contested**.
- **Boundaries as costs AND benefits — `[CONTESTED]`:** the field now frames boundaries as a trade-off (enhance boundary-item + item–context binding; can impair across-boundary temporal-order/associative memory), retrieval-context-dependent. E.g. "Event boundaries: costs and benefits for memory," *Cognition* (2025), DOI: 10.1016/j.cognition.2025.106178.
- **Review anchor — `[WELL-ESTABLISHED]`:** "Prediction error and event segmentation in episodic memory," *Neuroscience & Biobehavioral Reviews* (2024), 158, 105533. DOI: 10.1016/j.neubiorev.2024.105533.

### 1.4 Goal structure in segmentation — `[WELL-ESTABLISHED]` (as one causal dimension)

Changes in actor *goals/intentions* are a primary conceptual cause of perceived boundaries (alongside perceptual/spatial change), stated across Zacks & Tversky 2001, Zacks et al. 2007, Zacks 2020. Most targeted primary source:
- **Zacks, J. M. (2004).** Using movement and intentions to understand simple events. *Cognitive Science*, 28(6), 979–1008. DOI: 10.1207/s15516709cog2806_5.

Hierarchical goal/subgoal structure maps onto coarse-vs-fine segmentation (Kurby & Zacks 2008). *Gap flagged:* there is **no standalone flagship "goal structure in segmentation" paper** distinct from the general EST corpus — the topic lives inside the main theory papers. (Not inventing one.)

### 1.5 Computational / neural-network models — `[WELL-ESTABLISHED]`

- **Reynolds, J. R., Zacks, J. M., & Braver, T. S. (2007).** A computational model of event segmentation from perceptual prediction. *Cognitive Science*, 31(4), 613–643. DOI: 10.1080/15326900701399913. — Original RNN implementation.
- **Franklin, N. T., Norman, K. A., Ranganath, C., Zacks, J. M., & Gershman, S. J. (2020).** Structured Event Memory (SEM): A neuro-symbolic model of event cognition. *Psychological Review*, 127(3), 327–361. DOI: 10.1037/rev0000177. — **Leading current model:** probabilistic generative model over structured scenes; Bayesian inference over boundaries; learns schemata. *Directly relevant to vivarium's two-layer-mind question* — a principled bridge between a formal event/state representation and neural embeddings.
- **Baldassano, C., Chen, J., Zadbood, A., Pillow, J. W., Hasson, U., & Norman, K. A. (2017).** Discovering event structure in continuous narrative perception and memory. *Neuron*, 95(3), 709–721.e5. DOI: 10.1016/j.neuron.2017.06.041. — Data-driven HMM finding neural event boundaries as shifts between stable activity patterns; reveals a nested cortical timescale hierarchy + hippocampal boundary responses tied to memory.
- **Event-predictive ML line (Butz/Gumbsch, Tübingen)** — `[WELL-ESTABLISHED]` as published, `[SPECULATIVE]` on how tightly each maps to EST: Butz et al. (2021), Event-predictive cognition, *Topics in Cognitive Science* 13(1), 10–24, DOI: 10.1111/tops.12522; Gumbsch, Butz & Martius (2021), GateL0RD (sparsely-changing latent states), *NeurIPS 2021*, arXiv:2110.15949; Gumbsch et al. (2024), THICK world models (adaptive temporal abstractions), *ICLR 2024*.
- **Kumar et al. (2023).** Bayesian surprise predicts human event segmentation in story listening. *Cognitive Science* (PMC11654724). — Links a computational surprise signal to human boundary judgments.
- *Map of the model space:* "A Review of Mechanistic Models of Event Comprehension," arXiv:2409.18992 (2024).
- *Gap:* **Metcalf & Leake** — research program (Indiana Univ., event-schema/segmentation in case-based reasoning) is real, but **no single specific paper title/year/DOI could be confidently verified.** Flag rather than cite.

### 1.6 ★ HIGHEST-VALUE: EST applied to AI ★

Sorted by how *explicitly* the work grounds itself in EST.

**Directly EST-grounded LLM memory — `[WELL-ESTABLISHED]` that these exist and self-describe as EST-inspired:**
- **★ Fountas, Z., et al. (2025). Human-inspired Episodic Memory for Infinite Context LLMs (EM-LLM).** *ICLR 2025.* arXiv:2407.09450; code: github.com/em-llm/EM-LLM-model. — **The flagship result for vivarium.** Segments the LLM token stream into episodic "events" online using **Bayesian surprise** (prediction-error spikes) + graph-theoretic boundary refinement, then retrieves via two-stage (similarity + temporal-contiguity) — a near-direct import of EST + episodic-memory theory. Retrieval across ~10M tokens; beats InfLLM and often full-context on LongBench / ∞-Bench. **Strongest existing proof-of-concept that EST-style surprise segmentation improves an AI memory system.**
- **Zou, H., Sun, T., He, C., et al. (2026). ES-Mem: Event Segmentation-Based Memory for Long-Term Dialogue Agents.** arXiv:2601.07582. — Explicitly "drawing inspiration from Event Segmentation Theory"; dynamic segmentation + hierarchical memory with "boundary semantics." Very recent, **preprint/unreviewed** — `[SPECULATIVE]` on results.

**LLMs *as* event segmenters (cross-validating theory ↔ model) — `[WELL-ESTABLISHED]`:**
- **Michelmann, S., Kumar, M., Norman, K. A., & Toneva, M. (2025).** Large language models can segment narrative events similarly to humans. *Behavior Research Methods*, 57. DOI: 10.3758/s13428-024-02569-z. arXiv:2301.10297. — GPT-3 segments narrative into events correlated with human consensus (often closer to consensus than individual humans).
- **Panela, R. A., Barnett, A. J., Barense, M. D., & Herrmann, B. (2025).** Event Segmentation Applications in LLM-Enabled Automated Recall Assessments. arXiv:2502.13349 (a version in *Communications Psychology*; exact vol/DOI unconfirmed behind auth redirect). — Human–LLM segmentation agreement reportedly exceeds human–human.

**Event-predictive / surprise-based segmentation in RL & world models — `[CONTESTED]` whether "EST-grounded" vs. convergent:**
- Gumbsch GateL0RD / THICK (above) — clearest EST↔RL bridge; event-triggered latent updates function as options/temporal abstractions.
- "Enhancing Hierarchical RL through Change Point Detection in Time Series," arXiv:2510.24988 (2025) — boundaries from prediction-error/reward spikes to discover options/subgoals. **Convergent with EST but does not cite it** — mark the "EST → HRL options" linkage `[SPECULATIVE]` framing.
- Robotics: "Worth Remembering: Surprise-Gated Robot Episodic Memory," arXiv:2606.03787 (2026).

**Adjacent but NOT EST-grounded (flagged to prevent overclaim):**
- **Park, J. S., et al. (2023). Generative Agents.** *UIST '23.* DOI: 10.1145/3586183.3606763. — "Memory stream" scored by recency × importance × relevance; memories are **per-observation, NOT segmented at prediction-error boundaries.** Calling it EST-based would be an overclaim.
- **Packer, C., et al. (2023). MemGPT.** arXiv:2310.08560. — OS/virtual-memory-inspired tiered context; **not** surprise-segmented.

**Vivarium takeaway:** the two most transferable primitives are (1) **surprise/prediction-error-triggered boundaries for the cognitive level-of-detail hand-off** (EM-LLM and Gumbsch's event-triggered latent updates are the concrete engineering precedents), and (2) **the SEM generative-model formulation** (Franklin et al. 2020) as a principled bridge between formal event/state and neural embeddings. No published work applies EST to the specific vivarium/AAT use case (LLM-perturbed formal-state legibility) — that intersection appears open.

---

## 2. Temporal Integration Window (TIW) / the "specious present"

### 2.1 Pöppel's two-level hierarchical model — primary sources `[WELL-ESTABLISHED]`

Temporal perception is built hierarchically from elementary experiences (simultaneity, successiveness, temporal order, subjective present, continuity, duration), on two processing levels:
- **Lower level — "system states" / elementary integration (~30 ms):** ~30 ms functional system states (high-frequency neuronal oscillations) define successiveness; temporal-order judgments need an additional ~30–40 ms.
- **Upper level — temporal integration / subjective present (~2–3 s):** binds successive events into perceptual units of ~3 s constituting the "subjective present."

- **Pöppel, E. (1997).** A hierarchical model of temporal perception. *Trends in Cognitive Sciences*, 1(2), 56–61. DOI: 10.1016/S1364-6613(97)01008-5. — Canonical statement.
- **Pöppel, E. (2009).** Pre-semantically defined temporal windows for cognitive processing. *Phil. Trans. R. Soc. B*, 364(1525), 1887–1896. DOI: 10.1098/rstb.2009.0015. — Restates both mechanisms; the ~2–3 s window is claimed **pre-semantic** (fixed independent of stimulus meaning).
- **Wittmann, M., & Pöppel, E. (1999/2000).** Temporal mechanisms of the brain as fundamentals of communication… *Musicae Scientiae*, 3(1_suppl), 13–28. DOI: 10.1177/10298649000030S103. — Applies both windows to music/communication.

### 2.2 Evidence for the ~2–3 s window — phenomena `[WELL-ESTABLISHED]`; single-common-mechanism `[SPECULATIVE]`

- **Poetic line/verse length (flagship cross-cultural datum):** **Turner, F., & Pöppel, E. (1983).** The Neural Lyre: Poetic Meter, the Brain, and Time. *Poetry*, 142(5), 277–309 (Levinson Prize 1983). Across many unrelated languages, the metric line takes ~2.5–3.5 s to recite (peak ~3 s). (Reported measurement `[WELL-ESTABLISHED]`; "neural lyre" causal interpretation `[SPECULATIVE]`. Exact page range from secondary indexes, not confirmed against print issue.)
- **Spontaneous speech / intentional-action segmentation ~3 s** `[WELL-ESTABLISHED as reported]`. See also Schleidt, Eibl-Eibesfeldt & Pöppel (1987), "A universal constant in temporal segmentation of human short-term behaviour," *Naturwissenschaften*.
- **Duration reproduction** `[WELL-ESTABLISHED]`: veridical up to ~2–3 s, errors grow beyond; Weber's law for time holds to ~2–3 s then breaks.
- **Sensorimotor synchronization** `[WELL-ESTABLISHED]`: works for inter-stimulus intervals ~250 ms to ~2–3 s, then anticipatory synchronization is lost.
- **Bistable-figure reversals (~3 s)** `[CONTESTED]` — see §2.4.
- **Naturalistic extension:** Wöllner et al. (2014), Temporal Integration Windows for Naturalistic Visual Sequences, *PLOS ONE* 9(7):e102248, DOI: 10.1371/journal.pone.0102248.
- **Modern synthesis:** **Wittmann, M. (2011).** Moments in Time. *Frontiers in Integrative Neuroscience*, 5, 66. DOI: 10.3389/fnint.2011.00066. — Distinguishes the sub-second "functional moment" from the ~2–3 s "experienced moment" and a longer narrative "mental presence." *(Citation correction: the task brief attributed "Moments in Time" to Nature Reviews Neuroscience 2011 — that conflates two papers. "Moments in Time" is Front. Integr. Neurosci. 2011; the NRN paper is Wittmann 2013, below.)*

### 2.3 Neural basis — `[SPECULATIVE / partially CONTESTED]`

This is the weakest-specified part of the model; Pöppel/Wittmann are candid about it.
- Pöppel (2009) proposes low-frequency oscillations + **thalamocortical** loops for the 2–3 s window (vs. gamma-range for the ~30 ms window) — explicitly hypothesized.
- Wittmann (2011) invokes **working memory** as the substrate, declining specific structures.
- Adjacent duration account: **Wittmann, M. (2013).** The inner sense of time: how the brain creates a representation of duration. *Nature Reviews Neuroscience*, 14, 217–223. DOI: 10.1038/nrn3452. — "Climbing neural activation"/accumulator models, **insular cortex** as key hub. `[CONTESTED]` — one of several competing models (striatal SBF, cerebellar, distributed intrinsic).
- **Honest gap:** no primary source pins the 2–3 s window to a single, replicated, specific neural circuit. Treat any claim of a definite "3-second neural clock" as unsupported.

### 2.4 Critiques / contested aspects

- `[CONTESTED]` **Exactness of "3 s":** bistable-perception is the most attacked plank. Necker-cube mean percept durations are often ~5 s, not ~3 s, with large variance. Interrupted-presentation studies suggest reversals reflect **local visual adaptation/fatigue**, not a general integrator (*PLOS ONE* 2020, DOI: 10.1371/journal.pone.0227506; bioRxiv 2023 / PMC10268006).
- `[CONTESTED]` **One mechanism or many?** Cross-domain phenomena sit at different actual values (~2–5 s) and may have independent causes; the unification into one "pre-semantic integration window" is inference, not demonstration (Peter White's "three-second moment" critiques, Cardiff ORCA).
- `[CONTESTED]` **"Subjective present" debate:** literature increasingly distinguishes multiple "now" timescales (functional ~sub-second; experienced ~2–3 s; narrative ~tens of seconds) — richer than a single 3 s window; Wittmann (2011) himself develops this.

### 2.5 Background: "specious present"
Coined by **"E.R. Clay"** (pseudonym of E. Robert Kelly), *The Alternative* (1882); popularized by **William James**, *Principles of Psychology* (1890), ch. 15. Philosophical ancestor of Pöppel's empirical "subjective present." (Background only.)

---

## 3. ★ THE KEY OPEN QUESTION: cross-species, size-independent segmentation/integration rate? ★

**Structure of the answer:** (a) temporal *resolution* DOES scale with size — the contrast case; (b) a size-*independent* ~2–3 s segmentation unit HAS been proposed, thinly; (c) **no one has bridged this to the ~2 Hz communication-tempo literature — confirmed gap.**

### 3.1 Sensory temporal resolution scales WITH size/metabolism (the contrast) — `[WELL-ESTABLISHED]`

- **Healy, K., McNally, L., Ruxton, G. D., Cooper, N., & Jackson, A. L. (2013).** Metabolic rate and body size are linked with perception of temporal information. *Animal Behaviour*, 86(4), 685–696. DOI: 10.1016/j.anbehav.2013.06.018. — Phylogenetic analysis, 34 vertebrate species / 6 classes; best model R²≈0.79. CFF increases with mass-specific metabolic rate (coef 13.20±4.02, p<0.005), decreases with body mass (~2 Hz per 10 kg); range ~14 Hz (eel) to ~120 Hz (ground squirrel). *(Middle-author names render inconsistently across databases — "McNally" per PMC/St Andrews portal, occasionally "McNamara." Verify against DOI before print.)*
- Review corroboration: **Donner, K. (2021).** Temporal vision: measures, mechanisms and meaning. *J. Experimental Biology*, 224(15), jeb222679. DOI: 10.1242/jeb.222679.
- Cross-taxa compilation: Inger et al. (2014), *PLOS ONE* (~81 species; insects > vertebrates). Methodology caveats: "A flashing light may not be that flashy," *PLOS ONE* 2022, DOI: 10.1371/journal.pone.0279718.

**→ Temporal *resolution* is a size/metabolism-scaled quantity. This is the clean contrast to a size-independent rate.**

### 3.2 A size-INDEPENDENT ~2–3 s segmentation unit HAS been proposed — `[CONTESTED]` (real but thin)

- **Gerstner, G. E., & Fazio, V. A. (1995). Evidence of a Universal Perceptual Unit in Mammals.** *Ethology*, 101(2), 89–100. DOI: 10.1111/j.1439-0310.1995.tb00348.x. — **Strongest direct hit.** Measured vigilance "stare" posture durations in **7 mammals spanning giraffe → okapi → roe deer → raccoon → capuchin → red panda → kangaroo**; mean stares ~2.1–3.6 s, argued as a "universal perceptual unit" ~3 s **independent of large body-size differences.** *(Full text paywalled — HTTP 402; exact numbers/species from secondary summaries. Verify against PDF before quoting.)*
- **Pöppel (1997)** (above) — the ~2–3 s window proposed as pre-semantic, content-independent.
- **Schleidt, M., & Kien, J. (1997).** Segmentation in behavior and what it can tell us about brain function. *Human Nature*, 8(1), 77–111. DOI: 10.1007/s12110-997-1005-7. — Action units ~0.3–12 s (mostly 1–4 s), cross-cultural in humans, reported in non-human primates/other mammals.
- Human-only modern TIW work (naturalistic sequences; MMN-indexed subjective present) reaffirms a robust 2–3 s window but establishes existence, **not** size-invariance across species.

**Critical caveat `[CONTESTED]`:** the size-*independence* claim rests essentially on Gerstner & Fazio (1995, n=7) + the Pöppel/Schleidt human corpus. **No modern, large-N, phylogenetically-controlled study tests whether a segmentation/integration rate is *constant across body size* the way Healy et al. (2013) tested CFF.** The distinct finer "psychological/perceptual moment" (~50–60 ms, Stroud lineage) is a *different* timescale whose cross-species constancy is explicitly uncertain. (The "Constant Lifespan Theory" is `[SPECULATIVE]`, low-quality venue — flag, don't rely.)

### 3.3 Has anyone connected EST/TIW to the ~2 Hz communication-tempo literature? — **CONFIRMED GAP**

**No.** Verified two ways:
1. **Read the 2026 paper's references directly:** Amichay et al. (2026) does **not** cite or mention Pöppel, the specious present, the 3 s integration window, Zacks/EST, or the "perceptual moment." Its integration-window argument is framed purely at the ~300 ms single-neuron biophysics level (delta-band tracking) — it does **not** connect its ~2 Hz tempo to the ~2–3 s cognitive segmentation window or to any cross-species event-segmentation rate.
2. **Bidirectional searches failed:** "event segmentation" + "communication tempo"/"2 Hz"; "specious present"/"3-second window" + "isochronous"/"delta oscillation"/"animal communication"; "Kuramoto" + "event segmentation" — all returned only the Amichay paper (which makes no link) and the human TIW literature (which never mentions communication tempo). The two literatures co-cite neither each other nor a common bridging source.

**Therefore, stated plainly:** *No published work connects the Event-Segmentation-Theory / specious-present / temporal-integration-window (Pöppel ~3 s) literature to the ~2 Hz animal-communication-tempo literature.* The only structural coincidence is that both invoke "integration windows" — but at **different timescales** (~2–3 s cognitive segmentation vs. ~300 ms receiver-neuron integration → ~2 Hz optimum) and neither cites the other. **Any synthesis arguing these are the same phenomenon or a nested hierarchy would be novel — mark it `[SPECULATIVE]`.**

### 3.4 What appears genuinely unstudied (valuable negatives)
1. **No CFF-style allometry test of segmentation rate** — size-independence of a cognitive integration/segmentation rate has never been tested with Healy-style phylogenetic rigor. It rests on Gerstner & Fazio (1995), n=7.
2. **No link between the two "integration window" timescales across species** — ~300 ms receiver-integration (Amichay 2026) and ~2–3 s specious present (Pöppel) live in fully separate literatures; whether they form a nested hierarchy in animals is unaddressed.
3. **No connection at all between EST and communication tempo** — confirmed absent.
4. **Comparative event segmentation in non-human animals is itself nascent** — mostly inferred from rodent hippocampal/place-cell context boundaries; the raw material to test size-independence barely exists yet.

---

## 4. Citation verification — Amichay, Balasubramanian & Abrams (2026)

**VERDICT: VERIFIED** (with one framing nuance to correct).

**Corrected full citation:**
> Amichay, G., Balasubramanian, V., & Abrams, D. M. (2026). A widespread animal communication tempo may resonate with the receiver's brain. *PLOS Biology*, 24(4): **e3003735**. DOI: 10.1371/journal.pbio.3003735. Published April 14, 2026.

- Add the **eLocation ID e3003735** (the handed citation omitted it).
- **Volume/year internally consistent:** PLOS Biology vol. 23 = 2025, so vol. 24 = 2026; April = issue 4. No discrepancy.
- **Preprint lineage:** arXiv:2508.21530 [q-bio.NC] (submitted Aug 29, 2025) under the stronger title "A **universal** animal communication tempo resonates with the receiver's brain" — published version softened "universal"→"widespread" and "resonates"→"may resonate." Also PMC13078620.
- **Authors (all confirmed):** Guy Amichay (Northwestern, lead); Vijay Balasubramanian (UPenn / Santa Fe Institute / Oxford); Daniel M. Abrams (Northwestern).

**Claim-by-claim:**
- ✅ Widespread/near-universal, **size-independent** tempo — confirmed; persists "across 8 orders of magnitude in body weight."
- ✅ ~0.5–4 Hz range across many taxa — confirmed; explicitly the "delta band" / tempo "hotspot."
- ⚠️ **"Clustering around ~2 Hz" — PARTIAL / needs correction.** The *observed* tempo distribution peaks nearer **~3 Hz (median 3.45 Hz)**, not 2 Hz. The **~2 Hz is the neural-resonance (Arnold-tongue) center**, not the observed-signal cluster. **Do not conflate the two.**
- ✅ Attributed to receiver-neuron biophysics (integration/recovery limits, "a few hundred ms") — confirmed.
- ✅ Kuramoto oscillators — confirmed (small receiver circuits).
- ✅ Arnold tongue at ~2 Hz — confirmed; "widens as input strength increases," no harmonic tongues; the 2 Hz is the circuit's intrinsic resonance center.

---

## 5. Adjacent unifying concept: biological systems as change / prediction-error detectors

### 5.1 The principle and its frameworks

**No single review gathers all of** {spike-frequency adaptation, Weber–Fechner, habituation, allostasis, change/slow-change blindness, event segmentation} **under one "change-detector" banner** — searched, not found. What exists are partially-overlapping unifiers:
- `[WELL-ESTABLISHED]` **Efficient coding (root):** Barlow, H. B. (1961). Possible principles underlying the transformation of sensory messages. In *Sensory Communication* (pp. 217–234). MIT Press. — Redundancy reduction: neurons should represent the *changing/surprising* residual. (Book chapter, no DOI.)
- `[WELL-ESTABLISHED]` **Predictive coding (mechanistic unifier):** Rao, R. P. N., & Ballard, D. H. (1999). Predictive coding in the visual cortex… *Nature Neuroscience*, 2(1), 79–87. DOI: 10.1038/4580. — Feedback carries predictions; feedforward carries the residual *error*; derives adaptation-like effects as change-encoding.
- `[WELL-ESTABLISHED]` framework; universal-scope claim `[CONTESTED]` — **Free-energy / predictive processing:** Friston, K. (2010). The free-energy principle: a unified brain theory? *Nature Reviews Neuroscience*, 11(2), 127–138. DOI: 10.1038/nrn2787. And **Clark, A. (2013).** Whatever next? Predictive brains, situated agents… *Behavioral and Brain Sciences*, 36(3), 181–204. DOI: 10.1017/S0140525X12000477. — Closest thing to the "review you want," but argues the *general principle*; does not enumerate adaptation + change-blindness + segmentation as one list.

> **Honesty flag:** connecting those several phenomena as one principle is a *synthesis you would be making*, licensed by Barlow → Rao–Ballard → Friston/Clark — not something with a single off-the-shelf citation. Assert the component claim (`[WELL-ESTABLISHED]`: predictive coding treats adaptation/surround-suppression as change/prediction-error encoding); mark the full enumeration `[SPECULATIVE-as-single-framework]`.

### 5.2 Physiological scale: allostasis as predictive regulation — `[WELL-ESTABLISHED]`

The cleanest "regulate around anticipated change, not fixed absolute level":
- **Sterling, P., & Eyer, J. (1988).** Allostasis: A new paradigm… In *Handbook of Life Stress, Cognition and Health* (pp. 629–649). Wiley. (Origin of "allostasis": stability through change.)
- **Sterling, P. (2012).** Allostasis: A model of predictive regulation. *Physiology & Behavior*, 106(1), 5–15. DOI: 10.1016/j.physbeh.2011.06.004. — Brain predicts resource needs and pre-adjusts; error-*prevention* not correction.
- **McEwen, B. S. (1998).** Protective and damaging effects of stress mediators. *NEJM*, 338(3), 171–179. DOI: 10.1056/NEJM199801153380307. — Allostatic load (cumulative cost of chronic re-setting).
- **★ Bridge, allostasis → active inference:** **Barrett, L. F., & Simmons, W. K. (2015).** Interoceptive predictions in the brain. *Nature Reviews Neuroscience*, 16(7), 419–429. DOI: 10.1038/nrn3950. — Casts interoception/allostatic control as active inference over an internal generative model. Load-bearing for the "change-detection spans neuron → body → cortex" arc.

### 5.3 Predictive coding / active inference — exact citations
- Rao & Ballard (1999); Friston (2010) — above.
- **Friston, K., FitzGerald, T., Rigoli, F., Schwartenbeck, P., & Pezzulo, G. (2017).** Active inference: a process theory. *Neural Computation*, 29(1), 1–49. DOI: 10.1162/NECO_a_00912.
- **Parr, T., Pezzulo, G., & Friston, K. J. (2022).** *Active Inference: The Free Energy Principle in Mind, Brain, and Behavior.* MIT Press. ISBN 978-0-262-04535-3.

### 5.4 Where EST sits relative to predictive coding / active inference
- `[WELL-ESTABLISHED]` **EST is prediction-error-driven by original construction** (Zacks et al. 2007): continuous prediction via WM event models; boundary when error transiently spikes.
- `[WELL-ESTABLISHED]` **Explicitly connected to generative/predictive inference:** Franklin et al. (2020) SEM is a probabilistic generative model inferring boundaries via Bayesian inference (the computational-level statement of predictive coding). **Shin, Y. S. (2021).** Structuring memory through inference-based event segmentation. *Topics in Cognitive Science*, 13(1), 106–127. DOI: 10.1111/tops.12505. Language bridge: **Kuperberg, G. R., & Jaeger, T. F. (2016).** What do we mean by prediction in language comprehension? *Language, Cognition and Neuroscience*, 31(1), 32–59. DOI: 10.1080/23273798.2015.1102299 (+ Kuperberg 2021, "Tea with milk?", *Topics in Cognitive Science* — exact vol/pages unverified).
- `[CONTESTED]` **What is the true boundary driver?** Prediction error vs. **Bayesian surprise / belief update** vs. **latent-cause inference** — actively debated (Shin & DuBrow 2021; Davachi-lab "boundaries without prediction error"; Clewett et al. 2025; survey arXiv:2409.18992). *Honest framing:* EST was built on prediction error and has been explicitly connected to predictive coding — but do not assert prediction error as the settled *sole* mechanism.

### 5.5 Hierarchical timescales ↔ hierarchical prediction — `[WELL-ESTABLISHED]`
- **Kiebel, S. J., Daunizeau, J., & Friston, K. J. (2008).** A hierarchy of time-scales and the brain. *PLoS Computational Biology*, 4(11), e1000209. DOI: 10.1371/journal.pcbi.1000209. — Theoretical (predictive-coding) claim: cortical hierarchy recapitulates the environment's temporal hierarchy.
- **Hasson, U., Yang, E., Vallines, I., Heeger, D. J., & Rubin, N. (2008).** A hierarchy of temporal receptive windows in human cortex. *J. Neuroscience*, 28(10), 2539–2550. DOI: 10.1523/JNEUROSCI.5487-07.2008. — Empirical TRW hierarchy (early sensory ~100s ms → higher areas ~minutes).
- **Hasson, U., Chen, J., & Honey, C. J. (2015).** Hierarchical process memory. *Trends in Cognitive Sciences*, 19(6), 304–313. DOI: 10.1016/j.tics.2015.04.006.

Together (Kiebel = theory, Hasson = empirics, Kurby & Zacks 2008 = multi-timescale segmentation) these support "hierarchical prediction across nested timescales, with event boundaries at each level" — a defensible synthesis, though no single paper asserts all three as one claim. **This trio is the most direct existing scaffold for vivarium's "temporal cognitive level-of-detail" architecture.**

---

## Consolidated gaps / unverified items (honesty ledger)
- **Metcalf & Leake** EST/CBR — program real, no single paper verified.
- **Gerstner & Fazio (1995)** exact numbers/species — from secondary summaries (paywall); verify against PDF.
- **Healy et al. (2013)** middle-author spelling (McNally vs. McNamara) — verify against DOI.
- **Panela et al. (2025)** *Communications Psychology* published-version vol/DOI — arXiv confirmed, Nature metadata not.
- **Laughlin (1981)** Weber–Fechner efficient coding; **Rensink et al. 1997 / Simons & Levin 1997** change-blindness classics; **Kuperberg 2021, Shin & DuBrow 2021, Davachi "boundaries without prediction error"** — real leads, exact metadata not re-verified this session; confirm before citing.
- **No single unifying review** of the §5 change-detector enumeration exists — that synthesis is ours to make.
- **No published EST↔communication-tempo bridge, and no CFF-style allometry of segmentation rate** — the two most valuable *negatives*.
