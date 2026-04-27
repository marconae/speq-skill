use thiserror::Error;

#[derive(Debug)]
pub struct DecisionLogValidationResult {
    pub errors: Vec<DecisionLogError>,
    pub warnings: Vec<DecisionLogWarning>,
}

impl DecisionLogValidationResult {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn add_error(&mut self, error: DecisionLogError) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: DecisionLogWarning) {
        self.warnings.push(warning);
    }
}

impl Default for DecisionLogValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum DecisionLogError {
    #[error("Missing H1 heading '# Decision Log: {plan_name}'")]
    PlanLogMissingH1 { plan_name: String },

    #[error(
        "H1 heading does not match plan name: expected '# Decision Log: {expected}', found '{found}'"
    )]
    PlanLogH1Mismatch { expected: String, found: String },

    #[error("Missing 'Date:' line")]
    PlanLogMissingDate,

    #[error(
        "No valid sections found (expected ## Interview, ## Design Decisions, or ## Review Findings)"
    )]
    PlanLogNoSections,

    #[error("Missing H1 heading '# Architecture Decision Records'")]
    PermanentLogMissingH1,

    #[error("ADR numbers must start at 001, found ADR-{first:03}")]
    PermanentLogNotStartAt001 { first: u32 },

    #[error("Non-sequential ADR numbers: ADR-{prev:03} followed by ADR-{next:03}")]
    PermanentLogNonSequential { prev: u32, next: u32 },

    #[error("ADR-{number:03} missing required field: {field}")]
    PermanentLogMissingField { number: u32, field: String },

    #[error("ADR-{number:03} has invalid status '{status}'")]
    PermanentLogInvalidStatus { number: u32, status: String },
}

#[derive(Debug, PartialEq)]
pub enum DecisionLogWarning {
    PlanLogInvalidPromotesValue { value: String },
}

impl std::fmt::Display for DecisionLogWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionLogWarning::PlanLogInvalidPromotesValue { value } => {
                write!(
                    f,
                    "Invalid 'Promotes to ADR' value '{value}' (expected 'yes' or 'no')"
                )
            }
        }
    }
}

const VALID_PLAN_LOG_SECTIONS: &[&str] =
    &["## Interview", "## Design Decisions", "## Review Findings"];

const PROMOTES_TO_ADR_PREFIX: &str = "Promotes to ADR:";

pub fn validate_plan_log(content: &str, plan_name: &str) -> DecisionLogValidationResult {
    let mut result = DecisionLogValidationResult::new();
    let expected_h1 = format!("# Decision Log: {plan_name}");

    let mut h1_found = false;
    let mut date_found = false;
    let mut section_found = false;

    for raw_line in content.lines() {
        let line = raw_line.trim_end();

        if !h1_found && is_h1_line(line) {
            h1_found = true;
            if line.trim() != expected_h1 {
                result.add_error(DecisionLogError::PlanLogH1Mismatch {
                    expected: expected_h1.clone(),
                    found: line.trim().to_string(),
                });
            }
            continue;
        }

        if !date_found && line.trim_start().starts_with("Date:") {
            date_found = true;
        }

        if is_plan_log_section(line) {
            section_found = true;
        }

        if let Some(value) = extract_promotes_value(line) {
            let normalized = value.trim().to_lowercase();
            if normalized != "yes" && normalized != "no" {
                result.add_warning(DecisionLogWarning::PlanLogInvalidPromotesValue {
                    value: value.trim().to_string(),
                });
            }
        }
    }

    if !h1_found {
        result.add_error(DecisionLogError::PlanLogMissingH1 {
            plan_name: plan_name.to_string(),
        });
    }

    if !date_found {
        result.add_error(DecisionLogError::PlanLogMissingDate);
    }

    if !section_found {
        result.add_error(DecisionLogError::PlanLogNoSections);
    }

    result
}

