# Feature: `CHAR` type pushdown

The `CHAR` type pads stored values with spaces up to its declared length.

Pushdown keeps the padding of `CHAR(n)` columns intact.

## Background

The connector maps `CHAR` columns to fixed-length strings.

* Supported string functions:
  * `substr` with a constant length

## Scenarios

### Scenario: Pushdown of `substr`

* *GIVEN* a column of type `CHAR(10)`
* *WHEN* a query calls `substr` on the column
* *THEN* the connector SHALL push the call down to the database

### Scenario: Pad short values

* *GIVEN* a column of type `CHAR(10)`
* *WHEN* the value `abc` is stored
* *THEN* the connector SHALL return the value padded to ten characters
