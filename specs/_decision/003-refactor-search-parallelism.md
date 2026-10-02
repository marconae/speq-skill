# Decisions: refactor-search-parallelism

## ADR: Parallelize search with order-preserving collection

**ID:** parallelize-with-ordered-collect
**Plan:** refactor-search-parallelism
**Status:** Accepted

### Context

Search indexing and scoring run in parallel. Their output must be identical to a serial run: the same results, ranking, and tie-breaks.

### Decision

Parallel loops collect their results in source order. Future parallel work on search keeps this guarantee.

### Options Considered

- An unordered parallel channel: rejected, it reorders results nondeterministically.
