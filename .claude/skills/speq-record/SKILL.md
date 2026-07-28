---
name: speq-record
description: "Merge implemented spec deltas into the permanent specs library and archive the plan. Use when implementation is verified — after /speq-implement produces verification-report.md — or when the user asks to record, merge, or archive a finished plan. Arg: <plan-name>."
model: sonnet
---

# Spec Recorder (Orchestrator)

Thin orchestrator. It verifies preconditions and delegates the merge work to the `recorder-agent` sub-agent.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli` — Spec validation

The `recorder-agent` sub-agent invokes its own required skills.

## Workflow

### Phase 0: Load Project Hook (orchestrator)

Check for `.speq/record-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/record-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue normally, no mention.

Note it (not its full content) as a `Project Hook:` line in the `recorder-agent` brief below.

### Phase 1: Resolve Plan Name (orchestrator)

Get the plan name from the user prompt. If none is given, use `AskUserQuestion` to present the plans under `specs/_plans/`.

### Phase 2: Verify Preconditions (orchestrator)

```
Check: specs/_plans/<plan-name>/verification-report.md exists?
├─ Yes → Proceed
└─ No  → STOP: "Run /speq-implement <plan-name> first."
```

### Phase 3: Delegate to recorder-agent

Spawn the recorder sub-agent with the plan name:

```
Delegate to recorder-agent — Record <plan-name> into permanent specs

## Plan Name
<plan-name>

## Context
- Verification report confirmed at: specs/_plans/<plan-name>/verification-report.md
- Plan file: specs/_plans/<plan-name>/plan.md
- Delta specs: specs/_plans/<plan-name>/**/spec.md

## Your Task
Merge all delta specs into permanent specs per the `recorder-agent` workflow. Validate between merges. Archive the plan on success. If any library threshold is exceeded (scenarios > 10, domain features > 8), STOP before archiving and return a question for the user.

Project Hook: <if active, ".speq/record-hook.md — read it and apply it"; otherwise omit this line>

Return a summary of merged features and the archive path.
```

### Phase 4: Handle Threshold Escalations (orchestrator)

If the sub-agent returns threshold signals:

1. Use `AskUserQuestion` to get the user's organizational decision
2. Respawn `recorder-agent` with the decision, OR apply a trivial edit (for example, a file rename) directly
3. Archive only after all decisions are resolved

### Phase 5: Confirm Completion (orchestrator)

Report to user:

```
✓ Verification report confirmed
✓ All deltas merged
✓ Spec library validated
✓ Plan archived: specs/_recorded/NNN-<plan-name>
```

## Work Split (reference)

| Step | Performed by | Why |
|------|--------------|-----|
| Precondition checks, user questions | This skill (pins Sonnet) | Lightweight orchestration |
| Delta merge, validation, archive | `recorder-agent` sub-agent | Mechanical file surgery |

The split preserves rotation discipline: the orchestrator can respawn the sub-agent with a fresh context and keep its own state.

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Record without verification report | Implementation not proven |
| Orchestrator merges directly | Breaks rotation / context discipline |
| Assume split/domain decisions | User must confirm |
| Skip validation | Broken specs may result |
| Leave DELTA markers | Pollutes permanent specs |
