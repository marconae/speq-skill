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
                if !contains_rfc2119_keyword(&step.text) {
                    result.add_error(ValidationError::StepMissingRfc2119Keyword {
                        scenario: scenario.name.clone(),
                        step: step.text.clone(),
                    });
                }
            }
            StepKind::And if in_then_section => {
                if !contains_rfc2119_keyword(&step.text) {
                    result.add_error(ValidationError::StepMissingRfc2119Keyword {
                        scenario: scenario.name.clone(),
                        step: step.text.clone(),
                    });
                }
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

fn contains_rfc2119_keyword(text: &str) -> bool {
    RFC2119_KEYWORDS.iter().any(|kw| text.contains(kw))
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
    fn rejects_lowercase_keyword() {
        let mut spec = valid_spec();
        spec.scenarios[0].steps[2].text = "the system shall respond".to_string();
        let result = validate(&spec);
        assert!(
            result
                .errors
                .iter()
                .any(|e| matches!(e, ValidationError::StepMissingRfc2119Keyword { .. }))
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
}
