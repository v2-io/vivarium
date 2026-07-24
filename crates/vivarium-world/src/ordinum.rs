//! The ordinum, read — so the phase ladder can DRIVE nomos creation instead of us
//! picking nomos ad-hoc.
//!
//! **Why this exists** (Joseph, 2026-07-12): *"the info I'd really like to see is which
//! promises and charges are in progress vs not-started — the thing that will actually
//! drive the nomos creation in a more useful instead of ad-hoc way."* Every nomos built
//! so far (uplift, hydrosphere, climate) was chosen by a session's judgement, not by the
//! ladder. The ordinum already says what each phase must deliver; nobody was reading it.
//!
//! So: parse `tabularium/terrestris.ordinum.udon`, cross-reference each promise against
//! the [`crate::nomotheke`], and report the **maturity** of every promise in force —
//! which turns "what should I build next?" from taste into a lookup.
//!
//! **The maturity ladder** (`#detail-regula-design` §4c, Joseph's
//! acceptance-test discipline):
//!
//! | rung | meaning |
//! |---|---|
//! | `NotStarted` | prose only — no predicate, nothing claims it |
//! | `Specified`  | it has a `\|predicate` — something could now convict it |
//! | `Claimed`    | a nomos is named in `:kept-by` **and that nomos exists** |
//! | `Kept`       | claimed AND the predicate verified |
//!
//! **The acceptance-test rule, enforced here:** *a promise with no predicate cannot be
//! marked fulfilled at any epistemic level* — nothing has been defined that could convict
//! it. Those surface as `un-checkable`, which is itself the finding.
//!
//! `Kept` is deliberately NOT auto-derivable yet: the predicates are prose ("land rises
//! above sea level; land-fraction toward the early-Earth target band"). Claiming `Kept`
//! without running something that could refute it would be exactly the plausibility-as-
//! verification failure this project keeps catching. So the report stops at `Claimed` and
//! says so.
//!
//! **A `:kept-by` naming a nomos that does not exist in the nomotheke is an ERROR**, not a
//! rung — the ladder would be lying about its own coverage.
//!
//! Parser scope: the ordinum's actual subset only (indentation-nested `|element[slug]`
//! with `:field value` and body prose). Deliberately NOT a general udon parser — when
//! `libudon` is wired as a dependency this whole module's front half should be deleted in
//! favour of it. Marked so nobody mistakes it for one.

use crate::nomotheke;

/// Where a promise sits on the acceptance ladder.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Maturity {
    /// Prose only. No predicate, no keeper. Nothing could convict it.
    NotStarted,
    /// Has a `|predicate` — falsifiable, but nothing claims to keep it.
    Specified,
    /// A nomos is named in `:kept-by`, and that nomos is registered.
    Claimed,
    /// `:kept-by` names a nomos the nomotheke has never heard of. A lie in the ladder.
    BrokenKeeper,
}

