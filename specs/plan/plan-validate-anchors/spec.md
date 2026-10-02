# Feature: Plan Delta Anchor Validation

Validates the anchors of a plan's spec delta blocks during plan validation, with the same target-independent rules that `speq record` applies. Catches a delta block that `speq record` would reject before implementation begins.

## Background

* Plans are stored in `specs/_plans/<plan-name>/`
* A delta spec at `<domain>/<feature>/spec.md` in the plan targets `specs/<domain>/<feature>/spec.md`
* The anchor of a delta block is its first non-empty line
* Recognized anchors: `### Scenario: <name>`, `## Background`, and `# Feature: <name>`
* `DELTA:NEW` and `DELTA:REMOVED` MAY target a `### Scenario: <name>` anchor only
* Anchor checks apply the target-independent rules of `speq record`: recognized anchors, legal marker and anchor pairs, and repeated anchors
* Anchor checks are skipped when the target spec `specs/<domain>/<feature>/spec.md` does not exist
* A delta file that fails delta parsing produces an error naming the file, and its anchor checks are skipped

## Scenarios

### Scenario: Validate plan with CHANGED Background or description delta passes

* *GIVEN* a plan named "prose-changed" whose delta targets an existing feature
* *AND* the delta wraps `## Background` and `# Feature: <name>` in `<!-- DELTA:CHANGED -->` blocks
* *WHEN* the user runs `speq plan validate prose-changed`
* *THEN* the system SHALL report validation passed
* *AND* the system SHALL exit with code 0

### Scenario: Validate plan with DELTA:NEW or DELTA:REMOVED targeting Background or description reports errors

* *GIVEN* a plan named "prose-bad-kind" whose delta targets an existing feature
* *AND* the delta wraps `## Background` in a `<!-- DELTA:NEW -->` block and `# Feature: <name>` in a `<!-- DELTA:REMOVED -->` block
* *WHEN* the user runs `speq plan validate prose-bad-kind`
* *THEN* the system SHALL report one error per rejected block
* *AND* each error SHALL name the delta spec file, the marker, and the anchor
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate plan with unrecognized delta anchor reports an error

* *GIVEN* a plan named "prose-no-anchor" whose delta targets an existing feature
* *AND* a delta block starts with a bullet list instead of a recognized anchor
* *WHEN* the user runs `speq plan validate prose-no-anchor`
* *THEN* the system SHALL report an error listing the three recognized anchors
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate plan skips anchor checks for a new feature

* *GIVEN* a plan named "prose-new-feature" whose delta has no target spec under `specs/`
* *AND* the delta wraps `## Background` in a `<!-- DELTA:NEW -->` block
* *WHEN* the user runs `speq plan validate prose-new-feature`
* *THEN* the system SHALL report validation passed
* *AND* the system MUST NOT report any anchor error
* *AND* the system SHALL exit with code 0

### Scenario: Validate plan with a malformed delta marker reports an error

* *GIVEN* a plan named "nested-markers" whose delta targets an existing feature
* *AND* the delta opens a second delta marker before it closes the first one
* *WHEN* the user runs `speq plan validate nested-markers`
* *THEN* the system SHALL report an error that names the delta spec file and the malformed delta marker
* *AND* the system SHALL exit with non-zero code
