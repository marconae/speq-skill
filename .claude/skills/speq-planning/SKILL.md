---
name: speq-planning
description: Plan-authoring workflow — spec delta authoring, test mapping, plan.md/decision-log.md generation, expert-task tagging, headless escalation, and revision mode. Triggered by planner-agent.
---

# Plan Authoring

## Guiding Principles

- **BDD (Gherkin syntax)** — scenarios use GIVEN/WHEN/THEN; integration tests by default, unit tests only for isolated pure computation
- **EARS syntax** — spec narratives use unambiguous behavioral clauses
- **RFC 2119 keywords** — THEN steps use MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY (uppercase)
- **ADR (Nygard format)** — design sections capture Goals / Non-Goals / Architecture / Trade-offs / Key Interfaces

## Workflow

### 1. Discover Existing Specs

Use speq CLI to find related specs before writing:

```bash
speq domain list
speq feature list
speq search query "<relevant terms>"
speq feature get <domain>/<feature>
```

**Search first** when modifying existing behavior.

### 2. Author Spec Deltas

For each feature in scope:

```
specs/<domain>/<feature>/spec.md exists?
├─ Yes → DELTA markers (.claude/skills/speq-plan/references/delta-template.md)
└─ No  → Full spec (.claude/skills/speq-plan/references/feature-template.md)

Output: specs/_plans/<plan-name>/<domain>/<feature>/spec.md
```

### 3. Test Mapping and Verification

Every scenario requires two forms of external proof. No claims — only evidence.

**Integration tests** (mandatory per scenario):
- Map each scenario → integration test (file path + test name)
- One test per scenario by default; combine only when scenarios share setup and assertions
- Unit tests only for pure computation with no I/O

**Manual invocation** (mandatory per feature):
- Concrete commands that invoke the built software
- Expected observable output per command

### 4. Generate plan.md

Populate plan.md per `.claude/skills/speq-plan/references/plan-template.md`:

1. **Context** — why the change is being made
2. **Features** — table referencing spec delta files (NEVER embed spec content); immediately followed by an Impact entry describing user/operator-facing consequences (breaking changes called out, or "None")
3. **Design** — ADR for new features / major changes; skip for minor fixes
4. **Tasks** — work breakdown in implementation order
5. **Parallelization** — groups of tasks that can run concurrently
6. **Verification** — Scenario Coverage + Manual Testing + Checklist (from `specs/mission.md`)

### 5. Generate decision-log.md

Create `specs/_plans/<plan-name>/decision-log.md` from `.claude/skills/speq-plan/references/decision-log-plan-template.md`.

**What to capture:**
- **Interview section** — verbatim or close paraphrase of every Q&A exchange passed from the orchestrator
- **Design Decisions section** — one entry per significant choice made while authoring spec deltas or plan.md (architecture patterns, rejected alternatives, scope boundaries)
- **Review Findings section** — leave empty; populated in Revision Mode after `plan-reviewer` blockers, and by `speq-implement` after code review

**For each decision entry**, set `Promotes to ADR: yes` when the decision is:
- An architectural or workflow pattern adopted project-wide
- A deliberate rejection of a commonly expected approach
- A constraint that future planners need to know to avoid re-litigating

Set `Promotes to ADR: no` for local design choices, scope trims, and implementation details.

When a decision supersedes an earlier one, name the superseded decision's title in the entry — `recorder-agent` maps that title to the superseded ADR's slug when it promotes the entry.

### 6. Expert-Task Tagging (CRITICAL)

As tasks are decomposed, identify tasks that require deep reasoning. Tag them with `[expert]` at the end of the task line:

```markdown
- [ ] 2.1 Add CLI flag parsing
- [ ] 2.2 Implement lock-free queue for concurrent spec writes [expert]
- [ ] 2.3 Write integration tests for flag combinations
- [ ] 2.4 Refactor validator to preserve ordering invariants across plugins [expert]
```

**Use `[expert]` only when the task genuinely needs it:**
- Concurrency / ordering / race conditions
- Cross-file refactors with behavioral dependencies
- Novel algorithms or non-obvious correctness
- Security-sensitive code paths

**Do NOT tag tasks as expert for:**
- Standard CRUD, CLI flag plumbing, test fixtures
- Copy-paste from existing patterns
- Documentation or config changes

Over-tagging wastes tokens; under-tagging risks defects. The orchestrator routes `[expert]` tasks to `implementer-expert-agent` and all others to `implementer-agent`. Most tasks should be untagged.

### 7. Validate Plan

Before returning, run CLI validation:

```bash
speq plan validate <plan-name>
```

Fix any failures. Common fixes:
- Close unclosed delta markers with `<!-- /CHANGED -->`, `<!-- /NEW -->`, `<!-- /REMOVED -->`
- Uppercase RFC 2119 keywords
- Fix step formatting (bold keywords: `*GIVEN*`, `*WHEN*`, `*THEN*`, `*AND*`)

## Headless / Non-Interactive Mode

If the orchestrator's prompt states `Interview Mode: headless` (used by `speq-plan-pr`, never by the interactive `speq-plan`), there is no human to ask a clarifying question mid-planning. Adjust the escalation bar instead of blocking by default:

- **Assume and document.** For conventions, naming, implementation details, and any choice with a clearly conventional default, make the call and record it as a `decision-log.md` entry (Rationale explains why this default was chosen). This is the common case — most headless plans should finish without escalating.
- **Escalate only irreducible decisions** — ones that are irreversible, change what the feature does for a user, diverge architecturally (two genuinely incompatible designs), or touch security/compliance. Before escalating, save every file completed so far (plan.md, delta specs, decision-log.md) exactly as it stands — the orchestrator persists this partial state for human review, so it must be usable as-is.
- **Escalation format.** Return the response prefixed with the exact sentinel `OPEN QUESTIONS:` followed by a markdown bullet list of concrete questions (same bar as the interactive path's "signal back with a concrete question" — headless mode changes when to escalate, not the quality bar for what to escalate). Do not mix this sentinel into a normal completion report.

## Revision Mode

If the orchestrator respawns `planner-agent` with a `plan-reviewer` BLOCKER list instead of a fresh planning brief:

- Address only the named findings — revise the specific `plan.md` section, spec delta, or task line each one points to. Do not rewrite unrelated content.
- For each blocker resolved, add a `## Review Findings` entry to `decision-log.md` titled `[plan-review] <short finding title>`, with **Finding** (what `plan-reviewer` flagged), **Direction change** (what changed), and **Promotes to ADR** (per the rule above).
- Re-run `speq plan validate <plan-name>` before returning.
- If resolving a blocker surfaces a genuinely irreducible new decision, escalate it exactly as during initial planning (interactive: signal back with a concrete question; headless: `OPEN QUESTIONS:` sentinel).