impl Maturity {
    pub fn label(self) -> &'static str {
        match self {
            Maturity::NotStarted => "NOT-STARTED",
            Maturity::Specified => "specified",
            Maturity::Claimed => "claimed",
            Maturity::BrokenKeeper => "BROKEN-KEEPER",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Charge {
    pub slug: String,
    /// `gate` / `mech` / `earth` / `emergent` — the ARROW KIND, never a build-status.
    pub tag: String,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct Promise {
    pub slug: String,
    /// `.state` / `.regime` / `.capability` / `.limit`
    pub class: String,
    /// The nomos claimed to keep it, if any.
    pub kept_by: Option<String>,
    /// The charge it descends from.
    pub from: Option<String>,
    /// Its falsifiable core. Absent ⇒ un-checkable, and it cannot be called fulfilled.
    pub predicate: Option<String>,
    pub text: String,
}

impl Promise {
    pub fn maturity(&self) -> Maturity {
        match &self.kept_by {
            Some(n) if nomotheke::lookup(n).is_none() => Maturity::BrokenKeeper,
            Some(_) => Maturity::Claimed,
            None if self.predicate.is_some() => Maturity::Specified,
            None => Maturity::NotStarted,
        }
    }
    /// The acceptance-test rule: no predicate ⇒ nothing could ever convict it.
    pub fn is_uncheckable(&self) -> bool {
        self.predicate.is_none()
    }
}

#[derive(Clone, Debug)]
pub struct Phase {
    pub num: u32,
    pub name: String,
    pub slug: String,
    pub charges: Vec<Charge>,
    pub promises: Vec<Promise>,
}

#[derive(Clone, Debug, Default)]
pub struct Ordinum {
    pub phases: Vec<Phase>,
}

impl Ordinum {
    /// Parse the ordinum's udon subset. See the module note: this is NOT a general udon
    /// parser and must be replaced by `libudon` when that lands.
    pub fn parse(src: &str) -> Ordinum {
        let mut o = Ordinum::default();
        // Indent of the enclosing element, so we know when a phase/promise ends.
        let mut promise_indent = usize::MAX;

        for raw in src.lines() {
            let indent = raw.len() - raw.trim_start().len();
            let line = raw.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            // A |predicate belongs to the promise it is nested under.
            if let Some(rest) = line.strip_prefix("|predicate") {
                if indent > promise_indent {
                    if let Some(p) = o.phases.last_mut().and_then(|ph| ph.promises.last_mut()) {
                        let t = rest.trim();
                        match &mut p.predicate {
                            Some(existing) => {
                                existing.push(' ');
                                existing.push_str(t);
                            }
                            None => p.predicate = Some(t.to_string()),
                        }
                    }
                }
                continue;
            }

            if let Some(rest) = line.strip_prefix("|phase[") {
                let slug = rest.split(']').next().unwrap_or("").to_string();
                o.phases.push(Phase { num: 0, name: String::new(), slug, charges: vec![], promises: vec![] });
                promise_indent = usize::MAX;
                continue;
            }
            if let Some(rest) = line.strip_prefix("|charge[") {
                let slug = rest.split(']').next().unwrap_or("").to_string();
                let tag = field(rest, ":tag").unwrap_or_else(|| "-".into());
                if let Some(ph) = o.phases.last_mut() {
                    ph.charges.push(Charge { slug, tag, text: String::new() });
                }
                promise_indent = usize::MAX;
                continue;
            }
            if let Some(rest) = line.strip_prefix("|promise[") {
                let slug = rest.split(']').next().unwrap_or("").to_string();
                // `.class` sits immediately after the `]`.
                let after = rest.split(']').nth(1).unwrap_or("");
                let class = after
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_start_matches('.')
                    .to_string();
                let p = Promise {
                    slug,
                    class,
                    kept_by: field(rest, ":kept-by"),
                    from: field(rest, ":from"),
                    predicate: None,
                    text: String::new(),
                };
                if let Some(ph) = o.phases.last_mut() {
                    ph.promises.push(p);
                }
                promise_indent = indent;
                continue;
            }
            // Fields of the current phase.
            if let Some(v) = line.strip_prefix(":num ") {
                if let Some(ph) = o.phases.last_mut() {
                    ph.num = v.trim().parse().unwrap_or(0);
                }
                continue;
            }
            if let Some(v) = line.strip_prefix(":name ") {
                if let Some(ph) = o.phases.last_mut() {
                    ph.name = v.trim().to_string();
                }
                continue;
            }
            // Body prose: attach to the promise/charge currently open (best-effort).
            if !line.starts_with('|') && !line.starts_with(':') {
                if let Some(ph) = o.phases.last_mut() {
                    if indent > promise_indent {
                        if let Some(p) = ph.promises.last_mut() {
                            push_prose(&mut p.text, line);
                        }
                    } else if let Some(c) = ph.charges.last_mut() {
                        if c.text.is_empty() || !c.text.ends_with('.') {
                            push_prose(&mut c.text, line);
                        }
                    }
                }
            }
        }
        o
    }

    pub fn phase(&self, num: u32) -> Option<&Phase> {
        self.phases.iter().find(|p| p.num == num)
    }
}

fn push_prose(buf: &mut String, line: &str) {
    if !buf.is_empty() {
        buf.push(' ');
    }
    buf.push_str(line);
}

/// Read a `:field value` out of an element's header line. Values run to the next `:field`
/// or end of line; quoted values are unquoted.
fn field(header: &str, key: &str) -> Option<String> {
    let at = header.find(key)?;
    let rest = header[at + key.len()..].trim_start();
    let rest = rest.strip_prefix('"').map_or(rest, |r| r);
    let end = rest.find(" :").unwrap_or(rest.len());
    let v = rest[..end].trim().trim_matches('"').trim();
    if v.is_empty() {
        None
    } else {
        Some(v.to_string())
    }
}

/// The ordinum as shipped, compiled in — so the report cannot drift from the artifact.
pub const TERRESTRIS: &str = include_str!("../../../tabularium/terrestris.ordinum.udon");

/// Parse the shipped Terrestris ordinum.
pub fn terrestris() -> Ordinum {
    Ordinum::parse(TERRESTRIS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_shipped_ordinum() {
        let o = terrestris();
        assert!(o.phases.len() >= 9, "expected the full ladder, got {}", o.phases.len());
        let ab = o.phase(4).expect("abyssal");
        assert_eq!(ab.name, "Abyssal");
        assert!(ab.charges.iter().any(|c| c.slug == "emergent-land" && c.tag == "gate"));
    }

    #[test]
    fn the_land_promise_is_unkept_and_that_is_the_point() {
        // The whole reason this module exists: Abyssal's `emerged-land` — the LAND
        // ITSELF — has a falsifiable predicate but NO nomos claims to keep it. That is
        // the #1 gap in the ladder, and it should be what drives the next nomos.
        let o = terrestris();
        let ab = o.phase(4).unwrap();
        let land = ab.promises.iter().find(|p| p.slug == "emerged-land").expect("emerged-land");
        assert!(land.predicate.is_some(), "it is specified — something could convict it");
        assert_eq!(land.kept_by, None, "and NOTHING keeps it — the gap the ladder is pointing at");
        assert_eq!(land.maturity(), Maturity::Specified);
    }

    #[test]
    fn erosion_substrate_is_claimed_by_a_real_nomos() {
        // And the cross-check that makes the report trustworthy: a `:kept-by` must name
        // a nomos the nomotheke actually has, or the ladder is lying about its coverage.
        let o = terrestris();
        let ab = o.phase(4).unwrap();
        let ero = ab.promises.iter().find(|p| p.slug == "erosion-substrate").unwrap();
        assert_eq!(ero.kept_by.as_deref(), Some("erosion-tile"));
        assert_eq!(ero.maturity(), Maturity::Claimed, "erosion-tile is registered");
    }

    #[test]
    fn no_promise_in_the_ladder_names_a_nonexistent_nomos() {
        for ph in &terrestris().phases {
            for p in &ph.promises {
                assert_ne!(
                    p.maturity(),
                    Maturity::BrokenKeeper,
                    "phase {} promise '{}' is :kept-by '{}' — a nomos the nomotheke does not have",
                    ph.num,
                    p.slug,
                    p.kept_by.clone().unwrap_or_default()
                );
            }
        }
    }
}
