# Decisions: plan-a

## ADR: Use line scanner

**ID:** use-line-scanner
**Plan:** plan-a
**Status:** Superseded by faster-scanner

### Context

We need to parse decision log fragments without pulling in a full Markdown AST dependency.

### Decision

Parse the fragment format directly using a line-oriented state machine.
