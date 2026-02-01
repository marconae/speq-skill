pub mod parser;
pub mod report;
pub mod rules;

use std::path::Path;

use crate::feature::{self, FeaturePath};
use report::{ValidationError, ValidationResult};

pub fn run(path: &Path) -> Result<ValidationResult, ValidationError> {
    let content = std::fs::read_to_string(path).map_err(|_| ValidationError::FileNotFound {
        path: path.display().to_string(),
    })?;

    let parse_result = parser::parse(&content)?;
    let mut validation_result = rules::validate(&parse_result.spec);

    // Merge parser warnings into validation result
    for warning in parse_result.warnings {
        validation_result.add_warning(warning);
    }

    Ok(validation_result)
}

pub fn run_all(base: &Path) -> Vec<(FeaturePath, Result<ValidationResult, ValidationError>)> {
    feature::discover_features(base)
        .into_iter()
        .map(|fp| {
            let result = run(&fp.spec_path(base));
            (fp, result)
        })
        .collect()
}

pub fn run_domain(
    base: &Path,
    domain: &str,
) -> Vec<(FeaturePath, Result<ValidationResult, ValidationError>)> {
    feature::discover_features_in_domain(base, domain)
        .into_iter()
        .map(|fp| {
            let result = run(&fp.spec_path(base));
            (fp, result)
        })
        .collect()
}

pub fn run_feature(
    base: &Path,
    feature_path: &FeaturePath,
) -> Result<ValidationResult, ValidationError> {
    run(&feature_path.spec_path(base))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    const VALID_SPEC: &str = r#"# Feature: Test

The system SHALL do something.

## Background

* Context here.

## Scenarios

### Scenario: Basic

* *GIVEN* a setup
* *WHEN* an action occurs
* *THEN* the system SHALL respond
"#;

    const INVALID_SPEC: &str = "# Feature: Broken\n\nNo sections.\n";

    fn setup_test_hierarchy() -> TempDir {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path();

        fs::create_dir_all(base.join("cli/validate")).unwrap();
        fs::write(base.join("cli/validate/spec.md"), VALID_SPEC).unwrap();

        fs::create_dir_all(base.join("validation/doc")).unwrap();
        fs::write(base.join("validation/doc/spec.md"), VALID_SPEC).unwrap();

        fs::create_dir_all(base.join("validation/broken")).unwrap();
        fs::write(base.join("validation/broken/spec.md"), INVALID_SPEC).unwrap();

        tmp
    }

    #[test]
    fn run_returns_error_for_missing_file() {
        let result = run(Path::new("/nonexistent/spec.md"));
        assert!(matches!(result, Err(ValidationError::FileNotFound { .. })));
    }

    #[test]
    fn run_all_validates_all_features() {
        let tmp = setup_test_hierarchy();
        let results = run_all(tmp.path());

        assert_eq!(results.len(), 3);

        let valid_count = results.iter().filter(|(_, r)| r.is_ok()).count();
        assert_eq!(valid_count, 3);
    }

    #[test]
    fn run_domain_validates_only_domain_features() {
        let tmp = setup_test_hierarchy();
        let results = run_domain(tmp.path(), "validation");

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|(fp, _)| fp.domain == "validation"));
    }

    #[test]
    fn run_domain_returns_empty_for_nonexistent_domain() {
        let tmp = setup_test_hierarchy();
        let results = run_domain(tmp.path(), "nonexistent");

        assert!(results.is_empty());
    }

    #[test]
    fn run_feature_validates_single_feature() {
        let tmp = setup_test_hierarchy();
        let fp = FeaturePath::new("cli", "validate");
        let result = run_feature(tmp.path(), &fp);

        assert!(result.is_ok());
        assert!(result.unwrap().is_success());
    }

    #[test]
    fn run_feature_returns_error_for_missing_feature() {
        let tmp = setup_test_hierarchy();
        let fp = FeaturePath::new("cli", "nonexistent");
        let result = run_feature(tmp.path(), &fp);

        assert!(matches!(result, Err(ValidationError::FileNotFound { .. })));
    }

    #[test]
    fn run_all_includes_validation_errors() {
        let tmp = setup_test_hierarchy();
        let results = run_all(tmp.path());

        let broken = results
            .iter()
            .find(|(fp, _)| fp.feature == "broken")
            .unwrap();

        assert!(broken.1.as_ref().unwrap().errors.len() > 0);
    }
}
