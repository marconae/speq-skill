# Feature: Decision Log Validation

Validates the permanent Architecture Decision Records file at `specs/decision-log.md`. Catches malformed, gappy, or under-specified ADRs before they accumulate. The permanent log is the long-lived record promoted from per-plan decision logs by `speq record`, so structural integrity is non-negotiable.

## Background

* The permanent decision log lives at `specs/decision-log.md`
* The permanent log uses ADR (Nygard) format, NOT the lightweight per-plan format
* The file MUST start with H1 `# Architecture Decision Records`
* Each ADR MUST be an `## ADR-NNN: <Title>` heading where NNN is a zero-padded three-digit number
* ADR numbers MUST start at 001 and increment sequentially with no gaps
* Each ADR MUST contain `**Date:**`, `**Plan:**`, `**Status:**`, `### Context`, and `### Decision` fields
* `**Status:**` MUST be one of `Accepted`, `Superseded by ADR-NNN`, or `Deprecated`
* `### Options Considered` and `### Consequences` are OPTIONAL sections within an ADR
* The CLI command is `speq decision-log validate`

## Scenarios

### Scenario: Validate a well-formed permanent decision log

* *GIVEN* a file `specs/decision-log.md` exists
* *AND* the file has H1 `# Architecture Decision Records`
* *AND* the file contains ADR-001 with all required fields and `**Status:** Accepted`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report validation passed
* *AND* the system SHALL exit with code 0

### Scenario: Validate when permanent decision log is missing

* *GIVEN* no file exists at `specs/decision-log.md`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report error "decision-log.md not found"
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails on non-sequential ADR numbers

* *GIVEN* a file `specs/decision-log.md` exists
* *AND* the file contains ADR-001 followed by ADR-003 with no ADR-002
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error that ADR numbering MUST be sequential without gaps
* *AND* the system SHALL identify the missing number(s) in the error
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails on ADR missing required field

* *GIVEN* a file `specs/decision-log.md` exists
* *AND* the file contains ADR-001 missing the `**Status:**` field
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error identifying which ADR is missing which required field
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails on invalid Status value

* *GIVEN* a file `specs/decision-log.md` exists
* *AND* the file contains an ADR whose `**Status:**` value is `Pending`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error that Status MUST be one of `Accepted`, `Superseded by ADR-NNN`, or `Deprecated`
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails when ADRs do not start at 001

* *GIVEN* a file `specs/decision-log.md` exists
* *AND* the first ADR in the file is `ADR-002` (no `ADR-001`)
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error that ADR numbering MUST start at 001
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate passes when optional ADR sections are absent

* *GIVEN* a `specs/decision-log.md` contains a well-formed ADR-001 with all required fields but no `### Options Considered` and no `### Consequences`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report validation passed
* *AND* the system MUST NOT emit warnings about the absent optional sections
* *AND* the system SHALL exit with code 0
