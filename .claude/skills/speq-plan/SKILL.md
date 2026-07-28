---
name: speq-plan
description: Plan a feature or change through a clarifying interview, producing spec deltas, plan.md, and decision-log.md via planner-agent with adversarial review. Use when the user asks to plan, spec, design, or scope a new feature, a change or removal of existing behavior, a refactor, or a fix — before any implementation.
model: sonnet
---

# Spec Planner (Orchestrator)

This skill is a thin orchestrator. It runs the clarifying interview, collects context, and delegates the planning work (spec delta authoring, test mapping, task decomposition) to `planner-agent`.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli`: spec discovery and search

`planner-agent` and `plan-reviewer` invoke their own required skills.

## Workflow

### 0. Load Project Hook (orchestrator)

Check for `.speq/plan-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/plan-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue without mention.

### 1. Discovery (orchestrator)

Run the speq CLI:

```bash
speq domain list
speq feature list
speq search query "<relevant terms>"
```

Collect only enough context for good interview questions, not a full exploration.

### 2. Clarifying Interview (orchestrator)

Apply the Socratic Method via `AskUserQuestion`. Never assume. Decompose the problem space with MECE partitioning:

- **Probe**: surface hidden assumptions with open-ended questions
- **Partition**: present alternative solutions as MECE options
- **Challenge**: test design tradeoffs with counterexamples

Record the answers in a concise summary for the sub-agent.

### 3. Plan Name (orchestrator)

Pattern: `<verb>-<feature-scope>[-<qualifier>]`

| Verb | When |
|------|------|
| `add` | New feature |
| `change` | Modify existing |
| `remove` | Deprecate/delete |
| `refactor` | Restructure, same behavior |
| `fix` | Bug or spec mismatch |

### 4. Delegate to planner-agent

Spawn the planner sub-agent with everything it needs:

```
Delegate to planner-agent — Plan <plan-name>

## Plan Name
<plan-name>

## User Intent
<1-3 sentence summary of what the user wants>

## Clarifying Interview Results
<verbatim Q&A from the AskUserQuestion exchanges>

## Existing Context
<the exact `speq domain list` / `speq feature list` / `speq search query "..."` / `speq feature get` calls you ran, each followed by its output — name the query, not just the result>

## External Research
<any research already conducted, or "none — agent to research as needed">

## Project Hook
<if active: note ".speq/plan-hook.md — read it and apply it" — otherwise omit this section>

## Your Task
Produce spec deltas and plan.md per the `planner-agent` workflow. Tag tasks requiring deep reasoning with [expert] so the implementer orchestrator can route them to implementer-expert-agent.

Return the list of files created and the validation result.
```

### 5. Review planner-agent output (orchestrator)

When the sub-agent returns:

1. Confirm `speq plan validate <plan-name>` passed. Re-run if uncertain.
2. List all created files.
3. If the sub-agent escalated a question, resolve it with the user and respawn with the clarification.

### 6. Adversarial Plan Review (orchestrator)

Spawn `plan-reviewer`, a diabolus advocatus, to challenge the plan before handoff. Maximum 2 rounds total.

**Round 1:**

```
Delegate to plan-reviewer — Review <plan-name> (round 1)

## Plan Name
<plan-name>

## User Intent
<verbatim original request>

## Clarifying Interview Results
<verbatim Q&A>

## Plan Artifacts
plan.md, decision-log.md, and every specs/_plans/<plan-name>/**/spec.md delta

## Project Hook
<if active: note ".speq/plan-hook.md — read it and apply it" — otherwise omit this section>
```

It writes its findings to `specs/_plans/<plan-name>/review/round-1.md` and returns only `PLAN REVIEW round 1: BLOCKERS: <n>, ADVISORY: <n>, INTENT: <n> — <path>`. `INTENT` counts the BLOCKERs on the Intent Fidelity axis alone.

**If `INTENT > 0`:** the reviewer holds that the plan solves a different problem than the one asked. Read the Intent-Fidelity findings from the round file. Present them via `AskUserQuestion` before any revision. The user accepts the plan as-is, or gives guidance and you respawn `planner-agent` manually.

**If `INTENT == 0` and BLOCKER findings exist:**

**Plan Size classification** (compute before respawning `plan-reviewer` for round 2): the plan is `small` when all three hold — the plan-name's verb (per the verb table) is `fix`; `plan.md` has no `## Design` section; `decision-log.md`'s `## Design Decisions` section is empty. Otherwise `full`.

1. Respawn `planner-agent` with the path to `review/round-1.md`. Instruct it to read the BLOCKER findings from that file, execute each `Fix:` line, log each resolved blocker as a `[plan-review]`-prefixed `## Review Findings` entry in `decision-log.md`, and re-run `speq plan validate`.
2. Respawn `plan-reviewer` for round 2 with the same path plus the computed `Plan Size: small | full` field, so it confirms each round-1 BLOCKER is resolved before checking for new ones (or, on `small`, confirms and stops there).
3. Do not run a third round, even if round 2 raises new BLOCKERs.

**If BLOCKERs remain after round 2:** read the unresolved BLOCKER findings from `review/round-2.md` and use `AskUserQuestion`. The user accepts the risk and proceeds, or gives guidance and you respawn `planner-agent` manually.

**ADVISORY findings** never loop and are never persisted. Read them from the last round file and carry them into step 7's report. If round 2 ran confirm-only (`Plan Size: small`), it produced no ADVISORY findings of its own — read them from round 1's file instead.

### 7. Explain next steps (orchestrator)

- Report that the plan is created and list all created files
- Report ADVISORY findings from step 6, read from the round file
- Tell the user to run `/speq-implement <plan-name>` to continue
- Tell the user to run `/clear` to implement with a fresh context window
- If Claude Code is in "plan mode", call `ExitPlanMode` and ask to proceed with cleared context

## Spec Hierarchy (reference)

```
specs/
├── <domain>/<feature>/spec.md     # Permanent
├── _plans/<plan-name>/            # Active
└── _recorded/<plan-name>/         # Archived
```

## Work Split (reference)

| Step | Performed by | Why |
|------|--------------|-----|
| Discovery, interview, coordination | This skill (pins Sonnet) | Conversational, tool-call heavy |
| Spec delta authoring, ADR, task decomposition | `planner-agent` sub-agent | Reasoning-heavy; a defect here compounds through implementation |
| Adversarial review, revision loop | `plan-reviewer` sub-agent | Catches intent drift, infeasibility, and ambiguity before implementation |

Each sub-agent pins its own model and effort in its frontmatter, so planning quality does not depend on the parent session's configuration.

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Authoring plan.md or spec deltas in the orchestrator | `planner-agent` owns all plan authoring |
| Skipping the clarifying interview | Content comes from user answers, never assumptions |
| A third review round | Review is bounded to 2 rounds; after that the user decides |
| Persisting ADVISORY findings or looping on them | Report-only; they never gate |
| Embedding spec content in plan.md | Plans reference delta files |
