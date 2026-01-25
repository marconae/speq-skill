# Feature: CLI Feature List

The CLI SHALL provide a command to list feature specifications in a tree view format, optionally filtered by domain.

## Background

* Command syntax: `speq feature list [domain]`
* Output format is a tree view showing domain/feature hierarchy
* Exit code 0 on success, 1 on error

## Scenarios

### Scenario: List all features

* *GIVEN* a specs directory with multiple domains and features
* *WHEN* the user runs `speq feature list`
* *THEN* the system SHALL display all features grouped by domain in tree format
* *AND* the system SHALL exit with code 0

### Scenario: List features in specific domain

* *GIVEN* a specs directory with domain `validation` containing features
* *WHEN* the user runs `speq feature list validation`
* *THEN* the system SHALL display only features in the `validation` domain
* *AND* the system SHALL exit with code 0

### Scenario: Domain not found

* *GIVEN* no domain named `nonexistent` exists
* *WHEN* the user runs `speq feature list nonexistent`
* *THEN* the system SHALL report an error indicating the domain was not found
* *AND* the system SHALL exit with code 1

### Scenario: Empty specs directory

* *GIVEN* a specs directory with no domains or features
* *WHEN* the user runs `speq feature list`
* *THEN* the system SHALL display a message indicating no features found
* *AND* the system SHALL exit with code 0

### Scenario: Tree view format

* *GIVEN* domains `cli` with feature `validate` and `validation` with features `document-structure` and `scenario-structure`
* *WHEN* the user runs `speq feature list`
* *THEN* the output SHALL show a tree structure with domains as parents and features as children
