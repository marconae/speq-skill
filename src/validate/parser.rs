use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::validate::report::{ValidationError, ValidationWarning};

#[derive(Debug)]
pub struct ParseResult {
    pub spec: FeatureSpec,
    pub warnings: Vec<ValidationWarning>,
}

/// A parsed spec. Names keep inline code in backticks, and the description
/// and Background keep their source text, so display shows what grep finds in
/// the file. Validation reads `description_signal` and each step's
/// `normative_text`, which leave out inline code because inline code is a
/// literal value, not normative language.
#[derive(Debug, Default)]
pub struct FeatureSpec {
    pub feature_name: Option<String>,
    /// The source text between the first feature heading and the next
    /// heading, trimmed. For display only; `None` when empty.
    pub description: Option<String>,
    /// The last prose paragraph after the feature heading, outside scenarios
    /// and lists, without inline code. The missing-description check reads it
    /// in place of `description`.
    pub description_signal: Option<String>,
    /// The source text between the `## Background` heading and the next
    /// heading, trimmed. For display only; `None` when empty.
    pub background: Option<String>,
    /// Whether a `## Background` heading exists, whatever follows it.
    pub has_background: bool,
    pub has_scenarios_section: bool,
    pub scenarios: Vec<Scenario>,
}

#[derive(Debug)]
pub struct Scenario {
    pub name: String,
    pub steps: Vec<Step>,
}

/// One scenario step, kept as two texts because inline code is a literal
/// value, not normative language.
#[derive(Debug)]
pub struct Step {
    pub kind: StepKind,
    /// The step with each inline code span in single backticks. `speq feature
    /// get` and the search index read it.
    pub display_text: String,
    /// The step without its inline code spans. The RFC 2119 and keyword-casing
    /// checks read it, so a keyword inside inline code never counts.
    pub normative_text: String,
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
struct InlineText {
    with_code: String,
    without_code: String,
}

impl InlineText {
    fn push_text(&mut self, text: &str) {
        self.with_code.push_str(text);
        self.without_code.push_str(text);
    }

    fn push_code(&mut self, code: &str) {
        self.with_code.push('`');
        self.with_code.push_str(code);
        self.with_code.push('`');
    }

    fn clear(&mut self) {
        self.with_code.clear();
        self.without_code.clear();
    }
}

#[derive(Default)]
struct ParseContext {
    state: ParseState,
    current_scenario: Option<Scenario>,
    current_step_kind: Option<StepKind>,
    current_step_text: InlineText,
    heading_text: InlineText,
    description_signal_buffer: String,
    in_list_item: bool,
    warnings: Vec<ValidationWarning>,
    heading_starts: Vec<usize>,
    description_start: Option<usize>,
    background_start: Option<usize>,
}

pub fn parse(content: &str) -> Result<ParseResult, ValidationError> {
    let mut spec = FeatureSpec::default();
    let mut ctx = ParseContext::default();

    for (event, range) in Parser::new(content).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                ctx.heading_starts.push(range.start);
                handle_heading_start(&mut spec, &mut ctx, level);
            }
            Event::Text(text) => {
                handle_text(&mut ctx, &text);
            }
            Event::Code(code) => {
                handle_code(&mut ctx, &code);
            }
            Event::End(TagEnd::Heading(_)) => {
                handle_heading_end(&mut spec, &mut ctx, range.end);
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
    spec.description = section_text(content, ctx.description_start, &ctx.heading_starts);
    spec.background = section_text(content, ctx.background_start, &ctx.heading_starts);

    Ok(ParseResult {
        spec,
        warnings: ctx.warnings,
    })
}

fn section_text(content: &str, start: Option<usize>, heading_starts: &[usize]) -> Option<String> {
    let start = start?;
    let end = heading_starts
        .iter()
        .copied()
        .find(|&heading_start| heading_start >= start)
        .unwrap_or(content.len());
    let text = content[start..end].trim();
    (!text.is_empty()).then(|| text.to_string())
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
            ctx.heading_text.push_text(text);
        }
        ParseState::AfterFeatureHeading if !ctx.in_list_item => {
            ctx.description_signal_buffer.push_str(text);
        }
        ParseState::InEmphasis => {
            handle_emphasis_text(ctx, text);
        }
        ParseState::InListItem => {
            ctx.current_step_text.push_text(text);
        }
        _ => {}
    }
}

