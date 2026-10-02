# Feature: Plan Decision Log Validation

Validates the optional plan-level decision log during plan validation. Catches a malformed decision log before the plan is recorded.

## Background

* Plans are stored in `specs/_plans/<plan-name>/`
* A plan MAY contain a `decision-log.md` file in the plan-level (lightweight) format
* Plan-level decision logs use H1 `# Decision Log: <plan-name>` and at least one of `## Interview`, `## Design Decisions`, `## Review Findings`
* A decision entry's `Promotes to ADR:` value is `yes` or `no`

## Scenarios

### Scenario: Validate plan with valid decision-log.md passes

* *GIVEN* a plan named "with-decisions" exists with `plan.md` and a well-formed `decision-log.md`
* *WHEN* the user runs `speq plan validate with-decisions`
* *THEN* the system SHALL report validation passed
* *AND* the system SHALL exit with code 0

### Scenario: Validate plan without decision-log.md still passes

* *GIVEN* a plan named "no-decisions" exists with `plan.md` and no `decision-log.md`
* *WHEN* the user runs `speq plan validate no-decisions`
* *THEN* the system SHALL report validation passed
* *AND* the system MUST NOT report any error related to decision-log.md

### Scenario: Validate plan with decision-log having no sections

* *GIVEN* a plan named "decisions-no-sections" exists with a `decision-log.md` containing H1 but no `##` section headings
* *WHEN* the user runs `speq plan validate decisions-no-sections`
* *THEN* the system SHALL report an error that the decision log MUST contain at least one of "Interview", "Design Decisions", or "Review Findings" sections
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate plan with decision-log having invalid Promotes to ADR value

* *GIVEN* a plan named "decisions-bad-promote" exists with a decision entry whose `Promotes to ADR:` value is neither `yes` nor `no`
* *WHEN* the user runs `speq plan validate decisions-bad-promote`
* *THEN* the system SHOULD report a warning that `Promotes to ADR` MUST be `yes` or `no`
* *AND* the system SHALL exit with code 0

### Scenario: Validate plan with decision-log having wrong H1 heading

* *GIVEN* a plan named "decisions-bad-h1" exists with a `decision-log.md` whose first H1 is not `# Decision Log: <plan-name>`
* *WHEN* the user runs `speq plan validate decisions-bad-h1`
* *THEN* the system SHALL report an error that the decision log H1 MUST match `# Decision Log: <plan-name>`
* *AND* the system SHALL exit with non-zero code
