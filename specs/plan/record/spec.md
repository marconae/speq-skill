# Feature: CLI Record

The CLI SHALL provide a command to record approved plan deltas into permanent feature specs and archive the plan under a numbered folder.

## Background

* Command syntax: `speq record <plan-name>`
* Plans are located at `specs/_plans/<plan-name>/`
* Recorded plans are archived to `specs/_recorded/NNN-<plan-name>/`, where `NNN` is a zero-padded ordering number
* `NNN` is the count of existing entries under `specs/_recorded/` plus one, starting at `001`
* Delta markers: `<!-- DELTA:NEW -->`, `<!-- DELTA:CHANGED -->`, `<!-- DELTA:REMOVED -->`
* A `### Scenario: <name>` anchor matches the target spec heading of the same exact text
* No permanent spec is written until every delta spec of the plan has passed its anchor checks and merged
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
* *AND* `specs/_recorded/` already contains 2 archived entries
* *WHEN* the recording completes
* *THEN* the system SHALL move `specs/_plans/my-plan/` to `specs/_recorded/003-my-plan/`

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

* *GIVEN* an existing feature spec
* *AND* a `<!-- DELTA:CHANGED -->` or `<!-- DELTA:REMOVED -->` block whose anchor is absent from that spec
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL report an error naming the missing anchor
* *AND* the system SHALL NOT archive the plan
* *AND* the system SHALL exit with code 1

### Scenario: Rebuild index after recording

* *GIVEN* a successful recording of plan `my-plan`
* *AND* a search index exists
* *WHEN* the recording completes
* *THEN* the system SHALL rebuild the search index
* *AND* the system SHALL display the number of scenarios indexed

### Scenario: Leave every target spec unchanged when one delta spec fails

* *GIVEN* a plan with two delta specs targeting two existing feature specs
* *AND* the first delta spec is valid and the second carries a block that the system rejects
* *WHEN* the user runs `speq record`
* *THEN* the system MUST NOT modify the target spec of the first delta spec
* *AND* the system SHALL NOT archive the plan
* *AND* the system SHALL exit with code 1