fn handle_code(ctx: &mut ParseContext, code: &str) {
    match ctx.state {
        ParseState::InFeatureHeading | ParseState::InH2Heading | ParseState::InScenarioHeading => {
            ctx.heading_text.push_code(code);
        }
        ParseState::InListItem | ParseState::InEmphasis => {
            ctx.current_step_text.push_code(code);
        }
        _ => {}
    }
}

fn handle_emphasis_text(ctx: &mut ParseContext, text: &str) {
    let trimmed = text.trim();

    // Only look for step keywords if we haven't found one yet for this step
    // This prevents emphasized text within step content from being misinterpreted
    if ctx.current_step_kind.is_some() {
        ctx.current_step_text.push_text(text);
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
        None => ctx.current_step_text.push_text(text),
    }
}

fn handle_heading_end(spec: &mut FeatureSpec, ctx: &mut ParseContext, heading_end: usize) {
    let display = ctx.heading_text.with_code.trim();

    match ctx.state {
        ParseState::InFeatureHeading => {
            spec.feature_name = display
                .strip_prefix("Feature:")
                .or_else(|| display.strip_prefix("Feature"))
                .map(|s| s.trim().to_string());
            ctx.description_start.get_or_insert(heading_end);
            ctx.state = ParseState::AfterFeatureHeading;
        }
        ParseState::InH2Heading => {
            match ctx.heading_text.without_code.trim() {
                "Background" => {
                    spec.has_background = true;
                    ctx.background_start.get_or_insert(heading_end);
                }
                "Scenarios" => spec.has_scenarios_section = true,
                _ => {}
            }
            ctx.state = ParseState::AfterFeatureHeading;
        }
        ParseState::InScenarioHeading => {
            let name = display
                .strip_prefix("Scenario:")
                .map(|s| s.trim())
                .unwrap_or(display);
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
            display_text: ctx.current_step_text.with_code.trim().to_string(),
            normative_text: ctx.current_step_text.without_code.trim().to_string(),
        });
    }
    ctx.current_step_text.clear();
    if matches!(ctx.state, ParseState::InListItem) {
        ctx.state = ParseState::InScenario;
    }
}

