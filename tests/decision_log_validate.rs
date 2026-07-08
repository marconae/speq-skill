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
fn valid_decisions_dir_passes() {
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
fn absent_decisions_dir_passes() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "absent-dir");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("passed"));
}

#[test]
fn duplicate_slug_fails() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "duplicate-slug");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(
            predicate::str::contains("Duplicate").and(predicate::str::contains("use-line-scanner")),
        );
}

#[test]
fn unresolved_supersedes_fails() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "unresolved-supersedes");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(
            predicate::str::contains("dangling-supersede")
                .and(predicate::str::contains("ghost-slug")),
        );
}

#[test]
fn unresolved_status_slug_fails() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "unresolved-status-slug");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(
            predicate::str::contains("dangling-status").and(predicate::str::contains("ghost-slug")),
        );
}

#[test]
fn missing_id_fails() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "missing-id");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(
            predicate::str::contains("No identity").and(predicate::str::contains("001-plan-a.md")),
        );
}

#[test]
fn missing_h1_fails() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "missing-h1");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("001-plan-a.md"));
}

#[test]
fn missing_field_fails() {
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
fn invalid_status_fails() {
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
fn prose_mentioning_field_names_passes() {
    let tmp = TempDir::new().unwrap();
    setup_fixture(&tmp, "prose-mentions-field");

    cmd()
        .current_dir(tmp.path())
        .args(["decision-log", "validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("passed"));
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
