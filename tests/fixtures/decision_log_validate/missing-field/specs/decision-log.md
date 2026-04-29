# Architecture Decision Records

## ADR-001: Use line-oriented state machine for parsing

**Date:** 2026-04-27
**Plan:** add-decision-log-validation

### Context

Decision logs have a flat, regular Markdown structure that does not require a full AST.

### Decision

Use a line-oriented state machine to parse decision logs.
