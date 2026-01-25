# Feature: Test Valid Spec

This is a valid feature specification for testing the validator.

## Background

The system is in a known state.

## Scenarios

### Scenario: Validation passes

* *GIVEN* a valid specification exists
* *WHEN* the validator runs
* *THEN* the system SHALL report success
* *AND* the system SHALL exit with code 0
