---
name: speq-plan
description: |
  Plan mode workflow for creating feature specification deltas.
  Use when entering plan mode to define new features or changes.
  Invoke with /speq-plan when planning implementation work.
  Creates delta specs in specs/_plans/<plan-name>/.
  Triggers: planning mode, feature specification, spec planning, plan feature.
---

# Spec Planner

Create and manage feature specification deltas. Deltas are recorded to permanent specs via `/speq-record`.

## Required Skills

Invoke before starting:
- `/speq-code-tools` — Codebase exploration
- `/speq-ext-research` — API docs and design research
- `/speq-cli` — Spec discovery and search

## Guiding Principles

- Integration tests default; unit tests for isolated scenarios only

## Workflow

### 1. Discovery

Use speq CLI to explore existing specs:

```bash
# Browse structure
speq domain list                    # List domains
speq feature list                   # Tree view of all features
speq feature list <domain>          # Features in domain

# Semantic search (find related specs)
speq search query "error handling"  # Find scenarios about errors
speq search query "validation"      # Find validation-related specs

# Get specific content
speq feature get cli/validate                        # Full feature spec
speq feature get "cli/validate/Validation fails"    # Single scenario
```

**Search first** when modifying behavior — find related scenarios to understand scope.

### 2. Research

Invoke `/speq-ext-research` and conduct research for:
- External libraries and APIs
- Design patterns and architecture

### 3. Clarifying Interview

Use `AskUserQuestion` — never assume:
- Clarify vague requirements
- Choose between alternative solutions
- Confirm design tradeoffs

### 4. Planning

#### 4.1 Plan Name

Pattern: `<verb>-<feature-scope>[-<qualifier>]`

| Verb | When |
|------|------|
| `add` | New feature |
| `change` | Modify existing |
| `remove` | Deprecate/delete |
| `refactor` | Restructure, same behavior |
| `fix` | Bug or spec mismatch |

#### 4.2 Spec Deltas

```
specs/<domain>/<feature>/spec.md exists?
├─ Yes → DELTA markers (references/delta-template.md)
└─ No  → Full spec (references/feature-template.md)

Output: specs/_plans/<plan-name>/<domain>/<feature>/spec.md
```

#### 4.3 Test Mapping

| Scenario Type | Test Type |
|---------------|-----------|
| Multiple components | Integration |
| Single isolated unit | Unit |

#### 4.4 Design Section

For new features/major changes, add `## Design` to plan.md:
- Goals / Non-Goals
- Architecture
- Trade-offs
- Key Interfaces

Skip for minor fixes.

#### 4.5 Generate plan.md

1. Read `specs/mission.md` for commands and tech stack
2. Use `references/plan-template.md` as structure guide
3. Generate concrete content (no template placeholders)

### 5. Exit Plan Mode

Call `ExitPlanMode` when complete.

## Spec Hierarchy

```
specs/
├── <domain>/<feature>/spec.md     # Permanent
├── _plans/<plan-name>/            # Active
└── _recorded/<plan-name>/         # Archived
```

## RFC 2119 Keywords

THEN steps use: `MUST`, `MUST NOT`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `MAY`

## Verification Section

Read `specs/mission.md § Commands` for project-specific build/test/lint/format commands.

The plan's `## Verification` section MUST include tasks that produce evidence:

1. **Checklist tasks** — Build, test, lint, format commands from mission.md
2. **Manual testing tasks** — Concrete steps for each feature in the plan
3. **Scenario coverage** — Map each spec scenario to its test location

No placeholders. Use actual commands from the project's mission.md.
