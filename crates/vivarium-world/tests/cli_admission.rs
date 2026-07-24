//! Bin-level (argv → process → exit code / stdout) integration tests for the
//! builder-admission chain — the residuals `#form-builder-admission` names as open:
//! the gate lives in the *binary*, and the lib only exercised `requisite_chain`.
//! These walk the actual `vivarium` process end to end, so the refuse path and the
//! provisional-tag chain are convicted where they actually live.
//!
//! No test framework beyond std: cargo hands us the built binary's path in
//! `CARGO_BIN_EXE_vivarium`, and each test gets its own throwaway world dir so the
//! store, lock, and logs never cross between runs.
//!
//! Why the env seam: the shipped registry is *closed* (climate keeps precipitation,
//! isostasy keeps emerged land), so no shipped phase is Unmet and the binary never
//! refuses as-built. `VIVARIUM_TEST_FORCE_UNMET=<phase>` injects a synthetic unmet
//! quantity for one phase's admission check — a strictly-stricter seam (it can only
//! ADD an unmet need, never remove one), so it makes the refuse/waiver machinery
//! reachable deterministically without a backdoor that could launder an unlawful
//! build. See the bin's `maybe_forced_unmet`.

use std::path::PathBuf;
use std::process::Command;

fn vivarium() -> Command {
    Command::new(env!("CARGO_BIN_EXE_vivarium"))
}

/// A unique, empty world dir under the OS temp root — removed and recreated so a
/// prior run's store/manifest never leaks in. Uniqueness is pid + a process-local
/// atomic counter (no wall-clock — the determinism clippy bans `SystemTime::now`),
/// and the up-front `remove_dir_all` makes a pid recurrence across runs harmless.
fn fresh_world(tag: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let n = SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "vivarium-cli-test-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create world dir");
    dir
}

#[test]
fn status_shows_the_flux_web_and_the_ordinum_maturity_ladder() {
    // Happy-path smoke, and the display-debt close of #form-ordinum-governs-flux-web:
    // an admitted spine-only build (--epochs 0 leaves erosion/water unbuilt; its sole
    // requisite, seeded-asymmetry, is met by noise), then `status` printing BOTH
    // instruments the honesty surface owes — the flux web AND the maturity ladder,
    // walked at the bin level (not just unit-tested in the lib).
    let dir = fresh_world("status");
    let build = vivarium()
        .args(["build", dir.to_str().unwrap(), "--level", "6", "--epochs", "0"])
        .output()
        .expect("run build");
    assert!(
        build.status.success(),
        "build --epochs 0 should be admitted; stderr:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );

    let status = vivarium().args(["status", dir.to_str().unwrap()]).output().expect("run status");
    assert!(status.status.success(), "status exits 0");
    let out = String::from_utf8_lossy(&status.stdout);
    assert!(out.contains("promise maturity"), "status shows the ordinum maturity report:\n{out}");
    assert!(out.contains("emerged-land"), "the ladder's tracked promise is visible");
    assert!(out.contains("Kept is not shown"), "the Claimed≠Kept honesty note rides along");
    assert!(out.contains("flux web"), "the flux web is still reported beside it");
    assert!(!out.contains("tagged provisional"), "an admitted build tags nothing provisional:\n{out}");
}

#[test]
fn build_refuses_an_unmet_phase_with_exit_2_and_no_provisional_write() {
    // Residual (2): the refuse path, convicted through the binary. Force `erosion`
    // Unmet, run default admission (no --allow-unmet), and pin the contract: exit 2,
    // an announced refusal, and NOT a single provisional root written.
    let dir = fresh_world("refuse");
    let out = vivarium()
        .args(["build", dir.to_str().unwrap(), "--level", "6", "--epochs", "2"])
        .env("VIVARIUM_TEST_FORCE_UNMET", "erosion")
        .output()
        .expect("run strict build");

    assert_eq!(out.status.code(), Some(2), "an unmet phase under default admission exits 2");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stderr.contains("refused") || stdout.contains("REFUSED"),
        "the refusal is announced; stderr:\n{stderr}\nstdout:\n{stdout}"
    );
    assert!(
        stdout.contains("test-forced-unmet"),
        "the refusal names the unmet flux quantity:\n{stdout}"
    );

    // And status confirms the refused build left no provisional artifact behind.
    let status = vivarium().args(["status", dir.to_str().unwrap()]).output().expect("status");
    let s = String::from_utf8_lossy(&status.stdout);
    assert!(!s.contains("tagged provisional"), "a refused build writes no provisional root:\n{s}");
}

#[test]
fn waiver_writes_provisional_roots_that_status_surfaces() {
    // Residual (1): the provisional-tag chain, walked argv → build → status through
    // the binary (the lib closed it at the `requisite_chain` level only). Force
    // `erosion` Unmet, waive with --allow-unmet, and assert the waiver is loud in the
    // log AND that `status` then reports provisional roots — waived bytes are visibly
    // not-lawful, end to end.
    let dir = fresh_world("waive");
    let waived = vivarium()
        .args(["build", dir.to_str().unwrap(), "--level", "6", "--epochs", "2", "--allow-unmet"])
        .env("VIVARIUM_TEST_FORCE_UNMET", "erosion")
        .output()
        .expect("run waived build");

    assert!(
        waived.status.success(),
        "a waived build is admitted (exit 0); stderr:\n{}",
        String::from_utf8_lossy(&waived.stderr)
    );
    let wout = String::from_utf8_lossy(&waived.stdout);
    assert!(wout.contains("WAIVED"), "the waiver is loud in the build log:\n{wout}");

    let status = vivarium().args(["status", dir.to_str().unwrap()]).output().expect("status");
    let s = String::from_utf8_lossy(&status.stdout);
    assert!(
        s.contains("provisional"),
        "waived roots surface as provisional in status — the argv→status chain, not just the lib:\n{s}"
    );
}
