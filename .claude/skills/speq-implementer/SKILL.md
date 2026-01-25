---
name: speq-implementer
description: |
  Implementation workflow for executing approved plan deltas.
  Use after speq-planner creates a plan in specs/_plans/<plan-name>/.
  Invoke explicitly with /speq-implementer <plan-name> to implement features.
  Triggers: implement spec-driven plan, implement spec-driven feature
---

# Spec Implementer

Orchestrate implementation of plans in `specs/_plans/<plan-name>` using TDD.

Get plan name from user prompt or ask if none specified.

## Golden Rule

**No production code without a failing test first.**

## Orchestrator Role

The main agent acts as **orchestrator**:
- Creates and maintains `tasks.md` for persistence
- Spawns sub-agents for parallel task groups
- Updates task status after sub-agent completion
- Never implements directly — delegates all coding work

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
2. **Spawn sub-agent** — See `references/sub-agent-prompt.md`
3. **Await completion** — Sub-agent returns with results
4. **Mark completed** — Update tasks.md: `[~]` → `[x]` for completed tasks
5. **Update TaskTools** — `TaskUpdate(taskId, status: "completed")`
6. **Next group** — Proceed to next parallel group

**Sub-agent invocation:**

```
Task(
  subagent_type: "general-purpose",
  description: "Implement <group-name> tasks",
  prompt: <see references/sub-agent-prompt.md>
)
```

### Phase 4: Dead Code Removal

From plan's `## Dead Code Removal`:

1. Verify code exists
2. Delete code
3. Run tests → verify nothing broke

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
✓ Dead code removed
✓ Verification passed
✓ Report generated

Ready for: /speq-recorder <plan-name>
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
| `references/sub-agent-prompt.md` | Spawning implementation sub-agents |
| `references/tdd-cycle-checklist.md` | Sub-agent TDD reference |
| `references/task-flow.md` | Task lifecycle management |
| `references/verification-template.md` | Phase 6 report generation |
