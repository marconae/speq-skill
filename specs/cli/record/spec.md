# Feature: CLI Record

The CLI SHALL provide a command to record approved plan deltas into permanent feature specs and archive the plan.

## Background

* Command syntax: `speq record <plan-name>`
* Plans are located at `specs/_plans/<plan-name>/`
* Recorded plans are archived to `specs/_recorded/YYYY-MM-DD-<plan-name>/`
* Delta markers: `<!-- DELTA:NEW -->`, `<!-- DELTA:CHANGED -->`, `<!-- DELTA:REMOVED -->`
* Exit code 0 on success, 1 on error

## Scenarios

### Scenario: Record new feature

* *GIVEN* a plan with a new feature delta at `specs/_plans/my-plan/cli/new-cmd/spec.md`
* *AND* no existing spec at `specs/cli/new-cmd/spec.md`
* *WHEN* the user runs `speq record my-plan`
* *THEN* the system SHALL copy the delta spec to `specs/cli/new-cmd/spec.md`
* *AND* the system SHALL strip all delta markers from the recorded spec

### Scenario: Merge NEW scenario

* *GIVEN* a delta with `<!-- DELTA:NEW -->` marker around a scenario
* *AND* an existing feature spec
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL append the new scenario to the existing spec's Scenarios section

### Scenario: Merge CHANGED scenario

* *GIVEN* a delta with `<!-- DELTA:CHANGED -->` marker around a scenario named "Login"
* *AND* an existing feature spec with a scenario named "Login"
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL replace the existing "Login" scenario with the delta version

### Scenario: Merge REMOVED scenario

* *GIVEN* a delta with `<!-- DELTA:REMOVED -->` marker around a scenario named "Guest login"
* *AND* an existing feature spec with a scenario named "Guest login"
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL remove the "Guest login" scenario from the spec

### Scenario: Archive plan after recording

* *GIVEN* a successful recording of plan `my-plan`
* *AND* today's date is `2025-03-15`
* *WHEN* the recording completes
* *THEN* the system SHALL move `specs/_plans/my-plan/` to `specs/_recorded/2025-03-15-my-plan/`

### Scenario: Validate after merge

* *GIVEN* a plan with delta specs
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL validate each merged spec
* *AND* the system SHALL report any validation errors

### Scenario: Plan not found

* *GIVEN* no plan named `nonexistent` exists
* *WHEN* the user runs `speq record nonexistent`
* *THEN* the system SHALL report an error indicating the plan was not found
* *AND* the system SHALL exit with code 1

### Scenario: Recording fails on merge error

* *GIVEN* a delta referencing a scenario that does not exist for CHANGED operation
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL report an error
* *AND* the system SHALL NOT archive the plan
* *AND* the system SHALL exit with code 1
