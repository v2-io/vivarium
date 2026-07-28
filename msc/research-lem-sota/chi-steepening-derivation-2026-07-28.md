# Detachment + Davy–Lague steepening: derivation against the measured G-sweep

*Derivation note, 2026-07-28. Independent of the running base-level / halo spikes. Not claim canon — nothing here promotes a rate gate. Home until a segment owns a result: if a derivation *predicts* the measured bracket, that is a candidate FE for `#obs-chi-shape-is-erosions-criterion`; if it cannot, that is the no-go that keeps `(1+G/2)` from becoming lore.*

**Measured target** (`#obs-chi-shape-is-erosions-criterion` FE(5)–(6), six tiles, 3000 epochs from a stored stage, only `deposition` varied):

| $G$ | fitted ÷ pure-SPL | ÷ $(1+G)$ | ÷ $(1+G/2)$ |
|---:|---:|---:|---:|
| 0 | **1.00** | 1.000 | 1.000 |
| 0.25 | 1.14 | 0.914 | 1.013 |
| 0.5 | 1.32 | 0.877 | 1.056 |
| 1 (live) | 1.55 | 0.773 | 1.033 |
| 2 | 2.25 | 0.751 | 1.125 |

At $G=0$ the criterion is exact (shape residual 0.009, slope recovers $U/(k_{dt}A_0^{m})$ to three digits). So the offset *is* deposition. The Davy–Lague reading $1+G$ is **refuted** (fourth column declines 1.00→0.75). The tidy $(1+G/2)$ holds within 1.00–1.13 and is **underived**. Candidate mechanism in the segment: only part of the eroded volume is re-laid in-network; the rest reaches an outlet and is lost to the sea.

**Kernel scheme** (as of the G-sweep; fill-restore landed later as `0780feb` and is noted in §6):

1. Uplift: $h \leftarrow h+U$ (non-edge cells).
2. Priority-Flood fill (at sweep time: raise **kept** in $h$).
3. D8 receivers + MFD drainage $A$.
4. Implicit incision $n=1$: $h \leftarrow (h + f\,h_r)/(1+f)$, $f = k_{dt} A^{m}/d$, low→high so $h_r$ is already this-epoch post-incision.
5. Davy–Lague deposit: $q_s$ starts as this epoch’s incised volume; walk high→low; deposit height $G\,q_s/A$ (capped by available $q_s$); remainder passes to receiver; outlet remainder is lost.
6. Talus + creep.

Instrument: fitted $\mathrm{d}z/\mathrm{d}\chi$ over channel cells, divided by pure-SPL $U/(k_{dt}A_0^{m})$.

---

## 1. Pure detachment ($G=0$) — recovered exactly

One epoch, no deposition. Start-of-epoch height $h$ is also the end-of-epoch height at steady state.

- After uplift: $h+U$.
- After incision: $h^* = \bigl((h+U) + f\,h_r^*\bigr)/(1+f)$ with $h_r^*$ already post-incision.

Set $h^* = h$:

$$
f(h - h_r^*) = U \quad\Rightarrow\quad h - h_r^* = \frac{U\,d}{k_{dt} A^{m}}
$$

when $h_r^*$ is the neighbour’s steady height (no deposit). This is FE(2) of the χ segment: zero free parameters, $A$ = MFD drainage the incision step consumes. **Measured: exact at $G=0$.** The instrument and the algebra agree. Whatever fails at $G>0$ is not the χ construction.

---

## 2. Classic simultaneous Davy–Lague — predicts $1+G$, which is refuted

Continuous interior balance (Davy & Lague 2009 lineage; standard LEM write-up):

$$
\frac{\partial z}{\partial t} = U - E + D,\qquad
E = K A^{m} S^{n},\qquad
D = G\frac{Q_s}{A},\qquad
\frac{\partial Q_s}{\partial A} = E - D.
$$

At topographic steady state $\partial z/\partial t = 0$ and $E-D = U$, so $\partial Q_s/\partial A = U$. With $Q_s=0$ at the divide:

$$
Q_s = U A \qquad\text{(every interior point)}
$$

even though the **outlet** exports $U A_{\mathrm{basin}}$ to the sea. Export does **not** reduce interior $Q_s$. Then:

