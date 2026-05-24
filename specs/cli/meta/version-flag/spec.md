# Feature: Version Flag

Display the CLI version to help users verify their installation and report issues.

## Background

The speq CLI is distributed as a single binary. Users need to verify which version they have installed.

## Scenarios

### Scenario: Print version with --version flag

* *GIVEN* the speq CLI is installed
* *WHEN* the user runs `speq --version`
* *THEN* the CLI SHALL print the version in format `speq <semver>`
* *AND* the CLI SHALL exit with code 0

### Scenario: Print version with -V shorthand

* *GIVEN* the speq CLI is installed
* *WHEN* the user runs `speq -V`
* *THEN* the CLI SHALL print the version in format `speq <semver>`
* *AND* the CLI SHALL exit with code 0