fn handle_paragraph_end(spec: &mut FeatureSpec, ctx: &mut ParseContext) {
    if matches!(ctx.state, ParseState::AfterFeatureHeading)
        && !ctx.description_signal_buffer.is_empty()
    {
        spec.description_signal = Some(ctx.description_signal_buffer.trim().to_string());
        ctx.description_signal_buffer.clear();
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
        assert_eq!(
            result.spec.scenarios[0].steps[0].display_text,
            "a precondition"
        );
        assert_eq!(
            result.spec.scenarios[0].steps[0].normative_text,
            "a precondition"
        );
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

    const SCENARIO_PREAMBLE: &str =
        "# Feature: Test\n\nDesc\n\n## Background\n\n## Scenarios\n\n### Scenario: Test\n\n";

    fn parse_steps(steps: &str) -> Vec<Step> {
        let mut spec = parse(&format!("{SCENARIO_PREAMBLE}{steps}")).unwrap().spec;
        spec.scenarios.remove(0).steps
    }

    #[test]
    fn step_display_text_keeps_inline_code_in_backticks() {
        let steps = parse_steps("* *GIVEN* a column of type `CHAR(10)` exists\n");
        assert_eq!(steps[0].display_text, "a column of type `CHAR(10)` exists");
    }

    #[test]
    fn step_normative_text_leaves_out_inline_code() {
        let steps = parse_steps(
            "* *THEN* the system SHALL create `must_exist.txt` and report `may-fail`\n",
        );
        assert_eq!(
            steps[0].normative_text,
            "the system SHALL create  and report"
        );
        assert_eq!(
            steps[0].display_text,
            "the system SHALL create `must_exist.txt` and report `may-fail`"
        );
    }

    #[test]
    fn step_display_text_keeps_inline_code_inside_emphasis() {
        let steps = parse_steps("* *WHEN* the user runs *`speq --version`* now\n");
        assert_eq!(steps[0].display_text, "the user runs `speq --version` now");
        assert_eq!(steps[0].normative_text, "the user runs  now");
    }

    #[test]
    fn inline_code_is_never_a_step_keyword() {
        let steps = parse_steps("* *`GIVEN`* a precondition\n");
        assert!(steps.is_empty());
    }

    #[test]
    fn step_texts_join_a_wrapped_line_without_a_space() {
        let steps = parse_steps("* *THEN* the system SHALL\n  respond with `ok`\n");
        assert_eq!(steps[0].display_text, "the system SHALLrespond with `ok`");
        assert_eq!(steps[0].normative_text, "the system SHALLrespond with");
    }

    #[test]
    fn feature_name_keeps_inline_code_in_backticks() {
        let result = parse("# Feature: `CHAR` type\n").unwrap();
        assert_eq!(result.spec.feature_name.as_deref(), Some("`CHAR` type"));
    }

    #[test]
    fn scenario_name_keeps_inline_code_in_backticks() {
        let md = "# Feature: Test\n\n## Scenarios\n\n### Scenario: Pushdown of `substr`\n";
        let result = parse(md).unwrap();
        assert_eq!(result.spec.scenarios[0].name, "Pushdown of `substr`");
    }

    #[test]
    fn section_heading_with_inline_code_matches_on_text_without_code() {
        let result = parse("# Feature: Test\n\n## Background `notes`\n").unwrap();
        assert!(result.spec.has_background);
    }

    #[test]
    fn description_keeps_every_paragraph_as_written() {
        let md = "# Feature: Test\n\nFirst paragraph with `code`.\n\nSecond paragraph.\n\n## Background\n";
        let result = parse(md).unwrap();
        assert_eq!(
            result.spec.description.as_deref(),
            Some("First paragraph with `code`.\n\nSecond paragraph.")
        );
    }

    #[test]
    fn background_paragraph_after_description_stays_in_background() {
        let md = "# Feature: Test\n\nThe description.\n\n## Background\n\nThe background.\n\n## Scenarios\n";
        let result = parse(md).unwrap();
        assert_eq!(result.spec.description.as_deref(), Some("The description."));
        assert_eq!(result.spec.background.as_deref(), Some("The background."));
    }

    #[test]
    fn background_keeps_nested_list_items_as_written() {
        let md = "# Feature: Test\n\nDesc\n\n## Background\n\nIntro.\n\n* item\n  * nested `x`\n\n## Scenarios\n";
        let result = parse(md).unwrap();
        assert_eq!(
            result.spec.background.as_deref(),
            Some("Intro.\n\n* item\n  * nested `x`")
        );
    }

    #[test]
    fn empty_background_has_no_text_but_counts_as_present() {
        let md = "# Feature: Test\n\nDesc\n\n## Background\n\n## Scenarios\n";
        let result = parse(md).unwrap();
        assert!(result.spec.background.is_none());
        assert!(result.spec.has_background);
    }

    #[test]
    fn background_runs_to_end_of_file() {
        let md = "# Feature: Test\n\nDesc\n\n## Background\n\n* last item\n";
        let result = parse(md).unwrap();
        assert_eq!(result.spec.background.as_deref(), Some("* last item"));
    }

    #[test]
    fn background_heading_with_inline_code_keeps_its_text() {
        let md = "# Feature: Test\n\nDesc\n\n## Background `notes`\n\n* item\n";
        let result = parse(md).unwrap();
        assert_eq!(result.spec.background.as_deref(), Some("* item"));
    }

    #[test]
    fn description_ends_at_a_scenario_heading() {
        let md = "# Feature: Test\n\nDesc.\n\n### Scenario: First\n\n* *GIVEN* a precondition\n";
        let result = parse(md).unwrap();
        assert_eq!(result.spec.description.as_deref(), Some("Desc."));
    }

    #[test]
    fn description_comes_from_the_first_feature_heading() {
        let md = "# Feature: A\n\nFirst.\n\n# Feature: B\n\nSecond.\n";
        let result = parse(md).unwrap();
        assert_eq!(result.spec.description.as_deref(), Some("First."));
    }

    #[test]
    fn missing_description_has_no_text() {
        let result = parse("# Feature: Test\n\n## Background\n").unwrap();
        assert!(result.spec.description.is_none());
    }

    #[test]
    fn description_signal_is_the_last_prose_paragraph() {
        let md = "# Feature: Test\n\nThe description.\n\n## Background\n\nThe background.\n";
        let result = parse(md).unwrap();
        assert_eq!(
            result.spec.description_signal.as_deref(),
            Some("The background.")
        );
    }

    #[test]
    fn inline_code_never_reaches_the_description_signal() {
        let result = parse("# Feature: Test\n\n`only_code`\n").unwrap();
        assert!(result.spec.description_signal.is_none());
        assert_eq!(result.spec.description.as_deref(), Some("`only_code`"));
    }
}
