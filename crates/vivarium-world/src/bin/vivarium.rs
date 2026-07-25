//! `vivarium` — the world-lifecycle CLI: builder v0 + instruments.
//!
//! The operational front door of `#detail-builder-daemon`:
//! a **builder** process that advances a vivium in the background while any
//! number of read-only explorers watch through the store (the store is the bus
//! — no IPC). Subcommands:
//!
//! Run it as `bin/vivarium <cmd>` (a wrapper over the cargo invocation).
//!
//! **The world directory is optional everywhere and every command announces
//! which one it resolved, and why** — explicit argument, `$VIVARIUM_WORLD`, or
//! the shared default. Two silent fallbacks mean the commonest invocation acts
//! on a world the caller never named; the announce line is what stops "optional"
//! from meaning "unknowable."
//!
//! **Demand lives in the manifest** ( #form-manifest-prescribes-vivium FE(2) ):
//! `order`, `target_phase`, `level`, `frames`, `erosion_epochs`, `water_steps`.
//! Build flags SET it and it sticks, and `vivarium demand` shows or sets it
//! without building. There is no override layer: an earlier design let flags win
//! for one run, which meant `--frames 60` materialized sixty stages while the
//! views -- which read the manifest -- went on showing six. Persisting is free
//! because demand is in no key: writing it invalidates not one memo.
//!
//! - `vivarium new [dir] [name]` — individuate a world: write its manifest
//!   (fresh seed unless the dir already has one — identity is never re-minted).
//! - `vivarium build [dir] [--level L] [--epochs E] [--frames N]` — builder v0:
//!   sweep all six cube faces at level `L` through the initial-topography nomos
//!   (the breadth-first, whole-world degenerate beacon), then erode and settle
//!   the same tiles (`--epochs 0` skips both), then materialize the deep-time
//!   cooling stages. Appends `build.log`, maintains `status.json`, holds
//!   `builder.lock`; a second invocation on a LIVE build **attaches** instead of
//!   failing.
//! - `vivarium watch [dir] [--replay]` — the build reader: the globe repainted as
//!   roots land. Live follows a running builder; `--replay` walks the landing
//!   history. One mechanism, two ends ( #form-time-indexed-stage-chains FE(5) ).
//! - `vivarium status [dir]` — this world's demand, the **fidelity pyramid** (a
//!   census of roots by nomos × level), the flux audit, and the ordinum maturity.
//! - `vivarium info [dir]` — a one-shot from-space globe coloured by build-state.
//! - `vivarium attach [dir]` — follow a running build's log (Ctrl-C detaches;
//!   the builder is unaffected).
//!
//! Builder v0 is deliberately thin: no demand spool yet (explorers file demand
//! in the next increment), no beacon parsing from the manifest (the sweep IS
//! the whole-world beacon). It exists to make the decoupling REAL: run `build`,
//! walk away, run `status`/`watch`/`attach` from other terminals, run the globe
//! on the same dir — nothing coordinates except the store.
//!
//! Lives as a bin inside `vivarium-world` (not its own crate) to keep the
//! workspace Cargo.toml untouched while a parallel agent owns edits to it;
//! graduating to `crates/vivarium-cli` later is mechanical.

use std::io::Write as _;
use std::path::{Path, PathBuf};

use vivarium_world::audit;
use vivarium_world::lithosphere::MANTLE_TP_C;
use vivarium_world::mantle_thermal::{self, potential_temp_c};
use vivarium_world::nomotheke;
use vivarium_world::ordinum;
use vivarium_world::query::{Source, World};
use vivarium_world::spec::WorldSpec;
use vivarium_world::sphere::Face;
use vivarium_world::store::Store;

const TILE_NX: usize = 64;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // `-h` anywhere prints usage and stops — including after a subcommand, so
    // `vivarium build -h` answers rather than building a world. The one
    // exception is `explore`, which is a dispatch: swallowing its `--help` would
    // print this file's usage for a binary that has its own, and the caller
    // would have no way to reach the real one.
    if args.first().map(String::as_str) == Some("explore") {
        std::process::exit(cmd_explore(&args[1..]));
    }
    if args.iter().any(|a| a == "-h" || a == "--help") || args.is_empty() {
        print!("{}", usage());
        std::process::exit(if args.is_empty() { 2 } else { 0 });
    }
    let code = match args.first().map(String::as_str) {
        Some("new") => cmd_new(&args[1..]),
        Some("build") => cmd_build(&args[1..]),
        Some("status") => cmd_status(&args[1..]),
        Some("info") => cmd_info(&args[1..]),
        Some("watch") => cmd_watch(&args[1..]),
        Some("demand") => cmd_demand(&args[1..]),
        Some("attach") => cmd_attach(&args[1..]),
        Some("explore") => cmd_explore(&args[1..]),
        _ => {
            eprint!("{}", usage());
            2
        }
    };
    std::process::exit(code);
}

/// Usage, written to be read start-to-finish once: what a world *is* here, how
/// one is chosen, where the settings live, and only then the subcommands.
fn usage() -> String {
    "\
vivarium — build a world, watch it happen, watch it again.

  WHICH WORLD?  Every command takes an optional [dir] and otherwise falls back to
                $VIVARIUM_WORLD, then to ~/.cache/vivarium/globe-world — the same
                world `vivarium explore` opens. Every command
                prints which one it resolved and why, so \"optional\" never means
                \"unknowable\".

  WHAT IS IT BUILDING?  The manifest holds this vivium's DEMAND (level, frames,
                erosion_epochs, …). Build flags SET it and it sticks — every view
                reads the same manifest, so what you type is what the globe shows.
                `vivarium demand` shows it, and sets it without building.

  TWO KINDS OF SETTING.  Identity (seed, format) is in every memo key: changing it
                forks a different world, so no command here edits it. Demand is in
                no key at all: change it freely, mid-build, invalidating nothing.

