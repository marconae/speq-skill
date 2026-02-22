use crate::validate::parser::{FeatureSpec, Scenario, StepKind};
use crate::validate::report::{ValidationError, ValidationResult, ValidationWarning};

const RFC2119_KEYWORDS: &[&str] = &[
    "MUST",
    "MUST NOT",
    "SHALL",
    "SHALL NOT",
    "SHOULD",
    "SHOULD NOT",
    "MAY",
];

pub fn validate(spec: &FeatureSpec) -> ValidationResult {
    let mut result = ValidationResult::new();

    validate_document_structure(spec, &mut result);

    for scenario in &spec.scenarios {
        validate_scenario(scenario, &mut result);
    }

    result
}

fn validate_document_structure(spec: &FeatureSpec, result: &mut ValidationResult) {
    if spec.description.is_none()
        || spec
            .description
            .as_ref()
            .is_some_and(|d| d.trim().is_empty())
    {
        result.add_error(ValidationError::MissingFeatureDescription);
    }

    if !spec.has_background {
        result.add_error(ValidationError::MissingBackgroundSection);
    }

    if !spec.has_scenarios_section {
        result.add_error(ValidationError::MissingScenariosSection);
    }

    if spec.scenarios.is_empty() {
        result.add_error(ValidationError::NoScenarios);
    }
}

fn validate_scenario(scenario: &Scenario, result: &mut ValidationResult) {
    let has_given = scenario
        .steps
        .iter()
        .any(|s| matches!(s.kind, StepKind::Given));
    let has_when = scenario
        .steps
        .iter()
        .any(|s| matches!(s.kind, StepKind::When));
    let has_then = scenario
        .steps
        .iter()
        .any(|s| matches!(s.kind, StepKind::Then));

    if !has_given {
        result.add_error(ValidationError::ScenarioMissingGiven {
            scenario: scenario.name.clone(),
        });
    }

    if !has_when {
        result.add_error(ValidationError::ScenarioMissingWhen {
            scenario: scenario.name.clone(),
        });
    }

    if !has_then {
        result.add_error(ValidationError::ScenarioMissingThen {
            scenario: scenario.name.clone(),
        });
    }

    // Check RFC 2119 keywords in THEN steps (and AND steps following THEN)
    let mut in_then_section = false;
    for step in &scenario.steps {
        match step.kind {
            StepKind::Then => {
                in_then_section = true;
                check_rfc2119_in_step(&step.text, &scenario.name, result);
            }
            StepKind::And if in_then_section => {
                check_rfc2119_in_step(&step.text, &scenario.name, result);
            }
            StepKind::Given | StepKind::When => {
                in_then_section = false;
            }
            _ => {}
        }
    }

    // Check for too many AND steps
    let and_count = scenario
        .steps
        .iter()
        .filter(|s| matches!(s.kind, StepKind::And))
        .count();
    if and_count > 3 {
        result.add_warning(ValidationWarning::TooManyAndSteps {
            scenario: scenario.name.clone(),
            count: and_count,
        });
    }
}

fn check_rfc2119_in_step(step_text: &str, scenario_name: &str, result: &mut ValidationResult) {
    // Check for uppercase RFC 2119 keyword first
    if contains_rfc2119_keyword(step_text) {
        // Also check if there's a lowercase version alongside (we warn about it)
        if let Some(keyword) = find_lowercase_rfc2119_keyword(step_text) {
            result.add_warning(ValidationWarning::LowercaseRfcKeyword {
                keyword,
                step: step_text.to_string(),
            });
        }
        return;
    }

    // No uppercase keyword found, check for lowercase version
    if let Some(keyword) = find_lowercase_rfc2119_keyword(step_text) {
        // Lowercase keyword found - this counts as having a keyword, but warn
        result.add_warning(ValidationWarning::LowercaseRfcKeyword {
            keyword,
            step: step_text.to_string(),
        });
    } else {
        // No RFC 2119 keyword at all - this is an error
        result.add_error(ValidationError::StepMissingRfc2119Keyword {
            scenario: scenario_name.to_string(),
            step: step_text.to_string(),
        });
    }
}

fn is_word_boundary(text: &str, pos: usize) -> bool {
    if pos == 0 || pos >= text.len() {
        return true;
    }
    let bytes = text.as_bytes();
    let before = bytes[pos - 1].is_ascii_alphanumeric();
    let after = bytes[pos].is_ascii_alphanumeric();
    before != after
}

