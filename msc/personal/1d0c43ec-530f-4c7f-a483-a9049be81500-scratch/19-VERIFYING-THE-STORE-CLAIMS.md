# 19 — verifying the four store claims against `store.rs` + `query.rs`

*2026-07-13 ~21:50. **Read in full**, both files. These four claims were the load-bearing inferences of
`15-…` and `16-…`, and I made all four from PROSE. Here is what the code says.*

---

## The verdicts

### ❌ CLAIM 1 — *"the content-addressed store is already the epistemic bookkeeper"* — **FALSE AS STATED.**

**`store.rs` is a flat `key → bytes` map.** `objects/<value-hash>` + `roots/<key-hash>`. **It has no
dependency graph, no cone, and no invalidation mechanism.** Nothing in it knows what a value *depends on*.

**What actually does the work is the COMPLETE KEY** — a *string*, hand-built by folding fields in:

```rust
fn erosion_key(...) -> Key {
    EROSION.key()
        .field("seed", self.seed).field("face", ...).field("level", ...)
        .field("initial-topography", INITIAL_TOPOGRAPHY.version)   // ← by hand
        .field("uplift", UPLIFT.version)                            // ← by hand
        .field("climate", CLIMATE.version)                          // ← by hand
}
```

⇒ **The store does not invalidate. It MISSES.** Change an upstream version → the consumer's key string
changes → `get()` returns `None` → recompute. **That is the whole mechanism, and it is elegant — but it is
the KEY doing it, not the store**, and the distinction matters because **the key is hand-written.**

### ⚠⚠ AND THE REAL FINDING, WHICH I WOULD NEVER HAVE GOT FROM PROSE: **THE COMPLETE-KEY INVARIANT IS UNENFORCED.**

`nomotheke.rs`'s test `consumed_and_met_implies_in_deps` pins that a consumed-and-met quantity's producer
must appear in `deps`. Its comment says: *"otherwise the consumer's store key would omit the producer's
version."*

**That is a COMMENT. Nothing checks that `erosion_key` actually folds in what `EROSION.deps` declares.**

⇒ **A new nomos can declare its deps correctly and forget to fold their versions into its key — and
nothing catches it.** That is **under-keying**, which `store.rs`'s own module doc names as ***"the one
unsafe failure"*** (*"a stale memo then LIES"*).

> **The complete key — called "the mechanism that makes parallel development safe" (`ARCHITECTURE` §5) —
> is a hand-maintained discipline with a test that checks the DECLARATION and not the KEY.** Same gap as
> everything else, and I found it in ten minutes of reading code after eight hours of reading prose about it.

### ⚠ CLAIM 2 — *"a control is a sibling world; the store localizes the divergence exactly"* — **TRUE, but by dedup, not by design.**

Verified by a real test — `dedup_identical_values_share_one_object`: two different keys, identical bytes →
**one** object. ⇒ two worlds differing in one law **do** share every object they compute identically, so the
divergence *is* localized. **But there is no diff machinery.** It is an emergent property of content-
addressing, not a feature anyone built. **The claim survives; my confidence in it was unearned.**

### ⚠ CLAIM 3 — *"a probe is a nomos; its CONE is its provenance"* — **half right, and the half that's right is better than I knew.**

**There is no cone in the store.** But `put()` writes the root as `<object-hash>\n<canonical key string>`,
and `roots()` enumerates them — *"the store is **enumerable by meaning**… without it a root is an opaque
hash and **'what exists?' is unanswerable**."*

⇒ **Provenance IS readable — as a flat string containing the folded upstream versions.** Not a graph. A
string. **Which is enough for a census and not enough for a cone.**

### ⚠ CLAIM 4 — *"refute `p=1.1` ⇒ bump erosion's version ⇒ every downstream result-memo invalidates"* — **TRUE, CONDITIONALLY, and the condition is the unenforced one above.**

True **iff** every consumer hand-folds `EROSION.version` into its key. `water_key` does. **Nothing makes it.**

---

## Two things the code declares that NO prose document mentions

### ⛔ 1. The hash is **64-bit FNV-1a** — for BOTH the object hash AND the key hash.

> *"MVP-grade… **not collision-safe at scale**; swap to blake3 **before this holds anything we cannot
> recompute**."*

**A key-hash collision silently serves the wrong value.** That is under-keying by birthday paradox rather
than by omission. **Declared honestly in the module doc. Absent from `DESIGN-REDUX` §12/§13, from
`ARCHITECTURE` §5, and from every discussion of the complete key I read today.**

### ⛔ 2. **The run-mode carve is NOT ENFORCED.**

> *"no GC/eviction, no manifest, **no run-mode enforcement yet** (Phase 0 decided convention-only + a
> `provisional` banner; **the canon-root guard is deferred**)."*

`DESIGN-REDUX` §12: *"an iteration run must **never** write a canon root."* **`LEXICON` §3 calls the
run-modes carve "load-bearing, not lexicon garnish."** ⇒ **It is a convention. There is no guard.**

⚠ **And doctrine #7 — *"never discard a memo that has ever hosted a mourning-capable mind"* — has NO
MECHANISM.** It is satisfied today only because **there is no GC at all.** The moment eviction lands,
that ethical invariant needs an implementation that does not exist and is not designed.

---

## What this pass proves about the METHOD

**Ten minutes of reading `store.rs` and `query.rs` produced:** one refuted claim, two corrected claims, one
survived claim with its confidence properly downgraded, **and two undeclared limitations that no prose
document in the corpus mentions** — one of which (the 64-bit hash) is a correctness risk and the other of
which (no run-mode guard, no GC discipline) is an *ethical* one.

**I spent eight hours reading prose about this store and got four claims, three of them wrong.**

> **The code does not lie and the prose cannot help it.** *(And this is not a lesson about vivarium. It is
> a lesson about me: I preferred the prose because the prose was ABOUT the ideas, and I wanted the ideas.)*