COMMANDS

  new [dir] [name]              individuate a world — write its manifest + mint a
                                seed (never re-minted if one exists)

  demand [dir] [k=v …]          show this vivium's prescription, or set it:
                                  vivarium demand
                                  vivarium demand frames=60 level=9
                                fields: order target_phase level frames
                                        erosion_epochs water_steps

  build [dir] [--level L] [--epochs E] [--frames N] [--allow-unmet]
                                sweep all six cube faces at L through initial
                                topography, then erode and settle those tiles
                                (--epochs 0 skips both), then materialize the
                                deep-time cooling stages. Flags SET the manifest
                                and stick; demand is in no key, so nothing is
                                invalidated and every view sees the change.
                                Refuses erosion/water while flux needs are unmet,
                                unless --allow-unmet (writes provisional roots,
                                logs the waiver, and status marks them).
                                Re-running attaches to a live builder.

  watch [dir] [--replay] [--speed MS] [--frames N] [--width W] [--lon0 DEG]
                                the build reader — the globe repainted as roots
                                land. Live follows a running builder; --replay
                                walks the store's landing history instead. One
                                mechanism: replay is the same reader, done landing.
                                Replay orders by root LANDING time — build history,
                                not world-time.

  status [dir]                  demand + fidelity pyramid + water budget + flux
                                audit + ordinum maturity
  info [dir] [--width W] [--lon0 DEG] [--color|--no-color]
                                one-shot whole-sphere globe, coloured by build state
  attach [dir]                  follow a running build's log (Ctrl-C detaches;
                                the builder is unaffected)

  explore [dir] [--replay] [--level L] [--frames N] [--paint MODE]
                                the 3D explorer -- `watch` at a different fidelity
                                of attention. Same store, same reader, same
                                live-vs-replay semantics, same epistemic overlay;
                                it differs only in how it draws. Paint modes:
                                surface | provenance | water | seam. Press C on
                                anything that looks wrong and it writes a sighting.
                                (Runs as a separate binary, `vivarium-explore`, so
                                this CLI never links a renderer -- see below.)

EXAMPLE

  vivarium build --level 9 --frames 60   # flags stick; the globe sees them
  vivarium watch                      # in another — see it happen
  vivarium watch --replay             # afterwards — see it again
"
    .to_string()
}

/// Non-flag tokens in order (world dir, optional name, …), skipping values that
/// belong to known flags (`--level L`, `--epochs E`). Bare non-flag tokens that
/// are not values of a preceding flag remain positionals — so
/// `build --epochs 0 /path` resolves to `/path`, not `0`.
fn positionals(rest: &[String]) -> Vec<&str> {
    let flag_takes_value = |a: &str| {
        matches!(a, "--level" | "--epochs" | "--width" | "--lon0" | "--frames" | "--speed")
    };
    let mut out = Vec::new();
    let mut i = 0;
    while i < rest.len() {
        let a = rest[i].as_str();
        if a.starts_with('-') {
            if flag_takes_value(a) {
                i += 2; // skip flag and its value
                continue;
            }
            i += 1; // bare switch (--allow-unmet, --color, …)
            continue;
        }
        // `frames=60` is an assignment for `demand`, not a world directory.
        // Without this, the first assignment is taken as the path and the command
        // reports "no manifest" on a directory that does not exist. Matched
        // narrowly (a lowercase identifier before the `=`) so a real path
        // containing `=` is still a path.
        if !is_assignment(a) {
            out.push(a);
        }
        i += 1;
    }
    out
}

/// `key=value`, where key is a bare lowercase identifier.
fn is_assignment(tok: &str) -> bool {
    match tok.split_once('=') {
        Some((k, _)) => !k.is_empty() && k.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
        None => false,
    }
}

/// Resolve which world to act on, matching the globe's convention so
/// `vivarium status` and `vivarium explore` look at the SAME world by default:
/// the first non-flag positional wins (not merely `rest[0]` — flags may lead),
/// else `$VIVARIUM_WORLD`, else the shared default
/// `${XDG_CACHE_HOME:-~/.cache}/vivarium/globe-world`.
/// The world directory **and how it was chosen**.
///
/// The provenance is not decoration. Every command here takes an optional dir
/// and silently falls back twice, so the most common invocation — no path at all
/// — acts on a world the caller never named and cannot see from the command they
/// typed. Printing which world, and why that one, is the difference between an
/// instrument and a guess about what just happened.
fn world_dir_resolved(rest: &[String]) -> (PathBuf, String) {
    if let Some(p) = positionals(rest).first() {
        return (PathBuf::from(p), "given on the command line".into());
    }
    if let Ok(p) = std::env::var("VIVARIUM_WORLD") {
        return (PathBuf::from(p), "$VIVARIUM_WORLD".into());
    }
    let cache = std::env::var("XDG_CACHE_HOME").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into())).join(".cache")
    });
    (
        cache.join("vivarium").join("globe-world"),
        "default — no dir given and $VIVARIUM_WORLD unset; the same world the explorer opens".into(),
    )
}

/// One line naming the world every command is about to act on. Printed before
/// anything else, including before failures, so a command that errors still says
/// what it was pointed at.
fn announce_world(rest: &[String]) -> PathBuf {
    let (dir, why) = world_dir_resolved(rest);
    println!("world  {}\n       ({why})", dir.display());
    dir
}

