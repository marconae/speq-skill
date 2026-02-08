[speq-skill](../README.md) / [Docs](./index.md) / Workflow

---

# Workflow Guide

The speq-skill workflow starts with a one-time **Mission** bootstrap, then follows a repeating **Plan → Implement → Record** cycle.

```
/speq:mission → specs/mission.md  (once per project)
                       │
      ┌────────────────┼────────────────┐
      ▼                ▼                ▼
/speq:plan    →  /speq:implement  →  /speq:record     (repeat)
```

### Steps & References

| Step | Description |
|------|-------------|
| [/speq:mission](#speqmission) | One-time project bootstrap |
| [/speq:plan](#speqplan) | Create spec deltas |
| [/speq:implement](#speqimplement) | Implement plan deltas |
| [/speq:record](#speqrecord) | Merge deltas into permanent specs |
| [Utility Skills](#utility-skills) | Reusable skills |

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

1. **Project Type** — Determines brownfield (existing code) vs. greenfield (new project)
2. **Exploration** — For brownfield projects, explores tech stack, commands, structure
3. **Interview** — Asks clarifying questions about purpose, users, capabilities
4. **Generation** — Creates `specs/mission.md` with all gathered information

### Interview Topics

The agent covers 11 areas, grouping related questions to keep the interview focused:

| Topic | What the agent asks about |
|-------|--------------------------|
| Identity & Purpose | Project name, one-sentence summary, problem statement |
| Target Users | Personas, goals, typical workflows |
| Core Capabilities | 3–5 things the system does (what, not how) |
| Out of Scope | Explicit non-goals and unsupported features |
| Domain Glossary | Project-specific terms and their meanings |
| Tech Stack | Language, runtime, framework, database, testing |
| Commands | Build, test, lint/format, coverage |
| Project Structure | Directory layout and purpose of each directory |
| Architecture | High-level pattern, key components, data flow |
| Constraints | Technical, business, and performance limits |
| External Dependencies | Services/APIs the project depends on |

> [!NOTE]
> `/speq:mission` runs once per project. The following three steps form the repeating development cycle.

---

## `/speq:plan`

Create feature spec deltas including an implementation plan.

### Purpose

- Define new features or changes as spec deltas
- Stage changes in `specs/_plans/<plan-name>/`
- Prepare for implementation with a comprehensive plan

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

---

## `/speq:implement`

Implement approved plan deltas — orchestrates tasks, delegates to sub-agents, reviews code, and produces a verification report.

### Purpose

- Implement planned features and changes according to the spec deltas
- Guide implementation with targeted guardrails
- Generate verification evidence

### When to Use

After `/speq:plan` to implement a plan:

```bash
/speq:implement <plan-name>
```

### What It Does

1. Loads the plan and creates a task breakdown
2. Spawns sub-agents to work through tasks (with context rotation)
3. Loads targeted guardrails for clean code, unit testing and integration testing
4. Runs code review on changed files
5. Executes build, test, and lint verification
6. Generates a verification report

---

## `/speq:record`

Merge implemented spec deltas into the permanent spec library.

### Purpose

- Finalize implemented features
- Update permanent spec library
- Archive completed plans

### When to Use

After a successful `/speq:implement`:

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
6. **Optimize** — Check whether the specs should be re-organized so that the files are kept short and focused
7. **Archive** — Moves plan to `specs/_recorded/<plan-name>/`

---

## Utility Skills

Reusable guidance invoked by workflow skills:

| Skill | Purpose |
|-------|---------|
| `/speq:code-tools` | Semantic code navigation via Serena Model Context Protocol (MCP) |
| `/speq:ext-research` | External docs via Context7 and WebSearch |
| `/speq:code-guardrails` | Code quality guardrails |
| `/speq:git-discipline` | Git read-only rules |
| `/speq:cli` | speq CLI usage patterns |

See [MCP Servers](./mcp-servers.md) for details on Serena and Context7.