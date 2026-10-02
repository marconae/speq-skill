# Feature: Spec Hierarchy

The system SHALL organize feature specifications in a shallow two-level hierarchy of domain and feature to improve discoverability and scalability.

## Background

* Specs are stored at `specs/<domain>/<feature>/spec.md`
* Domains group related features (e.g., `cli/`, `validation/`)
* Maximum nesting depth is 2 levels (domain/feature)
* Top-level directories whose names start with `_` are reserved: `_plans/` and `_recorded/` hold plan management, and `_decision/` holds the permanent decision records
* Top-level directories whose names start with `.` are hidden and are not domains
* Domain and feature names use kebab-case

## Scenarios

### Scenario: Valid spec path structure

* *GIVEN* a feature specification file
* *WHEN* the system resolves the spec path
* *THEN* the path SHALL follow the pattern `specs/<domain>/<feature>/spec.md`

### Scenario: Discover all domains

* *GIVEN* a specs directory with multiple domain subdirectories
* *WHEN* the system lists domains
* *THEN* the system SHALL return all immediate subdirectories except those whose names start with `_` or `.`

### Scenario: Discover features in domain

* *GIVEN* a domain directory containing feature subdirectories
* *WHEN* the system lists features in that domain
* *THEN* the system SHALL return all subdirectories containing a `spec.md` file

### Scenario: Discover all features

* *GIVEN* a specs directory with domains and features
* *WHEN* the system discovers all features
* *THEN* the system SHALL return all features across all domains
* *AND* each feature SHALL include its domain and feature name

### Scenario: Reserved directories excluded

* *GIVEN* a specs directory containing `_plans/`, `_recorded/`, and `_decision/` directories
* *WHEN* the system discovers domains or features
* *THEN* the system SHALL NOT include `_plans`, `_recorded`, or `_decision` as domains

### Scenario: Empty domain ignored

* *GIVEN* a domain directory with no feature subdirectories
* *WHEN* the system discovers features
* *THEN* the system SHALL NOT include the empty domain in results