/// True when an explicit world dir appears as a non-flag positional.
fn dir_is_explicit(rest: &[String]) -> bool {
    !positionals(rest).is_empty()
}

fn flag(rest: &[String], name: &str) -> Option<u32> {
    rest.iter().position(|a| a == name).and_then(|i| rest.get(i + 1)).and_then(|v| v.parse().ok())
}

fn cmd_new(rest: &[String]) -> i32 {
    let dir = announce_world(rest);
    // Name is the second non-flag positional when dir is explicit; else default.
    let pos = positionals(rest);
    let name = if dir_is_explicit(rest) {
        pos.get(1).copied().unwrap_or("unnamed")
    } else {
        "unnamed"
    };
    match WorldSpec::load(&dir) {
        Ok(Some(spec)) => {
            println!("vivium already exists: \"{}\" seed {} — identity is never re-minted.", spec.name, spec.seed);
            println!("(the name is a label; edit `manifest` freely. A new SEED means a new world: use a new dir.)");
            0
        }
        Ok(None) => match WorldSpec::load_or_create(&dir, name) {
            Ok(spec) => {
                println!("vivium \"{}\" created at {} — seed {}", spec.name, dir.display(), spec.seed);
                0
            }
            Err(e) => {
                eprintln!("error: {e}");
                1
            }
        },
        Err(e) => {
            eprintln!("error reading manifest: {e}");
            1
        }
    }
}

// ---- builder v0 -------------------------------------------------------------

struct BuilderLog {
    log: std::fs::File,
    status_path: PathBuf,
}

impl BuilderLog {
    fn line(&mut self, msg: &str) {
        let stamped = format!("[{}] {msg}", wallclock());
        println!("{stamped}");
        let _ = writeln!(self.log, "{stamped}");
    }
    /// Hand-written JSON (std-only crate); fields are the attach/monitor contract.
    fn status(&self, phase: &str, done: usize, total: usize) {
        let body = format!(
            "{{\n  \"phase\": \"{phase}\",\n  \"done\": {done},\n  \"total\": {total},\n  \"pid\": {},\n  \"updated\": \"{}\"\n}}\n",
            std::process::id(),
            wallclock()
        );
        let tmp = self.status_path.with_extension("tmp");
        if std::fs::write(&tmp, body).is_ok() {
            let _ = std::fs::rename(&tmp, &self.status_path);
        }
    }
}

fn wallclock() -> String {
    // Seconds since epoch — honest and dependency-free; humane formatting can
    // come with a real time crate in the standalone-CLI graduation.
    let s = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("t+{s}")
}

/// Nomos for a builder phase name, if that phase is a declared nomos sweep.
fn phase_nomos(phase: &str) -> Option<&'static vivarium_world::nomotheke::NomosDecl> {
    use vivarium_world::nomotheke::{EROSION, INITIAL_TOPOGRAPHY, WATER};
    match phase {
        "initial-topography" => Some(&INITIAL_TOPOGRAPHY),
        "erosion" => Some(&EROSION),
        "water" => Some(&WATER),
        _ => None,
    }
}

/// Unmet flux quantities that would make this phase's nomos (and its direct
/// dep chain) unprincipled to run — the builder admission check.
fn phase_unmet_quantities(phase: &str) -> Vec<&'static str> {
    let Some(n) = phase_nomos(phase) else {
        return maybe_forced_unmet(phase, Vec::new());
    };
    let real = audit::requisite_chain(n)
        .into_iter()
        .filter(|line| matches!(line.supply, audit::Supply::Unmet))
        .map(|line| line.quantity)
        .collect();
    maybe_forced_unmet(phase, real)
}

