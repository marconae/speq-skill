[speq-skill](../README.md) / [Docs](./index.md) / Workflow

---

# Workflow Guide

**Jump to:** [Interactive Workflow](#steps--references) · [Headless PR Pipeline](#headless-pr-pipeline)

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
| [Headless PR Pipeline](#headless-pr-pipeline) | Autonomous plan/implement via a feat/ branch + PR |
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
├── decision-log.md                   # Design decisions (optional)
└── <domain>/<feature>/spec.md        # Delta specs
```

`planner-agent` creates `decision-log.md` automatically during the planning interview, capturing Q&A, design choices, and alternatives considered. Entries marked `Promotes to ADR: yes` are carried into the permanent `specs/decision-log.md` by `recorder-agent` during `/speq:record`. See [Decision Log](./decision-log.md).

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
2. Partitions tasks by tag — `[expert]`-tagged tasks route to `implementer-expert-agent`, all others to `implementer-agent` (see [Model Routing](./model-routing.md))
3. Spawns sub-agents to work through tasks (with context rotation)
4. Loads targeted guardrails for clean code, unit testing and integration testing
5. Runs code review on changed files via `code-reviewer`
6. Executes build, test, and lint verification
7. Generates a verification report

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
7. **Promote decisions** — Entries marked `Promotes to ADR: yes` in `decision-log.md` are appended to `specs/decision-log.md` as the next sequential ADR
8. **Archive** — Moves plan to `specs/_recorded/<plan-name>/`

---

## Headless PR Pipeline

`/speq:plan-pr` and `/speq:implement-pr` run the same Plan → Implement → Record cycle unattended. Autonomous pipelines can't run a live interview, so the Q&A has to be decoupled from planning itself: every decision that would normally be an `AskUserQuestion` prompt either gets a documented, conventional default, or turns into an open question posted on a PR for later reply. Human-in-the-loop is converted into an asynchronous process instead of a synchronous interview. Humans control the pipeline and still the intent via prompting and answering questions, just not in a live chat session.

```
/speq:plan-pr <intent>  →  PR (draft; + open questions if blocked)
                                   │
                    (reply on the PR, or /speq:plan <name> locally)
                                   │
                                   ▼
             /speq:implement-pr <name>  →  same PR, updated + marked ready
```

- **One branch per plan**: `feat/<plan-name>`, created by `/speq:plan-pr` and reused by `/speq:implement-pr` — both push to the same PR, there's no separate plan-only branch namespace.
- **Blocked state**: if planning hits a decision that genuinely needs a human (irreversible, architecturally divergent, or security/compliance relevant), `specs/_plans/<plan-name>/open-questions.md` is written, `plan.md` is flagged blocked, and the PR is opened as a draft with the questions posted as a comment. `/speq:implement-pr` refuses to proceed while this file exists.
- **Resuming**: either reply on the PR and re-run `/speq:plan-pr <plan-name>` (it re-fetches new comments/reviews as answers), or check out the branch and finish interactively with `/speq:plan <plan-name>`.
- **Headless defaults**: `/speq:implement-pr` auto-answers **yes** to `/speq:record`'s library-split question rather than stalling on it.
- **PR title & lifecycle**: the PR is titled with a conventional-commit feature title `<type>(<scope>): <slug>` derived from the plan-name (`add-search-candle` ⇒ `feat(search): add search candle`), not the `spec(plan):` commit prefix. `/speq:plan-pr` opens it as a **draft**; `/speq:implement-pr` marks it **ready** once the implementation is pushed.
- **Git/PR mechanics**: both skills delegate every branch/commit/push/PR operation to `git-agent` — the one sub-agent in this system permitted to write git history or touch a remote directly, keeping both orchestrators as thin as the interactive ones. See [Model Routing](./model-routing.md).

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
| `/speq:writing-guardrails` | Prose style rules for speq artifacts and GitHub PRs/issues/comments |

See [MCP Servers](./mcp-servers.md) for details on Serena and Context7.