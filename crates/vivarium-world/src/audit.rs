//! The pre-run requisite & conservation audit — the honest picture of
//! *what depends on what, whether it is met, and how well it is conserved*, read
//! straight off the declarations with **nothing running**.
//!
//! This is the payoff of declaring the coupling contract as data
//! (`VIVARIA-DEFINITIONS.md` §"the web"; `doc/plan/regula-conformance-design.md`
//! §3). Every function here is a pure graph query over the [`crate::nomotheke`]
//! registry — no store, no kernels, no simulation. Their whole job is to make
//! the questions the fidelity pyramid can't answer answerable *before* a line of
//! physics runs:
//!
//! - **"Can we rain principled water?"** → walk erosion/water's `consumes` back
//!   to producers; it bottoms out at [`crate::flux::PRECIPITATION`], which no
//!   nomos produces — [`Supply::Unmet`]. The answer is a printed chain, not an
//!   opinion.
//! - **"Is this coupling real or a typo?"** → a consumed quantity resolves to a
//!   producer or it does not; the vocabulary-closure test already rules out
//!   typos, so an `Unmet` here is a genuine gap, never a misspelling.
//! - **"Is X conserved end-to-end?"** → each producer's promise carries its
//!   conservation stance; the chain reads them off.
//!
//! **Scope (honest).** This audits the *registry* alone — the declared nomoi and
//! their flux interface. It is deliberately *not* the full world-conformance
//! audit: it takes no `regula`/`ordinum` and no store census, because the
//! world-level profile's shape is still Joseph's open call (regula-keep, per
//! `doc/plan/regula-conformance-design.md`, vs regula-collapse, per
//! `VIVARIA-DEFINITIONS.md`). What both agree on is exactly this layer — the
//! requisite/conservation queries over `consumes`↔`produces` — so it is built
//! here, on the common ground, ahead of that decision. Permit-licensing of an
//! `Unmet` need, and folding the audit against actual store artifacts, land with
//! the world-level layer.

use crate::nomotheke::{NomosDecl, Promise, NOMOTHEKE};

/// Where a consumed quantity's supply comes from.
#[derive(Clone, Copy)]
pub enum Supply {
    /// Produced by a registered nomos — an edge in the coupling graph.
    Met(&'static NomosDecl),
    /// Produced by **no** nomos. An honest gap: today an in-kernel fudge or an
    /// undeclared uniform assumption; later either filled by a producing nomos
    /// or licensed by a world-level permit. This is the "rain without a sky"
    /// finding when the quantity is precipitation.
    Unmet,
}

/// One consumed quantity, resolved to its supply.
pub struct Requisite {
    pub quantity: &'static str,
    pub supply: Supply,
}

/// The nomos that promises `quantity`, if any. A flux quantity is produced by at
/// most one nomos in a coherent registry (pinned by [`producers_are_unique`]);
/// this returns the first, which is therefore *the* producer.
pub fn producer_of(quantity: &str) -> Option<&'static NomosDecl> {
    NOMOTHEKE
        .iter()
        .copied()
        .find(|n| n.promises.iter().any(|p| p.quantity == quantity))
}

/// The specific promise carrying `quantity` (for its conservation stance).
pub fn promise_of(quantity: &str) -> Option<&'static Promise> {
    NOMOTHEKE
        .iter()
        .flat_map(|n| n.promises.iter())
        .find(|p| p.quantity == quantity)
}

/// The direct requisites of a nomos — each consumed quantity resolved to its
/// supply. One hop; use [`requisite_chain`] for the transitive closure.
pub fn requisites(nomos: &NomosDecl) -> Vec<Requisite> {
    nomos
        .consumes
        .iter()
        .map(|&quantity| Requisite { quantity, supply: match producer_of(quantity) {
            Some(p) => Supply::Met(p),
            None => Supply::Unmet,
        } })
        .collect()
}

/// Every unmet consumption across the whole registry: `(consumer, quantity)`
/// for each consumed quantity no nomos produces. The "what can't we principle
/// yet" list — the anti-whim planner's honest floor.
pub fn unmet_across_registry() -> Vec<(&'static NomosDecl, &'static str)> {
    let mut out = Vec::new();
    for n in NOMOTHEKE {
        for &q in n.consumes {
            if producer_of(q).is_none() {
                out.push((*n, q));
            }
        }
    }
    out
}

/// One line of a transitive requisite chain: its indentation depth, the
/// quantity, and where it is supplied from.
pub struct ChainLine {
    pub depth: usize,
    pub quantity: &'static str,
    pub supply: Supply,
}

/// The transitive requisite closure of a nomos: its consumed quantities, then
/// (for each met one) that producer's consumed quantities, and so on — the full
/// "if you want this, here is everything it ultimately rests on" tree, flattened
/// depth-first with indentation depths. Cycle-guarded on quantities (a coherent
/// registry is acyclic, but the guard makes a mis-declaration terminate rather
/// than loop).
pub fn requisite_chain(nomos: &NomosDecl) -> Vec<ChainLine> {
    let mut lines = Vec::new();
    let mut seen: Vec<&'static str> = Vec::new();
    walk(nomos, 0, &mut lines, &mut seen);
    lines
}

fn walk(nomos: &NomosDecl, depth: usize, lines: &mut Vec<ChainLine>, seen: &mut Vec<&'static str>) {
    for &quantity in nomos.consumes {
        let supply = match producer_of(quantity) {
            Some(p) => Supply::Met(p),
            None => Supply::Unmet,
        };
        lines.push(ChainLine { depth, quantity, supply });
        if let Supply::Met(producer) = supply {
            if !seen.contains(&quantity) {
                seen.push(quantity);
                walk(producer, depth + 1, lines, seen);
            }
        }
    }
}