/// Test/diagnostic seam: `VIVARIUM_TEST_FORCE_UNMET=<phase>` injects a synthetic
/// unmet quantity for that phase's admission check. It can only make admission
/// **stricter** (add an unmet need) — never weaker — so it cannot launder an
/// unlawful build into a lawful one; the worst it can do is refuse a phase that
/// would otherwise pass. It exists because the shipped registry is closed (no
/// phase is Unmet as-built), which would otherwise leave the binary's refuse and
/// waiver-to-provisional paths untestable end to end (#form-builder-admission
/// residuals). Absent the env var this is a no-op.
fn maybe_forced_unmet(phase: &str, mut real: Vec<&'static str>) -> Vec<&'static str> {
    if std::env::var("VIVARIUM_TEST_FORCE_UNMET").as_deref() == Ok(phase) {
        real.push("test-forced-unmet");
    }
    real
}

fn cmd_build(rest: &[String]) -> i32 {
    let dir = announce_world(rest);
    let allow_unmet = rest.iter().any(|a| a == "--allow-unmet");

    let spec = match WorldSpec::load_or_create(&dir, "unnamed") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };

    // The manifest holds this vivium's demand ( #form-manifest-prescribes-vivium
    // FE(2) ); a flag SETS it. There is no override layer, deliberately: an
    // earlier build here read the manifest and let flags win for one run only,
    // which meant `--frames 60` materialized sixty stages while the views — which
    // read the manifest — went on showing six. A setting the user typed and the
    // viewer ignores is worse than no setting. Persisting costs nothing, because
    // demand is in no key: writing it invalidates not one memo (FE(5)).
    let mut spec = spec;
    let before = spec.demand.clone();
    if let Some(l) = flag(rest, "--level") {
        spec.demand.level = l.min(20) as u8;
    }
    if let Some(e) = flag(rest, "--epochs") {
        spec.demand.erosion_epochs = e;
    }
    if let Some(f) = flag(rest, "--frames") {
        spec.demand.frames = f;
    }
    let changed: Vec<String> = [
        ("level", spec.demand.level as u32, before.level as u32),
        ("erosion_epochs", spec.demand.erosion_epochs, before.erosion_epochs),
        ("frames", spec.demand.frames, before.frames),
    ]
    .iter()
    .filter(|(_, now, was)| now != was)
    .map(|(n, now, was)| format!("{n} {was}→{now}"))
    .collect();
    if !changed.is_empty() {
        if let Err(e) = spec.save(&dir) {
            eprintln!("error writing manifest: {e}");
            return 1;
        }
    }
    let (level, epochs, frames) =
        (spec.demand.level, spec.demand.erosion_epochs, spec.demand.frames);

    // Single-builder discipline: hold builder.lock; if a LIVE builder holds it,
    // attach instead of failing (Joseph's preferred UX). A stale lock (dead pid)
    // is reclaimed. (Atomic create_new + RAII still owed — de-novo audit P1.)
    let lock_path = dir.join("builder.lock");
    if let Ok(text) = std::fs::read_to_string(&lock_path) {
        if let Ok(pid) = text.trim().parse::<i32>() {
            if pid_alive(pid) {
                println!("a builder (pid {pid}) is already working this vivium — attaching:\n");
                return tail_log(&dir, true);
            }
        }
    }
    if std::fs::write(&lock_path, std::process::id().to_string()).is_err() {
        eprintln!("error: cannot write {}", lock_path.display());
        return 1;
    }
    // Ensure the lock is cleared on every return path after we own it.
    struct LockGuard<'a>(&'a Path);
    impl Drop for LockGuard<'_> {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(self.0);
        }
    }
    let _lock = LockGuard(&lock_path);

    let store = match Store::open(&dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let world = World::new(&store, spec.seed);
    let log_file = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("build.log"));
    let Ok(log_file) = log_file else {
        eprintln!("error: cannot open build.log");
        return 1;
    };
    let mut out = BuilderLog { log: log_file, status_path: dir.join("status.json") };

    out.line(&format!(
        "builder v0 on vivium \"{}\" (seed {}, order {}, target phase {}) — initial-topography sweep L{level}, {}x{} tiles/face-row, erosion {epochs} epochs, {frames} frames{}",
        spec.name,
        spec.seed,
        spec.demand.order,
        spec.demand.target_phase,
        TILE_NX,
        TILE_NX,
        if allow_unmet { " (--allow-unmet)" } else { "" }
    ));
    if !changed.is_empty() {
        out.line(&format!("manifest updated: {} — the globe and every view see this too", changed.join(", ")));
    }

    // The whole-world degenerate beacon: every face, tiled at TILE_NX.
    let per_face = ((1u64 << level) as usize).div_ceil(TILE_NX);
    let total = 6 * per_face * per_face;
    let mut done = 0;
    let mut computed = 0;
    for phase in ["initial-topography", "erosion", "water"] {
        if phase != "initial-topography" && epochs == 0 {
            out.line("erosion + water skipped (--epochs 0)");
            break;
        }
        // Admission: refuse phases whose flux requisites are unmet unless waived.
        // `status` already prints this truth; the builder must not materialize
        // artifacts the formal contract calls unrunnable (de-novo audit P0).
        let unmet = phase_unmet_quantities(phase);
        if !unmet.is_empty() {
            if allow_unmet {
                out.line(&format!(
                    "{phase}: WAIVED unmet flux needs ({}) — roots will be tagged provisional; do not cite as lawful",
                    unmet.join("; ")
                ));
                world.set_provisional_writes(true);
            } else {
                out.line(&format!(
                    "{phase}: REFUSED — unmet flux needs: {}. re-run with --allow-unmet for provisional materialization, or keep a producer for these quantities first.",
                    unmet.join("; ")
                ));
                out.status("refused", done, total);
                eprintln!(
                    "error: phase `{phase}` refused — unmet flux: {}\n  (vivarium status explains the web; --allow-unmet to force provisional)",
                    unmet.join("; ")
                );
                return 2;
            }
        } else {
            world.set_provisional_writes(false);
        }
        done = 0;
        out.status(phase, 0, total);
        let t0 = std::time::Instant::now();
        for f in 0..6 {
            let face = Face::from_index(f);
            for tj in 0..per_face {
                for ti in 0..per_face {
                    let (oi, oj) = ((ti * TILE_NX) as u32, (tj * TILE_NX) as u32);
                    let src = match phase {
                        "initial-topography" => world.initial_topography(face, level, oi, oj, TILE_NX).1,
                        "erosion" => world.erosion_tile(face, level, oi, oj, TILE_NX, epochs).1,
                        // Water's step count was a bare `200` literal here — the
                        // same class of misplacement as erosion's flag, failing
                        // quietly instead of loudly (findings item 4). It is
                        // still an arbitrary number ( #obs-water-fill-never-settles
                        // measured that no criterion can replace it yet), but it
                        // is now an arbitrary number this WORLD asked for, in the
                        // file that records what this world asked for.
                        _ => world.water_tile(face, level, oi, oj, TILE_NX, epochs, spec.demand.water_steps).1,
                    };
                    done += 1;
                    if src == Source::Computed {
                        computed += 1;
                    }
                    if done % 64 == 0 || done == total {
                        out.status(phase, done, total);
                    }
                }
            }
            out.line(&format!("{phase}: face {f} done ({done}/{total} tiles, {computed} computed this run)"));
        }
        out.line(&format!("{phase}: swept {total} tiles in {:.1?} ({computed} computed, {} were hits)", t0.elapsed(), total - (computed)));
        world.set_provisional_writes(false);
        computed = 0;
    }
    // Cooling-stage reductions — the cost belongs here, at build time. Materialize
    // each stage's global scalars (derived sea + rock-mass-ledger integrals) as
    // store citizens under complete keys, so a fresh globe/explorer process HITS
    // them and never runs a cold ~393k-cell pour to warm a stage
    // (`#form-store-as-save` FE(6), decided: memoized ≡ store object; closes
    // `#form-builder-admission` FE(4)/#5 store-side half). Present stage first (the
    // live world), then the abyssal cooling chain. Always lawful (the isostasy
    // chain has no unmet flux), so these write non-provisional roots.
    //
    // `--frames N` sets how densely the chain is sampled — the one knob that
    // makes deep-time playback watchable at more than six frames. It refines by
    // BISECTION, so every coarser chain is a subset and every reduction already
    // in the store still hits ( #form-time-indexed-stage-chains FE(8)); asking
    // for a non-nested count rounds up, and the log says so rather than quietly
    // giving you a different number than you asked for.
    world.set_provisional_writes(false);
    {
        let refine = mantle_thermal::refinements_for(frames as usize);
        let stages = mantle_thermal::cooling_stages_refined(refine);
        if (frames as usize) != stages.len() {
            out.line(&format!(
                "--frames {frames} is not a nested count; using {} cooling stages (refinement {refine}) so existing reductions still hit",
                stages.len()
            ));
        }
        let mut tps: Vec<f64> = vec![MANTLE_TP_C];
        tps.extend(stages.iter().map(|&t| potential_temp_c(t)));
        let t0 = std::time::Instant::now();
        let mut seen = std::collections::BTreeSet::new();
        let mut computed = 0usize;
        for tp in tps {
            if !seen.insert(tp.to_bits()) {
                continue; // present may coincide exactly with a stage T_p
            }
            if world.epoch_reduction(tp).1 == Source::Computed {
                computed += 1;
            }
        }
        out.line(&format!(
            "cooling stages: {} materialized ({computed} computed, {} were hits) in {:.1?} — fresh processes warm every stage from the store, no cold pour",
            seen.len(),
            seen.len() - computed,
            t0.elapsed()
        ));
    }

    out.status("idle", done, total);
    out.line("build complete — the store is the save; explorers see everything already.");
    0
}

