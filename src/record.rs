use std::fs;
use std::path::{Path, PathBuf};

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

    #[error("{0}")]
    InvalidDeltaAnchor(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeltaKind {
    New,
    Changed,
    Removed,
}

impl DeltaKind {
    fn marker(&self) -> &'static str {
        match self {
            DeltaKind::New => "DELTA:NEW",
            DeltaKind::Changed => "DELTA:CHANGED",
            DeltaKind::Removed => "DELTA:REMOVED",
        }
    }
}

const SCENARIO_HEADING_PREFIX: &str = "### Scenario:";
const BACKGROUND_HEADING: &str = "## Background";
const FEATURE_HEADING_PREFIX: &str = "# Feature";

/// The section of a feature spec that a delta block targets.
///
/// The anchor owns the heading text, the match mode, and the headings that end
/// the section, so no caller passes a heading string or a stop prefix around.
/// A block whose first non-empty line names none of the recognized sections is
/// `Unrecognized` rather than absent, so the rule checks can report it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeltaAnchor {
    Scenario(String),
    Background,
    Description,
    Unrecognized,
}

impl DeltaAnchor {
    fn matches_heading(&self, line: &str) -> bool {
        match self {
            DeltaAnchor::Scenario(title) => line
                .strip_prefix(SCENARIO_HEADING_PREFIX)
                .is_some_and(|rest| rest.trim() == title),
            DeltaAnchor::Background => line == BACKGROUND_HEADING,
            DeltaAnchor::Description => line.starts_with(FEATURE_HEADING_PREFIX),
            DeltaAnchor::Unrecognized => false,
        }
    }

    fn stop_prefixes(&self) -> &'static [&'static str] {
        match self {
            DeltaAnchor::Scenario(_) => &["### ", "## "],
            DeltaAnchor::Background | DeltaAnchor::Description => &["## "],
            DeltaAnchor::Unrecognized => &[],
        }
    }

    fn label(&self) -> String {
        match self {
            DeltaAnchor::Scenario(title) => format!("{SCENARIO_HEADING_PREFIX} {title}"),
            DeltaAnchor::Background => BACKGROUND_HEADING.to_string(),
            DeltaAnchor::Description => format!("{FEATURE_HEADING_PREFIX}: <name>"),
            DeltaAnchor::Unrecognized => "no recognized anchor".to_string(),
        }
    }
}

/// One `<!-- DELTA:* -->` block parsed out of a delta spec.
///
/// `content` is the block body with the opening and closing markers stripped
/// and surrounding whitespace trimmed. `anchor` is classified from the first
/// non-empty line of `content` alone — no later line in the block can change
/// what the block targets.
#[derive(Debug, Clone)]
pub struct DeltaBlock {
    pub kind: DeltaKind,
    pub content: String,
    pub anchor: DeltaAnchor,
}

/// Record every delta spec of a plan into the permanent specs, then archive the plan.
///
/// Runs as four phases so that a rejected plan changes nothing: every delta spec
/// of the plan is checked, then every delta is merged in memory, and only a plan
/// that survives both is written and archived. A re-run after the author fixes a
/// rejected delta therefore applies each block exactly once. A file-system failure
/// during the write or archive phase is outside that guarantee.
///
/// Returns the `<domain>/<feature>` path of each recorded spec.
pub fn record_plan(specs_base: &Path, plan_name: &str) -> Result<Vec<String>, RecordError> {
    let plan_dir = specs_base.join("_plans").join(plan_name);

    if !plan_dir.exists() {
        return Err(RecordError::PlanNotFound(plan_name.to_string()));
    }

    let targets = plan_targets(specs_base, &plan_dir)?;

    check_plan_anchors(&targets)?;
    let merged_specs = merge_plan(&targets)?;
    write_specs(&merged_specs)?;
    archive_plan(specs_base, &plan_dir, plan_name)?;

    Ok(targets.into_iter().map(|target| target.feature).collect())
}

/// A delta spec of a plan paired with the permanent spec it targets.
///
/// Mapping a plan-relative delta path to a target path is one decision, so the
/// check, merge, and write phases read it here instead of each deriving it.
struct DeltaTarget {
    delta_path: PathBuf,
    feature: String,
    target_spec: PathBuf,
}

fn plan_targets(specs_base: &Path, plan_dir: &Path) -> Result<Vec<DeltaTarget>, RecordError> {
    let delta_specs = find_delta_specs(plan_dir)?;

    Ok(delta_specs
        .into_iter()
        .map(|delta_path| {
            let relative = delta_path.strip_prefix(plan_dir).unwrap_or(&delta_path);
            let feature_dir = relative.parent().unwrap_or(Path::new("")).to_path_buf();

            DeltaTarget {
                feature: feature_dir.display().to_string(),
                target_spec: specs_base.join(feature_dir).join("spec.md"),
                delta_path,
            }
        })
        .collect())
}

