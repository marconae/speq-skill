# Architecture Decision Records

## ADR-001: Use line-oriented state machine for parsing

**Date:** 2026-04-27
**Plan:** add-decision-log-validation
**Status:** Accepted

### Context

Decision logs have a flat, regular Markdown structure that does not require a full AST.

### Decision

Use a line-oriented state machine to parse decision logs.

## ADR-003: Register module in validate mod

**Date:** 2026-04-27
**Plan:** add-decision-log-validation
**Status:** Accepted

### Context

The decision_log module must be registered in the validate module tree.

### Decision

Add `pub mod decision_log;` to `src/validate/mod.rs`.
