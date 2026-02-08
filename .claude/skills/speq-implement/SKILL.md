---
name: speq-implement
description: "Orchestrate implementation of reviewed plans according to the spec deltas. Arg: <plan-name>."
---

# Spec Implementer

Orchestrate implementation of plans in `specs/_plans/<plan-name>`.

Get plan name from user prompt or ask if none specified.

## Required Skills

Invoke before starting:
- `/speq-code-tools` — Code navigation and editing
- `/speq-ext-research` — Library documentation
- `/speq-code-guardrails` — TDD cycle and quality standards
- `/speq-cli` — Spec discovery

Subagents must also invoke these skills plus `/speq-git-discipline`.

## Orchestrator Role

The main agent acts as **orchestrator**:
- Creates and maintains `tasks.md` for persistence
- Spawns sub-agents for parallel task groups
- Updates task status after sub-agent completion
- Never implements directly — delegates all coding work
- Rotates sub-agents to maintain fresh context windows

## Context Window Management

**Strategy:** Rotate sub-agents to maintain fresh context.

| Setting | Default | Description |
|---------|---------|-------------|
| `max_tasks_per_agent` | 5 | Tasks before considering rotation |
| `checkpoint_interval` | 2-3 | Tasks between progress reports |

**Rotation workflow:**
1. Sub-agent completes tasks, updates tasks.md after each
2. Sub-agent reports checkpoint after 2-3 tasks
3. If max_tasks reached OR sub-agent reports "ROTATION NEEDED":
   - Read tasks.md for current state
   - Note completed tasks from sub-agent return
   - Spawn fresh agent with remaining tasks
4. Continue until group complete

## Workflow

### Phase 1: Load Plan

```
Read: specs/_plans/<plan-name>/plan.md
```

Extract: feature specs, implementation tasks, parallelization groups, verification commands.

### Phase 2: Create Tasks

Generate `specs/_plans/<plan-name>/tasks.md` from plan.

**Format:**

```markdown
# Tasks: <plan-name>

## Phase 2: Implementation (Group A)
- [ ] 2.1 <task from plan>
- [ ] 2.2 <task from plan>

## Phase 2: Implementation (Group B)
- [ ] 2.3 <task from plan>

## Phase 3: Verification
- [ ] 3.1 Run test suite
- [ ] 3.2 Run linter
```

**Status markers:**
- `[ ]` pending
- `[~]` started
- `[x]` completed

Also create runtime tasks:
```
For each task in tasks.md:
  TaskCreate(subject, description, activeForm)
```

### Phase 3: Implement (Orchestrated)

For each parallel group in plan's `## Parallelization`:

1. **Mark started** — Update tasks.md: `[ ]` → `[~]` for group tasks
2. **Spawn subagent** — Use `implementer-agent` agent
3. **Await completion** — Sub-agent returns with results or rotation signal
4. **Handle rotation** — If sub-agent signals rotation, spawn fresh agent
5. **Mark completed** — Update tasks.md: `[~]` → `[x]` for completed tasks
6. **Update TaskTools** — `TaskUpdate(taskId, status: "completed")`
7. **Next group** — Proceed to next parallel group

**Subagent invocation:**

```python
Task(
  subagent_type="implementer-agent",
  description="Implement <group-name> tasks",
  prompt="""
## Your Tasks

{task_list}

## Context

- Plan: specs/_plans/{plan_name}/plan.md
- Tasks file: specs/_plans/{plan_name}/tasks.md
- Update tasks.md after each task completion
- Report checkpoint after every 2-3 tasks
"""
)
```

### Phase 4: Code Review

After implementation completes, review all changed files.

1. **Collect changed files** — `git diff --name-only <base>...HEAD`
2. **Spawn code-reviewer agent:**
   ```python
   Task(
     subagent_type="code-reviewer",
     description="Review implementation quality",
     prompt="""
   ## Changed Files

   {changed_files_list}

   ## Context

   - Plan: specs/_plans/{plan_name}/plan.md
   - Review for: guardrail violations, dead code, obsolete tests, bad comments, optimizations
   """
   )
   ```
3. **Process findings** — If findings exist:
   - Create fix tasks in `tasks.md`
   - Spawn `implementer-agent` with fix tasks
4. **Proceed to verification** — Phase 5 verifies all tests pass

### Phase 5: Verification

Execute commands from plan's `## Verification` section:

- Build → exit 0
- Test → 0 failures
- Lint → 0 errors
- Format → no changes

Update tasks.md verification tasks as completed.

### Phase 6: Verification Report

Generate using `references/verification-template.md`.

Save to: `specs/_plans/<plan-name>/verification-report.md`

### Phase 7: Completion

```
✓ All tasks in tasks.md marked [x]
✓ Code review passed (or findings fixed)
✓ Verification passed
✓ Report generated

Ready for: /speq-record <plan-name>
```

## Context Recovery

If context is lost or compacted:

1. Read `specs/_plans/<plan-name>/tasks.md`
2. Identify incomplete tasks (`[ ]` or `[~]`)
3. Resume from first incomplete task
4. Continue orchestration workflow

## References

| File | Use When |
|------|----------|
| `references/tdd-cycle-checklist.md` | Sub-agent TDD reference |
| `references/task-flow.md` | Task lifecycle management |
| `references/verification-template.md` | Phase 6 report generation |