fn pid_alive(pid: i32) -> bool {
    // Signal 0 probes existence without touching the process (unix).
    unsafe { libc_kill(pid, 0) == 0 }
}

// Minimal FFI shim so the std-only crate needs no libc dependency.
extern "C" {
    #[link_name = "kill"]
    fn libc_kill(pid: i32, sig: i32) -> i32;
}

// ---- instruments ------------------------------------------------------------

fn cmd_status(rest: &[String]) -> i32 {
    let dir = announce_world(rest);
    let seed = match WorldSpec::load(&dir) {
        Ok(Some(spec)) => {
            println!("vivium \"{}\" — seed {}", spec.name, spec.seed);
            let d = &spec.demand;
            println!(
                "demand (manifest — what THIS vivium asked for; never keyed, edit any time):\n  \
                 order {} · target phase {} · level {} · frames {} · erosion_epochs {} · water_steps {}",
                d.order, d.target_phase, d.level, d.frames, d.erosion_epochs, d.water_steps
            );
            spec.seed
        }
        Ok(None) => {
            println!("(no manifest — not yet a vivium; `vivarium new {}`)", dir.display());
            0
        }
        Err(e) => {
            eprintln!("manifest error: {e}");
            return 1;
        }
    };
    if let Ok(s) = std::fs::read_to_string(dir.join("status.json")) {
        println!("builder: {}", s.lines().collect::<Vec<_>>().join(" ").replace("  ", ""));
    }
    // `status` reports; it does not author. (It pulls `hydrosphere`, which memoizes.)
    let store = match Store::open_read_only(&dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("store error: {e}");
            return 1;
        }
    };
    let roots = match store.roots() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("census error: {e}");
            return 1;
        }
    };
    // The fidelity pyramid: nomos × level counts, levels descending (coarse at
    // the top — the half-population-pyramid Joseph pictured). Provisional roots
    // (waived flux admission) are counted and labeled — they must not look lawful.
    let mut census: std::collections::BTreeMap<(u8, String), (usize, usize)> =
        std::collections::BTreeMap::new();
    let mut unknown = 0;
    let mut provisional_total = 0;
    for r in &roots {
        if r.provisional {
            provisional_total += 1;
        }
        let nomos = r.key.split('@').next().unwrap_or("").to_string();
        let level = r
            .key
            .split('|')
            .find_map(|f| f.strip_prefix("level="))
            .and_then(|v| v.parse::<u8>().ok());
        match (nomos.is_empty(), level) {
            (false, Some(l)) => {
                let e = census.entry((l, nomos)).or_default();
                e.0 += 1;
                if r.provisional {
                    e.1 += 1;
                }
            }
            _ => unknown += 1,
        }
    }
    println!(
        "\nfidelity pyramid ({} roots{}; B = physics tier, declared/derived — the honesty column):",
        roots.len(),
        if provisional_total > 0 {
            format!(", {provisional_total} provisional")
        } else {
            String::new()
        }
    );
    if provisional_total > 0 {
        println!(
            "  ⚠ {provisional_total} root(s) tagged provisional (written under --allow-unmet) — not lawful *in vivia* evidence"
        );
    }
    println!("{:>5}  {:<14} {:>9}  {:>7}  {:>5}  ", "level", "nomos", "B dcl/drv", "tiles", "prov");
    let max = census.values().map(|(n, _)| *n).max().unwrap_or(1);
    for ((level, nomos), (n, p)) in &census {
        let b = match nomotheke::lookup(nomos) {
            Some(d) => format!("{}/{}", d.physics.letter(), d.derived_physics().letter()),
            None => "?/?".to_string(), // a root the registry doesn't know — itself a finding
        };
        let bar = "█".repeat((n * 40 / max).max(1));
        let pmark = if *p > 0 { format!("{p}") } else { "·".into() };
        println!("{level:>5}  {nomos:<14} {b:>9}  {n:>7}  {pmark:>5}  {bar}");
    }
    if unknown > 0 {
        println!("{unknown} pre-census roots (format v1 — valid, not attributable)");
    }

    // The hydrosphere — the planet's conserved water budget (the first reservoir/box
    // nomos). Reported in real units: what fraction of planetary mass is water, and
    // how it partitions across reservoirs. The honest root under precipitation.
    let world = World::new(&store, seed);
    let (h, _) = world.hydrosphere();
    let planet = vivarium_world::planet::Planet::EARTH;
    println!("\nwater budget (hydrosphere — a CONSERVED inventory from the ante-mundane water-mass fraction):");
    println!(
        "  total inventory  {:>10.3e} km³   ({:.1e} of planet mass, as chemical H₂O — declared, not conjured)",
        h.total_km3,
        vivarium_world::hydrosphere::WATER_MASS_FRACTION
    );
    println!(
        "  ├─ ocean+ice+gw  {:>10.3e} km³   (≈ {:.0} m global-equivalent depth)",
        h.ocean_km3,
        h.ocean_m_we(&planet)
    );
    println!(
        "  └─ atmosphere    {:>10.3e} km³   (≈ {:.1} mm global-equiv) — the stock rain will draw from",
        h.atmosphere_km3,
        h.atmosphere_m_we(&planet) * 1000.0
    );
    println!(
        "  conserved: {} (total − Σreservoirs = {:.0e} km³)",
        if h.conservation_residual_km3().abs() < 1.0 { "✓" } else { "✗ LEAK" },
        h.conservation_residual_km3()
    );
    // The declarative flux web + unmet-needs — the pre-run requisite audit,
    // read off the nomotheke with nothing running (the fidelity pyramid says
    // what EXISTS; this says what each nomos NEEDS and whether it is supplied).
    println!("\n{}", audit::render_flux_web().trim_end());
    // The ordinum ladder's maturity — which promises are claimed vs specified vs
    // not-started (the thing Joseph asked to SEE that drives nomos creation from a
    // look-up rather than session taste, #form-ordinum-governs-flux-web). Read off
    // the same `Promise::maturity()` engine the tests pin — no second ladder.
    println!("\n{}", ordinum::render_maturity().trim_end());
    0
}

