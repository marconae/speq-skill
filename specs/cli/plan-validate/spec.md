# Feature: Plan Validation

Validates plan structure and spec delta formatting before implementation begins. Catches formatting errors early to prevent malformed specs from being recorded.

## Background

* Plans are stored in `specs/_plans/<plan-name>/`
* A valid plan MUST contain a `plan.md` file
* A plan MAY contain spec deltas in `<domain>/<feature>/spec.md` files
* Spec deltas use DELTA markers: `<!-- DELTA:NEW -->`, `<!-- DELTA:CHANGED -->`, `<!-- DELTA:REMOVED -->`
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
