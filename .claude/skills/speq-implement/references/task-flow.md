# Task Flow (Work Breakdown Structure)

Dual-track task management following **Work Breakdown Structure** decomposition: `tasks.md` for persistence, TaskTools for runtime.

## Dual-Track System

| Track | Purpose | Survives Context Loss |
|-------|---------|----------------------|
| `tasks.md` | Persistent state | Yes |
| TaskTools | Runtime management | No |

**Keep both in sync.** Update tasks.md first, then TaskTools.

## tasks.md Format

```markdown
# Tasks: <plan-name>

## Phase 2: Implementation (Group A)
- [ ] 2.1 Implement login endpoint
- [~] 2.2 Implement logout endpoint

## Phase 2: Implementation (Group B)
- [ ] 2.3 Add auth middleware

## Phase 3: Verification
- [ ] 3.1 Run test suite
- [ ] 3.2 Run linter
```

**Status markers:**

| Marker | Status | Meaning |
|--------|--------|---------|
| `[ ]` | pending | Not started |
| `[~]` | started | Sub-agent working |
| `[x]` | completed | Verified done |

**PR Lifecycle section:** when the file starts with a `## PR Lifecycle` section, `speq-implement-pr` owns it as its checkpoint (see that skill's `references/checkpoint-protocol.md`). Its entries are unnumbered on purpose: task dispatch, Context Recovery, and implementer agents cover only the numbered `## Phase N` lines and never touch the lifecycle section.

## Task Lifecycle

```
tasks.md: [ ] → [~] → [x]
TaskTools: pending → in_progress → completed
```

### Orchestrator Actions

| Event | tasks.md | TaskTools |
|-------|----------|-----------|
| Create tasks | Write file | TaskCreate for each |
| Start group | `[ ]` → `[~]` | TaskUpdate(status: "in_progress") |
| Task done | `[~]` → `[x]` | TaskUpdate(status: "completed") |

## Parallelization

From plan's `## Parallelization` section. Each group is a knowledge cluster; one agent takes the whole group, routed by its hardest task (any `[expert]` task → `implementer-expert-agent`):

```markdown
## Parallelization

| Group | Tasks | Depends on | Knowledge |
|-------|-------|------------|-----------|
| A | 2.1, 2.2 | — | spec delta cli/auth; src/auth/, tests/auth_test |
| B | 2.3 | A | src/middleware/ |
| C | 2.4, 2.5 | A | spec delta cli/session; src/session/ |
```

**Execution order:**
1. Group A (one agent for the whole group)
2. Groups B and C (can run after A completes)

Plans from before the `Knowledge` column may lack it — routing and execution order do not depend on it; only the agent's orientation line does.

## Context Recovery

When resuming after context loss:

```
1. Read tasks.md
2. Find first non-[x] task in the ## Phase N sections (skip ## PR Lifecycle)
3. If [~], sub-agent was interrupted → restart that group
4. If [ ], group not started → begin normally
5. Sync TaskTools state if needed
```

## Error Handling

| Situation | Action |
|-----------|--------|
| Sub-agent fails | Keep tasks as `[~]`, report error |
| Partial completion | Mark completed tasks `[x]`, retry others |
| Test failure | Do not mark `[x]`, fix and retry |

## TaskTools Quick Reference

```python
# Create task
TaskCreate(
  subject="Implement login endpoint",
  description="Per spec cli/auth, add POST /login",
  activeForm="Implementing login endpoint"
)

# Start work
TaskUpdate(taskId="1", status="in_progress")

# Complete
TaskUpdate(taskId="1", status="completed")

# Check state
TaskList()
```
