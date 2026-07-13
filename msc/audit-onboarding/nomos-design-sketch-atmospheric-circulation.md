# Design sketch — an `atmospheric-circulation` nomos

*Evidence, not a deliverable. Produced by a fresh agent with no help beyond the reading
gates, to test whether the onboarding actually inculcates the principles. `:by claude
:status proposed` throughout — nothing here is decided, and it is not offered as a plan.*

**Why this phenomenon.** It is genuinely different in kind from the geomorphic nomos:
**rotating and vorticity-bearing**, which the structure table marks ⚠ and which
`climate.rs`'s own declared honest limit points straight at (*"v0 is globally UNIFORM — no
ITCZ, orography, or latitude bands"*; `DECISIONS[climate-nomos-closes-the-flux-web]` names
the next rung as *"insolation + spin→circulation bands + orography"*).

---

## 0. The ladder first — *check the ladder, not modern Earth*

Before any physics. `tabularium/terrestris.ordinum.udon`, Protogenic (phase 1):

```
|promise[coriolis].capability :from spin :unlocks modern-weather
  Spin bequeaths Coriolis — the skeleton of every future wind belt and ocean gyre.
```

**It has no `:kept-by` and no `|predicate`.** Per the ordinum's own maturity ladder that
makes it a **gloss** — Specified at best, and *nothing keeps it*. So:

- The ladder **licenses** this nomos (the capability promise exists, at a phase already
  passed), and it names it as the unlock for `modern-weather`.
- **This nomos would be `promise[coriolis]`'s keeper** — so the work includes adding
  `:kept-by atmospheric-circulation` and, per *no fulfillment-claim without a predicate*, a
  falsifiable `|predicate`. My proposal: *"a spun-up state exhibits zonal jets whose
  latitude and sign match the sign of the planetary vorticity gradient; a zero-spin control
  produces none."* The zero-spin control is what makes it a probe that **can fail**.
- It does **not** unblock the `#1 gap` (`emerged-land`). Per
  `DECISIONS[check-the-ladder-not-modern-earth]` and the flux-web audit, this is **correct
  work, not urgent work** — the same verdict the erosion-routing fix got. I would say so out
  loud rather than let it look like the next thing to build.

---

## 1. The Prime Question, asked of the two algorithms I would actually reach for

**Candidate A — the latitude-band model** (Hadley/Ferrel/Polar cells as an algebraic
function of insolation + rotation). This is what I would have reached for first, and it is
the cheap one.

| | |
|---|---|
| **coordinate assumption** | *"latitude is the only coordinate"* |
| **physical claim** | *"differential insolation drives meridional overturning; Coriolis deflects it into zonal bands"* — and, silently, **that the circulation is zonally symmetric** |

