# Feature: CLI Record Prose-Realignment Note

The CLI SHALL leave a note in the archived plan whenever recording changes the Background or the Feature description of a permanent spec, so that a reviewer can compare the prose before and after the change.

## Background

* Command syntax: `speq record <plan-name>`
* The note is written to `notes/prose-realignment.md` in the plan directory before the plan is archived, so the note moves into the archived plan directory

## Scenarios

### Scenario: Write prose-realignment note when Background or description changes

* *GIVEN* a plan whose delta changes a `## Background` or `# Feature: <name>` section
* *WHEN* the recording completes
* *THEN* the system SHALL write `notes/prose-realignment.md` into the archived plan directory
* *AND* the note SHALL name the feature path and the anchor of each change
* *AND* the note SHALL contain the text before the change and the text after the change

### Scenario: Omit prose-realignment note when only scenarios change

* *GIVEN* a plan whose delta blocks all target `### Scenario: <name>` anchors
* *WHEN* the recording completes
* *THEN* the system MUST NOT write `notes/prose-realignment.md`
