# Decisions: avoid-decision-log-conflicts

## ADR: Store decisions as one fragment per plan

**ID:** per-plan-decision-fragments
**Plan:** avoid-decision-log-conflicts
**Status:** Accepted

### Context

Parallel plans record decisions at the same time. A single shared decision file makes them conflict in git.

### Decision

Store one committed ADR fragment per plan. Different plans write different files, so git never conflicts. `speq decision-log show` assembles the fragments for reading.

### Options Considered

- One shared file with smarter merging or a write lock: rejected, it still forces coordination on one file.

## ADR: Identify ADRs by slug, not by number

**ID:** slug-identity-not-sequential-numbers
**Plan:** avoid-decision-log-conflicts
**Status:** Accepted

### Context

Sequential numbers need one shared counter. Parallel plans cannot agree on the next number without coordination.

### Decision

Identify each ADR by a kebab-case slug that is unique across all fragments. References between ADRs use slugs.