fn contains_rfc2119_keyword(text: &str) -> bool {
    RFC2119_KEYWORDS.iter().any(|kw| {
        let mut start = 0;
        while let Some(pos) = text[start..].find(kw) {
            let abs_pos = start + pos;
            let end_pos = abs_pos + kw.len();
            if is_word_boundary(text, abs_pos) && is_word_boundary(text, end_pos) {
                return true;
            }
            start = abs_pos + 1;
        }
        false
    })
}

fn find_lowercase_rfc2119_keyword(text: &str) -> Option<String> {
    let lowercase_patterns = [
        ("must not", "MUST NOT"),
        ("shall not", "SHALL NOT"),
        ("should not", "SHOULD NOT"),
        ("must", "MUST"),
        ("shall", "SHALL"),
        ("should", "SHOULD"),
        ("may", "MAY"),
    ];

    let text_lower = text.to_lowercase();

    for (pattern, uppercase) in lowercase_patterns {
        let mut start = 0;
        while let Some(pos) = text_lower[start..].find(pattern) {
            let abs_pos = start + pos;
            let end_pos = abs_pos + pattern.len();

            if is_word_boundary(&text_lower, abs_pos) && is_word_boundary(&text_lower, end_pos) {
                let actual = &text[abs_pos..end_pos];
                if actual != uppercase {
                    return Some(actual.to_string());
                }
            }

            start = abs_pos + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::parser::Step;

    fn valid_spec() -> FeatureSpec {
        FeatureSpec {
            feature_name: Some("Test".to_string()),
            description: Some("Description".to_string()),
            has_background: true,
            has_scenarios_section: true,
            scenarios: vec![Scenario {
                name: "Test scenario".to_string(),
                steps: vec![
                    Step {
                        kind: StepKind::Given,
                        text: "a precondition".to_string(),
                    },
                    Step {
                        kind: StepKind::When,
                        text: "an action".to_string(),
                    },
                    Step {
                        kind: StepKind::Then,
                        text: "the system SHALL respond".to_string(),
                    },
                ],
            }],
        }
    }

    #[test]
    fn valid_spec_passes() {
        let spec = valid_spec();
        let result = validate(&spec);
        assert!(result.is_success());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn error_when_missing_description() {
        let mut spec = valid_spec();
        spec.description = None;
        let result = validate(&spec);
        assert!(
            result
                .errors
                .contains(&ValidationError::MissingFeatureDescription)
        );
    }

    #[test]
    fn error_when_empty_description() {
        let mut spec = valid_spec();
        spec.description = Some("   ".to_string());
        let result = validate(&spec);
        assert!(
            result
                .errors
                .contains(&ValidationError::MissingFeatureDescription)
        );
    }

    #[test]
    fn error_when_missing_background() {
        let mut spec = valid_spec();
        spec.has_background = false;
        let result = validate(&spec);
        assert!(
            result
                .errors
                .contains(&ValidationError::MissingBackgroundSection)
        );
    }

    #[test]
    fn error_when_missing_scenarios_section() {
        let mut spec = valid_spec();
        spec.has_scenarios_section = false;
        let result = validate(&spec);
        assert!(
            result
                .errors
                .contains(&ValidationError::MissingScenariosSection)
        );
    }

    #[test]
    fn error_when_no_scenarios() {
        let mut spec = valid_spec();
        spec.scenarios.clear();
        let result = validate(&spec);
        assert!(result.errors.contains(&ValidationError::NoScenarios));
    }

    #[test]
    fn error_when_scenario_missing_given() {
        let mut spec = valid_spec();
        spec.scenarios[0]
            .steps
            .retain(|s| s.kind != StepKind::Given);
        let result = validate(&spec);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            ValidationError::ScenarioMissingGiven { scenario } if scenario == "Test scenario"
        )));
    }

    #[test]
    fn error_when_scenario_missing_when() {
        let mut spec = valid_spec();
        spec.scenarios[0].steps.retain(|s| s.kind != StepKind::When);
        let result = validate(&spec);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            ValidationError::ScenarioMissingWhen { scenario } if scenario == "Test scenario"
        )));
    }

    #[test]
    fn error_when_scenario_missing_then() {
        let mut spec = valid_spec();
        spec.scenarios[0].steps.retain(|s| s.kind != StepKind::Then);
        let result = validate(&spec);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            ValidationError::ScenarioMissingThen { scenario } if scenario == "Test scenario"
        )));
    }

    #[test]
    fn error_when_then_step_missing_rfc2119_keyword() {
        let mut spec = valid_spec();
        spec.scenarios[0].steps[2].text = "something happens".to_string();
        let result = validate(&spec);
        assert!(
            result
                .errors
                .iter()
                .any(|e| matches!(e, ValidationError::StepMissingRfc2119Keyword { .. }))
        );
    }

    #[test]
    fn accepts_must_keyword() {
        let mut spec = valid_spec();
        spec.scenarios[0].steps[2].text = "the system MUST respond".to_string();
        let result = validate(&spec);
        assert!(
            !result
                .errors
                .iter()
                .any(|e| matches!(e, ValidationError::StepMissingRfc2119Keyword { .. }))
        );
    }

    #[test]
    fn accepts_should_keyword() {
        let mut spec = valid_spec();
        spec.scenarios[0].steps[2].text = "the system SHOULD respond".to_string();
        let result = validate(&spec);
        assert!(
            !result
                .errors
                .iter()
                .any(|e| matches!(e, ValidationError::StepMissingRfc2119Keyword { .. }))
        );
    }

    #[test]
    fn accepts_may_keyword() {
        let mut spec = valid_spec();
        spec.scenarios[0].steps[2].text = "the system MAY respond".to_string();
        let result = validate(&spec);
        assert!(
            !result
                .errors
                .iter()
                .any(|e| matches!(e, ValidationError::StepMissingRfc2119Keyword { .. }))
        );
    }

    #[test]
    fn warns_on_lowercase_rfc_keyword() {
        let mut spec = valid_spec();
        spec.scenarios[0].steps[2].text = "the system shall respond".to_string();
        let result = validate(&spec);
        // Should be a warning, not an error
        assert!(result.is_success());
        assert!(result.warnings.iter().any(|w| matches!(
            w,
            ValidationWarning::LowercaseRfcKeyword { keyword, .. } if keyword == "shall"
        )));
    }

    #[test]
    fn warns_on_lowercase_must_keyword() {
        let mut spec = valid_spec();
        spec.scenarios[0].steps[2].text = "the system must respond".to_string();
        let result = validate(&spec);
        assert!(result.is_success());
        assert!(result.warnings.iter().any(|w| matches!(
            w,
            ValidationWarning::LowercaseRfcKeyword { keyword, .. } if keyword == "must"
        )));
    }

    #[test]
    fn no_warning_for_uppercase_rfc_keywords() {
        let spec = valid_spec();
        let result = validate(&spec);
        assert!(result.is_success());
        assert!(
            result
                .warnings
                .iter()
                .all(|w| !matches!(w, ValidationWarning::LowercaseRfcKeyword { .. }))
        );
    }

    #[test]
    fn warning_when_more_than_three_and_steps() {
        let mut spec = valid_spec();
        for i in 0..4 {
            spec.scenarios[0].steps.push(Step {
                kind: StepKind::And,
                text: format!("the system SHALL do thing {i}"),
            });
        }
        let result = validate(&spec);
        assert!(result.warnings.iter().any(|w| matches!(
            w,
            ValidationWarning::TooManyAndSteps { count, .. } if *count == 4
        )));
    }

    #[test]
    fn no_warning_when_three_or_fewer_and_steps() {
        let mut spec = valid_spec();
        for i in 0..3 {
            spec.scenarios[0].steps.push(Step {
                kind: StepKind::And,
                text: format!("the system SHALL do thing {i}"),
            });
        }
        let result = validate(&spec);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn no_false_positive_must_substring() {
        let text = "mustard is good";
        assert!(
            find_lowercase_rfc2119_keyword(text).is_none(),
            "should not flag 'must' inside 'mustard'"
        );
    }

    #[test]
    fn no_false_positive_may_substring() {
        let text = "maybe the system responds";
        assert!(
            find_lowercase_rfc2119_keyword(text).is_none(),
            "should not flag 'may' inside 'maybe'"
        );
    }

    #[test]
    fn no_false_positive_should_substring() {
        let text = "shoulder the burden";
        assert!(
            find_lowercase_rfc2119_keyword(text).is_none(),
            "should not flag 'should' inside 'shoulder'"
        );
    }

    #[test]
    fn no_false_positive_note_not_not() {
        let text = "SHALL note that sources will handle this";
        assert!(
            find_lowercase_rfc2119_keyword(text).is_none(),
            "should not flag 'not' inside 'note'"
        );
    }

    #[test]
    fn no_false_positive_contains_uppercase_keyword_as_substring() {
        assert!(
            !contains_rfc2119_keyword("MUSTard is good"),
            "should not match MUST inside MUSTard"
        );
    }
}
