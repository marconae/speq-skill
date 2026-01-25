# Feature: <feature_name>

<Concise statement of the business capability and its value. No implementation details. No UI assumptions.>

## Background

<Invariant conditions that apply to all scenarios>

## Scenarios

### Scenario: <scenario_name>

* *GIVEN* <initial state>
* *AND* <additional state if needed>
* *WHEN* <single triggering event>
* *THEN* <expected outcome with RFC 2119 keyword>
* *AND* <additional outcome if needed>

## Example

```markdown
# Feature: User Authentication

Enables users to securely access their accounts using credentials.

## Background

All authentication attempts are rate-limited to 5 per minute per IP.

## Scenarios

### Scenario: Successful login

* *GIVEN* a registered user exists
* *WHEN* the user submits valid credentials
* *THEN* the system SHALL authenticate the user
* *AND* the system SHALL create an active session
* *AND* the system SHOULD log the login event

### Scenario: Invalid credentials

* *GIVEN* a registered user exists
* *WHEN* the user submits invalid credentials
* *THEN* the system SHALL reject the authentication
* *AND* the system SHALL return an error message
* *AND* the system MUST NOT reveal which credential was wrong
```

## RFC 2119 Keywords

THEN steps MUST use one of:

| Keyword | Meaning |
|---------|---------|
| `MUST` / `SHALL` | Absolute requirement |
| `MUST NOT` / `SHALL NOT` | Absolute prohibition |
| `SHOULD` / `SHOULD NOT` | Recommended / not recommended |
| `MAY` | Optional |
