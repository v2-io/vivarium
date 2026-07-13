Now your volume question — because I think it's the sharpest thing you've asked, and it *decides the fork* rather than merely illustrating it. Let me actually do the arithmetic rather than gesture at it.

## The volume under the mesh: I worked it out, and the answer is a curvature term

Take 1-D for clarity (it generalises). Points at cell centres, spacing $\Delta$, values $h_i$. Draw the piecewise-linear mesh **through** those points. Now integrate the volume under it, over cell $i$ (from $x_i - \Delta/2$ to $x_i + \Delta/2$).

Left half: the interpolant runs from $\tfrac{h_{i-1}+h_i}{2}$ to $h_i$, so its mean is $\tfrac{h_{i-1}+3h_i}{4}$. Right half: mean $\tfrac{3h_i+h_{i+1}}{4}$. So the mesh's **average height over the cell** is

$$\frac{h_{i-1} + 6h_i + h_{i+1}}{8} \;=\; h_i \;+\; \frac{\Delta^2}{8}\,\nabla^2 h \;+\; O(\Delta^4)$$

**So the volume under the mesh does NOT equal $\sum h_i A_i$. It differs by a term proportional to the local curvature.** [derived — check me]

That single line answers a surprising amount:

- **Over flat or planar ground the two agree exactly.** Which is why nobody ever noticed.
- **The discrepancy is not noise. It is a signed, systematic function of curvature.** Convex ground (a ridge) loses volume; concave ground (a valley) gains it. **Bias again.**
- **And here's the part that should worry us:** erosion is precisely the process that *manufactures* ridges and valleys. So the mass error **correlates with the very structure the physics is generating.** That is a feedback loop between a discretisation artifact and the phenomenon. I want to flag that as a **hypothesis, not a result** — but it is exactly the shape of thing that produces a fake landscape that looks plausible.
- **Globally, on a closed surface, it cancels exactly.** $\sum(h_{i-1}+6h_i+h_{i+1})/8 = \sum h_i$, because the sums telescope — which is just the divergence theorem: the Laplacian of anything integrates to zero over a closed manifold. **So a global mass audit will pass while every local mass budget is wrong.** That is a genuinely nasty failure mode, and it is *precisely* the kind of thing a "conservation: exact ✓" checkmark would hide.

## And it decides the fork

Your question was *"is there a mesh with exact volumetric fidelity ± ε?"* The answer:

> **Not by interpolation. Only by *reconstruction* — and reconstruction is only available if you declare the stored number to be a CELL AVERAGE.**

Two different objects, and vivarium currently wants both from one array:

- **Point sample + interpolate.** The mesh passes *through* your data. Renders beautifully, smooth, intuitive. **Locally volume-wrong, by the curvature bias above.**
- **Cell average + reconstruct.** You *demand* $\frac{1}{A_i}\int_{\text{cell}} \tilde h = \bar h_i$ and solve for the surface $\tilde h$. **Volume-exact by construction, locally and globally** — that's its *defining property*. But the surface does **not pass through your stored values**, and in general it is **discontinuous at cell boundaries**. (This is exactly what finite-volume MUSCL/ENO reconstructions do, and why they look "wrong" if you plot them.)

**So the column-semantics decision isn't bookkeeping. It determines whether volumetric exactness is even *available*.** And the reason the conflation was so comfortable — your Minecraft intuition — is that a Minecraft column *is* both: a flat face (a cell average, trivially, since it's constant) *and* a point. The moment the surface stops being piecewise-constant, those two split, and nobody noticed the split happening.

## "Does it depend on the phenomenon?" — yes, and now we can say *why*

There is **no single principled mesh.** There is a principled **reconstruction per invariant**, and they are mutually exclusive:

| what you're doing      | the invariant the reconstruction must preserve |
| ---------------------- | ---------------------------------------------- |
| conserving mass        | **volume-exactness** (cell averages reproduce) |
| **drainage / routing** | **monotonicity — no spurious local minima**    |
| rendering / normals    | **$C^1$ continuity** (smooth shading)          |
| visibility, overhangs  | not a height-map at all                        |

And look at the drainage row, because it's the one that made me sit up. **Priority-Flood exists entirely to remove spurious pits.** If your *reconstruction* invents pits that aren't in the data, **you have manufactured the very problem the algorithm is there to solve** — and then spent compute solving it. A monotone (or at least extremum-preserving) reconstruction isn't a nicety for drainage; it's the thing that keeps the algorithm from fighting its own mesh. **That is a genuinely physical criterion**, unlike Delaunay's — which optimises the *interpolation error* of a linear fit and says nothing about physics whatsoever.

## Your triangle worry is real, and sharper than you framed it

**Triangulating a quad requires choosing a diagonal — and that choice is a physical assertion you never meant to make.** The two triangulations of the same four corner heights give **different volumes and different slopes**. You have asserted which way the surface folds, on no evidence. (The symmetric escape is the *bilinear* patch — which is what a "height map" actually is — or a centroid split into four. Delaunay picks the diagonal by a circumcircle criterion, which is an *interpolation-conditioning* rule masquerading as geometry.) And no, triangulating adds **no new degrees of freedom** — same vertices, more elements. What it adds is an **arbitrary asymmetry**, which is exactly what MFD's diagonals turned out to be.

## Your last two, briefly, because they're each a thread of their own

**"An ε on the points means what ε on the slopes?"** — It means **$\varepsilon/\Delta$**, and that is the whole answer and it is brutal: **differentiation amplifies error, and the amplification grows as you refine.** Halve the cells and the slope noise *doubles*. Which is why sub-grid noise must be **spatially correlated** (fBm) and never white — white noise at fine resolution produces garbage slopes, hence garbage stream power. That principle is already in `ASSUMPTIONS.md` for the precip jitter; it's the same theorem and nobody connected them.

**"Is each point a probability gradient — boundaries fated but nondeterministic?"** — This is coherent, it is not exotic, and I think it's where this ends up. It's **stochastic parameterisation** (weather models already do exactly this for sub-grid convection), and **fated noise makes it deterministic and replayable**, which no weather model can claim. The hard constraint: the sampled sub-grid surface must still **integrate to the declared volume** — i.e. you must sample **conditioned on the cell average**. Which is a bridge/conditioned-diffusion problem — *the same math family as your Doob h-transform / Schrödinger-bridge insight about lawful steering.* Same structure, different level of the ontology. I don't think that's a coincidence, but I'm not going to pretend I've thought it through.

Which of these do you want to pull on? My instinct is the **column-semantics fork** — because until it's decided, every mesh question above is unanswerable, and the code is currently answering it three different ways.