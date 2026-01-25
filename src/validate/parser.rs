use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::validate::report::ValidationError;

#[derive(Debug, Default)]
pub struct FeatureSpec {
    pub feature_name: Option<String>,
    pub description: Option<String>,
    pub has_background: bool,
    pub has_scenarios_section: bool,
    pub scenarios: Vec<Scenario>,
}

#[derive(Debug)]
pub struct Scenario {
    pub name: String,
    pub steps: Vec<Step>,
}

#[derive(Debug)]
pub struct Step {
    pub kind: StepKind,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepKind {
    Given,
    When,
    Then,
    And,
}

#[derive(Debug, Default)]
enum ParseState {
    #[default]
    Start,
    InFeatureHeading,
    AfterFeatureHeading,
    InH2Heading,
    InScenarioHeading,
    InScenario,
    InListItem,
    InEmphasis,
}

pub fn parse(content: &str) -> Result<FeatureSpec, ValidationError> {
    let parser = Parser::new(content);
    let mut spec = FeatureSpec::default();
    let mut state = ParseState::Start;
    let mut current_scenario: Option<Scenario> = None;
    let mut current_step_kind: Option<StepKind> = None;
    let mut current_step_text = String::new();
    let mut heading_text = String::new();
    let mut description_buffer = String::new();
    let mut in_list_item = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                // Save any pending scenario
                if let Some(scenario) = current_scenario.take() {
                    spec.scenarios.push(scenario);
                }

                match level {
                    HeadingLevel::H1 => {
                        state = ParseState::InFeatureHeading;
                        heading_text.clear();
                    }
                    HeadingLevel::H2 => {
                        state = ParseState::InH2Heading;
                        heading_text.clear();
                    }
                    HeadingLevel::H3 => {
                        state = ParseState::InScenarioHeading;
                        heading_text.clear();
                    }
                    _ => {}
                }
            }

            Event::Text(text) => match state {
                ParseState::InFeatureHeading
                | ParseState::InH2Heading
                | ParseState::InScenarioHeading => {
                    heading_text.push_str(&text);
                }
                ParseState::AfterFeatureHeading => {
                    if !in_list_item {
                        description_buffer.push_str(&text);
                    }
                }
                ParseState::InEmphasis => {
                    let trimmed = text.trim();
                    match trimmed {
                        "GIVEN" => current_step_kind = Some(StepKind::Given),
                        "WHEN" => current_step_kind = Some(StepKind::When),
                        "THEN" => current_step_kind = Some(StepKind::Then),
                        "AND" => current_step_kind = Some(StepKind::And),
                        _ => current_step_text.push_str(&text),
                    }
                }
                ParseState::InListItem => {
                    current_step_text.push_str(&text);
                }
                ParseState::InScenario => {
                    // Text outside list items in scenario
                }
                _ => {}
            },

            Event::End(TagEnd::Heading(_)) => match state {
                ParseState::InFeatureHeading => {
                    let trimmed = heading_text.trim();
                    if let Some(name) = trimmed.strip_prefix("Feature:") {
                        spec.feature_name = Some(name.trim().to_string());
                    } else if let Some(name) = trimmed.strip_prefix("Feature") {
                        spec.feature_name = Some(name.trim().to_string());
                    }
                    state = ParseState::AfterFeatureHeading;
                }
                ParseState::InH2Heading => {
                    let trimmed = heading_text.trim();
                    if trimmed == "Background" {
                        spec.has_background = true;
                    } else if trimmed == "Scenarios" {
                        spec.has_scenarios_section = true;
                    }
                    state = ParseState::AfterFeatureHeading;
                }
                ParseState::InScenarioHeading => {
                    let trimmed = heading_text.trim();
                    let name = trimmed
                        .strip_prefix("Scenario:")
                        .map(|s| s.trim())
                        .unwrap_or(trimmed);
                    current_scenario = Some(Scenario {
                        name: name.to_string(),
                        steps: Vec::new(),
                    });
                    state = ParseState::InScenario;
                }
                _ => {}
            },

            Event::Start(Tag::Item) => {
                in_list_item = true;
                if matches!(state, ParseState::InScenario | ParseState::InListItem) {
                    state = ParseState::InListItem;
                    current_step_kind = None;
                    current_step_text.clear();
                }
            }

            Event::End(TagEnd::Item) => {
                in_list_item = false;
                if let (Some(kind), Some(scenario)) =
                    (current_step_kind.take(), current_scenario.as_mut())
                {
                    scenario.steps.push(Step {
                        kind,
                        text: current_step_text.trim().to_string(),
                    });
                }
                current_step_text.clear();
                if matches!(state, ParseState::InListItem) {
                    state = ParseState::InScenario;
                }
            }

