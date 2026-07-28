[speq-skill](../README.md) / [Docs](./index.md) / Workflow

---

# Workflow Guide

**Jump to:** [Interactive workflow](#steps-and-references) · [Headless PR Pipeline](#headless-pr-pipeline)

The speq-skill workflow starts with a one-time **Mission** bootstrap, then repeats a **Plan → Implement → Record** cycle.

```
/speq:mission → specs/mission.md  (once per project)
                       │
      ┌────────────────┼────────────────┐
      ▼                ▼                ▼
/speq:plan    →  /speq:implement  →  /speq:record     (repeat)
```

### Steps and references

| Step | Description |
|------|-------------|
| [/speq:mission](#speqmission) | One-time project bootstrap |
| [/speq:plan](#speqplan) | Create spec deltas |
| [/speq:implement](#speqimplement) | Implement plan deltas |
| [/speq:record](#speqrecord) | Merge deltas into permanent specs |
| [/speq:audit](#speqaudit) | Health-check the spec library and guide fixes |
| [Headless PR Pipeline](#headless-pr-pipeline) | Autonomous plan and implement via a feat/ branch and PR |
| [Utility skills](#utility-skills) | Reusable skills |

---

## `/speq:mission`

Generate `specs/mission.md` through an interactive interview. Run once per project.

**When to use:** to start a new project with speq-skill, or to add specs to an existing codebase.

### What it does

1. **Project type** — Determines whether the project is brownfield (existing code) or greenfield (new project)
2. **Exploration** — For brownfield projects, explores the tech stack, commands, and structure
3. **Interview** — Asks clarifying questions about purpose, users, and capabilities
4. **Generation** — Creates `specs/mission.md` with all gathered information

### Interview topics

The agent covers 11 areas. It groups related questions to keep the interview focused:

| Topic | What the agent asks about |
|-------|--------------------------|
| Identity and Purpose | Project name, one-sentence summary, and problem statement |
| Target Users | Personas, goals, and typical workflows |
| Core Capabilities | 3–5 things the system does (what, not how) |
| Out of Scope | Explicit non-goals and unsupported features |
| Domain Glossary | Project-specific terms and their meanings |
| Tech Stack | Language, runtime, framework, database, and testing |
| Commands | Build, test, lint/format, and coverage |
| Project Structure | Directory layout and purpose of each directory |
| Architecture | High-level pattern, key components, and data flow |
| Constraints | Technical, business, and performance limits |
| External Dependencies | Services or APIs that the project depends on |

> [!NOTE]
> `/speq:mission` runs once per project. The next three steps form the repeating development cycle.

---

## `/speq:plan`

Create feature spec deltas and an implementation plan, staged in `specs/_plans/<plan-name>/`.

**When to use:** to start new feature development, to modify existing behavior, or to refactor spec-first.

### Output structure

```
specs/_plans/<plan-name>/
├── plan.md                           # Implementation plan
├── decision-log.md                   # Design decisions (optional)
└── <domain>/<feature>/spec.md        # Delta specs
```

`planner-agent` creates `decision-log.md` during the planning interview. The file records the questions, answers, design choices, and alternatives considered. Entries marked `Promotes to ADR: yes` become a new `specs/_decision/NNN-<plan-name>.md` fragment when `recorder-agent` runs `/speq:record`. See [Decision Log](./decision-log.md).

Before handoff, `plan-reviewer` challenges the plan on intent fidelity, feasibility, requirement quality, task breakdown, and prose. It sends BLOCKER findings back to `planner-agent` for revision, up to 2 rounds. Unresolved blockers escalate to the human. `planner-agent` logs resolved blockers as `[plan-review]`-prefixed `## Review Findings` entries in `decision-log.md`.

### Plan naming

| Verb | When |
|------|------|
| `add` | New feature |
| `change` | Modify existing |
| `remove` | Deprecate or delete |
| `refactor` | Restructure, same behavior |
| `fix` | Bug or spec mismatch |

Pattern: `<verb>-<feature-scope>[-<qualifier>]`

Examples: `add-user-auth`, `fix-validation-edge-case`, `refactor-search-module`

---

## `/speq:implement`

Implement approved plan deltas — orchestrate tasks, delegate to sub-agents, review code, and produce a verification report.

**When to use:** after `/speq:plan`, to implement a plan:

```bash
/speq:implement <plan-name>
```

### What it does

1. Loads the plan and creates a task breakdown
2. Routes each parallelization group as a whole, by its hardest task. A group with any `[expert]` task goes to `implementer-expert-agent`. All-untagged groups go to `implementer-agent` (see [Model Routing](./model-routing.md))
3. Spawns sub-agents to work through tasks (with context rotation)
4. Loads targeted guardrails for clean code, unit testing, and integration testing
5. Runs code review on changed files via `code-reviewer`
6. Executes build, test, and lint verification
7. Generates a verification report

---

## `/speq:record`

Merge implemented spec deltas into the permanent spec library.

**When to use:** after a successful `/speq:implement`:

```bash
/speq:record <plan-name>
```

### What it does

1. **Verify** — Checks that `verification-report.md` exists
2. **Load** — Reads plan and delta specs
3. **Merge** — Applies deltas to permanent specs with these markers:

| Marker | Action |
|--------|--------|
| `DELTA:NEW` | Append scenario |
| `DELTA:CHANGED` | Replace scenario with same name |
| `DELTA:REMOVED` | Delete scenario with same name |

4. **Clean** — Strips all DELTA markers
5. **Validate** — Runs `speq feature validate`
6. **Check thresholds** — Flags any feature with more than 10 scenarios, or any domain with more than 8 features, and asks you how to split it. It never reorganizes without your decision
7. **Promote decisions** — Writes entries marked `Promotes to ADR: yes` in `decision-log.md` to a new `specs/_decision/NNN-<plan-name>.md` fragment
8. **Archive** — Moves the plan to `specs/_recorded/NNN-<plan-name>/`, where `NNN` is a record-time sequence number

---

## Headless PR Pipeline

`/speq:plan-pr` and `/speq:implement-pr` run the same Plan → Implement → Record cycle unattended. In interactive mode, `AskUserQuestion` prompts for each decision. Without a live interview, each such decision takes a documented default, or becomes an open question posted on the PR for a later reply.

```
/speq:plan-pr <intent>  →  PR (draft; + open questions if blocked)
                                   │
                    (reply on the PR, or /speq:plan <name> locally)
                                   │
                                   ▼
             /speq:implement-pr <name>  →  same PR, updated + marked ready
```

- **One branch per plan**: `feat/<plan-name>`, created by `/speq:plan-pr` and reused by `/speq:implement-pr`. Both push to the same PR. There is no separate plan-only branch.
- **Blocked state**: If planning hits a decision that needs a human, the system writes `specs/_plans/<plan-name>/open-questions.md` and flags `plan.md` as blocked. This applies to irreversible decisions, decisions that diverge from the architecture, and decisions relevant to security or compliance. The PR opens as a draft, with the questions posted as a comment. `/speq:implement-pr` refuses to continue while this file exists.
- **Resuming**: Reply on the PR, then re-run `/speq:plan-pr <plan-name>`. It re-fetches new comments and reviews as answers. Or check out the branch and finish the plan interactively with `/speq:plan <plan-name>`.
- **Headless defaults**: `/speq:implement-pr` auto-answers **yes** to the library-split question of `/speq:record`.
- **End-to-end with a resumable checkpoint**: One `/speq:implement-pr` invocation runs all three phases in sequence: A (implement and commit), B (test and record), and C (ship-ready). It normally ends at a ready PR. Each phase writes a mark to the `## PR Lifecycle` section in the `tasks.md` file of the plan. If a run is interrupted — for example, by a usage-limit reset or a crash — a fresh invocation in the same working directory reads the marks. It then resumes from the correct phase. The checkpoint gives interruption resilience. It is not a contract for multiple invocations.
- **PR title and lifecycle**: The PR title uses a conventional-commit feature title `<type>(<scope>): <slug>`, derived from the plan name — for example, `add-search-candle` becomes `feat(search): add search candle`. This differs from the `spec(plan):` commit prefix. `/speq:plan-pr` opens the PR as a **draft**. `/speq:implement-pr` marks it **ready** once the implementation is pushed.
- **Git and PR mechanics**: Both skills run every branch, commit, push, and PR operation directly, per `/speq:git-operations`. They are the only components permitted to write git history or touch a remote. See [Model Routing](./model-routing.md).

---

## `/speq:audit`

Health-check a speq project in one read-only pass, then fix each finding after it asks your permission for the change.

**When to use:**

- To inherit or clone a speq project, and to check its state
- To check for spec-library drift on a regular basis

**Checks:** spec-library `<domain>/<feature>` structure, `speq feature validate`, decision-log format and validity, sync between `mission.md` and the spec library (delegated to `audit-agent`), unrecorded plans in `_plans/`, gitignore hygiene (`_recorded` ignored, `_decision` and `_plans` tracked), recorded-folder naming, library thresholds, and git hygiene.

**Output:** a BLUF summary — a verdict, a `✓/✗/⚠` checks table, and numbered remediations. Structural fixes (migrate an old `decision-log.md`, restructure domains) and the `/speq:mission` handoff for mission drift run only after you confirm.

---

## Utility skills

Reusable guidance invoked by workflow skills:

| Skill | Purpose |
|-------|---------|
| `/speq:code-tools` | Semantic code navigation via Serena Model Context Protocol (MCP) |
| `/speq:ext-research` | External docs via Context7 and WebSearch |
| `/speq:code-guardrails` | Code quality guardrails |
| `/speq:git-discipline` | Git read-only rules |
| `/speq:cli` | speq CLI usage patterns |
| `/speq:writing-guardrails` | Prose style rules for speq artifacts and GitHub pull requests, issues, and comments |

See [MCP Servers](./mcp-servers.md) for details on Serena and Context7.
