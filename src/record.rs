use std::fs;
use std::path::Path;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecordError {
    #[error("Plan not found: {0}")]
    PlanNotFound(String),

    #[error("Failed to read file: {path}")]
    FileReadError { path: String },

    #[error("Failed to write file: {path}")]
    FileWriteError { path: String },

    #[error("Failed to create directory: {path}")]
    DirCreateError { path: String },

    #[error("Failed to move directory: {from} -> {to}")]
    DirMoveError { from: String, to: String },

    #[error("Malformed delta marker at line {line}: {content}")]
    MalformedDelta { line: usize, content: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeltaKind {
    New,
    Changed,
    Removed,
}

#[derive(Debug, Clone)]
pub struct DeltaBlock {
    pub kind: DeltaKind,
    pub content: String,
    pub scenario_title: Option<String>,
}

pub fn record_plan(specs_base: &Path, plan_name: &str) -> Result<Vec<String>, RecordError> {
    let plan_dir = specs_base.join("_plans").join(plan_name);
    let recorded_dir = specs_base.join("_recorded").join(plan_name);

    if !plan_dir.exists() {
        return Err(RecordError::PlanNotFound(plan_name.to_string()));
    }

    let mut recorded_features = Vec::new();

    // Find all spec.md files in the plan
    let delta_specs = find_delta_specs(&plan_dir)?;

    for delta_path in delta_specs {
        let relative = delta_path
            .strip_prefix(&plan_dir)
            .unwrap()
            .parent()
            .unwrap();

        // Determine target path (domain/feature structure)
        let target_dir = specs_base.join(relative);
        let target_spec = target_dir.join("spec.md");

        // Read delta content
        let delta_content =
            fs::read_to_string(&delta_path).map_err(|_| RecordError::FileReadError {
                path: delta_path.display().to_string(),
            })?;

        let merged = if target_spec.exists() {
            // Merge with existing spec
            let existing =
                fs::read_to_string(&target_spec).map_err(|_| RecordError::FileReadError {
                    path: target_spec.display().to_string(),
                })?;
            merge_delta(&existing, &delta_content)?
        } else {
            // New feature - just strip markers
            strip_delta_markers(&delta_content)
        };

        // Create target directory if needed
        fs::create_dir_all(&target_dir).map_err(|_| RecordError::DirCreateError {
            path: target_dir.display().to_string(),
        })?;

        // Write merged spec
        fs::write(&target_spec, merged).map_err(|_| RecordError::FileWriteError {
            path: target_spec.display().to_string(),
        })?;

        recorded_features.push(relative.display().to_string());
    }

    // Archive the plan
    fs::create_dir_all(recorded_dir.parent().unwrap()).map_err(|_| {
        RecordError::DirCreateError {
            path: recorded_dir.parent().unwrap().display().to_string(),
        }
    })?;

    fs::rename(&plan_dir, &recorded_dir).map_err(|_| RecordError::DirMoveError {
        from: plan_dir.display().to_string(),
        to: recorded_dir.display().to_string(),
    })?;

    Ok(recorded_features)
}

pub fn find_delta_specs(plan_dir: &Path) -> Result<Vec<std::path::PathBuf>, RecordError> {
    let mut specs = Vec::new();
    find_delta_specs_recursive(plan_dir, &mut specs)?;
    Ok(specs)
}

fn find_delta_specs_recursive(
    dir: &Path,
    specs: &mut Vec<std::path::PathBuf>,
) -> Result<(), RecordError> {
    let entries = fs::read_dir(dir).map_err(|_| RecordError::FileReadError {
        path: dir.display().to_string(),
    })?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            find_delta_specs_recursive(&path, specs)?;
        } else if path.file_name().is_some_and(|n| n == "spec.md") {
            specs.push(path);
        }
    }

    Ok(())
}

