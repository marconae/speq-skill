# Feature: Basic Calculator Operations

Provides fundamental arithmetic operations for the calculator application.

## Background

The calculator application is initialized and ready to accept input.

## Scenarios

### Scenario: Addition of two positive numbers

* *GIVEN* the calculator is reset to zero
* *WHEN* the user enters "5 + 3"
* *THEN* the display SHALL show "8"

### Scenario: Subtraction resulting in negative number

* *GIVEN* the calculator is reset to zero
* *WHEN* the user enters "3 - 7"
* *THEN* the display SHALL show "-4"

### Scenario: Multiplication of decimal numbers

* *GIVEN* the calculator is reset to zero
* *WHEN* the user enters "2.5 * 4"
* *THEN* the display SHALL show "10"

### Scenario: Division by zero error

* *GIVEN* the calculator is reset to zero
* *WHEN* the user enters "10 / 0"
* *THEN* the display SHALL show "Error: Division by zero"
* *AND* the calculator SHALL remain in error state until cleared

### Scenario: Chained operations

* *GIVEN* the calculator is reset to zero
* *WHEN* the user enters "2 + 3 * 4"
* *THEN* the display SHALL show "14"
* *AND* the system SHALL respect operator precedence
