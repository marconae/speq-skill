# Feature: Catalog

The system SHALL keep a catalog of the books on the shelves.

## Background

* Each book in the catalog has a title, an author, and a shelf code.

## Scenarios

<!-- DELTA:CHANGED -->
### Scenario: Find a book by its author

* *GIVEN* a catalog with books from many authors
* *WHEN* a reader searches for the first and the last name of an author
* *THEN* the system SHALL list every book of that author, sorted by publication year
<!-- /DELTA:CHANGED -->

<!-- DELTA:REMOVED -->
### Scenario: Retire a damaged book

* *GIVEN* a book with torn pages and a broken spine
* *WHEN* the librarian marks the book as damaged
* *THEN* the system SHALL remove the book from the shelf list
<!-- /DELTA:REMOVED -->
