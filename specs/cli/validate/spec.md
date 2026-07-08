# Feature: CLI Validate Command

The CLI SHALL provide a validate command that checks feature specifications for structural correctness and reports errors and warnings to the user.

## Background

* The CLI binary is named `scope-kit`
* Feature specifications are located at `specs/<feature-name>/spec.md`
* Exit code 0 indicates success, exit code 1 indicates validation errors
* Warnings do not affect the exit code

## Scenarios

### Scenario: Validate existing feature spec

* *GIVEN* a feature specification exists at `specs/my-feature/spec.md`
* *WHEN* the user runs `scope-kit validate my-feature`
* *THEN* the system SHALL read the file at `specs/my-feature/spec.md`
* *AND* the system SHALL validate the file against the schema

### Scenario: Feature spec not found

* *GIVEN* no file exists at `specs/missing-feature/spec.md`
* *WHEN* the user runs `scope-kit validate missing-feature`
* *THEN* the system SHALL report an error indicating the file was not found
* *AND* the system SHALL exit with code 1

### Scenario: Validation passes with no issues

* *GIVEN* a valid feature specification exists
* *WHEN* the user runs the validate command
* *THEN* the system SHALL report success
* *AND* the system SHALL exit with code 0

### Scenario: Validation fails with errors

* *GIVEN* a feature specification with structural errors exists
* *WHEN* the user runs the validate command
* *THEN* the system SHALL report each error with a descriptive message
* *AND* the system SHALL exit with code 1

### Scenario: Validation passes with warnings

* *GIVEN* a feature specification with warnings but no errors exists
* *WHEN* the user runs the validate command
* *THEN* the system SHALL report each warning
* *AND* the system SHALL exit with code 0
