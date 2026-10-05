# Feature: Lending

The system SHALL lend books to registered members.

## Background

* Each member holds a library card with a card number.

## Scenarios

### Scenario: Borrow a book with a library card

* *GIVEN* a member with a valid library card
* *WHEN* the member checks out a book at the front desk
* *THEN* the system SHALL record the loan with a due date

### Scenario: Charge a fee for a late return

* *GIVEN* a loan whose due date passed ten days ago
* *WHEN* the member returns the book
* *THEN* the system SHALL charge a late fee for each overdue day

### Scenario: Send a reminder before the due date

* *GIVEN* a loan that ends in three days
* *WHEN* the nightly reminder job runs
* *THEN* the system SHALL email the member a reminder about the due date
