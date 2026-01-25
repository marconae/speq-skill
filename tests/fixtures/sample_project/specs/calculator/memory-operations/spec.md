# Feature: Calculator Memory Operations

Provides memory storage and recall functionality for intermediate calculation results.

## Background

The calculator application supports memory storage with M+, M-, MR, and MC buttons.

## Scenarios

### Scenario: Store value in memory

* *GIVEN* the calculator displays "42"
* *WHEN* the user presses "M+"
* *THEN* the value "42" SHALL be stored in memory
* *AND* the memory indicator SHALL be visible

### Scenario: Recall stored value

* *GIVEN* the memory contains "25"
* *AND* the calculator is reset to zero
* *WHEN* the user presses "MR"
* *THEN* the display SHALL show "25"

### Scenario: Clear memory

* *GIVEN* the memory contains "100"
* *WHEN* the user presses "MC"
* *THEN* the memory SHALL be cleared
* *AND* the memory indicator SHALL be hidden

### Scenario: Add to existing memory

* *GIVEN* the memory contains "50"
* *AND* the calculator displays "30"
* *WHEN* the user presses "M+"
* *THEN* the memory SHALL contain "80"