$$
U = E - G U \quad\Rightarrow\quad E = U(1+G) \quad\Rightarrow\quad
S^{n} = \frac{U(1+G)}{K A^{m}}.
$$

For $n=1$, slopes (and $\mathrm{d}z/\mathrm{d}\chi$) steepen by exactly **$1+G$**.

**This is the reading the G-sweep refutes.** Note carefully: the popular “outlet loss ⇒ factor $<1+G$” story is **not** what the continuous theory says. Continuous theory already has outlet export, and still gives $1+G$ in the interior. Any successful derivation of a smaller factor must break a *different* assumption (see §4–5), not “sediment leaves at the coast.”

---

## 3. Sequential incision-then-deposit — still $1+G$ if $d = GU$

The live kernel does **not** advance $E$ and $D$ simultaneously. It fully incises, then routes the **incised volume** as $q_s$.

Let $h$ be steady start-of-epoch height. After uplift and incision (all cells), then deposit $d_i$ on cell $i$:

$$
h_i = h_i^* + d_i,\qquad
h_i^* = \frac{(h_i+U) + f_i\,h_r^*}{1+f_i},
$$

with $h_r^*$ the receiver’s **post-incision, pre-deposit** height ($h_r^* = h_r - d_r$).

Eliminate $h_i^*$:

$$
f_i(h_i - h_r) = U + d_i(1+f_i) - f_i d_r
\quad\Rightarrow\quad
h_i - h_r = \frac{U}{f_i} + d_i\Bigl(\frac{1}{f_i}+1\Bigr) - d_r.
\tag{★}
$$

**Special case $d_i = d_r = d = G U$ (spatially uniform deposit height):**

$$
h_i - h_r = \frac{U}{f_i} + GU\Bigl(\frac{1}{f_i}+1\Bigr) - GU = \frac{U}{f_i}(1+G).
$$

So **sequential + uniform $d=GU$ recovers classic $1+G$**, not a reduced factor. The discrete order alone does not explain the sweep.

If the deposit law really puts $d = G\,q_s/A$ and $q_s = U A$ at steady state, then $d = GU$ and (★) is $1+G$. The measurement therefore implies: **either $q_s \neq U A$, or $d$ is not $G q_s/A$ in effect (caps / lakes / creep), or the landscape is not at the steady state the algebra assumes, or the χ fit is not measuring pure channel fluvial balance.**

---

## 4. Where $q_s = U A$ fails in *this* kernel

### 4.1 Headwater cap when $G \ge 1$

Code: `a = drainage[i].max(area)`, `deposit_vol = min(G * qs * area / a, qs)`.

At a true headwater $A \approx A_{\mathrm{cell}}$, `area/a ≈ 1`, so for $G \ge 1$ **all** local sediment is re-laid (`d = e`). Fluvial net export from that cell is zero. Topographic steady state then **cannot** be carried by fluvial E−D alone: uplift must be balanced by **talus/creep export** into the channel network (or the cell is not at SS). χ is evaluated on channelized cells (area threshold), so headwaters are outside the fit — but they still set how much sediment **enters** the channelized network. That boundary condition is not $q_s = U A$ with $A$ the fluvial catchment; it is $q_s = U A_{\mathrm{fluvial}} + Q_{\mathrm{hillslope}}$.

### 4.2 MFD incision $A$ vs D8 sediment tree

Incision and the $f$ in (★) use **MFD** $A$. Deposit routes $q_s$ on the **D8** tree. At a D8 node, $q_s$ is not the MFD-integrated flux the continuous derivation assumes matches $A$. The identity $d = G q_s/A_{\mathrm{MFD}}$ is then dimensionally the Davy–Lague form but **not** the same object as $G Q_s^{\mathrm{(MFD)}}/A_{\mathrm{MFD}}$. This alone can break $d=GU$.

### 4.3 Not fully at steady state

At $G=1$, shape residual is still 0.177 after 3000 epochs (FE(5)), not the $G=0$ floor of 0.009. The rate ratio is read on a landscape that is **still approaching**. A transient can bias $\mathrm{d}z/\mathrm{d}\chi$ relative to the true SS value. The sweep’s plateaus are “good enough to be monotone and tight in $G$,” not proven SS endpoints.

