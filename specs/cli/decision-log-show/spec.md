# Feature: Decision Log Show

Assembles the permanent Architecture Decision Records from the per-plan fragments under `specs/_decision/` and prints the full log to stdout on demand. Rendering on demand replaces a merged on-disk file, so parallel plans never contend for one shared log.

## Background

* The CLI command is `speq decision-log show`
* Permanent decision records live as fragment files under `specs/_decision/`, each named `NNN-<plan-name>.md`
* `show` reads every `specs/_decision/*.md` fragment and collects all `## ADR:` blocks
* Fragment files are ordered by their numeric `NNN-` prefix ascending; fragments sharing a prefix are ordered by filename ascending
* Within a fragment, ADRs are rendered in document order (top to bottom), keeping each plan's ADRs contiguous
* The assembled output begins with H1 `# Architecture Decision Records`
* Each rendered ADR retains its `**ID:**`, `**Plan:**`, `**Status:**`, `### Context`, and `### Decision`
* `show` writes only to stdout; it MUST NOT create or modify any file
* `show` exits with code 0 on success

## Scenarios

### Scenario: Assemble ADRs ordered by numeric fragment prefix

* *GIVEN* the directory `specs/_decision/` contains `001-plan-a.md` and `002-plan-b.md`
* *AND* each fragment contains one ADR
* *WHEN* the user runs `speq decision-log show`
* *THEN* the system SHALL print H1 `# Architecture Decision Records`
* *AND* the system SHALL print the ADR from `001-plan-a.md` before the ADR from `002-plan-b.md`
* *AND* the system SHALL exit with code 0

### Scenario: Break ordering ties between fragments by filename

* *GIVEN* the directory `specs/_decision/` contains two fragments `001-plan-a.md` and `001-plan-b.md`
* *AND* each fragment holds one ADR
* *WHEN* the user runs `speq decision-log show`
* *THEN* the system SHALL print the ADR from `001-plan-a.md` before the ADR from `001-plan-b.md`
* *AND* the system SHALL exit with code 0

### Scenario: Preserve authored order of ADRs within a fragment

* *GIVEN* a fragment `001-plan-a.md` holds two ADRs authored top-to-bottom as `**ID:** zeta-first` then `**ID:** alpha-second`
* *WHEN* the user runs `speq decision-log show`
* *THEN* the system SHALL print the ADR `zeta-first` before the ADR `alpha-second`
* *AND* the system SHALL exit with code 0

### Scenario: Print only the header when the decisions directory is absent

* *GIVEN* no directory exists at `specs/_decision/`
* *WHEN* the user runs `speq decision-log show`
* *THEN* the system SHALL print H1 `# Architecture Decision Records`
* *AND* the system MUST NOT print any ADR entry
* *AND* the system SHALL exit with code 0

### Scenario: Never write a merged file to disk

* *GIVEN* the directory `specs/_decision/` contains one fragment with a well-formed ADR
* *WHEN* the user runs `speq decision-log show`
* *THEN* the system SHALL print the assembled log to stdout
* *AND* the system MUST NOT create a file at `specs/decision-log.md`
* *AND* the system MUST NOT modify any file under `specs/_decision/`

### Scenario: Render each ADR with its required fields

* *GIVEN* the directory `specs/_decision/` contains a fragment whose ADR carries `**ID:**`, `**Date:**`, `**Plan:**`, `**Status:**`, `### Context`, and `### Decision`
* *WHEN* the user runs `speq decision-log show`
* *THEN* the assembled output SHALL contain the ADR title heading, its `**ID:**` slug, and its `### Decision` body
* *AND* the system SHALL exit with code 0
