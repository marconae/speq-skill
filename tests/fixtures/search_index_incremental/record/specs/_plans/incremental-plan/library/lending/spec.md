# Feature: Lending

The system SHALL lend books to registered members.

## Background

* Each member holds a library card with a card number.

## Scenarios

<!-- DELTA:NEW -->
### Scenario: Renew a loan online

* *GIVEN* a member with a loan that ends tomorrow
* *WHEN* the member renews the loan on the library website
* *THEN* the system SHALL extend the due date by two weeks
<!-- /DELTA:NEW -->
