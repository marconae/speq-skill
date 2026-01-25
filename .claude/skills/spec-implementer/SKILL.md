---
name: spec-implementer
description: |
  Implementation workflow for executing approved plan deltas.
  Use after spec-planner creates a plan in specs/_plans/<plan-name>/.
  Invoke explicitly with /spec-implementer <plan-name> to implement features.
  Triggers: implement spec-driven plan, implement spec-driven feature
---

# Spec Implementer

Implement plans in `specs/_plans/<plan-name>` using TDD.

Get plan name from user prompt or ask if none specified.

## Golden Rule

**No production code without a failing test first.**

## TDD Red Flags → Delete & Restart

- Code written before test
- Test passes immediately
- "I'll test after"
- "Too simple to test"

## Scope Rules

- Search codebase first — never assume something isn't implemented
- Write tests for NEW functionality only
- Do NOT refactor existing tests unless broken
- Do NOT add features not in the spec

## Workflow

### Phase 1: Load Plan

```
Read: specs/_plans/<plan-name>/plan.md
```

Extract: feature specs, implementation tasks, verification commands, dead code list.

### Phase 2: Create Tasks

```
For each task in plan's "## Implementation Tasks":
  TaskCreate(subject, description, activeForm)
```

See `references/task-flow.md` for task lifecycle.

### Phase 3: Implement (TDD Cycle)

For each task:

1. **Read spec** — Get scenarios via `speq feature get`
2. **Search** — Check if implementation exists
3. **RED** — Write ONE failing test → run → show failure
4. **GREEN** — Simplest code to pass → run → show pass
5. **REFACTOR** — Clean up → run tests + lint → show output
6. **Complete** — `TaskUpdate(taskId, status: "completed")`

See `references/tdd-cycle-checklist.md` for detailed checklist.

### Phase 4: Dead Code Removal

From plan's `## Dead Code Removal`:

1. Verify code exists
2. Delete code
3. Run tests → verify nothing broke

### Phase 5: Verification

Execute commands from plan's `## Verification` section. Show output for each:

- Build → exit 0
- Test → 0 failures
- Lint → 0 errors
- Format → no changes

### Phase 6: Verification Report

Generate using `references/verification-template.md`:

1. Copy test/coverage evidence
2. Copy tool evidence
3. Fill scenario coverage
4. Document limitations

Save to: `specs/_plans/<plan-name>/verification-report.md`

### Phase 7: Completion

```
✓ All tasks completed
✓ Dead code removed
✓ Verification passed
✓ Report generated

Ready for: /spec-recorder <plan-name>
```

## References

| File | Use When |
|------|----------|
| `references/tdd-cycle-checklist.md` | During Phase 3 |
| `references/task-flow.md` | Managing tasks |
| `references/verification-template.md` | Phase 6 |