/// `vivarium info` — a primitive from-space globe, coloured by build-state (the
/// deepest nomos each region has reached in the store). Deliberately thin: the
/// fuller register-separated, unit-bearing `info` report is separate future
/// work; this call renders the globe + a minimal frame and nothing more.
fn cmd_info(rest: &[String]) -> i32 {
    let dir = announce_world(rest);
    let seed = match WorldSpec::load(&dir) {
        Ok(Some(spec)) => {
            println!("vivium \"{}\" — seed {}   (from-space globe · build-state at a glance)", spec.name, spec.seed);
            spec.seed
        }
        Ok(None) => {
            println!("(no manifest — not yet a vivium; `vivarium new {}` then `build`)", dir.display());
            return 0;
        }
        Err(e) => {
            eprintln!("manifest error: {e}");
            return 1;
        }
    };
    // A VIEW's handle: reads served, writes refused and counted
    // ( #form-core-view-wall FE(2) ). `globe::render` pulls `erosion_tile` /
    // `initial_topography` for the deepest materialized field, and those pulls
    // compute-and-put on a miss — so an instrument pointed at a partially built
    // world was a latent second builder. It still renders (the pull computes);
    // it just cannot author.
    let store = match Store::open_read_only(&dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("store error: {e}");
            return 1;
        }
    };
    let roots = match store.roots() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("census error: {e}");
            return 1;
        }
    };
    if roots.is_empty() {
        println!("(nothing built yet — `vivarium build {}` first)", dir.display());
        return 0;
    }
    let width = flag(rest, "--width").unwrap_or(100).clamp(16, 240) as usize;
    let lon0 = rest
        .iter()
        .position(|a| a == "--lon0")
        .and_then(|i| rest.get(i + 1))
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0); // central meridian in degrees
    // Colour: forced on with --color (for piping into `less -R` / a file), off
    // with --no-color, else auto (TTY and NO_COLOR unset).
    let color = !rest.iter().any(|a| a == "--no-color")
        && (rest.iter().any(|a| a == "--color") || vivarium_world::globe::color_auto());

    let world = World::new(&store, seed);
    print!("\n{}", vivarium_world::globe::render(&world, &roots, width, lon0, color));
    0
}

