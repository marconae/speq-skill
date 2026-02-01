# Feature: Keyword Casing Validation

Validates that step keywords and RFC 2119 keywords use uppercase formatting. Produces warnings for lowercase usage to help maintain spec consistency.

## Background

* Step keywords are GIVEN, WHEN, THEN, AND
* RFC 2119 keywords are MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY
* All keywords MUST be uppercase in the final spec
* Lowercase keywords result in warnings (not errors) to allow incremental fixes

## Scenarios

### Scenario: Warn on lowercase step keyword

* *GIVEN* a spec with a step like "* *when* the user acts"
* *WHEN* the validator checks the spec
* *THEN* the system SHOULD report a warning about lowercase step keyword
* *AND* the warning SHALL indicate the keyword should be uppercase
* *AND* the system SHALL NOT report this as an error

### Scenario: Warn on lowercase RFC keyword in THEN step

* *GIVEN* a spec with a THEN step containing "the system shall respond"
* *WHEN* the validator checks the spec
* *THEN* the system SHOULD report a warning about lowercase RFC keyword
* *AND* the warning SHALL indicate the keyword should be uppercase
* *AND* the system SHALL NOT report this as an error

### Scenario: Accept uppercase step keywords

* *GIVEN* a spec with properly formatted steps using *GIVEN*, *WHEN*, *THEN*, *AND*
* *WHEN* the validator checks the spec
* *THEN* the system SHALL NOT report any casing warnings

### Scenario: Accept uppercase RFC keywords

* *GIVEN* a spec with THEN steps containing uppercase MUST, SHALL, SHOULD, or MAY
* *WHEN* the validator checks the spec
* *THEN* the system SHALL NOT report any casing warnings
