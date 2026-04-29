# Architecture Decision Records

## ADR-001: Use line-oriented state machine for parsing

**Date:** 2026-04-27
**Plan:** add-decision-log-validation
**Status:** Accepted

### Context

Decision logs are flat Markdown files with a regular structure: H1, H2 sections, and bold-prefixed fields. A full Markdown AST via pulldown-cmark would introduce an external dependency and more complexity than necessary for this constrained format.

### Decision

Parse decision logs using a line-oriented state machine that tracks current section and extracts fields by scanning for known prefixes.

### Options Considered

- Full Markdown AST via pulldown-cmark: more powerful but heavier for this use case.
- Regex-only approach: brittle for multi-line fields.

### Consequences

- Simpler implementation with no additional dependencies.
- Parser is less general but sufficient for the decision log format.
- Format changes require updating the state machine, not a grammar.
