---
name: speq-plan
description: Plan and create spec deltas for new features or changes to existing features.
model: sonnet
---

# Spec Planner (Orchestrator)

This skill is a **thin orchestrator**. It conducts the clarifying interview, collects context, and then delegates the heavy planning work (spec delta authoring, test mapping, task decomposition) to the `planner-agent` sub-agent.

**Why this split:** Orchestration (asking questions, reading a few files, shepherding the workflow) does not need expensive reasoning. The actual planning — architectural tradeoffs, MECE decomposition, ADR authoring — does. Splitting the work concentrates reasoning on the step that benefits from it.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli` — Spec discovery and search

The `planner-agent` sub-agent invokes `/speq-code-tools`, `/speq-ext-research`, and `/speq-cli` itself.

## Workflow

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

```python
Task(
  subagent_type="planner-agent",
  description="Plan <plan-name>",
  prompt="""
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

## Your Task
Produce spec deltas and plan.md per the `planner-agent` workflow. Tag tasks
requiring deep reasoning with [expert] so the implementer orchestrator can
route them to implementer-expert-agent.

Return the list of files created and the validation result.
"""
)
```

### 5. Review planner-agent output (orchestrator)

When the sub-agent returns:

1. Confirm `speq plan validate <plan-name>` passed (re-run if uncertain)
2. List all created files
3. If the sub-agent escalated a question back to you, resolve it with the user and respawn with the clarification

### 6. Explain next steps (orchestrator)

- Inform the user that the plan is created and ready for review
- List all created files
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

The sub-agent pins its own model and effort in its frontmatter, so planning quality is independent of the parent session's configuration.
