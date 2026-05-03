# Feature: Plan Validation

Validates plan structure and spec delta formatting before implementation begins. Catches formatting errors early to prevent malformed specs from being recorded.

## Background

* Plans are stored in `specs/_plans/<plan-name>/`
* A valid plan MUST contain a `plan.md` file
* A plan MAY contain a `decision-log.md` file in the plan-level (lightweight) format
* A plan MAY contain spec deltas in `<domain>/<feature>/spec.md` files
* Spec deltas use DELTA markers: `<!-- DELTA:NEW -->`, `<!-- DELTA:CHANGED -->`, `<!-- DELTA:REMOVED -->`
* Plan-level decision logs use H1 `# Decision Log: <plan-name>`, a `Date:` line, and at least one of `## Interview`, `## Design Decisions`, `## Review Findings`
* Steps MUST be formatted as `* *KEYWORD* <text>` (bullet, emphasized uppercase keyword)
* Step keywords (GIVEN, WHEN, THEN, AND) MUST be uppercase
* RFC 2119 keywords in THEN steps (MUST, SHALL, SHOULD, MAY, etc.) MUST be uppercase

## Scenarios

### Scenario: Validate plan with correctly formatted deltas

* *GIVEN* a plan named "test-plan" exists
* *AND* the plan contains a `plan.md` file
* *AND* the plan contains delta specs with properly formatted scenarios
* *WHEN* the user runs `speq plan validate test-plan`
* *THEN* the system SHALL report validation passed
* *AND* the system SHALL exit with code 0

### Scenario: Validate plan with missing plan.md

* *GIVEN* a plan named "broken-plan" exists
* *AND* the plan directory has no `plan.md` file
* *WHEN* the user runs `speq plan validate broken-plan`
* *THEN* the system SHALL report error "plan.md not found"
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate plan with malformed step formatting

* *GIVEN* a plan named "bad-format" exists
* *AND* the plan contains a delta spec with steps lacking bullet points
* *WHEN* the user runs `speq plan validate bad-format`
* *THEN* the system SHALL report formatting errors for each malformed step
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate plan with steps missing emphasized keywords

* *GIVEN* a plan named "no-emphasis" exists
* *AND* the plan contains a delta spec with steps like "WHEN action" instead of "* *WHEN* action"
* *WHEN* the user runs `speq plan validate no-emphasis`
* *THEN* the system SHALL report formatting errors indicating missing emphasis markers
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate plan with lowercase step keywords

* *GIVEN* a plan named "lowercase-steps" exists
* *AND* the plan contains a delta spec with steps like "* *when* action" instead of "* *WHEN* action"
* *WHEN* the user runs `speq plan validate lowercase-steps`
* *THEN* the system SHOULD report a warning that step keywords MUST be uppercase
* *AND* the system SHALL exit with code 0

### Scenario: Validate plan with lowercase RFC keywords

* *GIVEN* a plan named "lowercase-rfc" exists
* *AND* the plan contains a delta spec with THEN steps containing "shall" instead of "SHALL"
* *WHEN* the user runs `speq plan validate lowercase-rfc`
* *THEN* the system SHOULD report a warning that RFC 2119 keywords MUST be uppercase
* *AND* the system SHALL exit with code 0

### Scenario: Validate plan with delta markers not closed

* *GIVEN* a plan named "unclosed-delta" exists
* *AND* the plan contains a delta spec with `<!-- DELTA:NEW -->` but no closing marker
* *WHEN* the user runs `speq plan validate unclosed-delta`
* *THEN* the system SHALL report error about unclosed delta marker
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate plan without any delta specs

* *GIVEN* a plan named "refactor-only" exists
* *AND* the plan contains a `plan.md` file
* *AND* the plan contains no delta spec files
* *WHEN* the user runs `speq plan validate refactor-only`
* *THEN* the system SHALL report validation passed
* *AND* the system SHOULD note that no delta specs were found
* *AND* the system SHALL exit with code 0

### Scenario: Validate non-existent plan

* *GIVEN* no plan named "ghost-plan" exists
* *WHEN* the user runs `speq plan validate ghost-plan`
* *THEN* the system SHALL report error "plan not found"
* *AND* the system SHALL exit with non-zero code

### Scenario: Delta spec validation includes standard spec checks

* *GIVEN* a plan named "incomplete-spec" exists
* *AND* the plan contains a delta spec missing required sections
* *WHEN* the user runs `speq plan validate incomplete-spec`
* *THEN* the system SHALL report the same validation errors as `speq feature validate`
* *AND* the system SHALL exit with non-zero code

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

### Scenario: Validate plan with decision-log missing Date field

* *GIVEN* a plan named "decisions-no-date" exists with a `decision-log.md` that has no `Date:` line
* *WHEN* the user runs `speq plan validate decisions-no-date`
* *THEN* the system SHALL report an error indicating the decision log is missing the `Date:` field
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate plan with decision-log having no sections

* *GIVEN* a plan named "decisions-no-sections" exists with a `decision-log.md` containing H1 and Date but no `##` section headings
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