pub fn validate_permanent_log(content: &str) -> DecisionLogValidationResult {
    let mut result = DecisionLogValidationResult::new();
    let expected_h1 = "# Architecture Decision Records";

    let mut h1_found = false;
    for raw_line in content.lines() {
        let line = raw_line.trim_end();
        if is_h1_line(line) {
            h1_found = true;
            if line.trim() != expected_h1 {
                result.add_error(DecisionLogError::PermanentLogMissingH1);
            }
            break;
        }
    }
    if !h1_found {
        result.add_error(DecisionLogError::PermanentLogMissingH1);
    }

    let blocks = collect_adr_blocks(content);

    validate_adr_numbering(&blocks, &mut result);

    for block in &blocks {
        validate_adr_block(block, &mut result);
    }

    result
}

struct AdrBlock<'a> {
    number: u32,
    lines: Vec<&'a str>,
}

fn collect_adr_blocks(content: &str) -> Vec<AdrBlock<'_>> {
    let mut blocks: Vec<AdrBlock<'_>> = Vec::new();
    let mut current: Option<AdrBlock<'_>> = None;

    for raw_line in content.lines() {
        let trimmed = raw_line.trim_end();
        if let Some(number) = parse_adr_heading(trimmed) {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
            current = Some(AdrBlock {
                number,
                lines: vec![trimmed],
            });
        } else if let Some(block) = current.as_mut() {
            block.lines.push(trimmed);
        }
    }

    if let Some(block) = current.take() {
        blocks.push(block);
    }

    blocks
}

fn validate_adr_numbering(blocks: &[AdrBlock<'_>], result: &mut DecisionLogValidationResult) {
    if blocks.is_empty() {
        return;
    }

    let first = blocks[0].number;
    if first != 1 {
        result.add_error(DecisionLogError::PermanentLogNotStartAt001 { first });
    }

    for window in blocks.windows(2) {
        let prev = window[0].number;
        let next = window[1].number;
        if next != prev + 1 {
            result.add_error(DecisionLogError::PermanentLogNonSequential { prev, next });
        }
    }
}

fn validate_adr_block(block: &AdrBlock<'_>, result: &mut DecisionLogValidationResult) {
    let mut has_date = false;
    let mut has_plan = false;
    let mut has_status = false;
    let mut status_value: Option<String> = None;
    let mut has_context_heading = false;
    let mut has_decision_heading = false;

    for line in &block.lines {
        let trimmed = line.trim();
        if extract_bold_field(trimmed, "Date").is_some() {
            has_date = true;
        }
        if extract_bold_field(trimmed, "Plan").is_some() {
            has_plan = true;
        }
        if let Some(value) = extract_bold_field(trimmed, "Status") {
            has_status = true;
            status_value = Some(value.trim().to_string());
        }
        if trimmed == "### Context" {
            has_context_heading = true;
        }
        if trimmed == "### Decision" {
            has_decision_heading = true;
        }
    }

    if !has_date {
        result.add_error(DecisionLogError::PermanentLogMissingField {
            number: block.number,
            field: "**Date:**".to_string(),
        });
    }
    if !has_plan {
        result.add_error(DecisionLogError::PermanentLogMissingField {
            number: block.number,
            field: "**Plan:**".to_string(),
        });
    }
    if !has_status {
        result.add_error(DecisionLogError::PermanentLogMissingField {
            number: block.number,
            field: "**Status:**".to_string(),
        });
    }
    if !has_context_heading {
        result.add_error(DecisionLogError::PermanentLogMissingField {
            number: block.number,
            field: "### Context".to_string(),
        });
    }
    if !has_decision_heading {
        result.add_error(DecisionLogError::PermanentLogMissingField {
            number: block.number,
            field: "### Decision".to_string(),
        });
    }

    if let Some(status) = status_value
        && !is_valid_status(&status)
    {
        result.add_error(DecisionLogError::PermanentLogInvalidStatus {
            number: block.number,
            status,
        });
    }
}

fn is_h1_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("# ") && !trimmed.starts_with("##")
}

fn is_plan_log_section(line: &str) -> bool {
    let trimmed = line.trim();
    VALID_PLAN_LOG_SECTIONS.contains(&trimmed)
}

