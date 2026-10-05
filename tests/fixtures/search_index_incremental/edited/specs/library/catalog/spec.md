# Feature: Catalog

The system SHALL keep a catalog of the books on the shelves.

## Background

* Each book in the catalog has a title, an author, and a shelf code.

## Scenarios

### Scenario: Add a book to the catalog

* *GIVEN* a librarian with a new book in hand
* *WHEN* the librarian enters the title, the author, and the shelf code
* *THEN* the system SHALL store the book in the catalog

### Scenario: Find a book by its author

* *GIVEN* a catalog with books from many authors
* *WHEN* a reader searches for the first and the last name of an author
* *THEN* the system SHALL list every book of that author, sorted by publication year
