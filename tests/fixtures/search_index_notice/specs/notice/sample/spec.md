# Feature: Notice Sample

The system SHALL validate documents.

## Background

* A document is available for validation.

## Scenarios

### Scenario: Valid document passes validation

* *GIVEN* a well-formed document
* *WHEN* the document is validated
* *THEN* the system SHALL report that the document is valid
