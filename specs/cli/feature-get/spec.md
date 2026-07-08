# Feature: CLI Feature Get

The CLI SHALL provide a command to retrieve and display a feature specification or individual scenario.

## Background

* Command syntax: `speq feature get <path>`
* Path format: `<domain>/<feature>` for full spec, `<domain>/<feature>/<scenario>` for single scenario
* Output is formatted text (not raw markdown)
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
* *THEN* each step SHALL be prefixed with its keyword (GIVEN, WHEN, THEN, AND)
* *AND* steps SHALL be indented for readability
