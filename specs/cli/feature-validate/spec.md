# Feature: CLI Feature Validate

The CLI SHALL provide a command to validate feature specifications with support for validating all features, all features in a domain, or a single feature.

## Background

* Command syntax: `speq feature validate [target]`
* Target can be empty (all), domain name, or `domain/feature`
* Exit code 0 if all validations pass, 1 if any fail
* Warnings do not affect exit code

## Scenarios

### Scenario: Validate all features

* *GIVEN* a specs directory with multiple features
* *WHEN* the user runs `speq feature validate`
* *THEN* the system SHALL validate every feature spec
* *AND* the system SHALL report results for each feature

### Scenario: Validate all features in domain

* *GIVEN* a domain `validation` with multiple features
* *WHEN* the user runs `speq feature validate validation`
* *THEN* the system SHALL validate only features in the `validation` domain
* *AND* the system SHALL NOT validate features in other domains

### Scenario: Validate single feature

* *GIVEN* a feature at `specs/cli/validate/spec.md`
* *WHEN* the user runs `speq feature validate cli/validate`
* *THEN* the system SHALL validate only the specified feature
* *AND* the system SHALL report the validation result

### Scenario: All validations pass

* *GIVEN* all targeted features have valid specs
* *WHEN* the user runs the validate command
* *THEN* the system SHALL report success for each feature
* *AND* the system SHALL exit with code 0

### Scenario: Some validations fail

* *GIVEN* some targeted features have invalid specs
* *WHEN* the user runs the validate command
* *THEN* the system SHALL report errors for each failing feature
* *AND* the system SHALL continue validating remaining features
* *AND* the system SHALL exit with code 1

### Scenario: Domain not found

* *GIVEN* no domain named `nonexistent` exists
* *WHEN* the user runs `speq feature validate nonexistent`
* *THEN* the system SHALL report an error indicating the domain was not found
* *AND* the system SHALL exit with code 1

### Scenario: Feature not found

* *GIVEN* no feature at `cli/nonexistent` exists
* *WHEN* the user runs `speq feature validate cli/nonexistent`
* *THEN* the system SHALL report an error indicating the feature was not found
* *AND* the system SHALL exit with code 1

### Scenario: Summary at end

* *GIVEN* multiple features to validate
* *WHEN* the validation completes
* *THEN* the system SHALL display a summary with counts of passed, failed, and warnings
