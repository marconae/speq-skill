use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use std::fs;
use std::sync::OnceLock;
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

static MODEL_CACHED: OnceLock<()> = OnceLock::new();

/// Provision the embedding-model files into the system model cache once per
/// test process.
///
/// On the first call it downloads the three model files from HuggingFace into
/// `speq_skill::search::get_model_dir()` (the same path the binary reads),
/// mirroring what `install.sh`'s `provision_embedding_model` does during
/// installation. Subsequent calls and runs are a no-op because the files
/// persist on disk. The download is intentionally written to the system cache,
/// not a TempDir, so candle-backed search tests find a model without per-test
/// downloads.
fn ensure_model_cached() {
    MODEL_CACHED.get_or_init(|| {
        let model_dir = speq_skill::search::get_model_dir();
        std::fs::create_dir_all(&model_dir).expect("create model dir");
        let files = [
            (
                "https://huggingface.co/Snowflake/snowflake-arctic-embed-xs/resolve/main/model.safetensors",
                "model.safetensors",
            ),
            (
                "https://huggingface.co/Snowflake/snowflake-arctic-embed-xs/resolve/main/tokenizer.json",
                "tokenizer.json",
            ),
            (
                "https://huggingface.co/Snowflake/snowflake-arctic-embed-xs/resolve/main/config.json",
                "config.json",
            ),
        ];
        for (url, filename) in files {
            let dest = model_dir.join(filename);
            if dest.exists() {
                continue;
            }
            let tmp = format!("{}.tmp", dest.display());
            let status = std::process::Command::new("curl")
                .args(["-fsSL", url, "-o", &tmp])
                .status()
                .expect("invoke curl");
            assert!(status.success(), "Failed to download {filename}");
            std::fs::rename(&tmp, &dest).expect("rename model file into place");
        }
    });
}

