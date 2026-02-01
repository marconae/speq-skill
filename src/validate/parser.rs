use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::validate::report::{ValidationError, ValidationWarning};

#[derive(Debug)]
pub struct ParseResult {
    pub spec: FeatureSpec,
    pub warnings: Vec<ValidationWarning>,
}

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

#[derive(Default)]
struct ParseContext {
    state: ParseState,
    current_scenario: Option<Scenario>,
    current_step_kind: Option<StepKind>,
    current_step_text: String,
    heading_text: String,
    description_buffer: String,
    in_list_item: bool,
    warnings: Vec<ValidationWarning>,
}

pub fn parse(content: &str) -> Result<ParseResult, ValidationError> {
    let parser = Parser::new(content);
    let mut spec = FeatureSpec::default();
    let mut ctx = ParseContext::default();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                handle_heading_start(&mut spec, &mut ctx, level);
            }
            Event::Text(text) => {
                handle_text(&mut ctx, &text);
            }
            Event::End(TagEnd::Heading(_)) => {
                handle_heading_end(&mut spec, &mut ctx);
            }
            Event::Start(Tag::Item) => {
                handle_item_start(&mut ctx);
            }
            Event::End(TagEnd::Item) => {
                handle_item_end(&mut ctx);
            }
            Event::Start(Tag::Emphasis) => {
                if matches!(ctx.state, ParseState::InListItem) {
                    ctx.state = ParseState::InEmphasis;
                }
            }
            Event::End(TagEnd::Emphasis) => {
                if matches!(ctx.state, ParseState::InEmphasis) {
                    ctx.state = ParseState::InListItem;
                }
            }
            Event::End(TagEnd::Paragraph) => {
                handle_paragraph_end(&mut spec, &mut ctx);
            }
            _ => {}
        }
    }

    if let Some(scenario) = ctx.current_scenario {
        spec.scenarios.push(scenario);
    }

    Ok(ParseResult {
        spec,
        warnings: ctx.warnings,
    })
}

fn handle_heading_start(spec: &mut FeatureSpec, ctx: &mut ParseContext, level: HeadingLevel) {
    if let Some(scenario) = ctx.current_scenario.take() {
        spec.scenarios.push(scenario);
    }

    ctx.heading_text.clear();
    ctx.state = match level {
        HeadingLevel::H1 => ParseState::InFeatureHeading,
        HeadingLevel::H2 => ParseState::InH2Heading,
        HeadingLevel::H3 => ParseState::InScenarioHeading,
        _ => return,
    };
}

fn handle_text(ctx: &mut ParseContext, text: &str) {
    match ctx.state {
        ParseState::InFeatureHeading | ParseState::InH2Heading | ParseState::InScenarioHeading => {
            ctx.heading_text.push_str(text);
        }
        ParseState::AfterFeatureHeading => {
            if !ctx.in_list_item {
                ctx.description_buffer.push_str(text);
            }
        }
        ParseState::InEmphasis => {
            handle_emphasis_text(ctx, text);
        }
        ParseState::InListItem => {
            ctx.current_step_text.push_str(text);
        }
        _ => {}
    }
}

fn handle_emphasis_text(ctx: &mut ParseContext, text: &str) {
    let trimmed = text.trim();

    // Only look for step keywords if we haven't found one yet for this step
    // This prevents emphasized text within step content from being misinterpreted
    if ctx.current_step_kind.is_some() {
        ctx.current_step_text.push_str(text);
        return;
    }

    let step_kind = match trimmed {
        "GIVEN" => Some((StepKind::Given, false)),
        "WHEN" => Some((StepKind::When, false)),
        "THEN" => Some((StepKind::Then, false)),
        "AND" => Some((StepKind::And, false)),
        _ => match trimmed.to_uppercase().as_str() {
            "GIVEN" => Some((StepKind::Given, true)),
            "WHEN" => Some((StepKind::When, true)),
            "THEN" => Some((StepKind::Then, true)),
            "AND" => Some((StepKind::And, true)),
            _ => None,
        },
    };

    match step_kind {
        Some((kind, is_lowercase)) => {
            if is_lowercase {
                ctx.warnings.push(ValidationWarning::LowercaseStepKeyword {
                    keyword: trimmed.to_string(),
                });
            }
            ctx.current_step_kind = Some(kind);
        }
        None => ctx.current_step_text.push_str(text),
    }
}

fn handle_heading_end(spec: &mut FeatureSpec, ctx: &mut ParseContext) {
    let trimmed = ctx.heading_text.trim();

    match ctx.state {
        ParseState::InFeatureHeading => {
            spec.feature_name = trimmed
                .strip_prefix("Feature:")
                .or_else(|| trimmed.strip_prefix("Feature"))
                .map(|s| s.trim().to_string());
            ctx.state = ParseState::AfterFeatureHeading;
        }
        ParseState::InH2Heading => {
            match trimmed {
                "Background" => spec.has_background = true,
                "Scenarios" => spec.has_scenarios_section = true,
                _ => {}
            }
            ctx.state = ParseState::AfterFeatureHeading;
        }
        ParseState::InScenarioHeading => {
            let name = trimmed
                .strip_prefix("Scenario:")
                .map(|s| s.trim())
                .unwrap_or(trimmed);
            ctx.current_scenario = Some(Scenario {
                name: name.to_string(),
                steps: Vec::new(),
            });
            ctx.state = ParseState::InScenario;
        }
        _ => {}
    }
}

