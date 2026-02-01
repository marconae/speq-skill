use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

fn setup_test_specs() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let specs = tmp.path().join("specs");

    // Create valid hierarchy
    let cli_validate = specs.join("cli/validate");
    fs::create_dir_all(&cli_validate).unwrap();
    fs::write(
        cli_validate.join("spec.md"),
        r#"# Feature: CLI Validate

The system SHALL provide validation.

## Background

* Test context.

## Scenarios

### Scenario: Basic test

* *GIVEN* a setup
* *WHEN* an action occurs
* *THEN* the system SHALL respond
"#,
    )
    .unwrap();

    let validation_doc = specs.join("validation/document-structure");
    fs::create_dir_all(&validation_doc).unwrap();
    fs::write(
        validation_doc.join("spec.md"),
        r#"# Feature: Document Validation

The system SHALL validate documents.

## Background

* Documents have structure.

## Scenarios

### Scenario: Valid docs pass

* *GIVEN* a valid document
* *WHEN* validation runs
* *THEN* the system SHALL pass
"#,
    )
    .unwrap();

    tmp
}

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

mod feature_list {
    use super::*;

    #[test]
    fn lists_all_features() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/"))
            .stdout(predicate::str::contains("validate"))
            .stdout(predicate::str::contains("validation/"))
            .stdout(predicate::str::contains("document-structure"));
    }

    #[test]
    fn lists_domain_features() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "list", "cli"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/"))
            .stdout(predicate::str::contains("validate"));
    }

    #[test]
    fn empty_domain_shows_message() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "list", "nonexistent"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No features found"));
    }
}

mod feature_validate {
    use super::*;

    #[test]
    fn validates_all_specs() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/validate"))
            .stdout(predicate::str::contains("validation/document-structure"));
    }

    #[test]
    fn validates_single_domain() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "cli"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/validate"));
    }

    #[test]
    fn validates_single_feature() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "cli/validate"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/validate"))
            .stdout(predicate::str::contains("0 errors"));
    }

    #[test]
    fn reports_invalid_spec() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs/broken/test");
        fs::create_dir_all(&specs).unwrap();
        fs::write(specs.join("spec.md"), "# Feature: Broken\n\nNo sections.\n").unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "broken/test"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("✗"))
            .stdout(predicate::str::contains("error"));
    }

    #[test]
    fn no_features_found() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No features found"));
    }
}

mod domain_list {
    use super::*;

    #[test]
    fn lists_all_domains() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["domain", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/"))
            .stdout(predicate::str::contains("validation/"));
    }

    #[test]
    fn empty_specs_shows_message() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["domain", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No domains found"));
    }
}

mod feature_get {
    use super::*;

    #[test]
    fn gets_full_feature_spec() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "get", "cli/validate"])
            .assert()
            .success()
            .stdout(predicate::str::contains("CLI Validate"))
            .stdout(predicate::str::contains("Basic test"));
    }

    #[test]
    fn gets_single_scenario() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "get", "cli/validate/Basic test"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/validate/Basic test"))
            .stdout(predicate::str::contains("Given"));
    }

    #[test]
    fn feature_not_found() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "get", "cli/nonexistent"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("Feature not found"));
    }

    #[test]
    fn scenario_not_found() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "get", "cli/validate/Missing"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("Scenario 'Missing' not found"));
    }
}

mod search {
    use super::*;

    #[test]
    fn no_index_shows_error() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["search", "query", "validation"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("No search index found"));
    }

    #[test]
    #[serial]
    fn index_builds_successfully() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["search", "index"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Building search index"))
            .stdout(predicate::str::contains("Indexed"));
    }

    #[test]
    #[serial]
    fn search_finds_similar_scenarios() {
        let tmp = setup_test_specs();

        // First build the index
        cmd()
            .current_dir(tmp.path())
            .args(["search", "index"])
            .assert()
            .success();

        // Then search for validation-related content
        cmd()
            .current_dir(tmp.path())
            .args(["search", "query", "document validation"])
            .assert()
            .success()
            .stdout(predicate::str::contains("validation/document-structure"));
    }

    #[test]
    #[serial]
    fn search_with_limit() {
        let tmp = setup_test_specs();

        // First build the index
        cmd()
            .current_dir(tmp.path())
            .args(["search", "index"])
            .assert()
            .success();

        // Search with limit
        cmd()
            .current_dir(tmp.path())
            .args(["search", "query", "test", "--limit", "1"])
            .assert()
            .success();
    }

    #[test]
    #[serial]
    fn search_no_matches() {
        let tmp = setup_test_specs();

        // First build the index
        cmd()
            .current_dir(tmp.path())
            .args(["search", "index"])
            .assert()
            .success();

        // Search for something completely unrelated
        // Note: Semantic search may still find some results, so we just verify it runs
        cmd()
            .current_dir(tmp.path())
            .args(["search", "query", "xyzzy12345nonexistent"])
            .assert()
            .success();
    }
}

mod record {
    use super::*;

    #[test]
    fn record_nonexistent_plan_fails() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs/_plans")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["record", "nonexistent"])
            .assert()
            .code(1)
            .stderr(predicate::str::contains("Plan not found"));
    }

    #[test]
    fn record_plan_creates_spec() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs");

        // Create plan structure
        let plan_dir = specs.join("_plans/test-plan/test/feature");
        fs::create_dir_all(&plan_dir).unwrap();
        fs::write(
            plan_dir.join("spec.md"),
            r#"# Feature: Test Feature

A test feature.

## Background

* Test context.

## Scenarios

<!-- DELTA:NEW -->
### Scenario: New test

* *GIVEN* setup
* *WHEN* action
* *THEN* result SHALL happen
<!-- /DELTA:NEW -->
"#,
        )
        .unwrap();

        // Create _recorded directory
        fs::create_dir_all(specs.join("_recorded")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["record", "test-plan"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Recorded plan"))
            .stdout(predicate::str::contains("test/feature"));

        // Verify spec was created
        let recorded_spec = specs.join("test/feature/spec.md");
        assert!(recorded_spec.exists());

        let content = fs::read_to_string(&recorded_spec).unwrap();
        assert!(!content.contains("DELTA"));
        assert!(content.contains("### Scenario: New test"));

        // Verify plan was archived
        assert!(specs.join("_recorded/test-plan").exists());
        assert!(!specs.join("_plans/test-plan").exists());
    }
}
