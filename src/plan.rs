use std::path::Path;
use thiserror::Error;

use crate::record::find_delta_specs;
use crate::validate;
use crate::validate::report::{ValidationError, ValidationWarning};

#[derive(Debug, Error)]
pub enum PlanValidationError {
    #[error("Plan not found: {name}")]
    PlanNotFound { name: String },

    #[error("plan.md not found in plan directory")]
    PlanMdNotFound,

    #[error("Failed to read file: {path}")]
    FileReadError { path: String },
}

#[derive(Debug)]
pub struct DeltaMarkerError {
    pub file_path: String,
    pub marker_type: String,
    pub line_number: usize,
}

impl std::fmt::Display for DeltaMarkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: DELTA:{} not closed",
            self.file_path, self.line_number, self.marker_type
        )
    }
}

#[derive(Debug)]
pub struct SpecValidationResult {
    pub spec_path: String,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Default)]
pub struct PlanValidationResult {
    pub errors: Vec<String>,
    pub delta_marker_errors: Vec<DeltaMarkerError>,
    pub spec_paths: Vec<String>,
    pub spec_validation_errors: Vec<SpecValidationResult>,
    pub spec_validation_warnings: Vec<SpecValidationResult>,
}

impl PlanValidationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
            && self.delta_marker_errors.is_empty()
            && self.spec_validation_errors.is_empty()
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_delta_marker_error(&mut self, error: DeltaMarkerError) {
        self.delta_marker_errors.push(error);
    }

    pub fn distribute_spec_validation_result(
        &mut self,
        spec_path: String,
        result: validate::report::ValidationResult,
    ) {
        if !result.errors.is_empty() {
            self.spec_validation_errors.push(SpecValidationResult {
                spec_path: spec_path.clone(),
                errors: result.errors,
                warnings: vec![],
            });
        }
        if !result.warnings.is_empty() {
            self.spec_validation_warnings.push(SpecValidationResult {
                spec_path,
                errors: vec![],
                warnings: result.warnings,
            });
        }
    }
}

pub fn list_plans(base: &Path) -> Vec<String> {
    let plans_dir = base.join("_plans");
    let Ok(entries) = std::fs::read_dir(&plans_dir) else {
        return Vec::new();
    };

    let mut plans: Vec<String> = entries
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|name| !name.starts_with('.'))
        .collect();

    plans.sort();
    plans
}

pub fn validate_plan(
    base: &Path,
    plan_name: &str,
) -> Result<PlanValidationResult, PlanValidationError> {
    let plan_dir = base.join("_plans").join(plan_name);

    if !plan_dir.exists() {
        return Err(PlanValidationError::PlanNotFound {
            name: plan_name.to_string(),
        });
    }

    let plan_md = plan_dir.join("plan.md");
    if !plan_md.exists() {
        return Err(PlanValidationError::PlanMdNotFound);
    }

    let mut result = PlanValidationResult::new();

    // Find all delta spec files in the plan directory
    let delta_specs =
        find_delta_specs(&plan_dir).map_err(|_| PlanValidationError::FileReadError {
            path: plan_dir.display().to_string(),
        })?;

    for spec_path in &delta_specs {
        let relative_path = spec_path
            .strip_prefix(&plan_dir)
            .unwrap_or(spec_path)
            .display()
            .to_string();

        result.spec_paths.push(relative_path.clone());

        // Validate delta markers
        let content =
            std::fs::read_to_string(spec_path).map_err(|_| PlanValidationError::FileReadError {
                path: spec_path.display().to_string(),
            })?;

        validate_delta_markers(&content, &relative_path, &mut result);

        // Apply standard spec validation
        if let Ok(validation_result) = validate::run(spec_path) {
            result.distribute_spec_validation_result(relative_path, validation_result);
        }
    }

    Ok(result)
}

