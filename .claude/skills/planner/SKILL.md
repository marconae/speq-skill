---
name: planner
description: |
  Plan mode workflow for creating and managing feature specification deltas.
  Use when entering plan mode to define new features or changes to existing features.
  Invoke explicitly with /spec-planner when planning implementation work.
  Creates delta specs in specs/_plans/<plan-name>/ following schemas/delta.md.
---

# Spec Planner

Guide plan mode workflow for feature specification management.

## Spec Hierarchy

Specs use a domain/feature hierarchy:

```
specs/
├── <domain>/
│   └── <feature>/
│       └── spec.md
├── _plans/          # Active plans
│   └── <plan-name>/
│       └── <domain>/<feature>/spec.md
└── _recorded/       # Archived plans
```

## Workflow

### 1. Initialize Plan

```
Ask user for plan name
Create: specs/_plans/<plan-name>/plan.md
```

### 2. Discover Existing Features

```bash
speq feature list
speq feature list <domain>
```

### 3. For Each Feature

```
Check: specs/<domain>/<feature>/spec.md exists?
├─ Yes → Existing feature (use DELTA markers)
└─ No  → New feature (full spec)

Create: specs/_plans/<plan-name>/<domain>/<feature>/spec.md
```

### 4. Validate

```bash
speq feature validate <domain>/<feature>
```

### 5. Exit Plan Mode

Call `ExitPlanMode` when specs are complete and validated.

## Templates

### New Feature

Use full spec format from `schemas/feature.md`:

```markdown
# Feature: <name>

<description>

## Background

<invariants>

## Scenarios

### Scenario: <name>

* *GIVEN* <state>
* *WHEN* <event>
* *THEN* <outcome with RFC 2119 keyword>
```

### Existing Feature Changes

Use delta format from `schemas/delta.md`:

```markdown
## Scenarios

<!-- DELTA:NEW -->
### Scenario: <new scenario>
...
<!-- /DELTA:NEW -->

<!-- DELTA:CHANGED -->
### Scenario: <modified scenario>
...
<!-- /DELTA:CHANGED -->

<!-- DELTA:REMOVED -->
### Scenario: <removed scenario>
<!-- /DELTA:REMOVED -->
```

## RFC 2119 Keywords

THEN steps MUST use: `MUST`, `MUST NOT`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `MAY`

## Plan File Template

```markdown
# Plan: <plan-name>

## Summary

<brief description of changes>

## Features

| Domain | Feature | Status | Spec |
|--------|---------|--------|------|
| <domain> | <feature> | NEW/CHANGED | `<domain>/<feature>/spec.md` |

## Verification

1. Run `speq feature validate` to validate all
2. Run `speq feature validate <domain>` to validate domain
3. Run `speq feature validate <domain>/<feature>` for specific feature
4. Review scenarios for completeness
5. Exit plan mode for approval
```
