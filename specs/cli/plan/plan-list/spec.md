# Feature: CLI Plan List

The CLI SHALL provide a command to list all active plans in the `_plans/` directory.

## Background

* Command syntax: `speq plan list`
* An active plan is a subdirectory of `specs/_plans/`
* Output format shows plan names, one per line
* Exit code 0 on success

## Scenarios

### Scenario: List active plans

* *GIVEN* a `_plans/` directory containing plans `add-auth` and `fix-validation`
* *WHEN* the user runs `speq plan list`
* *THEN* the system SHALL display each plan name on its own line
* *AND* the system SHALL list plans in alphabetical order
* *AND* the system SHALL exit with code 0

### Scenario: No active plans

* *GIVEN* a `_plans/` directory with no plan subdirectories
* *WHEN* the user runs `speq plan list`
* *THEN* the system SHALL display "No active plans."
* *AND* the system SHALL exit with code 0

### Scenario: Plans directory does not exist

* *GIVEN* a specs directory with no `_plans/` subdirectory
* *WHEN* the user runs `speq plan list`
* *THEN* the system SHALL display "No active plans."
* *AND* the system SHALL exit with code 0
