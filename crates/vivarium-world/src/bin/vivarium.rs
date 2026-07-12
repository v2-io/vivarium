//! `vivarium` — the world-lifecycle CLI: builder v0 + instruments.
//!
//! The operational front door of `doc/plan/builder-explorer-decoupling.md`:
//! a **builder** process that advances a vivium in the background while any
//! number of read-only explorers watch through the store (the store is the bus
//! — no IPC). Subcommands:
//!
//! - `vivarium new <dir> [name]` — individuate a world: write its manifest
//!   (fresh seed unless the dir already has one — identity is never re-minted).
//! - `vivarium build <dir> [--level L] [--epochs E]` — builder v0: sweep all six
//!   cube faces at level `L` through the spine nomos (the breadth-first,
//!   whole-world degenerate beacon — "all of the world to the end of Phase 2"),
//!   then erode the same tiles (`--epochs 0` skips erosion). Appends `build.log`,
//!   maintains `status.json`, holds `builder.lock`; a second invocation on a
//!   LIVE build **attaches** (tails the log) instead of failing.
//! - `vivarium status <dir>` — the **fidelity pyramid**: a census of the store's
//!   roots by nomos × level (what exists, at what fidelity — readable while a
//!   build runs).
//! - `vivarium attach <dir>` — follow a running build's log (Ctrl-C detaches;
//!   the builder is unaffected).
//!
//! Builder v0 is deliberately thin: no demand spool yet (explorers file demand
//! in the next increment), no beacon parsing from the manifest (the sweep IS
//! the whole-world beacon). It exists to make the decoupling REAL: run `build`,
//! walk away, run `status`/`attach` from other terminals, run the globe on the
//! same dir — nothing coordinates except the store.
//!
//! Lives as a bin inside `vivarium-world` (not its own crate) to keep the
//! workspace Cargo.toml untouched while a parallel agent owns edits to it;
//! graduating to `crates/vivarium-cli` later is mechanical.

use std::io::Write as _;
use std::path::{Path, PathBuf};

use vivarium_world::audit;
use vivarium_world::nomotheke;
use vivarium_world::query::{Source, World};
use vivarium_world::spec::WorldSpec;
use vivarium_world::sphere::Face;
use vivarium_world::store::Store;

const TILE_NX: usize = 64;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match args.first().map(String::as_str) {
        Some("new") => cmd_new(&args[1..]),
        Some("build") => cmd_build(&args[1..]),
        Some("status") => cmd_status(&args[1..]),
        Some("attach") => cmd_attach(&args[1..]),
        _ => {
            eprintln!("usage: vivarium <new|build|status|attach> [world-dir] [options]");
            eprintln!("  (world-dir optional: defaults to $VIVARIUM_WORLD, else ~/.cache/vivarium/globe-world");
            eprintln!("   — the same world vivarium-globe opens, so status/build/globe agree by default)");
            eprintln!("  new [dir] [name]                    individuate a world (manifest + seed)");
            eprintln!("  build [dir] [--level L] [--epochs E]  builder v0: whole-world sweep at L (default 7)");
            eprintln!("                                       then erosion at E epochs (default 40; 0 = skip)");
            eprintln!("  status [dir]                        fidelity pyramid + flux/requisite audit");
            eprintln!("  attach [dir]                        follow a running build's log");
            2
        }
    };
    std::process::exit(code);
}

/// Resolve which world to act on, matching the globe's convention so
/// `vivarium status` and `vivarium-globe` look at the SAME world by default:
/// an explicit non-flag positional wins, else `$VIVARIUM_WORLD`, else the shared
/// default `${XDG_CACHE_HOME:-~/.cache}/vivarium/globe-world`.
fn world_dir(rest: &[String]) -> PathBuf {
    if let Some(first) = rest.first() {
        if !first.starts_with('-') {
            return PathBuf::from(first);
        }
    }
    if let Ok(p) = std::env::var("VIVARIUM_WORLD") {
        return PathBuf::from(p);
    }
    let cache = std::env::var("XDG_CACHE_HOME").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into())).join(".cache")
    });
    cache.join("vivarium").join("globe-world")
}

