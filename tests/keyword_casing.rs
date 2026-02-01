use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

/// Copy a fixture file to a temp directory for testing
fn setup_fixture(tmp: &TempDir, fixture_name: &str) {
    let fixture_path = Path::new("tests/fixtures/keyword_casing")
        .join(fixture_name)
        .join("spec.md");
    let dest_dir = tmp.path().join("specs/test/feature");
    fs::create_dir_all(&dest_dir).unwrap();
    fs::copy(&fixture_path, dest_dir.join("spec.md")).unwrap();
}

mod step_keywords {
    use super::*;

    #[test]
    fn warns_on_lowercase_given_keyword() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "lowercase-given");

        let output = cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "test/feature"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let stdout = String::from_utf8_lossy(&output);
        assert!(stdout.contains("WARN"));
        assert!(stdout.contains("given"));
        assert!(stdout.contains("should be uppercase"));
    }

    #[test]
    fn warns_on_lowercase_step_keyword() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "lowercase-when");

        let output = cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "test/feature"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let stdout = String::from_utf8_lossy(&output);
        assert!(stdout.contains("WARN"));
        assert!(stdout.contains("when"));
        assert!(stdout.contains("should be uppercase"));
        assert!(stdout.contains("0 errors"));
    }

    #[test]
    fn accepts_uppercase_step_keywords() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "uppercase-steps");

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "test/feature"])
            .assert()
            .success()
            .stdout(predicate::str::contains("0 warnings"));
    }
}

mod rfc_keywords {
    use super::*;

    #[test]
    fn warns_on_lowercase_must_keyword() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "lowercase-must");

        let output = cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "test/feature"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let stdout = String::from_utf8_lossy(&output);
        assert!(stdout.contains("WARN"));
        assert!(stdout.contains("must"));
        assert!(stdout.contains("should be uppercase"));
    }

    #[test]
    fn warns_on_lowercase_rfc_keyword_in_then_step() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "lowercase-shall");

        let output = cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "test/feature"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let stdout = String::from_utf8_lossy(&output);
        assert!(stdout.contains("WARN"));
        assert!(stdout.contains("shall"));
        assert!(stdout.contains("should be uppercase"));
        assert!(stdout.contains("0 errors"));
    }

    #[test]
    fn accepts_uppercase_rfc_keywords() {
        let tmp = TempDir::new().unwrap();
        setup_fixture(&tmp, "uppercase-rfc");

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "test/feature"])
            .assert()
            .success()
            .stdout(predicate::str::contains("0 warnings"));
    }
}
