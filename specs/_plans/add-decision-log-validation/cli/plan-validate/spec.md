# Feature: Plan Validation

Validates plan structure and spec delta formatting before implementation begins. Catches formatting errors early to prevent malformed specs from being recorded.

## Background

* Plans are stored in `specs/_plans/<plan-name>/`
* A valid plan MUST contain a `plan.md` file
* A plan MAY contain a `decision-log.md` file in the plan-level (lightweight) format
* A plan MAY contain spec deltas in `<domain>/<feature>/spec.md` files
* Plan-level decision logs use H1 `# Decision Log: <plan-name>`, a `Date:` line, and at least one of `## Interview`, `## Design Decisions`, `## Review Findings`

## Scenarios

<!-- DELTA:NEW -->
### Scenario: Validate plan with valid decision-log.md passes

* *GIVEN* a plan named "with-decisions" exists with `plan.md` and a well-formed `decision-log.md`
* *WHEN* the user runs `speq plan validate with-decisions`
* *THEN* the system SHALL report validation passed
* *AND* the system SHALL exit with code 0
<!-- /DELTA:NEW -->

<!-- DELTA:NEW -->
### Scenario: Validate plan without decision-log.md still passes

* *GIVEN* a plan named "no-decisions" exists with `plan.md` and no `decision-log.md`
* *WHEN* the user runs `speq plan validate no-decisions`
* *THEN* the system SHALL report validation passed
* *AND* the system MUST NOT report any error related to decision-log.md
<!-- /DELTA:NEW -->

<!-- DELTA:NEW -->
### Scenario: Validate plan with decision-log missing Date field

* *GIVEN* a plan named "decisions-no-date" exists with a `decision-log.md` that has no `Date:` line
* *WHEN* the user runs `speq plan validate decisions-no-date`
* *THEN* the system SHALL report an error indicating the decision log is missing the `Date:` field
* *AND* the system SHALL exit with non-zero code
<!-- /DELTA:NEW -->

<!-- DELTA:NEW -->
### Scenario: Validate plan with decision-log having no sections

* *GIVEN* a plan named "decisions-no-sections" exists with a `decision-log.md` containing H1 and Date but no `##` section headings
* *WHEN* the user runs `speq plan validate decisions-no-sections`
* *THEN* the system SHALL report an error that the decision log MUST contain at least one of "Interview", "Design Decisions", or "Review Findings" sections
* *AND* the system SHALL exit with non-zero code
<!-- /DELTA:NEW -->

<!-- DELTA:NEW -->
### Scenario: Validate plan with decision-log having invalid Promotes to ADR value

* *GIVEN* a plan named "decisions-bad-promote" exists with a decision entry whose `Promotes to ADR:` value is neither `yes` nor `no`
* *WHEN* the user runs `speq plan validate decisions-bad-promote`
* *THEN* the system SHOULD report a warning that `Promotes to ADR` MUST be `yes` or `no`
* *AND* the system SHALL exit with code 0
<!-- /DELTA:NEW -->

<!-- DELTA:NEW -->
### Scenario: Validate plan with decision-log having wrong H1 heading

* *GIVEN* a plan named "decisions-bad-h1" exists with a `decision-log.md` whose first H1 is not `# Decision Log: <plan-name>`
* *WHEN* the user runs `speq plan validate decisions-bad-h1`
* *THEN* the system SHALL report an error that the decision log H1 MUST match `# Decision Log: <plan-name>`
* *AND* the system SHALL exit with non-zero code
<!-- /DELTA:NEW -->
