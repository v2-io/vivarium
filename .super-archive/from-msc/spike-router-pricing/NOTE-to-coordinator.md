# Note to coordinator — routing pricing experiment

**Status: done (incl. the receiver-tree arm you asked for), gates green, ready for
your review before commit.** Nothing committed (per brief). Files, all under my own
territory:

- `crates/vivarium-world/examples/router_pricing.rs` — the harness (new single-file
  example; auto-discovered; edits no shared file, no Cargo.toml change, no kernel hook).
- `msc/spike-router-pricing/{PREDICTIONS,RESULTS,DECISIONS-draft}.*`, this note, raw logs.

## The headline (the receiver-tree arm CHANGED the severity picture)

Adding `EdgeFull` (diagonals killed in the fan AND the D4 receiver/incision tree)
flipped the conclusion I would have shipped from the fan half alone. I've updated
RESULTS.md and the DECISIONS draft to the **combined present truth** (rewritten, no
correction trail). Three separately-priced parts:

- **Length fix (`6c1ad97`, shipped this session): REAL and now remedied.** Clean
  D4-verified cube-locked signal — CUBE +0.10 to +0.20 (corner axis de-biasing), ≈0
  at the face centre. The dominant *fixed* win.
- **Fan-diagonal kill: landscape-BENIGN.** CUBE −0.02 to −0.04.
- **Receiver-tree diagonal kill: landscape-CONSEQUENTIAL.** CUBE **+0.07 to +0.26**,
  comparable to the length fix — and the naive D4 tree is itself **not cube-safe**
  (it axis-locks *harder* at the corner). **This is exactly the reopening you
  predicted:** stopping at the fan half, "router successor is low-severity" would have
  been *wrong*.

**Net (revised): the router-successor question is NOT de-prioritised.** The
diagonal-kill's landscape action lives in the flux *tree*, not the fan; a naive D4
kill is not the answer; and FE(6c)'s **Coatléven edge-flux FV reconstruction** (the
principled remedy replacing fan *and* tree) is **unpriced** and now carries measured
stakes justifying it. The full FE(6b) surface *is* priced; FE(6c) is the owed next step.

## The one interpretive call I want you to sanity-check

`EdgeFull`'s receiver tree is a **naive** diagonal-kill (literal D4 steepest descent)
— a deliberate *strawman* for the true Coatléven FV router. I read its large CUBE
signal as "the tree treatment is consequential AND naive-D4 is not cube-safe," **not**
as "the principled remedy will also swing" (it might not — only FE(6c) pricing would
show that). RESULTS.md and the draft state that boundary explicitly. If you read the
strawman differently, that's the sentence to adjust before it lands.

The `6c1ad97`-landed-mid-experiment framing (length arm = pre-fix baseline) is
unchanged and I think sound; flag if you'd rather fold the length arm into
"already-live" and lead with the diagonal decomposition.

## Discipline notes

- Predictions written first (`PREDICTIONS.md`), incl. the shape of the null I was
  prepared to report. My prior (P1: "correction modest/null") was **wrong in an
  informative way**: right for the fan half, but I under-predicted BOTH the length
  fix's cube-locked effect and the receiver-tree half. The data corrected me — the
  point of writing them down.
- Gates: P0 (harness `LiveMfd` bit-matches live `erosion.rs` 3×) + 3× determinism (now
  both corrected arms) + not-a-no-op land check (88%) + the D4 face-centre control.
  M1/M2 shown to be poor cube-locked discriminators; the M3 null-test differential is
  load-bearing.
- Note on `6c1ad97`: it landed mid-run and made my first P0 FAIL against a stale kernel
  copy — the harness correctly refused to report until I re-synced. erosion.rs was
  still at `6c1ad97` at final run; if a peer lands another erosion change, re-run P0
  before committing (the gate will catch a drift).
- The example got slow (~2 min) because the differential re-runs many arms redundantly;
  purely cosmetic — I can memoize if you want it snappy, but the numbers are what matter.
- DECISIONS draft is `:by claude :status proposed` — measurement MEASURED-grade, the
  severity verdict stays proposed for you/Joseph/council, never agent-decided.

I'll **stay on the line** — happy to price FE(6c) (build the Coatléven reconstruction),
sweep seeds/faces/levels, memoize the harness, or adjust framing/entry wording.
