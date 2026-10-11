# Feature: CLI Feature Get

The CLI SHALL provide a command to retrieve and display a feature specification or individual scenario.

## Background

* Command syntax: `speq feature get <path>`
* Path format: `<domain>/<feature>` for full spec, `<domain>/<feature>/<scenario>` for single scenario
* Exit code 0 on success, 1 on error

## Scenarios

### Scenario: Get full feature spec

* *GIVEN* a feature spec exists at `cli/validate/spec.md`
* *WHEN* the user runs `speq feature get cli/validate`
* *THEN* the system SHALL display the feature name as a heading
* *AND* the system SHALL display the feature description
* *AND* the system SHALL display all scenarios with their steps
* *AND* the system SHALL exit with code 0

### Scenario: Get single scenario

* *GIVEN* a feature spec at `cli/validate/spec.md` contains scenario "Basic test"
* *WHEN* the user runs `speq feature get cli/validate/Basic\ test`
* *THEN* the system SHALL display the path `cli/validate/Basic test`
* *AND* the system SHALL display only the matching scenario with its steps
* *AND* the system SHALL NOT display other scenarios
* *AND* the system SHALL exit with code 0

### Scenario: Feature not found

* *GIVEN* no feature spec exists at `cli/nonexistent`
* *WHEN* the user runs `speq feature get cli/nonexistent`
* *THEN* the system SHALL display an error message "Feature not found: cli/nonexistent"
* *AND* the system SHALL exit with code 1

### Scenario: Scenario not found

* *GIVEN* a feature spec at `cli/validate/spec.md` exists
* *AND* the spec does not contain scenario "Missing"
* *WHEN* the user runs `speq feature get cli/validate/Missing`
* *THEN* the system SHALL display an error message "Scenario 'Missing' not found in cli/validate"
* *AND* the system SHALL exit with code 1

### Scenario: Display step formatting

* *GIVEN* a scenario with GIVEN, WHEN, THEN, and AND steps
* *WHEN* the user runs `speq feature get <domain>/<feature>/<scenario>`
* *THEN* each step SHALL be prefixed with its keyword in title case (Given, When, Then, And)
* *AND* steps SHALL be indented for readability

### Scenario: Display the full feature description

* *GIVEN* a feature spec whose description has two paragraphs under the feature heading
* *AND* the Background section of that spec holds a paragraph
* *WHEN* the user runs `speq feature get <domain>/<feature>`
* *THEN* the system SHALL display both description paragraphs before the Background section
* *AND* the system SHALL display the Background paragraph only inside the Background section

### Scenario: Display the Background section

* *GIVEN* a feature spec whose Background section holds a paragraph, a list item, and a nested list item
* *WHEN* the user runs `speq feature get <domain>/<feature>`
* *THEN* the system SHALL display a `## Background` heading after the description and before the first scenario
* *AND* the system SHALL display the paragraph, the list item, and the nested list item exactly as they appear in the spec file

### Scenario: Omit an empty Background section

* *GIVEN* a feature spec whose `## Background` heading is followed directly by the `## Scenarios` heading
* *WHEN* the user runs `speq feature get <domain>/<feature>`
* *THEN* the system SHALL NOT display a `## Background` heading

### Scenario: Display inline code in steps

* *GIVEN* a scenario step whose text contains the inline code span `CHAR(10)`
* *WHEN* the user runs `speq feature get <domain>/<feature>/<scenario>`
* *THEN* the system SHALL display `CHAR(10)` in the step text, wrapped in backticks

### Scenario: Display inline code in feature and scenario names

* *GIVEN* a feature heading that contains the inline code span `CHAR`
* *AND* a scenario heading whose name ends with the inline code span `substr`
* *WHEN* the user runs `speq feature get <domain>/<feature>`
* *THEN* the system SHALL display the feature name with `CHAR` wrapped in backticks
* *AND* the system SHALL display the scenario name with `substr` wrapped in backticks

### Scenario: Get a scenario whose name contains inline code

* *GIVEN* a scenario heading whose name ends with the inline code span `substr`
* *WHEN* the user runs `speq feature get <domain>/<feature>/<scenario>` with the scenario name written as in the heading, where each inline code span uses single backticks
* *THEN* the system SHALL display that scenario
* *AND* the system SHALL exit with code 0