/// Phase 1: reject the plan when any delta block breaks an anchor rule.
///
/// Collects across every delta spec so one run names every violation of the plan
/// rather than only the first. A delta whose target spec does not exist yet is
/// skipped: it records as a whole new file, so its blocks anchor to nothing.
fn check_plan_anchors(targets: &[DeltaTarget]) -> Result<(), RecordError> {
    let mut violations = Vec::new();

    for target in targets {
        if !target.target_spec.exists() {
            continue;
        }

        let blocks =
            parse_deltas(&read_spec(&target.delta_path)?).map_err(|error| match error {
                RecordError::MalformedDelta { line, content } => {
                    RecordError::InvalidDeltaAnchor(format!(
                        "{}: malformed delta marker at line {line}: {content}",
                        target.delta_path.display()
                    ))
                }
                other => other,
            })?;
        violations.extend(
            check_anchor_rules(&blocks)
                .into_iter()
                .map(|message| format!("{}: {message}", target.delta_path.display())),
        );
    }

    if violations.is_empty() {
        return Ok(());
    }

    Err(RecordError::InvalidDeltaAnchor(violations.join("\n")))
}

/// Phase 2: merge every delta spec of the plan into memory, writing nothing.
fn merge_plan(targets: &[DeltaTarget]) -> Result<Vec<(PathBuf, String)>, RecordError> {
    let mut merged_specs = Vec::new();

    for target in targets {
        let delta = read_spec(&target.delta_path)?;

        let merged = if target.target_spec.exists() {
            let existing = read_spec(&target.target_spec)?;
            merge_delta(&existing, &delta).map_err(|error| match error {
                RecordError::InvalidDeltaAnchor(message) => RecordError::InvalidDeltaAnchor(
                    format!("{}: {message}", target.delta_path.display()),
                ),
                other => other,
            })?
        } else {
            strip_delta_markers(&delta)
        };

        merged_specs.push((target.target_spec.clone(), merged));
    }

    Ok(merged_specs)
}

/// Phase 3: commit the merged specs to disk.
fn write_specs(merged_specs: &[(PathBuf, String)]) -> Result<(), RecordError> {
    for (target_spec, content) in merged_specs {
        let target_dir = target_spec.parent().unwrap_or(Path::new("."));

        fs::create_dir_all(target_dir).map_err(|_| RecordError::DirCreateError {
            path: target_dir.display().to_string(),
        })?;

        fs::write(target_spec, content).map_err(|_| RecordError::FileWriteError {
            path: target_spec.display().to_string(),
        })?;
    }

    Ok(())
}

/// Phase 4: move the plan directory under the next `_recorded/NNN-<plan>` number.
fn archive_plan(specs_base: &Path, plan_dir: &Path, plan_name: &str) -> Result<(), RecordError> {
    let recorded_base = specs_base.join("_recorded");
    let next_number = count_recorded_entries(&recorded_base) + 1;
    let recorded_dir = recorded_base.join(format!("{next_number:03}-{plan_name}"));

    fs::create_dir_all(&recorded_base).map_err(|_| RecordError::DirCreateError {
        path: recorded_base.display().to_string(),
    })?;

    fs::rename(plan_dir, &recorded_dir).map_err(|_| RecordError::DirMoveError {
        from: plan_dir.display().to_string(),
        to: recorded_dir.display().to_string(),
    })
}

fn read_spec(path: &Path) -> Result<String, RecordError> {
    fs::read_to_string(path).map_err(|_| RecordError::FileReadError {
        path: path.display().to_string(),
    })
}

fn count_recorded_entries(recorded_base: &Path) -> usize {
    match fs::read_dir(recorded_base) {
        Ok(entries) => entries.flatten().count(),
        Err(_) => 0,
    }
}

/// Every `spec.md` under `plan_dir`, in a stable order.
///
/// The order is sorted because callers report one message per delta spec, and
/// directory-read order would otherwise vary between runs on the same plan.
pub fn find_delta_specs(plan_dir: &Path) -> Result<Vec<PathBuf>, RecordError> {
    let mut specs = Vec::new();
    find_delta_specs_recursive(plan_dir, &mut specs)?;
    specs.sort();
    Ok(specs)
}

