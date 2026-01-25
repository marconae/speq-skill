---
name: spec-planner
description: |
  Plan mode workflow for creating and managing feature specification deltas.
  Use when entering plan mode to define new features or changes to existing features.
  Invoke explicitly with /spec-planner when planning implementation work.
  Creates delta specs in specs/_plans/<plan-name>/ following the plan template.
  Triggers: planning mode, feature specification, spec planning, implementation planning.
---

# Spec Planner

You are creating and managing feature specification deltas. 

Note: Feature specification deltas are recorded into the permanent specs directory `specs/<domain>/<feature>` when implementation work is complete using the spec-recorder skill.

## Guiding Principles

| Principle | Rule |
|-----------|------|
| **Integration Test > Unit Test** | Integration tests default; unit tests for isolated scenarios |
| **Evidence Before Claims** | No claim without fresh command output |
| **Dead Code = Debt** | Identify and remove obsolete code in every plan |

## Workflow

### 1. Discovery

```bash
speq feature list
speq feature list <domain>
```

- Existing specs → modify with DELTA markers
- Missing features → add as new specs

### 2. Research

- Use `WebSearch` for topic research
- Context7 MCP for third-party library capabilities

### 3. Clarifying Interview

Use `AskUserQuestion` — MUST NOT assume:
- Clarify vague requirements
- Choose between solutions found in research
- Confirm design decisions and tradeoffs

### 4. Planning

#### 4.1 Generate Plan Name

Pattern: `<verb>-<feature-scope>[-<qualifier>]`

| Verb | When |
|------|------|
| `add` | New feature |
| `change` | Modify existing |
| `remove` | Deprecate/delete |
| `refactor` | Restructure, same behavior |
| `fix` | Spec/implementation mismatch, bugs |

#### 4.2 Generate Spec Deltas

```
specs/<domain>/<feature>/spec.md exists?
├─ Yes → Use DELTA markers (see references/delta-template.md)
└─ No  → Full spec (see references/feature-template.md)

Create: specs/_plans/<plan-name>/<domain>/<feature>/spec.md
```

#### 4.3 Identify Dead Code

| Look For | Action |
|----------|--------|
| Replaced functions | Mark for removal |
| Obsolete tests | Mark for removal |
| Unused imports/modules | Mark for removal |

#### 4.4 Plan Parallelization

```
Independent tasks → Parallel group
Dependent tasks → Sequential chain
```

#### 4.5 Map Scenarios to Tests

| Scenario Type | Test Type |
|---------------|-----------|
| Multiple components | Integration test |
| Isolated, single unit | Unit test |

#### 4.6 Generate plan.md

Create `specs/_plans/<plan-name>/plan.md` using template in `references/plan-template.md`.

### 5. Exit Plan Mode

Call `ExitPlanMode` when workflow is complete.

## Spec Hierarchy

```
specs/
├── <domain>/<feature>/spec.md     # Permanent specs
├── _plans/<plan-name>/            # Active plans
│   ├── plan.md
│   └── <domain>/<feature>/spec.md
└── _recorded/<plan-name>/         # Archived plans
```

## RFC 2119 Keywords

THEN steps MUST use: `MUST`, `MUST NOT`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `MAY`

## Verification Rules

### Evidence Requirements

| Claim | Required Evidence |
|-------|-------------------|
| "Tests pass" | Fresh run showing 0 failures |
| "Lint clean" | Output showing 0 errors |
| "Coverage ~80%" | Coverage command output from `specs/mission.md § Tech Stack` |
| "Feature works" | Integration test covering all scenarios |

**Code Coverage:** Run the coverage command from `specs/mission.md § Tech Stack`. Target ~80%. Evidence MUST be fresh command output.

### Completion Criteria

1. ALL scenarios have passing integration tests
2. ALL tests pass (unit + integration + e2e) - verified with measured test results as evidence
3. Code coverage ~80% (verified with measured coverage data as evidence)
4. Lint and format pass via tooling
5. Dead code removed
6. Evidence shown for each claim