pub fn parse_deltas(content: &str) -> Result<Vec<DeltaBlock>, RecordError> {
    let mut deltas = Vec::new();
    let mut current_kind: Option<DeltaKind> = None;
    let mut current_content = String::new();
    let mut in_delta = false;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if let Some(kind) = parse_delta_open(trimmed) {
            if in_delta {
                return Err(RecordError::MalformedDelta {
                    line: line_num + 1,
                    content: line.to_string(),
                });
            }
            in_delta = true;
            current_kind = Some(kind);
            current_content.clear();
        } else if parse_delta_close(trimmed).is_some() {
            if !in_delta {
                return Err(RecordError::MalformedDelta {
                    line: line_num + 1,
                    content: line.to_string(),
                });
            }

            let content_trimmed = current_content.trim().to_string();
            let scenario_title = extract_scenario_title(&content_trimmed);

            deltas.push(DeltaBlock {
                kind: current_kind.take().unwrap(),
                content: content_trimmed,
                scenario_title,
            });

            in_delta = false;
            current_content.clear();
        } else if in_delta {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }

    Ok(deltas)
}

fn parse_delta_open(line: &str) -> Option<DeltaKind> {
    if line == "<!-- DELTA:NEW -->" {
        Some(DeltaKind::New)
    } else if line == "<!-- DELTA:CHANGED -->" {
        Some(DeltaKind::Changed)
    } else if line == "<!-- DELTA:REMOVED -->" {
        Some(DeltaKind::Removed)
    } else {
        None
    }
}

fn parse_delta_close(line: &str) -> Option<DeltaKind> {
    if line == "<!-- /DELTA:NEW -->" {
        Some(DeltaKind::New)
    } else if line == "<!-- /DELTA:CHANGED -->" {
        Some(DeltaKind::Changed)
    } else if line == "<!-- /DELTA:REMOVED -->" {
        Some(DeltaKind::Removed)
    } else {
        None
    }
}

fn extract_scenario_title(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("### Scenario:") {
            return Some(
                trimmed
                    .trim_start_matches("### Scenario:")
                    .trim()
                    .to_string(),
            );
        }
    }
    None
}

pub fn merge_delta(existing: &str, delta: &str) -> Result<String, RecordError> {
    let deltas = parse_deltas(delta)?;

    if deltas.is_empty() {
        return Ok(existing.to_string());
    }

    let mut result = existing.to_string();

    for delta_block in deltas {
        match delta_block.kind {
            DeltaKind::New => {
                // Append new scenario before the final empty lines
                let trimmed = result.trim_end();
                result = format!("{}\n\n{}\n", trimmed, delta_block.content);
            }
            DeltaKind::Changed => {
                if let Some(title) = &delta_block.scenario_title {
                    result = replace_scenario(&result, title, &delta_block.content);
                }
            }
            DeltaKind::Removed => {
                if let Some(title) = &delta_block.scenario_title {
                    result = remove_scenario(&result, title);
                }
            }
        }
    }

    Ok(result)
}

fn replace_scenario(content: &str, title: &str, replacement: &str) -> String {
    let pattern = format!("### Scenario: {}", title);
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].trim().starts_with(&pattern) {
            // Skip old scenario until next heading or end
            while i < lines.len() {
                i += 1;
                if i >= lines.len() {
                    break;
                }
                let next_trimmed = lines[i].trim();
                if next_trimmed.starts_with("### ") || next_trimmed.starts_with("## ") {
                    break;
                }
            }
            // Insert replacement
            result.push(replacement);
            // Skip empty lines that might be between scenarios
            while i < lines.len() && lines[i].trim().is_empty() {
                i += 1;
            }
        } else {
            result.push(lines[i]);
            i += 1;
        }
    }

    result.join("\n")
}

