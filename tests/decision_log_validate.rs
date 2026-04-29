use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

fn setup_fixture(tmp: &TempDir, fixture_name: &str) {
    let fixture_path = Path::new("tests/fixtures/decision_log_validate")
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
fn valid_permanent_log_passes() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "valid");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("passed"));
}

#[test]
fn missing_permanent_log_fails() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "missing");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("not found"));
}

#[test]
fn non_sequential_adr_numbers_fail() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "non-sequential");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("ADR-003"));
}

#[test]
fn adr_missing_required_field_fails() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "missing-field");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Status"));
}

#[test]
fn invalid_status_value_fails() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "invalid-status");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Pending"));
}

#[test]
fn adr_must_start_at_001() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "non-start-001");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("001"));
}

#[test]
fn optional_sections_absent_passes() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "optional-absent");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .success();
}