### 4.4 Talus and creep after the fluvial step

Both run every epoch on the post-fluvial surface. They are mass-conserving redistributions that **lower channel slopes** relative to pure stream-power SS (especially near thresholds). They are on in every G-sweep row, so they are a **G-independent** (to first order) reduction of the fluvial $1+G$ factor — a candidate for “why everything sits below $1+G$” without explaining the *shape* of the residual vs $G$.

### 4.5 Fill composition at measurement time

The G-sweep predates fill-restore (`0780feb`). Routing, incision, and deposition all saw the **filled** surface; Priority-Flood ε-rock was in the bed. Post-repair, lakes are real, incision is masked under water, and lakes trap sediment with efficiency 1 until full. **Any derivation that becomes a gate must be re-checked against a G-sweep on the new composition.** The algebra of §1–3 still applies to subaerial channel cells; the numbers in the table are a property of the old bed.

---

## 5. Outlet-loss as $\varphi$: what *can* be derived, what cannot

The segment’s candidate: only a fraction of eroded volume is re-laid in-network.

### 5.1 Passive tracer on the deposit law (no local production)

If sediment is injected at area $A$ and only deposition $D = G Q_s/A$ acts, then $\mathrm{d}Q_s/\mathrm{d}A = -G Q_s/A$, so $Q_s \propto A^{-G}$. Fraction surviving from $A$ to outlet $A_o$:

$$
\tau(A\to A_o) = \bigl(A/A_o\bigr)^{G}.
$$

### 5.2 Uniform production, integrate

Produce at rate $U$ per area over the basin. Volume deposited before export:

$$
\int_0^{A_o} U\bigl(1 - (A/A_o)^{G}\bigr)\,\mathrm{d}A = U A_o \frac{G}{G+1}.
$$

So the basin-integrated **deposit fraction** is $G/(G+1)$ and the **export fraction** is $1/(G+1)$.

| $G$ | $G/(G+1)$ | $1 + G/2$ | $1+G$ | measured |
|---:|---:|---:|---:|---:|
| 0.25 | 0.20 | 1.125 | 1.25 | 1.14 |
| 0.5 | 0.33 | 1.25 | 1.5 | 1.32 |
| 1 | 0.50 | 1.5 | 2 | 1.55 |
| 2 | 0.67 | 2.0 | 3 | 2.25 |

Naive “steepening $= 1 + G\cdot(\text{deposit fraction})$” $= 1 + G^2/(G+1)$:

| $G$ | $1 + G^2/(G+1)$ | measured |
|---:|---:|---:|
| 0.25 | 1.05 | 1.14 |
| 0.5 | 1.17 | 1.32 |
| 1 | 1.50 | 1.55 |
| 2 | 2.33 | 2.25 |

Close at $G=1$ and $G=2$, **systematically low** at small $G$. It is not better than $1+G/2$ overall, and it is still a **guess** about how deposit fraction enters the χ slope — not a derivation from (★).

### 5.3 Why continuous theory forbids “export reduces the factor”

In §2, $Q_s = UA$ already **includes** that everything eventually exports. The steepening comes from **reworking**: sediment laid down must be cut again, so $E = U + D = U(1+G)$. Export is the boundary condition that sets the total load, not a discount on $G$.

So the segment’s “outlet-loss” mechanism, if it is real, must mean something **other** than continuous export: e.g. sequential one-pass routing where sediment is **not** re-incised in the same balance equations; or MFD/D8 mismatch so that “$A$” in $D=G q_s/A$ is not the $A$ that carries the load; or headwater/hillslope bypass (§4.1). Those are **kernel-structure** mechanisms, not the continuous outlet story.

---

## 6. $(1+G/2)$ as numerology

Define $r(G) = (\text{fitted}/\text{pure-SPL})/(1+G/2)$:

$$
r \in \{1.000,\ 1.013,\ 1.056,\ 1.033,\ 1.125\}.
$$

