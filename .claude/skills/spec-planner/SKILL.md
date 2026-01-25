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

#### 4.6 Design Section (for significant changes)

For new features and major changes, include `## Design` in plan.md:
- **Goals / Non-Goals** — Scope boundaries
- **Architecture** — Components, layers, data flow
- **Design Patterns** — Patterns used and rationale
- **Trade-offs** — Decisions made and alternatives considered
- **Key Interfaces** — Core types and signatures

Skip for small fixes and minor changes.

#### 4.7 Generate plan.md

Create `specs/_plans/<plan-name>/plan.md`:

1. Read `specs/mission.md` to get project-specific commands and tech stack
2. Use `references/plan-template.md` as structural guide (NOT copy-paste)
3. Generate concrete content for each section based on the actual plan

**Critical:** The template shows structure only. Replace ALL placeholders with actual content.

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

## Verification Section Guidelines

The plan's `## Verification` section MUST contain concrete, executable tasks—not template placeholders.

### Reading the Mission

Before writing verification, read `specs/mission.md § Commands` to get:
- Exact test command (e.g., `cargo test`, `npm test`)
- Exact lint command (e.g., `cargo clippy -- -D warnings`)
- Exact format command (e.g., `cargo fmt --check`)
- Exact coverage command (e.g., `cargo tarpaulin --out Html`)

### Verification Checklist (generate actual commands)

Write the verification section with actual commands from the mission:

```markdown
## Verification

### Checklist

| Step | Command | Expected |
|------|---------|----------|
| Build | `<actual build cmd>` | Exit 0 |
| Test | `<actual test cmd>` | 0 failures |
| Lint | `<actual lint cmd>` | 0 errors/warnings |
| Format | `<actual fmt cmd>` | No changes |
| Coverage | `<actual coverage cmd>` | ≥80% |

### Manual Testing

| Feature | Test Steps | Expected Result |
|---------|------------|-----------------|
| <feature from plan> | <concrete steps> | <observable outcome> |
```

### Manual Testing (generate from features)

For each feature in the plan, generate concrete manual test steps:

1. Look at the plan's `## Features` table
2. For each feature, write specific CLI commands or UI actions
3. Describe the expected observable outcome

Example (good):
```markdown
| domain list | Run `speq domain list` in project with specs/ | Lists: cli, core |
| feature get | Run `speq feature get cli/validate` | Prints feature spec |
```

Example (bad - template placeholder):
```markdown
| <feature-name> | <concrete steps> | <observable outcome> |
```

### Completion Criteria

1. ALL scenarios have passing integration tests
2. ALL tests pass (verified with fresh output)
3. Code coverage ≥80% (verified with fresh output)
4. Lint and format pass
5. Dead code removed
6. Manual testing completed for all features