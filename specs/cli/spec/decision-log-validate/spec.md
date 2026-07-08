# Feature: Decision Log Validation

Validates the permanent Architecture Decision Records stored as one fragment file per plan under `specs/_decision/`. Catches malformed, duplicate-identity, or dangling ADRs before they accumulate. Fragments are the long-lived record promoted from per-plan decision logs by `speq record`, so structural integrity is non-negotiable.

## Background

* The permanent decision records live as one fragment file per plan under the `specs/_decision/` directory
* Each fragment file is named `NNN-<plan-name>.md`, where `NNN` is a numeric ordering prefix
* Each fragment uses ADR (Nygard) format, NOT the lightweight per-plan format
* Each fragment file MUST start with H1 `# Decisions: <plan-name>`
* Each ADR MUST be a `## ADR: <Title>` heading
* Each ADR MUST carry an `**ID:**` field whose value is a kebab-case slug
* Every ADR `**ID:**` slug MUST be unique across all fragment files in `specs/_decision/`
* Each ADR MUST contain `**ID:**`, `**Plan:**`, `**Status:**`, `### Context`, and `### Decision`
* `**Status:**` MUST be one of `Accepted`, `Deprecated`, or `Superseded by <slug>`
* `**Supersedes:**` is OPTIONAL; when present its value MUST resolve to an existing ADR `**ID:**`
* When `**Status:**` is `Superseded by <slug>`, the `<slug>` MUST resolve to an existing ADR `**ID:**`
* Slug and status references MAY resolve to ADRs in a different fragment file
* `### Options Considered` and `### Consequences` are OPTIONAL sections within an ADR
* ADR ordering numbers are NOT validated; duplicate `NNN-` prefixes across fragment files are acceptable
* The CLI command is `speq decision-log validate`

## Scenarios

### Scenario: Validate a well-formed decisions directory

* *GIVEN* `specs/_decision/001-plan-a.md` holds an ADR `**ID:** use-line-scanner` with all required fields and `**Status:** Superseded by faster-scanner`
* *AND* `specs/_decision/002-plan-b.md` holds an ADR `**ID:** faster-scanner` with `**Supersedes:** use-line-scanner`
* *AND* both slug references resolve across the two fragments
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report validation passed
* *AND* the system SHALL exit with code 0

### Scenario: Validate passes when the decisions directory is absent

* *GIVEN* no directory exists at `specs/_decision/`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report validation passed
* *AND* the system SHALL exit with code 0

### Scenario: Validate fails on a duplicate ADR slug across fragments

* *GIVEN* the directory `specs/_decision/` exists
* *AND* two ADRs in different fragment files share the same `**ID:** use-line-scanner`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error that the ADR slug `use-line-scanner` is duplicated
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails on an ADR missing its ID field

* *GIVEN* a fragment file in `specs/_decision/` contains an ADR with no `**ID:**` field
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error identifying the ADR missing its `**ID:**` field
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails on an unresolved Supersedes target

* *GIVEN* a fragment file in `specs/_decision/` contains an ADR with `**Supersedes:** ghost-slug`
* *AND* no ADR in any fragment carries `**ID:** ghost-slug`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error that the `**Supersedes:**` target `ghost-slug` does not resolve
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails when a superseding status references an unknown ADR

* *GIVEN* a fragment file in `specs/_decision/` contains an ADR with `**Status:** Superseded by ghost-slug`
* *AND* no ADR in any fragment carries `**ID:** ghost-slug`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error that the superseding slug `ghost-slug` does not resolve
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails when a fragment lacks its H1 heading

* *GIVEN* a fragment file in `specs/_decision/` does not start with H1 `# Decisions: <plan-name>`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error identifying the fragment file missing its H1 heading
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails on ADR missing a required field

* *GIVEN* the directory `specs/_decision/` exists
* *AND* a fragment file contains an ADR missing the `**Status:**` field
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error identifying which ADR is missing which required field
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate fails on invalid Status value

* *GIVEN* the directory `specs/_decision/` exists
* *AND* a fragment file contains an ADR whose `**Status:**` value is `Pending`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report an error that Status MUST be one of `Accepted`, `Deprecated`, or `Superseded by <slug>`
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate passes when optional ADR sections are absent

* *GIVEN* the directory `specs/_decision/` contains a fragment with a well-formed ADR carrying all required fields but no `### Options Considered` and no `### Consequences`
* *WHEN* the user runs `speq decision-log validate`
* *THEN* the system SHALL report validation passed
* *AND* the system MUST NOT emit warnings about the absent optional sections
* *AND* the system SHALL exit with code 0
