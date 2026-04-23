---
name: speq-record
description: Merge implemented spec deltas into permanent specs library.
model: sonnet
---

# Spec Recorder (Orchestrator)

This skill is a **thin orchestrator**. It verifies preconditions and delegates the deterministic merge work to the `recorder-agent` sub-agent. Recording is mechanical file surgery and does not need deep reasoning.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli` — Spec validation

The `recorder-agent` sub-agent invokes `/speq-code-tools`, `/speq-cli`, and `/speq-git-discipline` itself.

## Workflow

### Phase 1: Resolve Plan Name (orchestrator)

Get plan name from user prompt. If none specified, use `AskUserQuestion` to present a list of plans under `specs/_plans/`.

### Phase 2: Verify Preconditions (orchestrator)

```
Check: specs/_plans/<plan-name>/verification-report.md exists?
├─ Yes → Proceed
└─ No  → STOP: "Run /speq-implement <plan-name> first."
```

### Phase 3: Delegate to recorder-agent

Spawn the recorder sub-agent with the plan name:

```python
Task(
  subagent_type="recorder-agent",
  description="Record <plan-name> into permanent specs",
  prompt="""
## Plan Name
<plan-name>

## Context
- Verification report confirmed at: specs/_plans/<plan-name>/verification-report.md
- Plan file: specs/_plans/<plan-name>/plan.md
- Delta specs: specs/_plans/<plan-name>/**/spec.md

## Your Task
Merge all delta specs into permanent specs per the `recorder-agent` workflow.
Validate between merges. Archive the plan on success. If any library threshold
is exceeded (scenarios > 10, domain features > 8), STOP before archiving and
return a question for the user.

Return a summary of merged features and the archive path.
"""
)
```

### Phase 4: Handle Threshold Escalations (orchestrator)

If the sub-agent returns threshold signals:

1. Use `AskUserQuestion` to gather user's organizational decision
2. Respawn `recorder-agent` with the decision, OR apply a small edit directly if the action is trivial (e.g., rename a file)
3. Only archive once all decisions are resolved

### Phase 5: Confirm Completion (orchestrator)

Report to user:

```
✓ Verification report confirmed
✓ All deltas merged
✓ Spec library validated
✓ Plan archived: specs/_recorded/YYYY-MM-DD-<plan-name>
```

## Work Split (reference)

| Step | Performed by | Why |
|------|--------------|-----|
| Precondition checks, user questions | This skill (pins Sonnet) | Lightweight orchestration |
| Delta merge, validation, archive | `recorder-agent` sub-agent | Mechanical file surgery |

Keeping orchestrator and sub-agent separate preserves the rotation discipline: if the spec library is very large, the sub-agent can be re-spawned with a fresh context without losing orchestration state.

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Record without verification report | Implementation not proven |
| Orchestrator merges directly | Breaks rotation / context discipline |
| Assume split/domain decisions | User must confirm |
| Skip validation | Broken specs may result |
| Leave DELTA markers | Pollutes permanent specs |
