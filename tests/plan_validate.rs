use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

/// Copy a fixture directory to a temp directory for testing
fn setup_fixture(tmp: &TempDir, fixture_name: &str) {
    let fixture_path = Path::new("tests/fixtures/plan_validate").join(fixture_name);
    let dest_path = tmp.path().join("specs/_plans").join(fixture_name);
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

mod plan_exists {
    use super::*;

    #[test]
    fn validates_plan_with_plan_md() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "valid-plan");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "valid-plan"])
            .assert()
            .success();
    }

    #[test]
    fn fails_when_plan_directory_missing() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs/_plans")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "nonexistent"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("Plan not found"));
    }

    #[test]
    fn fails_when_plan_md_missing() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = tmp.path().join("specs/_plans/incomplete");
        fs::create_dir_all(&plan_dir).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "incomplete"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("plan.md not found"));
    }
}

mod delta_markers {
    use super::*;

    #[test]
    fn passes_with_matched_delta_markers() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "valid-delta");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "valid-delta"])
            .assert()
            .success();
    }

    #[test]
    fn fails_with_unclosed_delta_new_marker() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "unclosed-new");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "unclosed-new"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("DELTA:NEW not closed"));
    }

    #[test]
    fn fails_with_unclosed_delta_changed_marker() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "unclosed-changed");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "unclosed-changed"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("DELTA:CHANGED not closed"));
    }

    #[test]
    fn fails_with_unclosed_delta_removed_marker() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "unclosed-removed");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "unclosed-removed"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("DELTA:REMOVED not closed"));
    }

    #[test]
    fn reports_line_number_for_unclosed_marker() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "unclosed-new");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "unclosed-new"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("line 11"));
    }
}

mod spec_validation {
    use super::*;

    #[test]
    fn passes_with_correctly_formatted_deltas() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "valid-delta");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "valid-delta"])
            .assert()
            .success()
            .stdout(predicate::str::contains("validation passed"));
    }

    #[test]
    fn fails_with_malformed_step_formatting() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "malformed-steps");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "malformed-steps"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("missing a GIVEN step"));
    }

    #[test]
    fn fails_with_steps_missing_emphasized_keywords() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "missing-emphasis");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "missing-emphasis"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("missing a GIVEN step"));
    }

    #[test]
    fn warns_on_lowercase_step_keywords() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "lowercase-steps");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "lowercase-steps"])
            .assert()
            .success()
            .stdout(predicate::str::contains("should be uppercase"));
    }

    #[test]
    fn warns_on_lowercase_rfc_keywords() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "lowercase-rfc");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "lowercase-rfc"])
            .assert()
            .success()
            .stdout(predicate::str::contains("should be uppercase"));
    }

    #[test]
    fn passes_plan_without_delta_specs() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "no-deltas");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "no-deltas"])
            .assert()
            .success();
    }

    #[test]
    fn fails_for_nonexistent_plan() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs/_plans")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "ghost-plan"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("Plan not found"));
    }

    #[test]
    fn includes_standard_spec_validation_errors() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "incomplete-spec");

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "validate", "incomplete-spec"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("ERROR"));
    }
}
