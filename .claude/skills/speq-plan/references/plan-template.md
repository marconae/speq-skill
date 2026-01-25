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

### Goals / Non-Goals

- Goals
    - <Primary objective this design achieves>
    - <Secondary objectives>
- Non-Goals
    - <What this design explicitly does NOT address>
    - <Scope boundaries to prevent creep>

### Architecture

High-level system structure: components, layers, data flow

```
┌─────────────┐     ┌─────────────┐
│ Component A │────▶│ Component B │
└─────────────┘     └─────────────┘
```

### Design Patterns

| Pattern | Where | Why |
|---------|-------|-----|
| <pattern> | <component> | <rationale> |

### Trade-offs

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

Optional: Tasks that can run concurrently

| Parallel Group | Tasks |
|----------------|-------|
| Group A | Task 1, Task 2 |
| Group B | Task 3, Task 4 |

Sequential dependencies:
- Group A → Group B (B depends on A)

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

### Checklist

<!-- Read specs/mission.md and fill with ACTUAL commands -->

| Step | Command | Expected |
|------|---------|----------|
| Build | `<from mission.md>` | Exit 0 |
| Test | `<from mission.md>` | 0 failures |
| Lint | `<from mission.md>` | 0 errors/warnings |
| Format | `<from mission.md>` | No changes |
| Coverage | `<from mission.md>` | ≥80% |

### Manual Testing

<!-- Generate from Features table above. One row per feature with concrete steps. -->

| Feature | Test Steps | Expected Result |
|---------|------------|-----------------|
| <feature from table above> | <actual CLI command or UI action> | <observable outcome> |

### Scenario Verification

<!-- Map each scenario from feature specs to a test -->

| Scenario | Test Type | Test Location |
|----------|-----------|---------------|
| <scenario-name> | Integration / Unit | `<test-path>` |

- **Integration test** — Default for all scenarios
- **Unit test** — Only if scenario is isolated and small

A feature is complete when ALL scenarios have passing integration tests.