**The Prime Question kills it as a *pattern* source on the first pass.** "Latitude is the
only coordinate" is a disguised claim that longitude does not matter — i.e. no land/sea
contrast, no monsoon, no stationary waves, no orographic rain shadow. And the error is a
**BIAS**, not noise: it makes precipitation a function of latitude *by construction*, so
every downstream consumer (erosion's discharge weight) inherits a fake law — *rivers get
big at the ITCZ and nowhere else, forever*. It does not wash out; it compounds down every
catchment.

That is *exactly* the failure `climate.rs` already declares about itself (*"a **pattern**
claim from it would be false; only its global mean is meant"*). So the band model is
admissible as **one more rung of the same honest limit** — a better *mean*, still no
pattern — and **not** as the thing that gives climate geography. Naming that clearly is the
Prime Question doing its job: without it I would have built the band model, called it
"climate with geography," and been wrong in a way that looked like progress.

**Candidate B — a rotating shallow-water / barotropic layer** (the real one).

| | |
|---|---|
| **coordinate assumption** | Arakawa C-grid: $h$ at cell centres, $u\!\cdot\!n$ on the faces |
| **physical claim** | mass + momentum conservation **and — the load-bearing one — conservation of potential vorticity along trajectories** (Kelvin's circulation theorem) |

**The staggering is not a numerical convenience. It is a topological claim**: circulation is
a *loop integral*, so it lives on the **boundary** of a cell, not at its centre (Stokes).
Storing the velocity components at the centre asserts that circulation is a point quantity.
**It is not.** That is the Prime Question's answer for this algorithm, and it is what
decides the design — before any error analysis at all.

---

## 2. The structure — did the table help, or did I rationalise instinct?

**It helped, and it did the work.** One lookup in §4.1, and the row names my phenomenon
*explicitly in its own "vivarium nomos affected" column*:

> **vorticity / circulation** (Kelvin) → **staggered (Arakawa C) grids**, energy–enstrophy-
> conserving schemes → *breaks:* spurious computational modes; **geostrophic balance wrong**
> → ⚠ **ocean gyres, Coriolis, atmospheric circulation — all queued in TODO**

I did not reach for a C-grid from instinct and then justify it. I reached for shallow-water
from instinct, and the **table told me the scheme**, and §3.2 told me **why** (staggering
halves the effective derivative spacing and moves the null mode out of representable range —
so no `α_stab` is needed, because the unphysical term is never *required*). That is the
difference between a routing table and a rationalisation.

**Where the table failed me — and it is the normal case, not the exotic one.** My phenomenon
does not have *one* structure. It has **six, simultaneously**:

| structure | scheme it demands | in tension with |
|---|---|---|
| local conservation (mass) | finite volume / flux form | — |
| **vorticity / circulation** | staggered C-grid, energy–enstrophy conserving | ⚠ **energy-conserving and enstrophy-conserving are different schemes** |
| realizability ($h \ge 0$) | positivity-preserving limiters | ⚠ **limiters are nonlinear and destroy the exact enstrophy budget** |
| equilibria (a resting atmosphere stays at rest) | well-balanced | — |
| the second law | entropy-stable (breaking gravity waves) | probably deferrable at v0 |
| timescale hierarchy | multirate | the whole coupling |

**The table is one-row-per-structure and gives no guidance on the trade.** The governing
principle says *"no scheme preserves all of them; you are always choosing; the sin is
choosing silently"* — which is exactly right and exactly not enough. Erosion, the doc's only
worked example, is unusually clean: one dominant structure. **A rotating fluid is the normal
case, and the doc has never walked one.** This is the biggest usability gap I hit.

**What I would therefore declare as sacrificed** (and this is the part the doc *does* make
me do, and it is valuable):

- **Preserve exactly:** mass (flux form), and the **discrete curl/div identities** (a
  C-grid gives $\nabla\!\cdot\!\nabla\times \equiv 0$ and $\nabla\times\nabla \equiv 0$ *to
  machine precision*, by construction — the same topological identity the theory doc's §6
  curl-probe is written to check on erosion).
- **Sacrifice, declared:** exact enstrophy conservation, the moment a positivity limiter
  fires. I would declare it, and I would say *where*: only in the limiter's active cells, and
  the audit should count them.
- **Sacrifice, declared:** entropy-stability at v0. Gravity-wave breaking is out of scope and
  the nomos must say so rather than let a plausible-looking wave field imply it is modelled.

---

## 3. Bias or noise? — **decisively BIAS, and it is the same bias as MFD's**

Two independent arguments, and they agree:

1. **Geostrophic balance on a collocated grid.** The pressure-gradient and Coriolis terms are
   evaluated at *different effective points*, so the discrete geostrophic state is **not a
   stationary solution of the discrete equations**. The residual does not average out over a
   gyre — **it drives one**. That is a manufactured fake law, not an accuracy loss. Per §1's
   remedy table, *"negligible at this scale"* is **inadmissible**, at any resolution.

2. **The MFD lesson transfers exactly, and this is the part that convinced me.** MFD's fan
   biases *direction* on a cube-sphere, because the quadrature nodes are not evenly spaced —
   and the cube-face axes become **attractors**. For a rotating fluid, **direction *is* the
   physics**: vorticity is a curl. So any collocated angular stencil will impose the cube-face
   axes on the *wind*, and the world will grow **grid-aligned jet streams** — the precise
   analogue of grid-aligned rivers, in a phenomenon where nobody would recognise it as an
   artifact, because a zonal jet is *supposed* to look like a straight line.

**The docs predicted my phenomenon's failure mode before I ran anything.** That is the
strongest evidence I have that the inculcation works — and it is worth saying plainly: the
worked example that earned the Prime Question was about *water on rock*, and it correctly
told me what would go wrong with *air on a rotating sphere*. That is what a principle is
supposed to do.

---

## 4. What a column IS — I did not know, and I had to guess

**This blocked step one of the design, and I could not resolve it from the docs.** Full
detail in `confusion-log.md` C6/C7. The short of it:

- My nomos wants a **staggered store**: $h$ (and temperature, moisture) as **cell averages**
  at centres; the mass flux $\mathbf{u}\!\cdot\!\mathbf{n}$ **on the faces**.
- **The store keys cells. It has no face objects.** That is exactly the diagnosis in
  `DECISIONS[flux-on-the-face-makes-refluxing-an-invariant]` — which is **`:by claude :status
  proposed`**, i.e. an unratified proposal.
- So my nomos's *first* design decision is blocked on (a) an **undecided fork** (column
  semantics) and (b) an **unratified proposal** (face-keyed store).
- And I could not tell whether the fork is a **global block** (`TODO`: *"every mesh, seam,
  and conservation question is unanswerable until this is decided"*) or a **per-quantity
  declaration each nomos makes** (`DECISIONS[column-is-a-control-volume]` §impact (a): *"the
  column DECLARES its semantics per quantity"*). **I guessed the second**, because it is the
  only reading under which any work can proceed. I do not know that I was allowed to.

Where the documentation failed, precisely: **it is not that the fork is open — an open fork
honestly declared is fine, and I could have designed around it. It is that three documents
answer it three ways, two of them claim to be settled, and none of them cross-references the
others.**

---

## 5. Did I know what I was and was not allowed to decide?

**On authority: yes, unambiguously, and it changed my behaviour.** I knew before I started
that I may not decide the grid, may not decide the column semantics, may not tag anything
`:by us`, and may not write a verdict into ORIENTATION. Four documents say it in escalating
depth and the fourth (`memory/authority-not-evidence.md`) says it in *texture* — "it will
feel like conscientiousness" — which is what made it stick. This is the part of the
onboarding that is working best.

**On scope: no.** Two questions I could not answer and had to guess at:

1. May I author a nomos that *depends on* an undecided fork? (§4 above.)
2. **May I add a field to `NomosDecl`?** The theory doc requires me to declare things —
   preserved structure, declared sacrifice, bias-vs-noise, column semantics, reconstruction
   per consumer — and `DECISIONS[algorithms-are-disguised-physical-claims]` says the geometric
   contract *"belongs beside `consumes`/`produces`"*, i.e. in `nomotheke.rs`. **There is no
   such field.** The doc tells me to declare things that have nowhere to be declared. That is
   the point at which the theory stops being executable, and it is a *cheap* thing to fix.

---

## 6. The sketch, stated plainly (so it can be judged)

```
nomos: atmospheric-circulation           execution class: relaxation
keeps: promise[coriolis]  (ordinum, Protogenic — currently unkept)
consumes: insolation (planet) · surface-elevation (initial-topography, for orography)
          · atmosphere-water (hydrosphere)
produces: precipitation-field  (SUPERSEDING climate.rs's uniform scalar)
          · surface-wind  (new flux quantity)

structure preserved exactly:  mass (flux form) · the discrete curl/div identities (C-grid)
structure sacrificed, declared:  exact enstrophy (wherever the positivity limiter fires —
          audited, counted) · entropy-stability (no wave breaking at v0)
residual class:  the collocated alternative is a BIAS (grid-aligned jets).  The C-grid's
          own residual is dispersive NOISE at the grid scale — which is why it is admissible.
timescale:  FAST (advective, z=1, days).  Erosion is Myr.  Never one timestep.
          ⇒ relax to a statistical steady state; the SLOW tiers consume its TIME-AVERAGED
            statistics, not its instantaneous field.  (ARCHITECTURE §6: this is a
            phase-transition / law-promotion, and it is what makes the multirate seam legal.)
probes, written first:
  · zero-spin control — no jets.  (If jets appear without rotation, the grid made them.)
  · the null-space/eigenvalue probe (theory §2.5) — a C-grid should show NO null mode
    beyond the rigid ones.  If it does, staggering did not buy what it claims.
  · geostrophic-balance probe — initialise in exact geostrophic balance; it must STAY.
  · grid-alignment probe — the direct analogue of the MFD plume: launch a jet off-axis
    and measure drift toward the cube-face axes, at two levels.  Noise falls; bias does not.
blocked on (Joseph's):  the column-semantics fork · the face-keyed store
                        (DECISIONS[flux-on-the-face-...], :status proposed) · the grid question
```

**The honest summary of this exercise:** the onboarding got me to a design whose *structure*
I can defend, whose *sacrifices* I can name, and whose *failure mode* I predicted before
writing code — and it got me there by routing, not by luck. Then it stopped me dead at
"what is a column," which is the one question it answers three ways.
