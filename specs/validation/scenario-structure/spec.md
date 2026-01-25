# Feature: Scenario Structure Validation

The validator SHALL ensure that each scenario follows the GIVEN-WHEN-THEN structure with at least one step of each type.

## Background

* Scenarios MUST contain at least one GIVEN step
* Scenarios MUST contain at least one WHEN step
* Scenarios MUST contain at least one THEN step
* AND steps extend the previous GIVEN, WHEN, or THEN step
* Steps are formatted as `* *KEYWORD* <text>`

## Scenarios

### Scenario: Valid scenario structure

* *GIVEN* a scenario with at least one GIVEN, one WHEN, and one THEN step
* *WHEN* the validator checks the scenario structure
* *THEN* the system SHALL report no structure errors for that scenario

### Scenario: Missing GIVEN step

* *GIVEN* a scenario with WHEN and THEN steps but no GIVEN step
* *WHEN* the validator checks the scenario structure
* *THEN* the system SHALL report an error indicating the scenario is missing a GIVEN step

### Scenario: Missing WHEN step

* *GIVEN* a scenario with GIVEN and THEN steps but no WHEN step
* *WHEN* the validator checks the scenario structure
* *THEN* the system SHALL report an error indicating the scenario is missing a WHEN step

### Scenario: Missing THEN step

* *GIVEN* a scenario with GIVEN and WHEN steps but no THEN step
* *WHEN* the validator checks the scenario structure
* *THEN* the system SHALL report an error indicating the scenario is missing a THEN step

### Scenario: Multiple steps of each type allowed

* *GIVEN* a scenario with multiple GIVEN steps followed by multiple WHEN steps followed by multiple THEN steps
* *WHEN* the validator checks the scenario structure
* *THEN* the system SHALL report no structure errors for that scenario

### Scenario: Warning for excessive AND steps

* *GIVEN* a scenario with more than 3 AND steps total
* *WHEN* the validator checks the scenario structure
* *THEN* the system SHOULD report a warning indicating the scenario has too many AND steps
* *AND* the system SHALL NOT report this as an error

### Scenario: No warning for acceptable AND count

* *GIVEN* a scenario with exactly 3 AND steps
* *WHEN* the validator checks the scenario structure
* *THEN* the system SHALL NOT report a warning about AND steps
