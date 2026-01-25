# Feature: RFC 2119 Keyword Validation

The validator SHALL ensure that scenario steps use RFC 2119 keywords to express normative requirements.

## Background

* RFC 2119 defines keywords: MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY
* Keywords MUST appear in uppercase
* Each step (GIVEN, WHEN, THEN, AND) SHOULD contain at least one RFC 2119 keyword
* GIVEN and WHEN steps describe state and actions, which MAY not require normative keywords
* THEN steps describe expected outcomes and MUST contain normative keywords

## Scenarios

### Scenario: Step contains RFC 2119 keyword

* *GIVEN* a scenario step containing the keyword "SHALL"
* *WHEN* the validator checks the step for RFC 2119 compliance
* *THEN* the system SHALL report no keyword errors for that step

### Scenario: Step contains MUST keyword

* *GIVEN* a scenario step containing the keyword "MUST"
* *WHEN* the validator checks the step for RFC 2119 compliance
* *THEN* the system SHALL accept the step as valid

### Scenario: Step contains MUST NOT keyword

* *GIVEN* a scenario step containing the keyword "MUST NOT"
* *WHEN* the validator checks the step for RFC 2119 compliance
* *THEN* the system SHALL accept the step as valid

### Scenario: Step contains SHOULD keyword

* *GIVEN* a scenario step containing the keyword "SHOULD"
* *WHEN* the validator checks the step for RFC 2119 compliance
* *THEN* the system SHALL accept the step as valid

### Scenario: Step contains MAY keyword

* *GIVEN* a scenario step containing the keyword "MAY"
* *WHEN* the validator checks the step for RFC 2119 compliance
* *THEN* the system SHALL accept the step as valid

### Scenario: Step missing RFC 2119 keyword

* *GIVEN* a scenario step without any RFC 2119 keyword
* *WHEN* the validator checks the step for RFC 2119 compliance
* *THEN* the system SHALL report an error indicating the step is missing a normative keyword
* *AND* the error message SHALL identify the scenario and step

### Scenario: Lowercase keyword not accepted

* *GIVEN* a scenario step containing "shall" in lowercase
* *WHEN* the validator checks the step for RFC 2119 compliance
* *THEN* the system SHALL report an error indicating the step is missing a normative keyword
* *AND* the system SHALL NOT accept lowercase variants as valid keywords
