[speq-skill](../README.md) / [Docs](./index.md) / Workflow

---

# Workflow Guide

The speq-skill workflow follows a three-phase cycle: **Plan → Implement → Record**.

```
/speq:plan     →    /speq:implement    →    /speq:record
     │                   │                       │
     ▼                   ▼                       ▼
Creates plan.md     Implementation          Merges deltas into
with deltas         + verification          permanent specs library
```

---

## `/speq:mission`

Create a project mission file through an interactive interview.

### Purpose

- Initialize specs for a new project
- Document an existing codebase
- Generate `specs/mission.md` with project context

### When to Use

- Starting a new project with speq-skill
- Adding specs to an existing codebase
- Updating project documentation

### What It Does

1. **Project Detection** — Determines brownfield (existing code) vs greenfield (new project)
2. **Exploration** — For brownfield projects, explores tech stack, commands, structure
3. **Interview** — Asks clarifying questions about purpose, users, capabilities
4. **Generation** — Creates `specs/mission.md` with all gathered information

### Invoked Skills

| Skill | Purpose |
|-------|---------|
| `/speq:code-tools` | Codebase exploration |
| `/speq:ext-research` | Tech stack research |
| `/speq:cli` | Spec structure reference |

---

## `/speq:plan`

Create feature specification deltas for implementation.

### Purpose

- Define new features or changes as spec deltas
- Stage changes in `specs/_plans/<plan-name>/`
- Prepare for implementation with clear specifications

### When to Use

- Starting new feature development
- Modifying existing behavior
- Refactoring with spec-first approach

### Output Structure

```
specs/_plans/<plan-name>/
├── plan.md                           # Implementation plan
└── <domain>/<feature>/spec.md        # Delta specs
```

### Plan Naming Conventions

| Verb | When |
|------|------|
| `add` | New feature |
| `change` | Modify existing |
| `remove` | Deprecate/delete |
| `refactor` | Restructure, same behavior |
| `fix` | Bug or spec mismatch |

Pattern: `<verb>-<feature-scope>[-<qualifier>]`

Examples: `add-user-auth`, `fix-validation-edge-case`, `refactor-search-module`

### Invoked Skills

| Skill | Purpose |
|-------|---------|
| `/speq:code-tools` | Codebase exploration |
| `/speq:ext-research` | API docs and design research |
| `/speq:cli` | Spec discovery and search |

---

## `/speq:implement`

Execute TDD implementation of approved plan deltas.

### Purpose

- Implement features following TDD cycle
- Generate verification evidence
- Prepare for recording to permanent specs

### When to Use

After `/speq:plan` creates a plan:

```bash
/speq:implement <plan-name>
```

### TDD Cycle

```
RED    → Write failing test, run it, show failure
GREEN  → Minimal code to pass, run test, show pass
REFACTOR → Clean up, run test + lint, show output
```

**Golden Rule:** No production code without a failing test first.

### Sub-Agents

The orchestrator spawns specialized sub-agents:

| Agent | Purpose |
|-------|---------|
| `implementer-agent` | Executes implementation tasks |
| `code-reviewer` | Reviews changed files for quality |

Sub-agents rotate to maintain fresh context windows.

### Verification Report

After implementation, generates `verification-report.md` with:

- Build status
- Test results
- Lint output
- Manual testing evidence

### Invoked Skills

| Skill | Purpose |
|-------|---------|
| `/speq:code-tools` | Code navigation and editing |
| `/speq:ext-research` | Library documentation |
| `/speq:code-guardrails` | TDD cycle and quality standards |
| `/speq:cli` | Spec discovery |
| `/speq:git-discipline` | Git read-only rules (sub-agents) |

---

## `/speq:record`

Merge approved plan deltas into permanent specs.

### Purpose

- Finalize implemented features
- Update permanent spec library
- Archive completed plans

### When to Use

After `/speq:implement` generates a verification report:

```bash
/speq:record <plan-name>
```

### What It Does

1. **Verify** — Checks `verification-report.md` exists
2. **Load** — Reads plan and delta specs
3. **Merge** — Applies deltas to permanent specs using markers:

| Marker | Action |
|--------|--------|
| `DELTA:NEW` | Append scenario |
| `DELTA:CHANGED` | Replace scenario with same name |
| `DELTA:REMOVED` | Delete scenario with same name |

4. **Clean** — Strips all DELTA markers
5. **Validate** — Runs `speq feature validate`
6. **Archive** — Moves plan to `specs/_recorded/<plan-name>/`

### Invoked Skills

| Skill | Purpose |
|-------|---------|
| `/speq:code-tools` | File operations |
| `/speq:cli` | Spec validation |

---

## Utility Skills

Reusable guidance invoked by workflow skills:

| Skill | Purpose |
|-------|---------|
| `/speq:code-tools` | Semantic code navigation via Serena MCP |
| `/speq:ext-research` | External docs via Context7 and WebSearch |
| `/speq:code-guardrails` | TDD workflow and code quality guardrails |
| `/speq:git-discipline` | Git read-only rules |
| `/speq:cli` | speq CLI usage patterns |

See [MCP Servers](./mcp-servers.md) for details on Serena and Context7.

---

## Best Practices

### Search First

Before modifying behavior, search for related specs:

```bash
speq search query "error handling"
speq search query "validation"
```

### Never Assume

Use `AskUserQuestion` for:
- Clarifying vague requirements
- Choosing between alternatives
- Confirming design tradeoffs

### Evidence Rule

No claim without evidence. Run commands, show output, then claim.

### Small Specs

Keep specs focused:
- Max ~10 scenarios per spec
- Split large features into sub-features
- Use domains to organize related features

### Clean Context

Use `/clear` between workflow phases to start with fresh context windows.