fn handle_item_start(ctx: &mut ParseContext) {
    ctx.in_list_item = true;
    if matches!(ctx.state, ParseState::InScenario | ParseState::InListItem) {
        ctx.state = ParseState::InListItem;
        ctx.current_step_kind = None;
        ctx.current_step_text.clear();
    }
}

fn handle_item_end(ctx: &mut ParseContext) {
    ctx.in_list_item = false;
    if let (Some(kind), Some(scenario)) =
        (ctx.current_step_kind.take(), ctx.current_scenario.as_mut())
    {
        scenario.steps.push(Step {
            kind,
            text: ctx.current_step_text.trim().to_string(),
        });
    }
    ctx.current_step_text.clear();
    if matches!(ctx.state, ParseState::InListItem) {
        ctx.state = ParseState::InScenario;
    }
}

fn handle_paragraph_end(spec: &mut FeatureSpec, ctx: &mut ParseContext) {
    if matches!(ctx.state, ParseState::AfterFeatureHeading) && !ctx.description_buffer.is_empty() {
        spec.description = Some(ctx.description_buffer.trim().to_string());
        ctx.description_buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_feature_heading() {
        let md = "# Feature: User Login\n\nDescription here";
        let result = parse(md).unwrap();
        assert_eq!(result.spec.feature_name, Some("User Login".to_string()));
    }

    #[test]
    fn parses_feature_description() {
        let md = "# Feature: User Login\n\nThis is the description.\n\n## Background";
        let result = parse(md).unwrap();
        assert_eq!(
            result.spec.description,
            Some("This is the description.".to_string())
        );
    }

    #[test]
    fn parses_background_section() {
        let md = "# Feature: Test\n\nDesc\n\n## Background\n\nSome background";
        let result = parse(md).unwrap();
        assert!(result.spec.has_background);
    }

    #[test]
    fn parses_scenarios_section() {
        let md = "# Feature: Test\n\nDesc\n\n## Background\n\n## Scenarios\n\n### Scenario: First";
        let result = parse(md).unwrap();
        assert!(result.spec.has_scenarios_section);
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
        let result = parse(md).unwrap();
        assert_eq!(result.spec.scenarios.len(), 1);
        assert_eq!(result.spec.scenarios[0].name, "User logs in");
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
        let result = parse(md).unwrap();
        assert_eq!(result.spec.scenarios[0].steps.len(), 1);
        assert_eq!(result.spec.scenarios[0].steps[0].kind, StepKind::Given);
        assert_eq!(result.spec.scenarios[0].steps[0].text, "a precondition");
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
        let result = parse(md).unwrap();
        assert_eq!(result.spec.scenarios[0].steps[0].kind, StepKind::When);
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
        let result = parse(md).unwrap();
        assert_eq!(result.spec.scenarios[0].steps[0].kind, StepKind::Then);
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
        let result = parse(md).unwrap();
        assert_eq!(result.spec.scenarios[0].steps.len(), 2);
        assert_eq!(result.spec.scenarios[0].steps[1].kind, StepKind::And);
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
        let result = parse(md).unwrap();
        assert_eq!(result.spec.scenarios.len(), 2);
        assert_eq!(result.spec.scenarios[0].name, "First");
        assert_eq!(result.spec.scenarios[1].name, "Second");
    }

    #[test]
    fn handles_empty_content() {
        let result = parse("").unwrap();
        assert!(result.spec.feature_name.is_none());
        assert!(result.spec.description.is_none());
        assert!(!result.spec.has_background);
        assert!(result.spec.scenarios.is_empty());
    }

    #[test]
    fn warns_on_lowercase_step_keyword() {
        let md = r#"# Feature: Test

Desc

## Background

## Scenarios

### Scenario: Test

* *given* a precondition
"#;
        let result = parse(md).unwrap();
        assert_eq!(result.spec.scenarios[0].steps.len(), 1);
        assert_eq!(result.spec.scenarios[0].steps[0].kind, StepKind::Given);
        assert!(result.warnings.iter().any(|w| matches!(
            w,
            ValidationWarning::LowercaseStepKeyword { keyword } if keyword == "given"
        )));
    }

    #[test]
    fn warns_on_lowercase_when_keyword() {
        let md = r#"# Feature: Test

Desc

## Background

## Scenarios

### Scenario: Test

* *when* something happens
"#;
        let result = parse(md).unwrap();
        assert_eq!(result.spec.scenarios[0].steps[0].kind, StepKind::When);
        assert!(result.warnings.iter().any(|w| matches!(
            w,
            ValidationWarning::LowercaseStepKeyword { keyword } if keyword == "when"
        )));
    }

    #[test]
    fn no_warning_for_uppercase_step_keywords() {
        let md = r#"# Feature: Test

Desc

## Background

## Scenarios

### Scenario: Test

* *GIVEN* a precondition
* *WHEN* something happens
* *THEN* the system SHALL respond
* *AND* something else
"#;
        let result = parse(md).unwrap();
        assert!(result.warnings.is_empty());
    }
}
