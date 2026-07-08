# Decisions: plan-a

## ADR: Use line scanner

**ID:** use-line-scanner
**Plan:** plan-a
**Status:** Accepted

### Context

Decision logs have a flat, regular Markdown structure that does not require a full AST.

### Decision

Use a line-oriented state machine to parse decision logs.
