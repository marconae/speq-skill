# Decisions: refactor-search-parallelism

## ADR: Parallelize with order-preserving collect, never an unordered channel

**ID:** parallelize-with-ordered-collect
**Plan:** refactor-search-parallelism
**Status:** Accepted

### Context

`speq search` parallelizes two size-scaling loops — the `index_specs` parse loop and the `search_specs` scoring loop — with rayon. The refactor MUST preserve byte-identical output: the same result set, ranking, and tie-breaks as the serial code, per the `cli/feature-search` spec.

### Decision

Both parallelized loops use rayon's `IndexedParallelIterator` (`par_iter()` over a `Vec`/slice) with `collect()`, which preserves source order. `search_specs` collects `(score, &scenario)` pairs in original index order, leaving the existing `sort_by` and its tie-break for equal scores unchanged. `index_specs` collects per-feature scenario batches in feature-discovery order, then flattens in order.

### Options Considered

| Option | Verdict |
|--------|---------|
| Ordered `collect()` over `IndexedParallelIterator` | Chosen — parallelism with byte-identical output |
| Unordered parallel channel or work-queue into a shared `Vec` | Rejected — reorders scenarios nondeterministically, changing index layout and equal-score tie-breaks |

### Consequences

Parallelization gains a measured 38.24% index-build speedup without changing observable behavior, so no `cli/feature-search` spec delta is needed. Future parallel work on this code path must keep using ordered collects to hold the same guarantee.
