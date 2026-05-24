# Feature: CLI Domain List

The CLI SHALL provide a command to list all domains in the specs directory.

## Background

* Command syntax: `speq domain list`
* A domain is a top-level directory in `specs/` (excluding `_plans`, `_recorded`, and hidden directories)
* Output format shows domain names with trailing slash
* Exit code 0 on success

## Scenarios

### Scenario: List all domains

* *GIVEN* a specs directory with domains `cli`, `core`, and `validation`
* *WHEN* the user runs `speq domain list`
* *THEN* the system SHALL display each domain name followed by `/`
* *AND* the system SHALL list domains in alphabetical order
* *AND* the system SHALL exit with code 0

### Scenario: Empty specs directory

* *GIVEN* a specs directory with no domains
* *WHEN* the user runs `speq domain list`
* *THEN* the system SHALL display "No domains found."
* *AND* the system SHALL exit with code 0
