---
name: speq-plan
description: Plan a feature or change through a clarifying interview, producing spec deltas, plan.md, and decision-log.md via planner-agent with adversarial review. Use when the user asks to plan, spec, design, or scope a new feature, a change or removal of existing behavior, a refactor, or a fix — before any implementation.
model: sonnet
---

# Spec Planner (Orchestrator)

This skill is a **thin orchestrator**: it conducts the clarifying interview, collects context, and delegates the heavy planning work (spec delta authoring, test mapping, task decomposition) to the `planner-agent` sub-agent. Orchestration is cheap; reasoning is expensive — the split concentrates reasoning where defects compound.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli` — Spec discovery and search

`planner-agent` and `plan-reviewer` invoke their own required skills.

## Workflow

### 0. Load Project Hook (orchestrator)

Check for `.speq/plan-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/plan-hook.md". Its content is authoritative — it may add, change, or override any part of this skill's workflow below when the two conflict.
- **Absent:** continue normally, no mention.

### 1. Discovery (orchestrator)

Use speq CLI to understand what exists:

```bash
speq domain list
speq feature list
speq search query "<relevant terms>"
```

This is lightweight — enough context to ask good clarifying questions, not a full exploration.

### 2. Clarifying Interview (orchestrator)

Apply the **Socratic Method** via `AskUserQuestion` — never assume. Decompose the problem space using **MECE partitioning**:

- **Probe** — surface hidden assumptions with open-ended questions
- **Partition** — present alternative solutions as MECE options
- **Challenge** — test design tradeoffs through guided counterexamples

Record answers in a concise interview summary to pass to the sub-agent.

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
<output of relevant `speq search` / `speq feature get` calls>

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

1. Confirm `speq plan validate <plan-name>` passed (re-run if uncertain)
2. List all created files
3. If the sub-agent escalated a question back to you, resolve it with the user and respawn with the clarification

### 6. Adversarial Plan Review (orchestrator)

`planner-agent` is both author and, until now, sole judge. Before handing the plan off, spawn `plan-reviewer` — a diabolus advocatus — to challenge it. Bounded to 2 rounds total.

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

**If `INTENT > 0`:** the reviewer's case is that the plan solves a different problem than the one asked for — read the Intent-Fidelity findings from the round file and use `AskUserQuestion` to present them before attempting any revision; the user accepts the plan as-is, or gives guidance and you respawn `planner-agent` manually.

**If `INTENT == 0` and BLOCKER findings exist:**

1. Respawn `planner-agent` with the path to `review/round-1.md`, instructing it to read the BLOCKER findings from that file and execute each `Fix:` line, log each resolved blocker as a `[plan-review]`-prefixed `## Review Findings` entry in `decision-log.md`, and re-run `speq plan validate`.
2. Respawn `plan-reviewer` for **round 2**, passing that same path so it confirms each round-1 BLOCKER is actually resolved before checking for new ones.
3. Do not loop a third time, even if round 2 surfaces new BLOCKERs.

**If BLOCKERs remain after round 2** — read the unresolved BLOCKER findings from `review/round-2.md` and use `AskUserQuestion`: present the remaining blockers, let the user accept the risk and proceed, or give guidance and respawn `planner-agent` manually.

**ADVISORY findings** are never looped on or persisted — read them from the last round file and carry them into step 7's report so the user sees them before implementing.

### 7. Explain next steps (orchestrator)

- Inform the user that the plan is created and ready for review
- List all created files
- Report any ADVISORY findings from step 6 (read from the round file) so the user sees them before implementing
- Inform the user to call `/speq-implement <plan-name>` to continue
- Inform the user to call `/clear` to start implementing with a fresh context window
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
| Spec delta authoring, ADR, task decomposition | `planner-agent` sub-agent | Reasoning-heavy; defect here compounds through implementation |
| Adversarial review, revision loop | `plan-reviewer` sub-agent | Catches intent drift, infeasibility, and ambiguity before implementation, not after |

Each sub-agent pins its own model and effort in its frontmatter, so planning quality is independent of the parent session's configuration.

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Authoring plan.md or spec deltas in the orchestrator | `planner-agent` owns all plan authoring |
| Skipping the clarifying interview | Content comes from user answers, never assumptions |
| A third review round | Review is bounded to 2 rounds — after that, the user decides |
| Persisting ADVISORY findings or looping on them | Report-only; they never gate |
| Embedding spec content in plan.md | Plans reference delta files |
