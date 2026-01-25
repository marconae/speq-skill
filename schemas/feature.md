# Feature: <feature_name>

<Concise statement of the business capability and its value. No implementation details. No UI assumptions.>

## Background

<Invariant conditions that apply to all scenarios>

## Scenarios

### Scenario: <scenario_name>

* *GIVEN* <initial state>
* *AND* <invalid or exceptional state>
* *WHEN* <single triggering event>
* *THEN* <expected rejection or failure>
* *AND* <error message or system response>
* *AND* <state that must not change>

### Scenario: Successful login

* *GIVEN* a registered user exists
* *WHEN* the user submits valid credentials
* *THEN* the system SHALL authenticate the user
* *AND* the system SHALL create an active session
* *AND* the system SHOULD log the login event
