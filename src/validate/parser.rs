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

#[derive(Default)]
struct ParseContext {
    state: ParseState,
    current_scenario: Option<Scenario>,
    current_step_kind: Option<StepKind>,
    current_step_text: String,
    heading_text: String,
    description_buffer: String,
    in_list_item: bool,
}

pub fn parse(content: &str) -> Result<FeatureSpec, ValidationError> {
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

    Ok(spec)
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
    match text.trim() {
        "GIVEN" => ctx.current_step_kind = Some(StepKind::Given),
        "WHEN" => ctx.current_step_kind = Some(StepKind::When),
        "THEN" => ctx.current_step_kind = Some(StepKind::Then),
        "AND" => ctx.current_step_kind = Some(StepKind::And),
        _ => ctx.current_step_text.push_str(text),
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
        let spec = parse(md).unwrap();
        assert_eq!(spec.feature_name, Some("User Login".to_string()));
    }

    #[test]
    fn parses_feature_description() {
        let md = "# Feature: User Login\n\nThis is the description.\n\n## Background";
        let spec = parse(md).unwrap();
        assert_eq!(
            spec.description,
            Some("This is the description.".to_string())
        );
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