/// `vivarium watch` — the build reader: the globe, repainted as roots land.
///
/// Live and `--replay` are deliberately the **same** code below the frame
/// source: both build a `Vec<RootEntry>` census and hand it to `globe::render`,
/// so what you see building is pixel-identical to what you see replaying
/// ( #form-time-indexed-stage-chains FE(5) — one mechanism, and this is where it
/// is either honoured or quietly broken). The only difference is where the next
/// census comes from: the filesystem, or the landing history already on disk.
///
/// It never blocks the builder and never writes: no lock is taken, and the
/// process is a reader of the store bus like any other explorer
/// ( #form-builder-admission , #form-core-view-wall ).
fn cmd_watch(rest: &[String]) -> i32 {
    let dir = announce_world(rest);
    let replay = rest.iter().any(|a| a == "--replay");
    let width = flag(rest, "--width").unwrap_or(100).clamp(16, 240) as usize;
    let speed_ms = flag(rest, "--speed").unwrap_or(if replay { 250 } else { 500 }) as u64;
    let max_frames = flag(rest, "--frames").unwrap_or(60) as usize;
    let lon0 = rest
        .iter()
        .position(|a| a == "--lon0")
        .and_then(|i| rest.get(i + 1))
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let color = !rest.iter().any(|a| a == "--no-color")
        && (rest.iter().any(|a| a == "--color") || vivarium_world::globe::color_auto());

    let seed = match WorldSpec::load(&dir) {
        Ok(Some(spec)) => spec.seed,
        Ok(None) => {
            println!("(no manifest — not yet a vivium; `vivarium new {}` then `build`)", dir.display());
            return 0;
        }
        Err(e) => {
            eprintln!("manifest error: {e}");
            return 1;
        }
    };
    // The reader is a view: it takes no lock, and now it structurally cannot
    // write either — `globe::render`'s field pulls compute on a miss but the put
    // is refused ( #form-core-view-wall FE(2), #form-builder-admission FE(1) ).
    // Watching a build can no longer add to the build.
    let store = match Store::open_read_only(&dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("store error: {e}");
            return 1;
        }
    };
    let world = World::new(&store, seed);

    if replay {
        let landings = match vivarium_world::watch::landings(&dir) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("replay error: {e}");
                return 1;
            }
        };
        if landings.is_empty() {
            println!("(nothing in the store to replay — `vivarium build {}` first)", dir.display());
            return 0;
        }
        let bounds = vivarium_world::watch::frame_bounds(&landings, max_frames);
        for (n, &end) in bounds.iter().enumerate() {
            let census: Vec<_> = landings[..end].iter().map(|l| l.root.clone()).collect();
            paint(
                &world,
                &census,
                width,
                lon0,
                color,
                &format!(
                    "replay  frame {}/{}  ·  {end} of {} roots landed  ·  ordering: BUILD history (root landing time), not world-time",
                    n + 1,
                    bounds.len(),
                    landings.len()
                ),
            );
            if n + 1 < bounds.len() {
                std::thread::sleep(std::time::Duration::from_millis(speed_ms));
            }
        }
        println!(
            "\nreplayed {} roots in {} frames. What this ordering IS: the order the builder wrote them.\n\
             What it is NOT: world-time. The interior column above is how much world-time interior exists to replay\n\
             at all — a nomos with one distinct time-index has only endpoints ( #form-time-indexed-stage-chains FE(2) ).",
            landings.len(),
            bounds.len()
        );
        return 0;
    }

    // Live: poll the roots census; repaint only when it changes, so a quiet
    // builder does not strobe the terminal.
    let mut last = 0usize;
    let mut ever_saw_builder = false;
    loop {
        let roots = store.roots().unwrap_or_default();
        let builder = builder_status(&dir);
        let live = builder.is_some();
        ever_saw_builder |= live;
        if roots.len() != last {
            let head = match &builder {
                Some(s) => format!("live  ·  {s}  ·  {} roots", roots.len()),
                None => format!("live  ·  no builder running  ·  {} roots (static census)", roots.len()),
            };
            paint(&world, &roots, width, lon0, color, &head);
            last = roots.len();
        }
        if ever_saw_builder && !live {
            println!("\n(builder finished — {} roots. `vivarium watch --replay` to watch it again.)", last);
            return 0;
        }
        if !ever_saw_builder && last > 0 {
            println!(
                "\n(no builder running; this is the store as it stands. Start one with `vivarium build {}`,\n\
                 or `vivarium watch --replay` to watch how it got here.)",
                dir.display()
            );
            return 0;
        }
        std::thread::sleep(std::time::Duration::from_millis(speed_ms));
    }
}

/// One frame: clear, globe, header, and the declared-but-unseen honesty block.
fn paint(world: &World, roots: &[vivarium_world::store::RootEntry], w: usize, lon0: f64, color: bool, head: &str) {
    print!("\x1b[H\x1b[2J"); // home + clear; the poller owns the screen
    println!("{head}\n");
    print!("{}", vivarium_world::globe::render(world, roots, w, lon0, color));
    println!("\n{}", vivarium_world::watch::honesty_block(roots).trim_end());
    let _ = std::io::stdout().flush();
}

/// The builder's own `status.json`, condensed to one line — `None` when no live
/// builder holds the lock.
fn builder_status(dir: &Path) -> Option<String> {
    let alive = std::fs::read_to_string(dir.join("builder.lock"))
        .ok()
        .and_then(|t| t.trim().parse::<i32>().ok())
        .is_some_and(pid_alive);
    if !alive {
        return None;
    }
    let text = std::fs::read_to_string(dir.join("status.json")).ok()?;
    let get = |k: &str| -> Option<String> {
        text.split(&format!("\"{k}\""))
            .nth(1)?
            .split(',')
            .next()?
            .trim_start_matches([':', ' ', '"'])
            .trim_end_matches(['"', ' ', '\n', '}'])
            .to_string()
            .into()
    };
    Some(format!(
        "{} {}/{}",
        get("phase").unwrap_or_else(|| "?".into()),
        get("done").unwrap_or_else(|| "?".into()),
        get("total").unwrap_or_else(|| "?".into())
    ))
}

