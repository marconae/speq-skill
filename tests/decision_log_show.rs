use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

fn setup_fixture(tmp: &TempDir, fixture_name: &str) {
    let fixture_path = Path::new("tests/fixtures/decision_log_show")
        .join(fixture_name)
        .join("specs");
    let dest_path = tmp.path().join("specs");
    copy_dir_recursive(&fixture_path, &dest_path).unwrap();
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

#[test]
fn show_orders_by_prefix() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "ordered-fragments");

    let assert = cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("# Architecture Decision Records"));

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let scanner = stdout.find("## ADR: Use line scanner").unwrap();
    let faster = stdout.find("## ADR: Faster scanner").unwrap();
    assert!(scanner < faster, "stdout: {stdout}");
}

#[test]
fn show_breaks_fragment_ties_by_filename() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "tied-fragments");

    let assert = cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "show"])
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let plan_a = stdout.find("## ADR: Use line scanner").unwrap();
    let plan_b = stdout.find("## ADR: Faster scanner").unwrap();
    assert!(plan_a < plan_b, "stdout: {stdout}");
}

#[test]
fn show_preserves_intra_fragment_order() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "intra-fragment-order");

    let assert = cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "show"])
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let zeta = stdout.find("## ADR: Zeta first").unwrap();
    let alpha = stdout.find("## ADR: Alpha second").unwrap();
    assert!(zeta < alpha, "stdout: {stdout}");
}

#[test]
fn show_absent_dir_header_only() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "absent-dir");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "show"])
        .assert()
        .success()
        .stdout(predicate::eq("# Architecture Decision Records\n"));
}

#[test]
fn show_writes_no_file() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "single-fragment");

    let fragment_path = tmp.path().join("specs/_decision/001-plan-a.md");
    let before = fs::read_to_string(&fragment_path).unwrap();

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "show"])
        .assert()
        .success();

    assert!(!tmp.path().join("specs/decision-log.md").exists());
    let after = fs::read_to_string(&fragment_path).unwrap();
    assert_eq!(before, after);
}

#[test]
fn show_renders_required_fields() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "single-fragment");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "show"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("## ADR: Use line scanner")
                .and(predicate::str::contains("**ID:** use-line-scanner"))
                .and(predicate::str::contains("### Decision")),
        );
}