No derivation in §1–5 produces a factor $1/2$ in front of $G$. The deposit-fraction $\varphi = G/(G+1)$ is $1/2$ **only at $G=1$**, not as a universal coefficient. Treating $1+G/2$ as law would mint a constant that exists only as a five-point coincidence (plus a plausible-sounding story continuous theory does not support).

**Verdict on $(1+G/2)$:** remains an **observed regularity**; must not become a gate, a nomos default, or a comment that implies derivation. The segment’s fencing is correct; this note is the reason the fence holds.

---

## 7. What would make the rate half a gate

A gate needs $r_{\mathrm{pred}}(G)$ such that measured $r$ matches within probe noise, derived from the **actual** discrete operators (not from continuous Davy–Lague alone). Minimum path:

1. **Sediment budget probe** (read-only on a long settle, G-sweep companion): per epoch, on channelized cells and whole tile,
   - volume incised $\sum e\,A_{\mathrm{cell}}$,
   - volume deposited $\sum d\,A_{\mathrm{cell}}$,
   - volume exported at outlets,
   - check $q_s$ vs $U A$ at mid-catchment D8 nodes (and vs MFD $A$).
   If $d \approx GU$ and $q_s \approx UA$ but χ-slope $\neq (1+G)$, the bug is in the χ instrument or non-SS. If $q_s \neq UA$ or $d \neq GU$, the budget *is* the derivation’s missing input.

2. **Fluviial-only control:** G-sweep with `diffusivity_m2 = 0` and talus disabled (or repose $\to\infty$). If the ratio moves to $1+G$, creep/talus were the discount and $1+G$ is the fluvial law (gate = pure-SPL×$(1+G)$ under those flags). If it stays near 1.55, the discount is inside E–D sequential structure / A-mismatch.

3. **Re-run on post-fill-restore composition** before any nomos threshold cites the old table.

4. **Only then** attempt a closed form from (★) with measured $d_i(A)$ (e.g. fit $d(A) = c U (A_{\mathrm{cell}}/A)^{p}$ from the budget probe and substitute).

Until (1)–(2) exist, promoting the rate half is inventing a law.

---

## 8. Bottom line

| Claim | Status after this derivation |
|---|---|
| $G=0$ ⇒ χ rate exact | **Derived and measured** — holds |
| Simultaneous Davy–Lague ⇒ factor $1+G$ | **Derived** — and **refuted as a description of the live sweep** |
| Sequential + $d=GU$ ⇒ factor $1+G$ | **Derived** — so sequential order alone does not explain the refutation |
| Outlet export in continuous theory ⇒ factor $<1+G$ | **False** — continuous theory has export and still $1+G$ |
| $(1+G/2)$ | **Not derived**; five-point regularity; do not promote |
| Deposit-fraction $\varphi=G/(G+1)$ | Derived as passive-tracer basin integral; **no justified map** to χ-slope that beats $1+G/2$ |
| Rate half as gate | **Blocked** until sediment budget + hillslope-off controls exist (and post-fill-restore re-measure) |

**The no-go (present truth):** there is no derivation from the kernel’s named deposit law plus continuous sediment conservation that both (a) is forced by the equations and (b) predicts the measured G-sweep. The classical $1+G$ *is* forced by those equations under steady $q_s=UA$ and $d=GU$, and the data reject it — therefore at least one of ($q_s=UA$, $d=GU$, SS, fluvial-only, MFD≡D8 load) is false in the measured runs. Finding which is an instrument task (§7), not more algebra on the continuous model.

**Composition caveat:** G-sweep numbers are pre-`0780feb` (fill kept in bed). Segment replacement for the fill repair will need to say whether the rate-half table is historical or re-measured.

---

## References (for verification)

- Davy, P., & Lague, D. (2009). Fluvial erosion/transport equation of landscape evolution models revisited. *JGR Earth Surface*. (continuous $E$/$D$ form; $G$.)
- Perron, J. T., & Royden, L. (2013). An integral approach to bedrock river profile analysis. *ESPL* 38:570–576. (χ; pure SPL slope.)
- Live kernel: `crates/vivarium-world/src/erosion.rs` — `incise`, `deposit`, `Fluvial::erode` order.
- Measurement home: `#obs-chi-shape-is-erosions-criterion` FE(5)–(6); probe `examples/chi_convergence_probe`.
