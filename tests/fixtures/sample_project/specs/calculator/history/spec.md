# Feature: Calculation History

Maintains a log of previous calculations for reference and reuse.

## Background

The calculator stores up to 100 most recent calculations in memory.

## Scenarios

### Scenario: View recent calculations

* *GIVEN* the user has performed 5 calculations
* *WHEN* the user opens the history panel
* *THEN* all 5 calculations SHALL be displayed
* *AND* the most recent calculation SHALL appear first

### Scenario: Reuse previous result

* *GIVEN* the history contains "15 + 25 = 40"
* *WHEN* the user taps on that history entry
* *THEN* "40" SHALL be loaded into the current calculation
* *AND* the user MAY continue calculating from that value

### Scenario: Clear history

* *GIVEN* the history contains multiple entries
* *WHEN* the user selects "Clear History"
* *THEN* all history entries SHALL be removed
* *AND* the history panel SHALL show "No calculations yet"

### Scenario: History persists across sessions

* *GIVEN* the user has performed calculations
* *WHEN* the application is closed and reopened
* *THEN* the calculation history SHALL be preserved