fn extract_promotes_value(line: &str) -> Option<&str> {
    let idx = line.find(PROMOTES_TO_ADR_PREFIX)?;
    let after = &line[idx + PROMOTES_TO_ADR_PREFIX.len()..];
    let trimmed = after.trim();
    let unbolded = trimmed.strip_prefix("**").unwrap_or(trimmed);
    Some(unbolded.trim())
}

fn parse_adr_heading(line: &str) -> Option<u32> {
    let trimmed = line.trim_start();
    let rest = trimmed.strip_prefix("## ADR-")?;
    let colon_idx = rest.find(':')?;
    let digits = &rest[..colon_idx];
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    digits.parse::<u32>().ok()
}

fn extract_bold_field<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let prefix = format!("**{name}:**");
    let idx = line.find(&prefix)?;
    Some(&line[idx + prefix.len()..])
}

fn is_valid_status(status: &str) -> bool {
    let trimmed = status.trim();
    if trimmed == "Accepted" || trimmed == "Deprecated" {
        return true;
    }
    if let Some(rest) = trimmed.strip_prefix("Superseded by ADR-") {
        return !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit());
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PLAN_LOG: &str = "# Decision Log: my-plan\n\nDate: 2026-04-27\n\n## Design Decisions\n\n- **Decision:** Use line scanning.\n- **Alternatives:** AST.\n- **Rationale:** Simpler.\n- **Promotes to ADR:** yes\n";

    const VALID_PERMANENT_LOG: &str = "# Architecture Decision Records\n\n## ADR-001: Use line-oriented state machine\n\n**Date:** 2026-04-27\n**Plan:** add-decision-log-validation\n**Status:** Accepted\n\n### Context\n\nFlat markdown.\n\n### Decision\n\nLine scanning.\n\n### Options Considered\n\n- AST.\n\n### Consequences\n\n- Simpler.\n";

    #[test]
    fn valid_plan_log_passes() {
        let result = validate_plan_log(VALID_PLAN_LOG, "my-plan");
        assert!(result.is_success(), "errors: {:?}", result.errors);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn plan_log_missing_h1_fails() {
        let content = "Date: 2026-04-27\n\n## Design Decisions\n";
        let result = validate_plan_log(content, "my-plan");
        assert!(result.errors.contains(&DecisionLogError::PlanLogMissingH1 {
            plan_name: "my-plan".to_string(),
        }));
    }

    #[test]
    fn plan_log_h1_mismatch_fails() {
        let content = "# Decision Log: wrong-name\n\nDate: 2026-04-27\n\n## Design Decisions\n";
        let result = validate_plan_log(content, "my-plan");
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::PlanLogH1Mismatch { expected, found }
                if expected == "# Decision Log: my-plan" && found == "# Decision Log: wrong-name"
        )));
    }

    #[test]
    fn plan_log_missing_date_fails() {
        let content = "# Decision Log: my-plan\n\n## Design Decisions\n";
        let result = validate_plan_log(content, "my-plan");
        assert!(
            result
                .errors
                .contains(&DecisionLogError::PlanLogMissingDate)
        );
    }

    #[test]
    fn plan_log_no_sections_fails() {
        let content = "# Decision Log: my-plan\n\nDate: 2026-04-27\n";
        let result = validate_plan_log(content, "my-plan");
        assert!(result.errors.contains(&DecisionLogError::PlanLogNoSections));
    }

    #[test]
    fn plan_log_accepts_interview_section() {
        let content = "# Decision Log: my-plan\n\nDate: 2026-04-27\n\n## Interview\n\n- Q: x\n";
        let result = validate_plan_log(content, "my-plan");
        assert!(result.is_success(), "errors: {:?}", result.errors);
    }

    #[test]
    fn plan_log_accepts_review_findings_section() {
        let content =
            "# Decision Log: my-plan\n\nDate: 2026-04-27\n\n## Review Findings\n\n- finding\n";
        let result = validate_plan_log(content, "my-plan");
        assert!(result.is_success(), "errors: {:?}", result.errors);
    }

    #[test]
    fn plan_log_bad_promote_warns_not_errors() {
        let content = "# Decision Log: my-plan\n\nDate: 2026-04-27\n\n## Design Decisions\n\n- **Promotes to ADR:** maybe\n";
        let result = validate_plan_log(content, "my-plan");
        assert!(result.is_success(), "errors: {:?}", result.errors);
        assert_eq!(result.warnings.len(), 1);
        assert!(matches!(
            &result.warnings[0],
            DecisionLogWarning::PlanLogInvalidPromotesValue { value } if value == "maybe"
        ));
    }

    #[test]
    fn plan_log_promote_yes_or_no_case_insensitive() {
        let content = "# Decision Log: my-plan\n\nDate: 2026-04-27\n\n## Design Decisions\n\n- **Promotes to ADR:** YES\n- **Promotes to ADR:** No\n";
        let result = validate_plan_log(content, "my-plan");
        assert!(result.is_success());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn valid_permanent_log_passes() {
        let result = validate_permanent_log(VALID_PERMANENT_LOG);
        assert!(result.is_success(), "errors: {:?}", result.errors);
    }

    #[test]
    fn permanent_log_missing_h1_fails() {
        let content = "## ADR-001: Foo\n\n**Date:** 2026\n**Plan:** p\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(
            result
                .errors
                .contains(&DecisionLogError::PermanentLogMissingH1)
        );
    }

    #[test]
    fn permanent_log_wrong_h1_fails() {
        let content = "# Wrong Title\n\n## ADR-001: Foo\n\n**Date:** 2026\n**Plan:** p\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(
            result
                .errors
                .contains(&DecisionLogError::PermanentLogMissingH1)
        );
    }

    #[test]
    fn permanent_log_first_not_001_fails() {
        let content = "# Architecture Decision Records\n\n## ADR-002: Foo\n\n**Date:** 2026\n**Plan:** p\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(
            result
                .errors
                .contains(&DecisionLogError::PermanentLogNotStartAt001 { first: 2 })
        );
    }

    #[test]
    fn permanent_log_non_sequential_fails() {
        let content = "# Architecture Decision Records\n\n## ADR-001: A\n\n**Date:** 2026\n**Plan:** p\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n\n## ADR-003: C\n\n**Date:** 2026\n**Plan:** p\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(
            result
                .errors
                .contains(&DecisionLogError::PermanentLogNonSequential { prev: 1, next: 3 })
        );
    }

    #[test]
    fn permanent_log_missing_status_fails() {
        let content = "# Architecture Decision Records\n\n## ADR-001: A\n\n**Date:** 2026\n**Plan:** p\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::PermanentLogMissingField { number: 1, field } if field == "**Status:**"
        )));
    }

    #[test]
    fn permanent_log_missing_context_heading_fails() {
        let content = "# Architecture Decision Records\n\n## ADR-001: A\n\n**Date:** 2026\n**Plan:** p\n**Status:** Accepted\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::PermanentLogMissingField { number: 1, field } if field == "### Context"
        )));
    }

    #[test]
    fn permanent_log_invalid_status_fails() {
        let content = "# Architecture Decision Records\n\n## ADR-001: A\n\n**Date:** 2026\n**Plan:** p\n**Status:** Pending\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::PermanentLogInvalidStatus { number: 1, status } if status == "Pending"
        )));
    }

    #[test]
    fn permanent_log_status_superseded_passes() {
        let content = "# Architecture Decision Records\n\n## ADR-001: A\n\n**Date:** 2026\n**Plan:** p\n**Status:** Superseded by ADR-002\n\n### Context\n\nx\n\n### Decision\n\ny\n\n## ADR-002: B\n\n**Date:** 2026\n**Plan:** p\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(result.is_success(), "errors: {:?}", result.errors);
    }

    #[test]
    fn permanent_log_status_deprecated_passes() {
        let content = "# Architecture Decision Records\n\n## ADR-001: A\n\n**Date:** 2026\n**Plan:** p\n**Status:** Deprecated\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(result.is_success(), "errors: {:?}", result.errors);
    }

    #[test]
    fn permanent_log_optional_sections_absent_passes() {
        let content = "# Architecture Decision Records\n\n## ADR-001: A\n\n**Date:** 2026\n**Plan:** p\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_permanent_log(content);
        assert!(result.is_success(), "errors: {:?}", result.errors);
    }
}