/// Resolve the system cache path as a `String` for passing via `SPEQ_CACHE_DIR`.
fn system_cache_dir() -> String {
    speq_skill::search::get_cache_path()
        .to_string_lossy()
        .into_owned()
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

mod plan_list {
    use super::*;

    #[test]
    fn lists_active_plans() {
        cmd()
            .current_dir("tests/fixtures/plan_list")
            .args(["plan", "list"])
            .assert()
            .success()
            .stdout("add-auth\nfix-validation\n");
    }

    #[test]
    fn no_plans_shows_message() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs/_plans")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No active plans"));
    }

    #[test]
    fn missing_plans_dir_shows_message() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No active plans"));
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
    #[serial]
    fn auto_indexes_on_first_search() {
        ensure_model_cached();
        let tmp = setup_test_specs();
        let cache_dir = system_cache_dir();

        // Search WITHOUT running 'speq search index' first
        // Should auto-build index and return results
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "query", "validation"])
            .assert()
            .success()
            .stdout(predicate::str::contains("validation/document-structure"));
    }

    #[test]
    #[serial]
    fn index_builds_successfully() {
        ensure_model_cached();
        let tmp = setup_test_specs();
        let cache_dir = system_cache_dir();

        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "index"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Building search index"))
            .stdout(predicate::str::contains("Indexed"));
    }

    #[test]
    #[serial]
    fn search_finds_similar_scenarios() {
        ensure_model_cached();
        let tmp = setup_test_specs();
        let cache_dir = system_cache_dir();

        // First build the index
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "index"])
            .assert()
            .success();

        // Then search for validation-related content
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "query", "document validation"])
            .assert()
            .success()
            .stdout(predicate::str::contains("validation/document-structure"));
    }

    #[test]
    #[serial]
    fn search_with_limit() {
        ensure_model_cached();
        let tmp = setup_test_specs();
        let cache_dir = system_cache_dir();

        // First build the index
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "index"])
            .assert()
            .success();

        // Search with limit
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "query", "test", "--limit", "1"])
            .assert()
            .success();
    }

    #[test]
    #[serial]
    fn search_no_matches() {
        ensure_model_cached();
        let tmp = setup_test_specs();
        let cache_dir = system_cache_dir();

        // First build the index
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "index"])
            .assert()
            .success();

        // Search for something completely unrelated
        // Note: Semantic search may still find some results, so we just verify it runs
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "query", "xyzzy12345nonexistent"])
            .assert()
            .success();
    }

    #[test]
    #[serial]
    fn search_index_builds_with_candle_backend() {
        ensure_model_cached();
        let tmp = setup_test_specs();
        let cache_dir = system_cache_dir();

        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "index"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Indexed"));
    }

    #[test]
    #[serial]
    fn search_loads_model_from_cache_offline() {
        ensure_model_cached();
        let tmp = setup_test_specs();
        let cache_dir = system_cache_dir();

        // Build index first
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "index"])
            .assert()
            .success();

        // Run query — model already in cache, no network access
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "query", "validation"])
            .assert()
            .success();
    }

    #[test]
    #[serial]
    fn search_reports_actionable_error_when_model_missing() {
        // Point SPEQ_CACHE_DIR at an empty TempDir — no model files present.
        // This test must NOT call ensure_model_cached().
        let empty_cache = TempDir::new().unwrap();
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .env(
                "SPEQ_CACHE_DIR",
                empty_cache.path().to_string_lossy().as_ref(),
            )
            .args(["search", "query", "validation"])
            .assert()
            .failure()
            .stdout(predicate::str::contains("model").and(predicate::str::contains("models")));
    }

    #[test]
    #[serial]
    fn search_model_and_index_cache_layout() {
        ensure_model_cached();
        let tmp = setup_test_specs();
        let cache_dir = system_cache_dir();
        let model_dir = speq_skill::search::get_model_dir();

        // Model files must exist in the model cache
        assert!(model_dir.join("model.safetensors").exists());
        assert!(model_dir.join("tokenizer.json").exists());
        assert!(model_dir.join("config.json").exists());

        // Build index and verify it lands in the indexes/ subdir
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "index"])
            .assert()
            .success();

        assert!(
            speq_skill::search::get_cache_path()
                .join("indexes")
                .exists()
        );
    }

    #[test]
    #[serial]
    fn search_runs_without_onnx_runtime() {
        ensure_model_cached();
        let tmp = setup_test_specs();
        let cache_dir = system_cache_dir();

        // candle is the only inference backend; this asserts that index
        // building succeeds with no ONNX library present.
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "index"])
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

        // Verify plan was archived with date prefix
        // The archive should be at _recorded/YYYY-MM-DD-test-plan
        let recorded_dir = specs.join("_recorded");
        let entries: Vec<_> = fs::read_dir(&recorded_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert_eq!(entries.len(), 1, "Expected exactly one recorded plan");

        let archived_name = entries[0].file_name().to_string_lossy().to_string();
        assert!(
            archived_name.ends_with("-test-plan"),
            "Archive should end with plan name"
        );

        // Verify date prefix format (YYYY-MM-DD-)
        let date_prefix = &archived_name[..11];
        assert!(
            date_prefix.chars().nth(4) == Some('-')
                && date_prefix.chars().nth(7) == Some('-')
                && date_prefix.chars().nth(10) == Some('-'),
            "Archive should have date prefix format YYYY-MM-DD-"
        );

        assert!(!specs.join("_plans/test-plan").exists());
    }

    #[test]
    #[serial]
    fn record_rebuilds_search_index() {
        ensure_model_cached();
        let cache_dir = system_cache_dir();
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs");

        // Create an existing feature to establish an initial index
        let existing_dir = specs.join("existing/feature");
        fs::create_dir_all(&existing_dir).unwrap();
        fs::write(
            existing_dir.join("spec.md"),
            r#"# Feature: Existing Feature

An existing feature.

## Background

* Context.

## Scenarios

### Scenario: Original scenario

* *GIVEN* original setup
* *WHEN* original action
* *THEN* original result SHALL happen
"#,
        )
        .unwrap();

        // Build initial index
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "index"])
            .assert()
            .success();

        // Create plan with new scenario
        let plan_dir = specs.join("_plans/test-plan/new/feature");
        fs::create_dir_all(&plan_dir).unwrap();
        fs::write(
            plan_dir.join("spec.md"),
            r#"# Feature: New Feature

A new feature added via plan.

## Background

* New context.

## Scenarios

<!-- DELTA:NEW -->
### Scenario: Unique searchable scenario

* *GIVEN* unique searchable setup
* *WHEN* unique searchable action
* *THEN* unique searchable result SHALL happen
<!-- /DELTA:NEW -->
"#,
        )
        .unwrap();

        // Create _recorded directory
        fs::create_dir_all(specs.join("_recorded")).unwrap();

        // Record the plan - should rebuild index and show indexed count
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["record", "test-plan"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Indexed"));

        // Verify the new scenario is now searchable
        cmd()
            .current_dir(tmp.path())
            .env("SPEQ_CACHE_DIR", &cache_dir)
            .args(["search", "query", "unique searchable"])
            .assert()
            .success()
            .stdout(predicate::str::contains("new/feature"));
    }
}