fn remove_scenario(content: &str, title: &str) -> String {
    let pattern = format!("### Scenario: {}", title);
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;
    let mut last_was_empty = false;

    while i < lines.len() {
        if lines[i].trim().starts_with(&pattern) {
            // Skip scenario until next heading
            while i < lines.len() {
                i += 1;
                if i >= lines.len() {
                    break;
                }
                let next_trimmed = lines[i].trim();
                if next_trimmed.starts_with("### ") || next_trimmed.starts_with("## ") {
                    break;
                }
            }
            // Skip trailing empty lines
            while i < lines.len() && lines[i].trim().is_empty() {
                i += 1;
            }
        } else {
            // Collapse multiple empty lines
            let is_empty = lines[i].trim().is_empty();
            if is_empty && last_was_empty {
                i += 1;
                continue;
            }
            last_was_empty = is_empty;
            result.push(lines[i]);
            i += 1;
        }
    }

    result.join("\n")
}

pub fn strip_delta_markers(content: &str) -> String {
    let mut result = Vec::new();
    let mut in_removed = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Track REMOVED blocks to skip their content
        if trimmed == "<!-- DELTA:REMOVED -->" {
            in_removed = true;
            continue;
        }
        if trimmed == "<!-- /DELTA:REMOVED -->" {
            in_removed = false;
            continue;
        }

        // Skip all delta markers
        if parse_delta_open(trimmed).is_some() || parse_delta_close(trimmed).is_some() {
            continue;
        }

        // Skip content inside REMOVED blocks
        if in_removed {
            continue;
        }

        result.push(line);
    }

    // Clean up multiple consecutive empty lines
    let joined = result.join("\n");
    let mut clean = String::new();
    let mut last_was_empty = false;

    for line in joined.lines() {
        let is_empty = line.trim().is_empty();
        if is_empty && last_was_empty {
            continue;
        }
        if !clean.is_empty() {
            clean.push('\n');
        }
        clean.push_str(line);
        last_was_empty = is_empty;
    }

    clean.push('\n');
    clean
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn parses_new_delta() {
        let content = r#"## Scenarios

<!-- DELTA:NEW -->
### Scenario: User logs in

* *GIVEN* a user exists
* *WHEN* they log in
* *THEN* they SHALL be authenticated
<!-- /DELTA:NEW -->
"#;

        let deltas = parse_deltas(content).unwrap();
        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0].kind, DeltaKind::New);
        assert_eq!(deltas[0].scenario_title, Some("User logs in".to_string()));
    }

    #[test]
    fn parses_changed_delta() {
        let content = r#"<!-- DELTA:CHANGED -->
### Scenario: Old feature

Updated content
<!-- /DELTA:CHANGED -->"#;

        let deltas = parse_deltas(content).unwrap();
        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0].kind, DeltaKind::Changed);
    }

    #[test]
    fn parses_removed_delta() {
        let content = r#"<!-- DELTA:REMOVED -->
### Scenario: Deprecated feature
<!-- /DELTA:REMOVED -->"#;

        let deltas = parse_deltas(content).unwrap();
        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0].kind, DeltaKind::Removed);
        assert_eq!(
            deltas[0].scenario_title,
            Some("Deprecated feature".to_string())
        );
    }

    #[test]
    fn parses_multiple_deltas() {
        let content = r#"## Scenarios

<!-- DELTA:NEW -->
### Scenario: First
* content
<!-- /DELTA:NEW -->

<!-- DELTA:CHANGED -->
### Scenario: Second
* updated
<!-- /DELTA:CHANGED -->

<!-- DELTA:REMOVED -->
### Scenario: Third
<!-- /DELTA:REMOVED -->
"#;

        let deltas = parse_deltas(content).unwrap();
        assert_eq!(deltas.len(), 3);
        assert_eq!(deltas[0].kind, DeltaKind::New);
        assert_eq!(deltas[1].kind, DeltaKind::Changed);
        assert_eq!(deltas[2].kind, DeltaKind::Removed);
    }

    #[test]
    fn error_on_nested_delta_markers() {
        let content = r#"<!-- DELTA:NEW -->
### Scenario: Test
<!-- DELTA:CHANGED -->
nested
<!-- /DELTA:CHANGED -->
<!-- /DELTA:NEW -->
"#;

        let result = parse_deltas(content);
        assert!(matches!(result, Err(RecordError::MalformedDelta { .. })));
    }

    #[test]
    fn error_on_close_without_open() {
        let content = r#"### Scenario: Test
<!-- /DELTA:NEW -->
"#;

        let result = parse_deltas(content);
        assert!(matches!(result, Err(RecordError::MalformedDelta { .. })));
    }

    #[test]
    fn strips_markers_preserves_content() {
        let content = r#"# Feature: Test

## Background

Context here.

## Scenarios

<!-- DELTA:NEW -->
### Scenario: New one

* *GIVEN* setup
* *WHEN* action
* *THEN* result SHALL happen
<!-- /DELTA:NEW -->
"#;

        let stripped = strip_delta_markers(content);

        assert!(!stripped.contains("DELTA"));
        assert!(stripped.contains("### Scenario: New one"));
        assert!(stripped.contains("# Feature: Test"));
    }

    #[test]
    fn strips_removed_content() {
        let content = r#"# Feature: Test

## Scenarios

<!-- DELTA:REMOVED -->
### Scenario: Gone

* Old content
<!-- /DELTA:REMOVED -->

### Scenario: Stays

* *GIVEN* valid
"#;

        let stripped = strip_delta_markers(content);

        assert!(!stripped.contains("Gone"));
        assert!(stripped.contains("### Scenario: Stays"));
    }

    #[test]
    fn merge_appends_new_scenario() {
        let existing = r#"# Feature: Test

## Scenarios

### Scenario: Existing

* *GIVEN* something
"#;

        let delta = r#"## Scenarios

<!-- DELTA:NEW -->
### Scenario: New one

* *GIVEN* new thing
<!-- /DELTA:NEW -->
"#;

        let merged = merge_delta(existing, delta).unwrap();

        assert!(merged.contains("### Scenario: Existing"));
        assert!(merged.contains("### Scenario: New one"));
    }

    #[test]
    fn merge_returns_existing_when_no_deltas() {
        let existing = "# Feature: Test\n\nContent here.\n";
        let delta = "No delta markers here.\n";

        let merged = merge_delta(existing, delta).unwrap();
        assert_eq!(merged, existing);
    }

    #[test]
    fn merge_replaces_changed_scenario() {
        let existing = r#"# Feature: Test

## Scenarios

### Scenario: Login

* *GIVEN* old setup
* *WHEN* old action
* *THEN* old result

### Scenario: Other

* *GIVEN* other
"#;

        let delta = r#"<!-- DELTA:CHANGED -->
### Scenario: Login

* *GIVEN* new setup
* *WHEN* new action
* *THEN* new result SHALL happen
<!-- /DELTA:CHANGED -->
"#;

        let merged = merge_delta(existing, delta).unwrap();

        assert!(merged.contains("new setup"));
        assert!(!merged.contains("old setup"));
        assert!(merged.contains("### Scenario: Other"));
    }

    #[test]
    fn merge_removes_scenario() {
        let existing = r#"# Feature: Test

## Scenarios

### Scenario: Keep

* *GIVEN* keep this

### Scenario: Remove

* *GIVEN* remove this

### Scenario: Also Keep

* *GIVEN* also keep
"#;

        let delta = r#"<!-- DELTA:REMOVED -->
### Scenario: Remove
<!-- /DELTA:REMOVED -->
"#;

        let merged = merge_delta(existing, delta).unwrap();

        assert!(merged.contains("### Scenario: Keep"));
        assert!(merged.contains("### Scenario: Also Keep"));
        assert!(!merged.contains("### Scenario: Remove"));
        assert!(!merged.contains("remove this"));
    }

    #[test]
    fn extract_scenario_title_returns_none_for_no_scenario() {
        let content = "Just some text\nwithout scenario heading";
        assert_eq!(extract_scenario_title(content), None);
    }

    #[test]
    fn record_plan_creates_new_feature() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path();

        // Create plan with new feature
        let plan_dir = specs.join("_plans/test-plan/domain/feature");
        fs::create_dir_all(&plan_dir).unwrap();
        fs::write(
            plan_dir.join("spec.md"),
            r#"# Feature: New Feature

Description here.

## Background

* Context.

## Scenarios

<!-- DELTA:NEW -->
### Scenario: Test

* *GIVEN* setup
* *WHEN* action
* *THEN* result SHALL happen
<!-- /DELTA:NEW -->
"#,
        )
        .unwrap();

        // Create _recorded directory
        fs::create_dir_all(specs.join("_recorded")).unwrap();

        let result = record_plan(specs, "test-plan").unwrap();

        assert_eq!(result.len(), 1);
        assert!(result[0].contains("domain/feature"));

        // Verify spec created
        let spec_content = fs::read_to_string(specs.join("domain/feature/spec.md")).unwrap();
        assert!(spec_content.contains("### Scenario: Test"));
        assert!(!spec_content.contains("DELTA"));

        // Verify plan archived
        assert!(specs.join("_recorded/test-plan").exists());
        assert!(!specs.join("_plans/test-plan").exists());
    }

    #[test]
    fn record_plan_merges_with_existing() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path();

        // Create existing feature
        let feature_dir = specs.join("domain/feature");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(
            feature_dir.join("spec.md"),
            r#"# Feature: Existing

Description.

## Background

* Context.

## Scenarios

### Scenario: Original

* *GIVEN* original
"#,
        )
        .unwrap();

        // Create plan with delta
        let plan_dir = specs.join("_plans/test-plan/domain/feature");
        fs::create_dir_all(&plan_dir).unwrap();
        fs::write(
            plan_dir.join("spec.md"),
            r#"## Scenarios

<!-- DELTA:NEW -->
### Scenario: Added

* *GIVEN* added
<!-- /DELTA:NEW -->
"#,
        )
        .unwrap();

        fs::create_dir_all(specs.join("_recorded")).unwrap();

        let result = record_plan(specs, "test-plan").unwrap();
        assert_eq!(result.len(), 1);

        let spec_content = fs::read_to_string(specs.join("domain/feature/spec.md")).unwrap();
        assert!(spec_content.contains("### Scenario: Original"));
        assert!(spec_content.contains("### Scenario: Added"));
    }

    #[test]
    fn record_plan_not_found() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("_plans")).unwrap();

        let result = record_plan(tmp.path(), "nonexistent");
        assert!(matches!(result, Err(RecordError::PlanNotFound(_))));
    }

    #[test]
    fn replace_scenario_at_end_of_file() {
        let content = r#"# Feature

## Scenarios

### Scenario: Last

* old content"#;

        let replacement = "### Scenario: Last\n\n* new content";
        let result = replace_scenario(content, "Last", replacement);

        assert!(result.contains("new content"));
        assert!(!result.contains("old content"));
    }

    #[test]
    fn remove_scenario_at_end_of_file() {
        let content = r#"# Feature

## Scenarios

### Scenario: Keep

* keep this

### Scenario: Remove

* remove this"#;

        let result = remove_scenario(content, "Remove");

        assert!(result.contains("### Scenario: Keep"));
        assert!(!result.contains("### Scenario: Remove"));
    }

    #[test]
    fn remove_scenario_collapses_empty_lines() {
        let content = "### Scenario: A\n\ncontent\n\n\n\n### Scenario: B\n\nmore";
        let result = remove_scenario(content, "A");

        // Should not have multiple consecutive empty lines
        assert!(!result.contains("\n\n\n"));
    }
}
