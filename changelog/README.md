# changelog/ — where the rubber meets the road

Dated, **frozen** entries for the end-user perspective: what the world can show and do now that it couldn't before. Each entry is a snapshot of a date, append-only — an entry is *history the moment it lands*, so it cannot go stale (the failure mode that killed the hand-maintained front doors on 2026-07-13 was *maintained* docs cited as current law; these are deliberately the opposite).

**The entry bar:** the external perspective moved. New things visible in `vivarium explore` / `watch` / `info`, new commands or interactions, world behavior a user would notice. Internal commits that don't move the external view need no entry — that's fine and good, not a gap.

**The entry is the acceptance check** (Joseph, 2026-07-28): all the physics and code in the world isn't useful if the rendered world still shows fBm circles passed off as rising land. So an entry is behavior-driven development's bottom line applied to a planet — *show the world doing the new thing, from the store, reproducibly*. Two practices make this a mechanism rather than a hope:

1. **Declare the expected entry before building** (the pre-registration clause of `#norm-caught-disciplines-become-mechanisms` FE(4), at outcome altitude): one sentence of "when this lands, an entry can show X" — e.g. *"the fill repair's entry will show closed depressions persisting in the depression map where the previous cohort shows none."* If the work lands and that entry cannot be written, the distance is measured and queued, not waved past.
2. **Entries are captured, not composed.** Standard captures, each self-labelling with seed/cohort/command: `vivarium info --width 100 --no-color` (whole-globe ASCII), `vivarium watch --frames 1` (interior census + coverage), `vivarium status` (demand + flux + maturity), explorer sighting capture (`C` — screenshot + full state beside the world). Hand-drawn or retouched renders are forbidden by the referent rule below; if the honest capture is unimpressive, the unimpressive capture is the entry.

An agent's *interpretation* of a render can still be off — the entry doesn't replace probes, it complements them at the altitude where Joseph's eye is the calibrated instrument. What it guarantees is weaker and load-bearing: the ball either visibly reached the end of the row, or the remaining distance is named.

**An unwritable entry has two diagnoses, and they queue differently:** the work didn't reach the surface (queue the work), or *no instrument can render this scale or layer yet* (queue the instrument). The second is a real and distinct finding — the L13 beacon patch existed for hours before the region window could show it, and during that gap the missing thing was the exploration tool, not the physics. An entry that says "built, currently invisible, because no view reaches X" is a valid entry and a direct instrument requisition.

**The rules, few and load-bearing:**

- **Every shot has a referent** ( `#norm-no-depiction-without-referent` extends to documents): an ASCII render, screenshot, or number names its **seed, source cohort (`src=`), and the command that produced it** — so any reader can re-derive it, and no image is a mock-up.
- **Before/after pairs are the ideal form.** The cohort-safe loaders make "before" a reproducible read of a named older cohort, not a memory. Where no visible tell exists yet, a ` ```udon ` snippet of the new reality (a manifest line, a declaration) stands in.
- **Entries are frozen.** Errata get a dated postscript, never an edit. History lives here and in git; current truth lives in `core/`.
- **Filename:** `DATE-NN-slug.md`, NN ordering multiple entries per date.
- Meta and depth are welcome when they serve the reader; the gravity is toward *what you can see and do*.

Who reads this: Joseph, catching up on what changed at the level he actually experiences it; future sessions, wanting the user-visible arc without archaeology; and eventually anyone the project is shown to.