/// `vivarium demand [key=value …]` — show this vivium's prescription, or change it.
///
/// This exists because the manifest gained a read path before it gained a write
/// one: `build --frames 60` used to override for a single run and leave the
/// manifest alone, so the *viewer* — which reads the manifest — kept showing six
/// frames no matter what was typed at the builder, and the only way to change it
/// was to hand-edit a file nobody had been told about. Build flags now persist,
/// so this command is for reading the prescription, or changing it without
/// starting a build.
fn cmd_demand(rest: &[String]) -> i32 {
    let dir = announce_world(rest);
    let mut spec = match WorldSpec::load(&dir) {
        Ok(Some(s)) => s,
        Ok(None) => {
            println!("(no manifest — not yet a vivium; `vivarium new` first)");
            return 0;
        }
        Err(e) => {
            eprintln!("manifest error: {e}");
            return 1;
        }
    };
    // Read assignments from the raw args, NOT from `positionals` — that helper
    // now filters `k=v` out precisely so it is never mistaken for a world dir.
    let assignments: Vec<&str> =
        rest.iter().map(String::as_str).filter(|a| is_assignment(a)).collect();

    for a in &assignments {
        let (k, v) = a.split_once('=').expect("filtered on '='");
        let d = &mut spec.demand;
        let num = |v: &str| v.parse::<u32>();
        let ok = match k {
            "order" => {
                d.order = v.to_string();
                true
            }
            "target_phase" => num(v).map(|n| d.target_phase = n).is_ok(),
            "level" => num(v).map(|n| d.level = n.min(20) as u8).is_ok(),
            "frames" => num(v).map(|n| d.frames = n).is_ok(),
            "erosion_epochs" => num(v).map(|n| d.erosion_epochs = n).is_ok(),
            "water_steps" => num(v).map(|n| d.water_steps = n).is_ok(),
            other => {
                eprintln!(
                    "error: `{other}` is not a demand field.\n  \
                     fields: order target_phase level frames erosion_epochs water_steps\n  \
                     (identity — seed, format — is deliberately NOT settable: changing it would fork a different world)"
                );
                return 2;
            }
        };
        if !ok {
            eprintln!("error: `{v}` is not a number for `{k}`");
            return 2;
        }
    }

    if !assignments.is_empty() {
        if let Err(e) = spec.save(&dir) {
            eprintln!("error writing manifest: {e}");
            return 1;
        }
    }

    let d = &spec.demand;
    println!("vivium \"{}\" — seed {}", spec.name, spec.seed);
    println!("\ndemand — what THIS vivium asks for (never keyed; changing it invalidates no memo):");
    println!("  order           {}", d.order);
    println!("  target_phase    {}", d.target_phase);
    println!("  level           {}   cube-face subdivision the builder sweeps", d.level);
    println!(
        "  frames          {}   deep-time cooling stages ({} after rounding up to a nested count)",
        d.frames,
        mantle_thermal::stage_count(mantle_thermal::refinements_for(d.frames as usize))
    );
    println!("  erosion_epochs  {}   arbitrary — see ASSUMPTIONS.md 'erosion run length'", d.erosion_epochs);
    println!("  water_steps     {}   arbitrary — see ASSUMPTIONS.md 'water fill steps'", d.water_steps);
    if assignments.is_empty() {
        println!("\nset any of them:  vivarium demand frames=60 level=9");
    } else {
        println!("\nsaved. `vivarium build` now uses these; the globe's T-key playback reads `frames`.");
    }
    0
}

fn cmd_attach(rest: &[String]) -> i32 {
    let dir = announce_world(rest);
    tail_log(&dir, true)
}

/// Follow build.log (poll-based tail; the builder is a plain file-appender, so
/// detaching never disturbs it — the store-is-the-bus discipline).
fn tail_log(dir: &Path, follow: bool) -> i32 {
    let path = dir.join("build.log");
    let mut offset = 0u64;
    loop {
        if let Ok(text) = std::fs::read_to_string(&path) {
            let bytes = text.as_bytes();
            if (bytes.len() as u64) > offset {
                print!("{}", &text[offset as usize..]);
                let _ = std::io::stdout().flush();
                offset = bytes.len() as u64;
            }
        }
        if !follow {
            return 0;
        }
        // A live builder keeps the lock; when it releases, drain once and exit.
        let live = std::fs::read_to_string(dir.join("builder.lock"))
            .ok()
            .and_then(|t| t.trim().parse::<i32>().ok())
            .is_some_and(pid_alive);
        if !live {
            println!("(builder finished)");
            return 0;
        }
        std::thread::sleep(std::time::Duration::from_millis(400));
    }
}

/// `vivarium explore` -- dispatch to the 3D explorer.
///
/// **Exec, not a subcommand.** A true in-process subcommand would link Bevy into
/// this binary, which is the core/view wall violated at the most basic level:
/// the world frame's own CLI would depend on a renderer, and `vivarium status`
/// would pay for a GPU stack to print a census. Dispatching by exec is how git
/// reaches `git-lfs`, and it buys the ergonomics without the coupling.
///
/// The lookup order is deliberate: a sibling of THIS executable first, so a
/// `cargo run` from the repo or a `target/release` pair works without anything
/// installed, then PATH.
fn cmd_explore(rest: &[String]) -> i32 {
    let exe = "vivarium-explore";
    let sibling = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.join(exe)));
    let program = match &sibling {
        Some(p) if p.is_file() => p.clone(),
        _ => PathBuf::from(exe),
    };
    match std::process::Command::new(&program).args(rest).status() {
        Ok(st) => st.code().unwrap_or(0),
        Err(e) => {
            eprintln!(
                "error: cannot run `{}`: {e}\n  \
                 The explorer is a separate binary so this CLI never links a renderer\n  \
                 ( #form-core-view-wall ). Install it with:  bin/install vivarium-explore\n  \
                 or run it directly:  cargo run --release -p vivarium-explore",
                program.display()
            );
            127
        }
    }
}
