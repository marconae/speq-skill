# Delta Specification Template

Captures feature changes in plans before applying to permanent specs.

## Rules

1. **New Feature**: Full spec (Feature heading, Background, all Scenarios)
2. **Existing Feature**: Only `## Scenarios` section with markers around changes
3. **Applying**: Merge deltas into permanent specs, remove markers

## Markers

```
<!-- DELTA:NEW -->     <!-- /DELTA:NEW -->
<!-- DELTA:CHANGED --> <!-- /DELTA:CHANGED -->
<!-- DELTA:REMOVED --> <!-- /DELTA:REMOVED -->
```

## Example

```markdown
## Scenarios

<!-- DELTA:NEW -->
### Scenario: User resets password

* *GIVEN* a registered user exists
* *WHEN* the user requests a password reset
* *THEN* the system SHALL send a reset link
<!-- /DELTA:NEW -->

<!-- DELTA:CHANGED -->
### Scenario: User logs in

* *GIVEN* a registered user exists
* *WHEN* the user submits valid credentials
* *THEN* the system SHALL enforce two-factor authentication
<!-- /DELTA:CHANGED -->

<!-- DELTA:REMOVED -->
### Scenario: Guest checkout
<!-- /DELTA:REMOVED -->
```