/// True when the first token is an explicit world dir (a non-flag positional).
fn dir_is_explicit(rest: &[String]) -> bool {
    rest.first().map(|f| !f.starts_with('-')).unwrap_or(false)
}

fn flag(rest: &[String], name: &str) -> Option<u32> {
    rest.iter().position(|a| a == name).and_then(|i| rest.get(i + 1)).and_then(|v| v.parse().ok())
}

fn cmd_new(rest: &[String]) -> i32 {
    let dir = world_dir(rest);
    // Name is the positional AFTER an explicit dir; otherwise a label default.
    let name = if dir_is_explicit(rest) { rest.get(1).map(String::as_str).unwrap_or("unnamed") } else { "unnamed" };
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

fn cmd_build(rest: &[String]) -> i32 {
    let dir = world_dir(rest);
    let level = flag(rest, "--level").unwrap_or(7).min(20) as u8;
    let epochs = flag(rest, "--epochs").unwrap_or(40);

    let spec = match WorldSpec::load_or_create(&dir, "unnamed") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };

    // Single-builder discipline: hold builder.lock; if a LIVE builder holds it,
    // attach instead of failing (Joseph's preferred UX). A stale lock (dead pid)
    // is reclaimed.
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
        "builder v0 on vivium \"{}\" (seed {}) — spine sweep L{level}, {}x{} tiles/face-row, erosion {epochs} epochs",
        spec.name, spec.seed, TILE_NX, TILE_NX
    ));

    // The whole-world degenerate beacon: every face, tiled at TILE_NX.
    let per_face = ((1u64 << level) as usize).div_ceil(TILE_NX);
    let total = 6 * per_face * per_face;
    let mut done = 0;
    let mut computed = 0;
    for phase in ["spine", "erosion", "water"] {
        if phase != "spine" && epochs == 0 {
            out.line("erosion + water skipped (--epochs 0)");
            break;
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
                        "spine" => world.spine_tile(face, level, oi, oj, TILE_NX).1,
                        "erosion" => world.erosion_tile(face, level, oi, oj, TILE_NX, epochs).1,
                        _ => world.water_tile(face, level, oi, oj, TILE_NX, epochs, 200).1,
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
        computed = 0;
    }
    out.status("idle", done, total);
    out.line("build complete — the store is the save; explorers see everything already.");
    let _ = std::fs::remove_file(&lock_path);
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
    let dir = world_dir(rest);
    let seed = match WorldSpec::load(&dir) {
        Ok(Some(spec)) => {
            println!("vivium \"{}\" — seed {}", spec.name, spec.seed);
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
    let store = match Store::open(&dir) {
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
    // the top — the half-population-pyramid Joseph pictured).
    let mut census: std::collections::BTreeMap<(u8, String), usize> = std::collections::BTreeMap::new();
    let mut unknown = 0;
    for (key, _) in &roots {
        let nomos = key.split('@').next().unwrap_or("").to_string();
        let level = key
            .split('|')
            .find_map(|f| f.strip_prefix("level="))
            .and_then(|v| v.parse::<u8>().ok());
        match (nomos.is_empty(), level) {
            (false, Some(l)) => *census.entry((l, nomos)).or_default() += 1,
            _ => unknown += 1,
        }
    }
    println!("\nfidelity pyramid ({} roots; B = physics tier, declared/derived — the honesty column):", roots.len());
    println!("{:>5}  {:<14} {:>9}  {:>7}  ", "level", "nomos", "B dcl/drv", "tiles");
    let max = census.values().copied().max().unwrap_or(1);
    for ((level, nomos), n) in &census {
        let b = match nomotheke::lookup(nomos) {
            Some(d) => format!("{}/{}", d.physics.letter(), d.derived_physics().letter()),
            None => "?/?".to_string(), // a root the registry doesn't know — itself a finding
        };
        let bar = "█".repeat((n * 40 / max).max(1));
        println!("{level:>5}  {nomos:<14} {b:>9}  {n:>7}  {bar}");
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
    0
}

fn cmd_attach(rest: &[String]) -> i32 {
    let dir = world_dir(rest);
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
