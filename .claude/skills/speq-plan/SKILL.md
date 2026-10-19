---
name: speq-plan
description: "Plans a feature or change through a clarifying interview, producing spec deltas, plan.md, and decision-log.md with adversarial review. Use to plan, spec, design, or scope a feature, change, removal, refactor, or fix, before any implementation."
model: sonnet
---

# Spec Planner (Orchestrator)

This skill is a thin orchestrator. It runs the clarifying interview, collects context, and delegates the planning work (spec delta authoring, test mapping, task decomposition) to `planner-agent`.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli`: spec discovery

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
grep -rhE -A3 '^## ADR:|^\*\*(ID|Status):\*\*' specs/_decision/ 2>/dev/null || echo none   # ADR digest, per /speq-cli
```

Name the features in the list that relate to the request, per `/speq-cli`'s absence check.

Note whether `specs/architecture.md` exists. Pass its path, do not read it.

Collect only enough context for good interview questions, not a full exploration.

### 2. Clarifying Interview (orchestrator)

Apply the Socratic Method by asking the user. Never assume. Decompose the problem space with MECE partitioning:

- **Probe**: surface hidden assumptions with open-ended questions
- **Partition**: present alternative solutions as MECE options
- **Challenge**: test design tradeoffs with counterexamples

**Split check.** If the work spans more than one independent phase, or more than about 4 features, propose one plan per phase. Plan the first phase now. Each later plan names the earlier plan in its Existing Context.

Keep each exchange as one line: `- Q: <question> → A: <answer>`, a few words each. This is the only copy of the interview: step 3 writes it into `decision-log.md` and the briefs point to it.

### 3. Plan Name (orchestrator)

Pattern: `<verb>-<feature-scope>[-<qualifier>]`

Once the name is set, write `specs/_plans/<plan-name>/decision-log.md` with the H1 `# Decision Log: <plan-name>` and an `## Interview` section holding the step 2 lines. Do not paste the interview into any brief.

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
Read `## Interview` in `specs/_plans/<plan-name>/decision-log.md`. Keep it as is. Do not copy it into other files.

## Existing Context
<the exact `speq domain list` / `speq feature list` / `speq feature get` calls you ran, each followed by its output (the ADR digest, not the full `speq decision-log show`; name features you want read, do not paste `speq feature get` output); name `specs/architecture.md` by path when it exists, do not paste it>

## External Research
<any research already conducted, or "none — agent to research as needed">

## Project Hook
<if active: note ".speq/plan-hook.md — read it and apply it" — otherwise omit this section>

## Your Task
Produce spec deltas and plan.md per the `planner-agent` workflow. Tag tasks requiring deep reasoning with [expert] so the implementer orchestrator can route them to implementer-expert-agent.

If the interview agreed a split, end `plan.md` Context with a `Follow-up plans` list (name and one-line scope each).

Return the list of files created and the validation result.
```

### 5. Review planner-agent output (orchestrator)

When the sub-agent returns:

1. Confirm `speq plan validate <plan-name>` passed. Re-run if uncertain.
2. List all created files.
3. If the sub-agent escalated a question, resolve it with the user and respawn with the clarification.

### 6. Adversarial Plan Review (orchestrator)

Spawn `plan-reviewer` once to challenge the plan before handoff. There is one review round.

```
Delegate to plan-reviewer — Review <plan-name>

## Plan Name
<plan-name>

## User Intent
<verbatim original request>

## Clarifying Interview Results
Read `## Interview` in `specs/_plans/<plan-name>/decision-log.md`.

## Plan Artifacts
plan.md, decision-log.md, every specs/_plans/<plan-name>/**/spec.md delta, and specs/_plans/<plan-name>/architecture.md when present

## Project Hook
<if active: note ".speq/plan-hook.md — read it and apply it" — otherwise omit this section>
```

It writes its findings to `specs/_plans/<plan-name>/review/review.md` and returns only `PLAN REVIEW: BLOCKERS: <n>, FINDINGS: <n>, INTENT: <n> — <path>`. `INTENT` counts the BLOCKERs on the Intent Fidelity axis alone.

**If `INTENT > 0`:** the reviewer holds that the plan solves a different problem than the one asked. Read the Intent-Fidelity findings from the review file. Ask the user about them before any fix pass. The user accepts the plan as-is, or gives guidance and you respawn `planner-agent` manually.

**If `INTENT == 0` and `BLOCKERS > 0`:** respawn `planner-agent` once with the path to `review/review.md`. Instruct it to read the BLOCKER findings from that file, execute each `Fix:` line, log a `[plan-review]`-prefixed `## Review Findings` entry in `decision-log.md` only for a blocker whose fix changed a design decision, re-run `speq plan validate`, and return its per-finding `Resolved:`/`Could not resolve:` report per `/speq-planning`'s Revision Mode. Do not review again.

**Any `Could not resolve:` line, or a failed `speq plan validate`:** ask the user, one line of context each plus a pointer to the review file. The user accepts the risk and proceeds, or gives guidance and you respawn `planner-agent` manually.

**Non-blocking findings** never loop and are never persisted. Read them from the review file and carry them into step 7's report.

### 7. Explain next steps (orchestrator)

Print to the terminal:

- `plan.md`'s `## Impact` section
- Print `ADR candidates: none`, or list the title of each `Promotes to ADR: yes` entry in `decision-log.md`
- Report non-blocking findings from step 6, read from the review file
- If step 6 ran a fix pass, name it in one line ("N review findings fixed") and do not restate each one
- Tell the user to run `/speq-implement <plan-name>` to continue
- Tell the user to run `/clear` to implement with a fresh context window

## Spec Hierarchy (reference)

```
specs/
├── <domain>/<feature>/spec.md     # Permanent
├── _plans/<plan-name>/            # Active
└── _recorded/<plan-name>/         # Archived
```

## Work Split (reference)

| Step | Performed by |
|------|--------------|
| Discovery, interview, coordination | This skill |
| Spec delta authoring, plan context and decision log, task decomposition | `planner-agent` sub-agent |
| Adversarial review, revision loop | `plan-reviewer` sub-agent |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Skipping the clarifying interview | Content comes from user answers, never assumptions |
| Running a second review after the fix pass | One review, one fix pass; unresolved findings go to the user |
| Persisting non-blocking findings or looping on them | Report-only; they never gate |
