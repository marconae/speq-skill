# Plan: <plan-name>

<!--
STRUCTURAL TEMPLATE - DO NOT COPY-PASTE
Generate actual content for each section based on your specific plan.
Read `specs/mission.md` for project-specific commands.
-->

## Summary

One-paragraph description of what this plan achieves (max two sentences).

## Design

Required for new features and significant changes. Skip for small fixes.

### Context

The problem, the forces at play, and why it needs a design decision.

- **Goals** — <what this design achieves>
- **Non-Goals** — <what this design explicitly does NOT address>

### Decision

The chosen approach: architecture, patterns, and key interfaces.

#### Architecture

High-level system structure: components, layers, data flow

```
┌─────────────┐     ┌─────────────┐
│ Component A │────▶│ Component B │
└─────────────┘     └─────────────┘
```

#### Patterns

| Pattern | Where | Why |
|---------|-------|-----|
| <pattern> | <component> | <rationale> |

### Consequences

| Decision | Alternatives Considered | Rationale |
|----------|------------------------|-----------|
| <choice made> | <other options> | <why this choice> |

## Features

| Feature | Status | Spec |
|---------|--------|------|
| <feature-name> | NEW / CHANGED / REMOVED | `<path>/spec.md` |

Status values:
- **NEW** — Feature doesn't exist yet
- **CHANGED** — Modifying existing feature behavior
- **REMOVED** — Deprecating/deleting feature

## Impact

What changes for users, operators, or downstream systems once this ships. Call out breaking changes explicitly. Write "None" if there is no user-facing impact.

## Requirements

Optional: High-level requirements if not fully captured in feature specs

| Requirement | Details |
|-------------|---------|
| ... | ... |

## Dependencies

Optional: External dependencies, libraries, or prerequisite work

## Migration

Optional: For changes affecting existing data/structure

| Current | New |
|---------|-----|
| ... | ... |

## Implementation Tasks

1. Task description
2. Task description
3. ...

## Parallelization

Optional: task groups for the implement orchestrator. Each group is a knowledge cluster — a vertical slice of one spec delta plus the source and test files it governs. Fixtures, module code, and the feature's tests belong in the same group, not in separate layer groups.

| Group | Tasks | Depends on | Knowledge |
|-------|-------|------------|-----------|
| A: <cluster name> | 1.1-1.4, 3.1 | — | spec delta `<domain>/<feature>`; `src/<module>/`, `<test-file-path>` |
| B: <cluster name> | 2.1-2.3 | A (shares `src/<module>/`) | spec delta `<domain>/<other-feature>`; `src/<module>/`, `<test-file-path>` |

- **Knowledge** — the group's spec delta path(s) plus the source and test files they govern. The implement orchestrator passes this entry to the group's agent as its orientation pointer.
- Tasks that share a spec delta or a source module default into one group.
- Overlapping Knowledge entries across groups are a consolidation signal, not a parallelism opportunity — merge the groups, or declare a dependency and run them in sequence.

## Dead Code Removal

Required: Identify obsolete code to remove

| Type | Location | Reason |
|------|----------|--------|
| Function | `<path>` | Replaced by X |
| Test | `<path>` | Tests removed feature |
| Module | `<path>` | No longer used |

## Verification

<!--
IMPORTANT: Generate actual commands from specs/mission.md § Commands.
Do NOT copy placeholders below. Replace with real values.
-->

### Scenario Coverage

<!-- Map EVERY scenario from feature specs to an integration test. No gaps allowed. -->

| Scenario | Test Type | Test Location | Test Name |
|----------|-----------|---------------|-----------|
| <scenario from spec> | Integration / Unit | `<test-file-path>` | `<test_function_name>` |

- **Integration test** — default for all scenarios
- **Unit test** — only for pure computation with no I/O or side effects
- A feature is complete when ALL its scenarios have passing tests

### Manual Testing

<!-- One entry per feature. Concrete commands against the built software. -->

| Feature | Command | Expected Output |
|---------|---------|-----------------|
| <feature from table above> | `<actual CLI command or action>` | <observable outcome> |

### Checklist

<!-- Read specs/mission.md § Commands. Fill with ACTUAL commands, no placeholders. -->

| Step | Command | Expected |
|------|---------|----------|
| Build | `<from mission.md>` | Exit 0 |
| Test | `<from mission.md>` | 0 failures |
| Lint | `<from mission.md>` | 0 errors/warnings |
| Format | `<from mission.md>` | No changes |