            Event::Start(Tag::Emphasis) => {
                if matches!(state, ParseState::InListItem) {
                    state = ParseState::InEmphasis;
                }
            }

            Event::End(TagEnd::Emphasis) => {
                if matches!(state, ParseState::InEmphasis) {
                    state = ParseState::InListItem;
                }
            }

            Event::End(TagEnd::Paragraph) => {
                if matches!(state, ParseState::AfterFeatureHeading) && !description_buffer.is_empty()
                {
                    spec.description = Some(description_buffer.trim().to_string());
                    description_buffer.clear();
                }
            }

            _ => {}
        }
    }

    // Save final scenario
    if let Some(scenario) = current_scenario {
        spec.scenarios.push(scenario);
    }

    Ok(spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_feature_heading() {
        let md = "# Feature: User Login\n\nDescription here";
        let spec = parse(md).unwrap();
        assert_eq!(spec.feature_name, Some("User Login".to_string()));
    }

    #[test]
    fn parses_feature_description() {
        let md = "# Feature: User Login\n\nThis is the description.\n\n## Background";
        let spec = parse(md).unwrap();
        assert_eq!(spec.description, Some("This is the description.".to_string()));
    }

    #[test]
    fn parses_background_section() {
        let md = "# Feature: Test\n\nDesc\n\n## Background\n\nSome background";
        let spec = parse(md).unwrap();
        assert!(spec.has_background);
    }

    #[test]
    fn parses_scenarios_section() {
        let md = "# Feature: Test\n\nDesc\n\n## Background\n\n## Scenarios\n\n### Scenario: First";
        let spec = parse(md).unwrap();
        assert!(spec.has_scenarios_section);
    }

    #[test]
    fn parses_scenario_name() {
        let md = r#"# Feature: Test

Desc

## Background

## Scenarios

### Scenario: User logs in

* *GIVEN* a user exists
"#;
        let spec = parse(md).unwrap();
        assert_eq!(spec.scenarios.len(), 1);
        assert_eq!(spec.scenarios[0].name, "User logs in");
    }

    #[test]
    fn parses_given_step() {
        let md = r#"# Feature: Test

Desc

## Background

## Scenarios

### Scenario: Test

* *GIVEN* a precondition
"#;
        let spec = parse(md).unwrap();
        assert_eq!(spec.scenarios[0].steps.len(), 1);
        assert_eq!(spec.scenarios[0].steps[0].kind, StepKind::Given);
        assert_eq!(spec.scenarios[0].steps[0].text, "a precondition");
    }

    #[test]
    fn parses_when_step() {
        let md = r#"# Feature: Test

Desc

## Background

## Scenarios

### Scenario: Test

* *WHEN* something happens
"#;
        let spec = parse(md).unwrap();
        assert_eq!(spec.scenarios[0].steps[0].kind, StepKind::When);
    }

    #[test]
    fn parses_then_step() {
        let md = r#"# Feature: Test

Desc

## Background

## Scenarios

### Scenario: Test

* *THEN* the system SHALL respond
"#;
        let spec = parse(md).unwrap();
        assert_eq!(spec.scenarios[0].steps[0].kind, StepKind::Then);
    }

    #[test]
    fn parses_and_step() {
        let md = r#"# Feature: Test

Desc

## Background

## Scenarios

### Scenario: Test

* *GIVEN* first
* *AND* second
"#;
        let spec = parse(md).unwrap();
        assert_eq!(spec.scenarios[0].steps.len(), 2);
        assert_eq!(spec.scenarios[0].steps[1].kind, StepKind::And);
    }

    #[test]
    fn parses_multiple_scenarios() {
        let md = r#"# Feature: Test

Desc

## Background

## Scenarios

### Scenario: First

* *GIVEN* something

### Scenario: Second

* *WHEN* another thing
"#;
        let spec = parse(md).unwrap();
        assert_eq!(spec.scenarios.len(), 2);
        assert_eq!(spec.scenarios[0].name, "First");
        assert_eq!(spec.scenarios[1].name, "Second");
    }

    #[test]
    fn handles_empty_content() {
        let spec = parse("").unwrap();
        assert!(spec.feature_name.is_none());
        assert!(spec.description.is_none());
        assert!(!spec.has_background);
        assert!(spec.scenarios.is_empty());
    }
}