fn validate_delta_markers(content: &str, file_path: &str, result: &mut PlanValidationResult) {
    let marker_types = ["NEW", "CHANGED", "REMOVED"];

    for marker_type in marker_types {
        let open_marker = format!("<!-- DELTA:{} -->", marker_type);
        let close_marker = format!("<!-- /DELTA:{} -->", marker_type);

        let mut open_positions: Vec<usize> = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Only match markers that are standalone (start of trimmed line)
            // This avoids matching markers inside backticks or other inline content
            if trimmed.starts_with(&open_marker) {
                open_positions.push(line_num + 1);
            }
            if trimmed.starts_with(&close_marker) && open_positions.pop().is_none() {
                result.add_error(format!(
                    "{}:{}: Found closing DELTA:{} without matching open marker",
                    file_path,
                    line_num + 1,
                    marker_type
                ));
            }
        }

        // Report any unclosed markers
        for line_num in open_positions {
            result.add_delta_marker_error(DeltaMarkerError {
                file_path: file_path.to_string(),
                marker_type: marker_type.to_string(),
                line_number: line_num,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_plan(tmp: &TempDir, plan_name: &str) -> std::path::PathBuf {
        let plan_dir = tmp.path().join("specs/_plans").join(plan_name);
        fs::create_dir_all(&plan_dir).unwrap();
        fs::write(plan_dir.join("plan.md"), "# Plan\n").unwrap();
        plan_dir
    }

    #[test]
    fn validates_existing_plan_with_plan_md() {
        let tmp = TempDir::new().unwrap();
        create_plan(&tmp, "test-plan");

        let result = validate_plan(&tmp.path().join("specs"), "test-plan");
        assert!(result.is_ok());
        assert!(result.unwrap().is_success());
    }

    #[test]
    fn error_when_plan_directory_missing() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs/_plans")).unwrap();

        let result = validate_plan(&tmp.path().join("specs"), "nonexistent");
        assert!(matches!(
            result,
            Err(PlanValidationError::PlanNotFound { .. })
        ));
    }

    #[test]
    fn error_when_plan_md_missing() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = tmp.path().join("specs/_plans/incomplete");
        fs::create_dir_all(&plan_dir).unwrap();
        // No plan.md created

        let result = validate_plan(&tmp.path().join("specs"), "incomplete");
        assert!(matches!(result, Err(PlanValidationError::PlanMdNotFound)));
    }

    #[test]
    fn validates_matched_delta_markers() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = create_plan(&tmp, "good-markers");
        let spec_dir = plan_dir.join("test/feature");
        fs::create_dir_all(&spec_dir).unwrap();
        fs::write(
            spec_dir.join("spec.md"),
            r#"# Feature: Test

Description.

## Background

* Context.

## Scenarios

<!-- DELTA:NEW -->
### Scenario: New one

* *GIVEN* setup
* *WHEN* action
* *THEN* result SHALL happen
<!-- /DELTA:NEW -->
"#,
        )
        .unwrap();

        let result = validate_plan(&tmp.path().join("specs"), "good-markers").unwrap();
        assert!(result.is_success());
    }

    #[test]
    fn error_on_unclosed_delta_new() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = create_plan(&tmp, "unclosed-new");
        let spec_dir = plan_dir.join("test/feature");
        fs::create_dir_all(&spec_dir).unwrap();
        fs::write(
            spec_dir.join("spec.md"),
            r#"# Feature: Test

## Scenarios

<!-- DELTA:NEW -->
### Scenario: New one

* *GIVEN* setup
"#,
        )
        .unwrap();

        let result = validate_plan(&tmp.path().join("specs"), "unclosed-new").unwrap();
        assert!(!result.is_success());
        assert_eq!(result.delta_marker_errors.len(), 1);
        assert_eq!(result.delta_marker_errors[0].marker_type, "NEW");
        assert_eq!(result.delta_marker_errors[0].line_number, 5);
    }

    #[test]
    fn error_on_unclosed_delta_changed() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = create_plan(&tmp, "unclosed-changed");
        let spec_dir = plan_dir.join("test/feature");
        fs::create_dir_all(&spec_dir).unwrap();
        fs::write(
            spec_dir.join("spec.md"),
            r#"# Feature: Test

## Scenarios

<!-- DELTA:CHANGED -->
### Scenario: Changed one
"#,
        )
        .unwrap();

        let result = validate_plan(&tmp.path().join("specs"), "unclosed-changed").unwrap();
        assert!(!result.is_success());
        assert_eq!(result.delta_marker_errors.len(), 1);
        assert_eq!(result.delta_marker_errors[0].marker_type, "CHANGED");
    }

    #[test]
    fn error_on_unclosed_delta_removed() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = create_plan(&tmp, "unclosed-removed");
        let spec_dir = plan_dir.join("test/feature");
        fs::create_dir_all(&spec_dir).unwrap();
        fs::write(
            spec_dir.join("spec.md"),
            r#"# Feature: Test

## Scenarios

<!-- DELTA:REMOVED -->
### Scenario: Removed one
"#,
        )
        .unwrap();

        let result = validate_plan(&tmp.path().join("specs"), "unclosed-removed").unwrap();
        assert!(!result.is_success());
        assert_eq!(result.delta_marker_errors.len(), 1);
        assert_eq!(result.delta_marker_errors[0].marker_type, "REMOVED");
    }

    #[test]
    fn reports_line_number_for_unclosed_marker() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = create_plan(&tmp, "line-number-test");
        let spec_dir = plan_dir.join("test/feature");
        fs::create_dir_all(&spec_dir).unwrap();
        fs::write(
            spec_dir.join("spec.md"),
            "line 1\nline 2\nline 3\n<!-- DELTA:NEW -->\nline 5\n",
        )
        .unwrap();

        let result = validate_plan(&tmp.path().join("specs"), "line-number-test").unwrap();
        assert_eq!(result.delta_marker_errors[0].line_number, 4);
    }

    #[test]
    fn includes_spec_validation_errors() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = create_plan(&tmp, "invalid-spec");
        let spec_dir = plan_dir.join("test/feature");
        fs::create_dir_all(&spec_dir).unwrap();
        // Missing Background and Scenarios sections
        fs::write(
            spec_dir.join("spec.md"),
            r#"# Feature: Test

<!-- DELTA:NEW -->
No proper structure here.
<!-- /DELTA:NEW -->
"#,
        )
        .unwrap();

        let result = validate_plan(&tmp.path().join("specs"), "invalid-spec").unwrap();
        assert!(!result.is_success());
        assert!(!result.spec_validation_errors.is_empty());
    }

    #[test]
    fn includes_spec_validation_warnings() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = create_plan(&tmp, "warning-spec");
        let spec_dir = plan_dir.join("test/feature");
        fs::create_dir_all(&spec_dir).unwrap();
        // Valid structure but lowercase keywords
        fs::write(
            spec_dir.join("spec.md"),
            r#"# Feature: Test

Description.

## Background

* Context.

## Scenarios

<!-- DELTA:NEW -->
### Scenario: New one

* *given* setup
* *WHEN* action
* *THEN* result shall happen
<!-- /DELTA:NEW -->
"#,
        )
        .unwrap();

        let result = validate_plan(&tmp.path().join("specs"), "warning-spec").unwrap();
        // Should still pass (warnings don't fail validation)
        assert!(result.is_success());
        assert!(!result.spec_validation_warnings.is_empty());
    }
}