/// Render the whole registry's flux web + unmet-needs summary as text — the
/// section `vivarium status` prints beside the fidelity pyramid. Kept here (not
/// in the CLI) so the wording is unit-testable.
pub fn render_flux_web() -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    let _ = writeln!(s, "flux web (declared coupling; ← producer, ✗ = unmet — no producer):");
    for n in NOMOTHEKE {
        let _ = writeln!(s, "  {}", n.name);
        for r in requisites(n) {
            match r.supply {
                Supply::Met(p) => {
                    let _ = writeln!(s, "      consumes {:<34} ← {}", r.quantity, p.name);
                }
                Supply::Unmet => {
                    let _ = writeln!(s, "      consumes {:<34} ✗ UNMET (no nomos produces this)", r.quantity);
                }
            }
        }
        for p in n.promises {
            let _ = writeln!(s, "      produces {:<34} [{}]", p.quantity, p.conservation.label());
        }
    }
    let unmet = unmet_across_registry();
    if unmet.is_empty() {
        let _ = writeln!(s, "\nunmet flux needs: none — every consumed quantity has a producer.");
    } else {
        let _ = writeln!(s, "\nunmet flux needs ({}):", unmet.len());
        for (consumer, q) in &unmet {
            let _ = writeln!(s, "  {} needs {q} — but no nomos produces it", consumer.name);
        }
        // The named specimen, if present, gets the plain-language verdict.
        if unmet.iter().any(|(_, q)| *q == crate::flux::PRECIPITATION) {
            let _ = writeln!(
                s,
                "  → \"can we rain principled water?\" No: precipitation has no producer yet\n\
                 \x20   (the atmosphere reservoir → water-cycle chain, TODO §water-system).\n\
                 \x20   erosion's incision and water's fill run on an in-kernel rain assumption\n\
                 \x20   until then — which is why their derived Earth-fidelity folds to placeholder."
            );
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flux;
    use crate::nomotheke::{EROSION, SPINE, WATER};

    #[test]
    fn producers_are_unique() {
        // No two nomoi may promise the same quantity — else `producer_of` is
        // ambiguous and the coupling graph is ill-defined. (A future "two rungs
        // fill the same slot" case is a slot/occupant choice at the world layer,
        // not two producers of one quantity in one registry.)
        for q in flux::VOCABULARY {
            let producers = NOMOTHEKE.iter().filter(|n| n.promises.iter().any(|p| p.quantity == *q)).count();
            assert!(producers <= 1, "quantity {q:?} is produced by {producers} nomoi — ambiguous");
        }
    }

    #[test]
    fn erosion_surface_is_met_by_the_spine() {
        let met: Vec<_> = requisites(&EROSION)
            .into_iter()
            .filter_map(|r| match r.supply {
                Supply::Met(p) if r.quantity == flux::SURFACE_ELEVATION => Some(p.name),
                _ => None,
            })
            .collect();
        assert_eq!(met, vec!["spine-tile"], "erosion's surface input resolves to the spine");
    }

    #[test]
    fn uplift_is_met_by_its_own_nomos() {
        // The consolidation's point: uplift is a SEPARATE nomos erosion consumes,
        // not a hidden term. So rock-uplift-rate resolves to the uplift nomos, and
        // erosion's requisites include it as Met.
        assert_eq!(producer_of(flux::ROCK_UPLIFT_RATE).map(|n| n.name), Some("uplift-tile"));
        let met_uplift = requisites(&EROSION).into_iter().any(|r| {
            r.quantity == flux::ROCK_UPLIFT_RATE && matches!(r.supply, Supply::Met(p) if p.name == "uplift-tile")
        });
        assert!(met_uplift, "erosion consumes rock-uplift-rate, met by the uplift nomos");
    }

    #[test]
    fn precipitation_is_met_and_the_chain_is_closed() {
        // Was the live UNMET gap; the climate nomos now produces precipitation, so
        // NOTHING in the registry is unmet — every consumed quantity has a
        // producer, and the whole water chain closes back to the hydrosphere.
        assert_eq!(producer_of(flux::PRECIPITATION).map(|n| n.name), Some("climate"));
        assert!(unmet_across_registry().is_empty(), "the declared flux web is fully met — no dangling needs");
    }

    #[test]
    fn water_chain_reaches_the_hydrosphere_all_met() {
        // The transitive closure of water now closes: eroded-surface (→erosion) →
        // surface-elevation (→spine) + rock-uplift (→uplift) + precipitation
        // (→climate) → atmosphere-water (→hydrosphere, which consumes nothing).
        // Every line is Met, and the chain reaches the atmosphere-water stock.
        let chain = requisite_chain(&WATER);
        assert!(
            chain.iter().all(|l| matches!(l.supply, Supply::Met(_))),
            "every requisite of water is met — the chain is closed"
        );
        assert!(
            chain.iter().any(|l| l.quantity == flux::PRECIPITATION && matches!(l.supply, Supply::Met(p) if p.name == "climate")),
            "water's rain resolves to the climate nomos"
        );
        assert!(
            chain.iter().any(|l| l.quantity == flux::ATMOSPHERE_WATER && l.depth > 0),
            "the chain reaches the conserved atmosphere-water stock transitively"
        );
    }

    #[test]
    fn spine_has_no_requisites() {
        assert!(requisites(&SPINE).is_empty(), "the spine is the root input — it consumes nothing");
    }

    #[test]
    fn render_reports_the_closed_chain() {
        let text = render_flux_web();
        assert!(text.contains("unmet flux needs: none"), "the rendered web reports the chain fully met");
        assert!(!text.contains("UNMET"), "no dangling need remains to flag");
        assert!(text.contains("conserved"), "conservation stances are still shown");
    }
}