fn find_delta_specs_recursive(dir: &Path, specs: &mut Vec<PathBuf>) -> Result<(), RecordError> {
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
        } else if let Some(close_kind) = parse_delta_close(trimmed) {
            if !in_delta {
                return Err(RecordError::MalformedDelta {
                    line: line_num + 1,
                    content: line.to_string(),
                });
            }

            if current_kind.as_ref() != Some(&close_kind) {
                return Err(RecordError::MalformedDelta {
                    line: line_num + 1,
                    content: line.to_string(),
                });
            }

            let content_trimmed = current_content.trim().to_string();
            let anchor = extract_anchor(&content_trimmed);

            deltas.push(DeltaBlock {
                kind: current_kind.take().unwrap(),
                content: content_trimmed,
                anchor,
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

    if in_delta {
        return Err(RecordError::MalformedDelta {
            line: content.lines().count(),
            content: current_content,
        });
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

fn extract_anchor(content: &str) -> DeltaAnchor {
    let Some(line) = first_non_empty_line(content) else {
        return DeltaAnchor::Unrecognized;
    };

    if let Some(title) = line.strip_prefix(SCENARIO_HEADING_PREFIX) {
        return DeltaAnchor::Scenario(title.trim().to_string());
    }
    if line == BACKGROUND_HEADING {
        return DeltaAnchor::Background;
    }
    if line.starts_with(FEATURE_HEADING_PREFIX) {
        return DeltaAnchor::Description;
    }
    DeltaAnchor::Unrecognized
}

fn first_non_empty_line(content: &str) -> Option<&str> {
    content.lines().map(str::trim).find(|line| !line.is_empty())
}

/// One message per rule violation, empty when every block is legal.
///
/// This is the sole owner of the `(DeltaKind, DeltaAnchor)` legality matrix, so
/// the `speq plan validate` gate and the `speq record` gate cannot drift apart.
/// The rules hold against the delta file alone and read no target spec, which is
/// what lets both gates share them. Whether an anchor exists in the target spec
/// is settled at merge time instead.
///
/// A message names no file, because the caller knows which delta file the blocks
/// came from and prefixes each message with that path.
pub fn check_anchor_rules(blocks: &[DeltaBlock]) -> Vec<String> {
    let mut messages: Vec<String> = blocks.iter().filter_map(illegal_pair_message).collect();
    messages.extend(repeated_anchor_messages(blocks));
    messages
}

fn illegal_pair_message(block: &DeltaBlock) -> Option<String> {
    match (&block.kind, &block.anchor) {
        (_, DeltaAnchor::Unrecognized) => Some(unrecognized_anchor_message(&block.content)),
        (DeltaKind::New, anchor @ (DeltaAnchor::Background | DeltaAnchor::Description)) => {
            Some(new_on_prose_anchor_message(anchor))
        }
        (DeltaKind::Removed, anchor @ (DeltaAnchor::Background | DeltaAnchor::Description)) => {
            Some(removed_on_prose_anchor_message(anchor))
        }
        (DeltaKind::New | DeltaKind::Changed | DeltaKind::Removed, DeltaAnchor::Scenario(_))
        | (DeltaKind::Changed, DeltaAnchor::Background | DeltaAnchor::Description) => None,
    }
}

fn unrecognized_anchor_message(content: &str) -> String {
    let opening = match first_non_empty_line(content) {
        Some(line) => format!("Delta block starts with `{line}`"),
        None => "Delta block is empty".to_string(),
    };
    format!(
        "{opening}, so it names no section to merge into. \
         The first non-empty line of a delta block MUST be \
         `{SCENARIO_HEADING_PREFIX} <name>`, `{BACKGROUND_HEADING}`, or \
         `{FEATURE_HEADING_PREFIX}: <name>`. Move that heading inside the delta marker."
    )
}

fn new_on_prose_anchor_message(anchor: &DeltaAnchor) -> String {
    format!(
        "{} MUST NOT target `{}`. That section is required, so it already exists in the \
         target spec. Use {} instead.",
        DeltaKind::New.marker(),
        anchor.label(),
        DeltaKind::Changed.marker()
    )
}

fn removed_on_prose_anchor_message(anchor: &DeltaAnchor) -> String {
    format!(
        "{} MUST NOT target `{}`. That section is required, so removing it produces an \
         invalid spec. Use {} instead.",
        DeltaKind::Removed.marker(),
        anchor.label(),
        DeltaKind::Changed.marker()
    )
}

fn repeated_anchor_messages(blocks: &[DeltaBlock]) -> Vec<String> {
    let mut groups: Vec<(&DeltaAnchor, Vec<&'static str>)> = Vec::new();

    for block in blocks {
        if block.anchor == DeltaAnchor::Unrecognized {
            continue;
        }
        match groups
            .iter()
            .position(|(anchor, _)| *anchor == &block.anchor)
        {
            Some(index) => groups[index].1.push(block.kind.marker()),
            None => groups.push((&block.anchor, vec![block.kind.marker()])),
        }
    }

    groups
        .iter()
        .filter(|(_, markers)| markers.len() > 1)
        .map(|(anchor, markers)| {
            format!(
                "Anchor `{}` appears in {} delta blocks ({}). A recognized anchor MAY appear \
                 in at most one delta block per delta file, whatever the markers, because the \
                 merged result would otherwise depend on the order of the blocks. Write one \
                 block carrying the final text.",
                anchor.label(),
                markers.len(),
                markers.join(", ")
            )
        })
        .collect()
}

/// Merge every delta block of `delta` into `existing`.
///
/// Which `(DeltaKind, DeltaAnchor)` pairs are legal belongs to `check_anchor_rules`,
/// which runs over every block before the first block is applied. What is left to
/// decide per block is therefore one behavior per marker kind: `New` appends,
/// `Changed` replaces, `Removed` removes. Whether the anchor exists in `existing`
/// is settled here, because only here is the target text known.
pub fn merge_delta(existing: &str, delta: &str) -> Result<String, RecordError> {
    let blocks = parse_deltas(delta)?;

    if blocks.is_empty() {
        return Ok(existing.to_string());
    }

    let violations = check_anchor_rules(&blocks);
    if !violations.is_empty() {
        return Err(RecordError::InvalidDeltaAnchor(violations.join("\n")));
    }

    let mut merged = existing.to_string();

    for block in &blocks {
        merged = apply_block(&merged, block)?;
    }

    Ok(merged)
}

fn apply_block(content: &str, block: &DeltaBlock) -> Result<String, RecordError> {
    match block.kind {
        DeltaKind::New => Ok(format!("{}\n\n{}\n", content.trim_end(), block.content)),
        DeltaKind::Changed => replace_section(content, &block.anchor, &block.content)
            .ok_or_else(|| anchor_not_found_error(block)),
        DeltaKind::Removed => {
            remove_section(content, &block.anchor).ok_or_else(|| anchor_not_found_error(block))
        }
    }
}

fn anchor_not_found_error(block: &DeltaBlock) -> RecordError {
    RecordError::InvalidDeltaAnchor(format!(
        "{marker} targets `{anchor}`, which the target spec does not hold. A {marker} block \
         MUST name a section that already exists. Check the anchor against the target spec.",
        marker = block.kind.marker(),
        anchor = block.anchor.label()
    ))
}

fn locate_section(content: &str, anchor: &DeltaAnchor) -> Option<(usize, usize)> {
    let lines: Vec<&str> = content.lines().collect();
    let start = lines
        .iter()
        .position(|line| anchor.matches_heading(line.trim()))?;

    let stop_prefixes = anchor.stop_prefixes();
    let end = lines[start + 1..]
        .iter()
        .position(|line| {
            let trimmed = line.trim();
            stop_prefixes
                .iter()
                .any(|prefix| trimmed.starts_with(prefix))
        })
        .map_or(lines.len(), |offset| start + 1 + offset);

    Some((start, end))
}

fn replace_section(content: &str, anchor: &DeltaAnchor, replacement: &str) -> Option<String> {
    let replacement_lines: Vec<&str> = replacement.trim().lines().collect();
    splice_section(content, anchor, &replacement_lines)
}

fn remove_section(content: &str, anchor: &DeltaAnchor) -> Option<String> {
    splice_section(content, anchor, &[])
}

fn splice_section(content: &str, anchor: &DeltaAnchor, replacement: &[&str]) -> Option<String> {
    let (start, end) = locate_section(content, anchor)?;

    let lines: Vec<&str> = content.lines().collect();
    let head = &lines[..start];
    let tail = &lines[end..];

    let mut merged: Vec<&str> = head[..len_without_trailing_blanks(head)].to_vec();
    append_block(&mut merged, replacement);
    append_block(&mut merged, tail);

    let mut result = merged.join("\n");
    if content.ends_with('\n') {
        result.push('\n');
    }

    Some(result)
}

fn append_block<'a>(merged: &mut Vec<&'a str>, block: &[&'a str]) {
    if block.is_empty() {
        return;
    }
    if !merged.is_empty() {
        merged.push("");
    }
    merged.extend_from_slice(block);
}

fn len_without_trailing_blanks(lines: &[&str]) -> usize {
    lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .map_or(0, |index| index + 1)
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

    fn replace_scenario(content: &str, title: &str, replacement: &str) -> String {
        replace_section(
            content,
            &DeltaAnchor::Scenario(title.to_string()),
            replacement,
        )
        .unwrap()
    }

    fn remove_scenario(content: &str, title: &str) -> String {
        remove_section(content, &DeltaAnchor::Scenario(title.to_string())).unwrap()
    }

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
        assert_eq!(
            deltas[0].anchor,
            DeltaAnchor::Scenario("User logs in".to_string())
        );
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
            deltas[0].anchor,
            DeltaAnchor::Scenario("Deprecated feature".to_string())
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
    fn error_on_mismatched_close_marker() {
        let content = r#"<!-- DELTA:NEW -->
### Scenario: Test
<!-- /DELTA:CHANGED -->
"#;

        let result = parse_deltas(content);
        assert!(matches!(result, Err(RecordError::MalformedDelta { .. })));
    }

    #[test]
    fn error_on_unclosed_delta_at_end_of_file() {
        let content = r#"<!-- DELTA:REMOVED -->
### Scenario: Test
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
    fn merge_rejects_a_changed_scenario_the_target_spec_does_not_hold() {
        let delta = "<!-- DELTA:CHANGED -->\n### Scenario: Logout\n\n* *GIVEN* a session\n<!-- /DELTA:CHANGED -->\n";

        let error = merge_delta(SPEC, delta).unwrap_err();

        assert!(matches!(error, RecordError::InvalidDeltaAnchor(_)));
        let message = error.to_string();
        assert!(message.contains("### Scenario: Logout"), "{message}");
        assert!(message.contains("DELTA:CHANGED"), "{message}");
    }

    #[test]
    fn merge_rejects_a_removed_scenario_the_target_spec_does_not_hold() {
        let delta = "<!-- DELTA:REMOVED -->\n### Scenario: Logout\n<!-- /DELTA:REMOVED -->\n";

        let error = merge_delta(SPEC, delta).unwrap_err();

        assert!(matches!(error, RecordError::InvalidDeltaAnchor(_)));
        let message = error.to_string();
        assert!(message.contains("### Scenario: Logout"), "{message}");
        assert!(message.contains("DELTA:REMOVED"), "{message}");
    }

    #[test]
    fn merge_rejects_an_illegal_marker_and_anchor_pair() {
        let delta = "<!-- DELTA:NEW -->\n## Background\n\n* Context two.\n<!-- /DELTA:NEW -->\n";

        let error = merge_delta(SPEC, delta).unwrap_err();

        assert!(matches!(error, RecordError::InvalidDeltaAnchor(_)));
        assert!(error.to_string().contains("DELTA:CHANGED"));
    }

    #[test]
    fn merge_reports_every_rule_violation_of_one_delta_file() {
        let delta = "<!-- DELTA:NEW -->\n## Background\n\n* Context.\n<!-- /DELTA:NEW -->\n\
                     <!-- DELTA:CHANGED -->\n* not a heading\n<!-- /DELTA:CHANGED -->\n";

        let error = merge_delta(SPEC, delta).unwrap_err();

        assert_eq!(error.to_string().lines().count(), 2);
    }

    #[test]
    fn merge_replaces_the_background_section_in_place() {
        let delta =
            "<!-- DELTA:CHANGED -->\n## Background\n\n* Context two.\n<!-- /DELTA:CHANGED -->\n";

        let merged = merge_delta(SPEC, delta).unwrap();

        assert!(
            merged.contains("* Context two.\n\n## Scenarios"),
            "{merged}"
        );
        assert!(!merged.contains("* Context one."));
        assert_eq!(merged.matches("## Background").count(), 1);
    }

    #[test]
    fn merge_replaces_the_feature_description_in_place() {
        let delta = "<!-- DELTA:CHANGED -->\n# Feature: Test Feature\n\nThe system SHALL do other things.\n<!-- /DELTA:CHANGED -->\n";

        let merged = merge_delta(SPEC, delta).unwrap();

        assert!(merged.starts_with("# Feature: Test Feature\n"));
        assert!(
            merged.contains("other things.\n\n## Background"),
            "{merged}"
        );
        assert!(!merged.contains("The system SHALL do things."));
    }

    #[test]
    fn merge_renames_the_feature_heading() {
        let delta = "<!-- DELTA:CHANGED -->\n# Feature: Renamed Feature\n\nThe system SHALL do things.\n<!-- /DELTA:CHANGED -->\n";

        let merged = merge_delta(SPEC, delta).unwrap();

        assert!(merged.starts_with("# Feature: Renamed Feature\n"));
        assert!(!merged.contains("# Feature: Test Feature"));
        assert!(merged.contains("## Background"));
    }

    #[test]
    fn merge_rejects_a_changed_background_the_target_spec_does_not_hold() {
        let bare = "# Feature: Bare\n\nText.\n\n## Scenarios\n\n### Scenario: One\n";
        let delta =
            "<!-- DELTA:CHANGED -->\n## Background\n\n* Context.\n<!-- /DELTA:CHANGED -->\n";

        let error = merge_delta(bare, delta).unwrap_err();

        assert!(matches!(error, RecordError::InvalidDeltaAnchor(_)));
        assert!(error.to_string().contains("## Background"));
    }

    #[test]
    fn merge_checks_every_block_before_it_applies_any() {
        let delta = "<!-- DELTA:CHANGED -->\n### Scenario: Login\n\n* *GIVEN* a member\n<!-- /DELTA:CHANGED -->\n\
                     <!-- DELTA:NEW -->\n## Background\n\n* Context two.\n<!-- /DELTA:NEW -->\n";

        let error = merge_delta(SPEC, delta).unwrap_err();

        assert!(matches!(error, RecordError::InvalidDeltaAnchor(_)));
    }

    const SPEC: &str = r#"# Feature: Test Feature

The system SHALL do things.

## Background

* Context one.

## Scenarios

### Scenario: Login

* *GIVEN* a user

### Scenario: Login fails

* *GIVEN* a bad password
"#;

    fn scenario(title: &str) -> DeltaAnchor {
        DeltaAnchor::Scenario(title.to_string())
    }

    fn located_text(content: &str, anchor: &DeltaAnchor) -> Option<String> {
        let (start, end) = locate_section(content, anchor)?;
        let lines: Vec<&str> = content.lines().collect();
        Some(lines[start..end].join("\n").trim_end().to_string())
    }

    #[test]
    fn locate_section_returns_the_line_range_of_a_scenario() {
        assert_eq!(locate_section(SPEC, &scenario("Login")), Some((10, 14)));
    }

    #[test]
    fn locate_section_ends_a_scenario_at_the_next_scenario() {
        assert_eq!(
            located_text(SPEC, &scenario("Login")),
            Some("### Scenario: Login\n\n* *GIVEN* a user".to_string())
        );
    }

    #[test]
    fn locate_section_ends_the_last_scenario_at_the_end_of_the_file() {
        assert_eq!(
            located_text(SPEC, &scenario("Login fails")),
            Some("### Scenario: Login fails\n\n* *GIVEN* a bad password".to_string())
        );
    }

    #[test]
    fn locate_section_matches_a_scenario_title_exactly() {
        assert_eq!(locate_section(SPEC, &scenario("Login f")), None);
        assert_eq!(locate_section(SPEC, &scenario("ogin")), None);
    }

    #[test]
    fn locate_section_ends_background_at_the_next_section_heading() {
        assert_eq!(
            located_text(SPEC, &DeltaAnchor::Background),
            Some("## Background\n\n* Context one.".to_string())
        );
    }

    #[test]
    fn locate_section_ends_the_description_at_the_next_section_heading() {
        assert_eq!(
            located_text(SPEC, &DeltaAnchor::Description),
            Some("# Feature: Test Feature\n\nThe system SHALL do things.".to_string())
        );
    }

    #[test]
    fn locate_section_finds_the_description_whatever_the_feature_name() {
        let renamed = "# Feature: Something Else\n\nText.\n\n## Background\n\n* Context.\n";
        assert_eq!(
            located_text(renamed, &DeltaAnchor::Description),
            Some("# Feature: Something Else\n\nText.".to_string())
        );
    }

    #[test]
    fn locate_section_returns_none_when_the_section_is_absent() {
        assert_eq!(locate_section(SPEC, &scenario("Logout")), None);
        assert_eq!(
            locate_section("# Feature: Bare\n", &DeltaAnchor::Background),
            None
        );
    }

    #[test]
    fn locate_section_returns_none_for_an_unrecognized_anchor() {
        assert_eq!(locate_section(SPEC, &DeltaAnchor::Unrecognized), None);
    }

    #[test]
    fn locate_section_returns_none_for_empty_content() {
        assert_eq!(locate_section("", &DeltaAnchor::Description), None);
    }

    #[test]
    fn replace_section_keeps_one_empty_line_before_the_next_heading() {
        let merged = replace_section(
            SPEC,
            &DeltaAnchor::Background,
            "## Background\n\n* Context two.",
        )
        .unwrap();

        assert!(merged.contains("* Context two.\n\n## Scenarios"));
        assert!(!merged.contains("* Context one."));
    }

    #[test]
    fn replace_section_inserts_one_empty_line_where_the_source_had_none() {
        let cramped = "# Feature: Cramped\n\nText.\n## Background\n* Context.\n## Scenarios\n";

        let merged = replace_section(
            cramped,
            &DeltaAnchor::Background,
            "## Background\n\n* New context.",
        )
        .unwrap();

        assert!(merged.contains("Text.\n\n## Background\n\n* New context.\n\n## Scenarios"));
    }

    #[test]
    fn replace_section_returns_none_when_the_anchor_is_absent() {
        let merged = replace_section(SPEC, &scenario("Logout"), "### Scenario: Logout");

        assert_eq!(merged, None);
    }

    #[test]
    fn replace_section_preserves_the_trailing_newline() {
        let with_newline = replace_section(
            SPEC,
            &scenario("Login"),
            "### Scenario: Login\n\n* *GIVEN* a member",
        )
        .unwrap();
        assert!(with_newline.ends_with("a bad password\n"));

        let without_newline = replace_section(
            SPEC.trim_end(),
            &scenario("Login"),
            "### Scenario: Login\n\n* *GIVEN* a member",
        )
        .unwrap();
        assert!(without_newline.ends_with("a bad password"));
    }

    #[test]
    fn replace_section_rewrites_the_feature_heading_and_description() {
        let merged = replace_section(
            SPEC,
            &DeltaAnchor::Description,
            "# Feature: Renamed Feature\n\nThe system SHALL do other things.",
        )
        .unwrap();

        assert!(merged.starts_with("# Feature: Renamed Feature\n"));
        assert!(merged.contains("other things.\n\n## Background"));
        assert!(!merged.contains("# Feature: Test Feature"));
    }

    #[test]
    fn remove_section_leaves_one_empty_line_at_the_seam() {
        let merged = remove_section(SPEC, &scenario("Login")).unwrap();

        assert!(merged.contains("## Scenarios\n\n### Scenario: Login fails"));
        assert!(!merged.contains("* *GIVEN* a user"));
    }

    #[test]
    fn remove_section_matches_a_scenario_title_exactly() {
        let merged = remove_section(SPEC, &scenario("Login")).unwrap();

        assert!(merged.contains("### Scenario: Login fails"));
        assert!(merged.contains("* *GIVEN* a bad password"));
    }

    #[test]
    fn remove_section_returns_none_when_the_anchor_is_absent() {
        let merged = remove_section(SPEC, &scenario("Logout"));

        assert_eq!(merged, None);
    }

    #[test]
    fn remove_section_at_the_end_of_the_file_leaves_no_trailing_empty_line() {
        let merged = remove_section(SPEC, &scenario("Login fails")).unwrap();

        assert!(merged.ends_with("* *GIVEN* a user\n"));
        assert!(!merged.ends_with("\n\n"));
    }

    #[test]
    fn remove_section_empties_a_document_that_holds_only_that_section() {
        let content = "### Scenario: Only\n\n* *GIVEN* a thing\n";

        let merged = remove_section(content, &scenario("Only")).unwrap();

        assert!(merged.trim().is_empty());
    }

    fn block(kind: DeltaKind, content: &str) -> DeltaBlock {
        DeltaBlock {
            kind,
            content: content.to_string(),
            anchor: extract_anchor(content),
        }
    }

    #[test]
    fn check_anchor_rules_accepts_every_legal_marker_and_anchor_pair() {
        let blocks = vec![
            block(DeltaKind::New, "### Scenario: Added"),
            block(DeltaKind::Changed, "### Scenario: Updated"),
            block(DeltaKind::Removed, "### Scenario: Dropped"),
            block(DeltaKind::Changed, "## Background\n\n* Context."),
            block(DeltaKind::Changed, "# Feature: Renamed\n\nDescription."),
        ];

        assert_eq!(check_anchor_rules(&blocks), Vec::<String>::new());
    }

    #[test]
    fn check_anchor_rules_accepts_no_blocks() {
        assert_eq!(check_anchor_rules(&[]), Vec::<String>::new());
    }

    #[test]
    fn check_anchor_rules_rejects_unrecognized_anchor() {
        let blocks = vec![block(DeltaKind::Changed, "* A bullet, not a heading")];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("### Scenario: <name>"));
        assert!(messages[0].contains("## Background"));
        assert!(messages[0].contains("# Feature: <name>"));
        assert!(messages[0].contains("* A bullet, not a heading"));
        assert!(messages[0].contains("inside the delta marker"));
    }

    #[test]
    fn check_anchor_rules_rejects_new_on_background() {
        let blocks = vec![block(DeltaKind::New, "## Background\n\n* Context.")];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("DELTA:NEW"));
        assert!(messages[0].contains("## Background"));
        assert!(messages[0].contains("DELTA:CHANGED"));
    }

    #[test]
    fn check_anchor_rules_rejects_removed_on_description() {
        let blocks = vec![block(DeltaKind::Removed, "# Feature: Gone\n\nDescription.")];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("DELTA:REMOVED"));
        assert!(messages[0].contains("# Feature"));
        assert!(messages[0].contains("DELTA:CHANGED"));
    }

    #[test]
    fn check_anchor_rules_rejects_one_anchor_carried_by_two_blocks_of_one_kind() {
        let blocks = vec![
            block(DeltaKind::New, "### Scenario: Login\n\n* first"),
            block(DeltaKind::New, "### Scenario: Login\n\n* second"),
        ];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("### Scenario: Login"));
        assert!(messages[0].contains("DELTA:NEW"));
    }

    #[test]
    fn check_anchor_rules_rejects_one_anchor_carried_by_two_kinds() {
        let blocks = vec![
            block(DeltaKind::New, "### Scenario: Login\n\n* added"),
            block(DeltaKind::Changed, "### Scenario: Login\n\n* updated"),
        ];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("### Scenario: Login"));
        assert!(messages[0].contains("DELTA:NEW"));
        assert!(messages[0].contains("DELTA:CHANGED"));
    }

    #[test]
    fn check_anchor_rules_accepts_two_blocks_on_different_anchors() {
        let blocks = vec![
            block(DeltaKind::Changed, "### Scenario: Login"),
            block(DeltaKind::Changed, "### Scenario: Login fails"),
        ];

        assert_eq!(check_anchor_rules(&blocks), Vec::<String>::new());
    }

    #[test]
    fn check_anchor_rules_exempts_unrecognized_anchors_from_the_repeat_rule() {
        let blocks = vec![
            block(DeltaKind::Changed, "* one bullet"),
            block(DeltaKind::Changed, "* another bullet"),
        ];

        assert_eq!(check_anchor_rules(&blocks).len(), 2);
    }

    #[test]
    fn check_anchor_rules_reports_every_violation_of_the_file() {
        let blocks = vec![
            block(DeltaKind::New, "## Background\n\n* first"),
            block(DeltaKind::Changed, "## Background\n\n* second"),
            block(DeltaKind::Changed, "* not a heading"),
        ];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 3);
    }

    #[test]
    fn extract_anchor_returns_unrecognized_for_prose_without_a_heading() {
        let content = "Just some text\nwithout scenario heading";
        assert_eq!(extract_anchor(content), DeltaAnchor::Unrecognized);
    }

    #[test]
    fn extract_anchor_reads_scenario_title() {
        let content = "### Scenario: User logs in\n\n* *GIVEN* a user exists";
        assert_eq!(
            extract_anchor(content),
            DeltaAnchor::Scenario("User logs in".to_string())
        );
    }

    #[test]
    fn extract_anchor_reads_background_heading() {
        let content = "## Background\n\n* Command syntax: `speq record`";
        assert_eq!(extract_anchor(content), DeltaAnchor::Background);
    }

    #[test]
    fn extract_anchor_reads_feature_heading_by_prefix() {
        let content = "# Feature: Any Name At All\n\nDescription.";
        assert_eq!(extract_anchor(content), DeltaAnchor::Description);
    }

    #[test]
    fn extract_anchor_skips_leading_empty_lines() {
        let content = "\n\n   \n## Background\n\n* Context.";
        assert_eq!(extract_anchor(content), DeltaAnchor::Background);
    }

    #[test]
    fn extract_anchor_reads_the_first_non_empty_line_only() {
        let content = "* A bullet comes first\n\n### Scenario: Not the anchor";
        assert_eq!(extract_anchor(content), DeltaAnchor::Unrecognized);
    }

    #[test]
    fn extract_anchor_rejects_background_heading_with_extra_words() {
        let content = "## Background and Context\n\n* Context.";
        assert_eq!(extract_anchor(content), DeltaAnchor::Unrecognized);
    }

    #[test]
    fn extract_anchor_returns_unrecognized_for_empty_content() {
        assert_eq!(extract_anchor(""), DeltaAnchor::Unrecognized);
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

        // Verify plan archived with date prefix
        let recorded_dir = specs.join("_recorded");
        let entries: Vec<_> = fs::read_dir(&recorded_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert_eq!(entries.len(), 1, "Expected exactly one recorded plan");

        let archived_name = entries[0].file_name().to_string_lossy().to_string();
        assert_eq!(
            archived_name, "001-test-plan",
            "Archive should be numbered NNN-<plan>, starting at 001"
        );

        assert!(!specs.join("_plans/test-plan").exists());
    }

    #[test]
    fn record_plan_numbers_second_plan_sequentially() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path();

        for plan in ["plan-one", "plan-two"] {
            let plan_dir = specs.join(format!("_plans/{plan}/domain/feature"));
            fs::create_dir_all(&plan_dir).unwrap();
            fs::write(
                plan_dir.join("spec.md"),
                r#"# Feature: New Feature

Description here.

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
        }

        record_plan(specs, "plan-one").unwrap();
        record_plan(specs, "plan-two").unwrap();

        assert!(specs.join("_recorded/001-plan-one").exists());
        assert!(specs.join("_recorded/002-plan-two").exists());
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

    fn write_spec(path: &Path, content: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    #[test]
    fn record_plan_reports_every_anchor_violation_of_the_plan() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path();
        write_spec(&specs.join("a/one/spec.md"), SPEC);
        write_spec(&specs.join("b/two/spec.md"), SPEC);
        write_spec(
            &specs.join("_plans/p/a/one/spec.md"),
            "<!-- DELTA:NEW -->\n## Background\n\n* Context two.\n<!-- /DELTA:NEW -->\n",
        );
        write_spec(
            &specs.join("_plans/p/b/two/spec.md"),
            "<!-- DELTA:CHANGED -->\n* not a heading\n<!-- /DELTA:CHANGED -->\n",
        );

        let error = record_plan(specs, "p").unwrap_err();

        let message = error.to_string();
        assert!(matches!(error, RecordError::InvalidDeltaAnchor(_)));
        assert_eq!(message.lines().count(), 2, "{message}");
        assert!(message.contains("a/one/spec.md"), "{message}");
        assert!(message.contains("b/two/spec.md"), "{message}");
        assert!(specs.join("_plans/p").exists());
    }

    #[test]
    fn record_plan_skips_anchor_checks_for_a_feature_with_no_target_spec() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path();
        write_spec(
            &specs.join("_plans/p/a/one/spec.md"),
            "# Feature: Fresh\n\nText.\n\n<!-- DELTA:NEW -->\n## Background\n\n* Context.\n<!-- /DELTA:NEW -->\n\n## Scenarios\n\n### Scenario: One\n",
        );

        let recorded = record_plan(specs, "p").unwrap();

        assert_eq!(recorded, vec!["a/one".to_string()]);
        let written = fs::read_to_string(specs.join("a/one/spec.md")).unwrap();
        assert!(!written.contains("DELTA"));
        assert!(written.contains("## Background"));
        assert!(specs.join("_recorded/001-p").exists());
    }

    #[test]
    fn record_plan_writes_no_spec_when_a_later_delta_fails_to_merge() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path();
        write_spec(&specs.join("a/one/spec.md"), SPEC);
        write_spec(&specs.join("b/two/spec.md"), SPEC);
        write_spec(
            &specs.join("_plans/p/a/one/spec.md"),
            "<!-- DELTA:NEW -->\n### Scenario: Added\n\n* *GIVEN* a thing\n<!-- /DELTA:NEW -->\n",
        );
        write_spec(
            &specs.join("_plans/p/b/two/spec.md"),
            "<!-- DELTA:CHANGED -->\n### Scenario: Absent\n\n* *GIVEN* a thing\n<!-- /DELTA:CHANGED -->\n",
        );

        let error = record_plan(specs, "p").unwrap_err();

        assert!(matches!(error, RecordError::InvalidDeltaAnchor(_)));
        assert!(error.to_string().contains("b/two/spec.md"), "{error}");
        assert_eq!(
            fs::read_to_string(specs.join("a/one/spec.md")).unwrap(),
            SPEC
        );
        assert_eq!(
            fs::read_to_string(specs.join("b/two/spec.md")).unwrap(),
            SPEC
        );
        assert!(specs.join("_plans/p").exists());
        assert!(!specs.join("_recorded/001-p").exists());
    }

    #[test]
    fn record_plan_names_the_delta_file_that_holds_a_malformed_marker() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path();
        write_spec(&specs.join("a/one/spec.md"), SPEC);
        write_spec(&specs.join("b/two/spec.md"), SPEC);
        write_spec(
            &specs.join("_plans/p/a/one/spec.md"),
            "<!-- DELTA:NEW -->\n### Scenario: Added\n\n* *GIVEN* a thing\n<!-- /DELTA:NEW -->\n",
        );
        write_spec(
            &specs.join("_plans/p/b/two/spec.md"),
            "<!-- DELTA:NEW -->\n<!-- DELTA:CHANGED -->\n### Scenario: Login\n\n* *GIVEN* a thing\n<!-- /DELTA:CHANGED -->\n<!-- /DELTA:NEW -->\n",
        );

        let error = record_plan(specs, "p").unwrap_err();

        assert!(matches!(error, RecordError::InvalidDeltaAnchor(_)));
        assert!(error.to_string().contains("b/two/spec.md"), "{error}");
        assert!(
            error.to_string().contains("malformed delta marker"),
            "{error}"
        );
    }

    #[test]
    fn find_delta_specs_returns_paths_in_a_stable_order() {
        let tmp = TempDir::new().unwrap();
        let plan = tmp.path();
        for feature in ["c/three", "a/one", "b/two"] {
            write_spec(&plan.join(feature).join("spec.md"), SPEC);
        }

        let found = find_delta_specs(plan).unwrap();

        let mut sorted = found.clone();
        sorted.sort();
        assert_eq!(found, sorted);
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
