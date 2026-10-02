# Feature: CLI Record Delta Anchors

The CLI SHALL merge each delta block of a plan into the section that its anchor names, and SHALL reject a delta block whose marker and anchor it cannot apply.

## Background

* Command syntax: `speq record <plan-name>`
* The anchor of a delta block is its first non-empty line
* Recognized anchors: `### Scenario: <name>`, `## Background`, and `# Feature: <name>`
* A `## Background` or `# Feature: <name>` anchor wraps that heading in place inside the delta file
* `DELTA:CHANGED` MAY target any recognized anchor
* `DELTA:NEW` and `DELTA:REMOVED` MAY target a `### Scenario: <name>` anchor only
* A `# Feature: <name>` anchor matches the first `# Feature` heading of the target spec by prefix, so a delta MAY change the feature name in the heading
* The `<domain>/<feature>` directory path is unchanged by a heading rename
* A `## Background` or `# Feature: <name>` section ends at the next `## ` heading
* A recognized anchor MAY appear in at most one delta block per delta file, whatever the marker kinds
* Anchor checks are skipped for a delta spec whose target feature spec does not exist

## Scenarios

### Scenario: Merge CHANGED Background section

* *GIVEN* an existing feature spec with a `## Background` section
* *AND* a delta with a `<!-- DELTA:CHANGED -->` block whose first line is `## Background`
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL replace the `## Background` section of the target spec with the block content
* *AND* the system SHALL keep the `## Scenarios` section that follows it
* *AND* the system SHALL keep one empty line between the replacement and the next heading

### Scenario: Merge CHANGED Feature description

* *GIVEN* an existing feature spec whose first heading is `# Feature: <name>`
* *AND* a delta with a `<!-- DELTA:CHANGED -->` block whose first line starts with `# Feature`
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL replace the heading and the description of the target spec with the block content
* *AND* the system SHALL keep the `## Background` section that follows it

### Scenario: Merge CHANGED Feature description that renames the heading

* *GIVEN* an existing feature spec whose first heading is `# Feature: <name>`
* *AND* a delta with a `<!-- DELTA:CHANGED -->` block whose first line is `# Feature: <other name>`
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL replace the heading of the target spec with `# Feature: <other name>`
* *AND* the system MUST NOT move or rename the `<domain>/<feature>` directory
* *AND* the system SHALL exit with code 0

### Scenario: Record a new feature spec without anchor checks

* *GIVEN* a plan whose delta has no target spec under `specs/`
* *AND* the delta wraps `## Background` in a `<!-- DELTA:NEW -->` block
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL write the target spec with the delta markers stripped
* *AND* the system MUST NOT report an anchor error
* *AND* the system SHALL exit with code 0

### Scenario: Reject DELTA:NEW targeting Background or Feature description

* *GIVEN* an existing feature spec
* *AND* a delta with a `<!-- DELTA:NEW -->` block whose first line is `## Background` or starts with `# Feature`
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL report an error naming the marker, the anchor, and `DELTA:CHANGED` as the correct marker
* *AND* the system SHALL NOT archive the plan
* *AND* the system SHALL exit with code 1

### Scenario: Reject DELTA:REMOVED targeting Background or Feature description

* *GIVEN* an existing feature spec
* *AND* a delta with a `<!-- DELTA:REMOVED -->` block whose first line is `## Background` or starts with `# Feature`
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL report an error naming the marker, the anchor, and `DELTA:CHANGED` as the correct marker
* *AND* the system SHALL NOT archive the plan
* *AND* the system SHALL exit with code 1

### Scenario: Reject delta block with unrecognized anchor

* *GIVEN* an existing feature spec
* *AND* a delta with a block whose first non-empty line is not a recognized anchor
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL report an error that lists the three recognized anchors and instructs the author to put the heading inside the marker
* *AND* the system SHALL NOT archive the plan
* *AND* the system SHALL exit with code 1

### Scenario: Reject two delta blocks with the same kind and anchor

* *GIVEN* an existing feature spec
* *AND* a delta file with two blocks of the same marker anchored at the same heading
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL report an error naming the repeated anchor
* *AND* the system SHALL NOT archive the plan
* *AND* the system SHALL exit with code 1

### Scenario: Reject two delta blocks of different kinds on the same anchor

* *GIVEN* an existing feature spec
* *AND* a delta file with a `<!-- DELTA:NEW -->` block and a `<!-- DELTA:CHANGED -->` block anchored at the same heading
* *WHEN* the user runs `speq record`
* *THEN* the system SHALL report an error naming the repeated anchor and both markers
* *AND* the system SHALL NOT archive the plan
* *AND* the system SHALL exit with code 1
