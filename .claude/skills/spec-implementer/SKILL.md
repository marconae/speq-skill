---
name: spec-implementer
description: |
  Implementation workflow for executing approved plan deltas.
  Use after spec-planner creates a plan in specs/_plans/<plan-name>/.
  Invoke explicitly with /spec-implementer <plan-name> to implement features.
  Triggers: implement spec-driven plan, implement spec-driven feature
---

# Spec Implementer

Implement plans in `specs/_plans/<plan-name>` using strict rules with mandatory evidence verification.

Get the name of the plan from the user prompt or ask if none is specified.

## Workflow Position

```
/spec-planner → /spec-implementer → /spec-recorder
     │                  │                  │
     ▼                  ▼                  ▼
Creates plan.md    Implements code    Merges deltas
with deltas        via TDD cycle      to permanent specs
```

## Implementation Workflow

**TDD: NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST.**

### Red-Green-Refactor

| Phase | Do | Don't |
|-------|-----|-------|
| **RED** | One behavior, clear name, run test, verify FAILS | Multiple assertions, vague names |
| **GREEN** | Simplest code to pass, run test, verify PASSES | Extra features, "improvements" |
| **REFACTOR** | Remove duplication, improve names, verify STILL PASSES | Add behavior |

### Verification (Mandatory)

- **RED:** Test fails for expected reason (missing feature, not typo)
- **GREEN:** Test passes, all other tests still pass, no warnings

### Red Flags → Delete & Restart

- Code before test
- Test passes immediately
- "I'll test after"
- "Too simple to test"

## Evidence Before Claims

**Mandatory:** No claim of success without running verification commands and showing output.

### Required Evidence

| Claim | Required Evidence | Insufficient |
|-------|-------------------|--------------|
| "Test fails (RED)" | Fresh test output showing failure | "It should fail" |
| "Test passes (GREEN)" | Output showing 0 failures | Previous run |
| "All tests pass" | Full test suite output | Single test |
| "Lint clean" | Linter output, 0 errors | "I ran it earlier" |
| "Build succeeds" | Build command with exit 0 | Assumption |

### Stop Signals

Pause and run verification when about to:
- Use uncertain language: "should", "probably", "seems to"
- Express satisfaction: "Great!", "Perfect!", "Done!"
- Mark task complete or move to next task

**Rule:** If you haven't run the command in this message, you cannot claim it passes.

## Clean Code Principles

| Principle | Meaning |
|-----------|---------|
| **KISS** | Simplest solution that works |
| **YAGNI** | Build for now, not hypotheticals |
| **DRY** | Extract duplication, don't copy-paste |
| **Single Responsibility** | One function = one purpose |

### Functions
- Small and focused
- Few arguments (≤3 ideal)
- No side effects
- No boolean flags—split into separate methods

### Avoid
- Over-engineering beyond what's asked
- Adding features not in the spec
- Refactoring code you didn't change

## Implementer Rules

- Search codebase first, never assume something isn't implemented
- Use subagents for expensive operations (coding, searching, analysis)
- Write tests for NEW functionality only
- Do NOT refactor existing tests unless broken

### Priority Order

1. Proven Functionality (code works)
2. Integration Tests (scenarios covered)
3. Unit Tests (isolated units)
4. Implementation (clean code)

## 9-Phase Workflow

### Phase 1: Load Plan

```
Read: specs/_plans/<plan-name>/plan.md
```

Extract from plan:
- Feature specs to implement
- Implementation tasks
- Verification commands
- Dead code to remove

### Phase 2: Create Tasks

Use `TaskCreate` for each Implementation Task from plan.md:

```
For each task in plan's "## Implementation Tasks":
  TaskCreate(
    subject: "<task description>",
    description: "<details from plan>",
    activeForm: "<present continuous form>"
  )
```

### Phase 3: Implement (TDD Cycle)

For each task, follow `references/tdd-cycle-checklist.md`:

1. **Read feature spec** — Get scenarios from `specs/_plans/<plan-name>/<domain>/<feature>/spec.md`
2. **Search first** — Check if implementation already exists
3. **RED** — Write ONE failing test, run, verify failure, show output
4. **GREEN** — Simplest code to pass, run, verify pass, show output
5. **REFACTOR** — Clean up, run tests + lint, show output
6. **Update task** — `TaskUpdate(taskId, status: "completed")`

### Phase 4: Dead Code Removal

From plan's `## Dead Code Removal` table:

```
For each row:
  1. Verify code exists
  2. Delete code
  3. Run tests to verify nothing broke
```

### Phase 5: Verification Checklist

Execute each command from plan's `## Verification > Checklist`:

| Step | Action |
|------|--------|
| Build | Run command, verify exit 0 |
| Test | Run command, verify 0 failures |
| Lint | Run command, verify 0 errors |
| Format | Run command, verify no changes |
| Coverage | Run command, verify ≥80% |

**Show output for each step.**

### Phase 6: Manual Testing

Execute each test from plan's `## Verification > Manual Testing`:

```
For each row in Manual Testing table:
  1. Run the test steps
  2. Verify expected result
  3. Show output
```

### Phase 7: Scenario Verification

Cross-check plan's `## Verification > Scenario Verification`:

```
For each scenario:
  1. Verify test exists at specified location
  2. Verify test passes
  3. Mark scenario as covered
```

All scenarios MUST have passing tests.

### Phase 8: Verification Report

Generate report using `references/verification-template.md`:

1. **Date** — Set "Generated" to current date (YYYY-MM-DD)
2. **Test Evidence** — Copy coverage % and test counts from Phase 5 output
3. **Tool Evidence** — Copy linter/formatter output from Phase 5
4. **Scenario Coverage** — Fill table from Phase 7 cross-check
5. **Notes** — Document any limitations or known issues

Save to: `specs/_plans/<plan-name>/verification-report.md`

**Re-runs:** Always overwrite existing report with fresh evidence.

### Phase 9: Completion

When all phases pass:

```
✓ All implementation tasks completed
✓ Dead code removed
✓ Verification checklist passed
✓ Manual tests passed
✓ All scenarios have tests
✓ Verification report generated

Ready for: /spec-recorder <plan-name>
```

## Anti-Patterns

| Pattern | Why It's Wrong |
|---------|----------------|
| Assume code missing without searching | May duplicate existing code |
| Write code before test | Violates TDD |
| Skip RED verification | Can't confirm test tests the right thing |
| Add "extra" test coverage | Scope creep, busy work |
| Refactor existing tests | Out of scope, risk breaking things |
| Claim without output | No evidence |

## References

- `references/tdd-cycle-checklist.md` — Per-iteration RED/GREEN/REFACTOR checklist
- `references/evidence-requirements.md` — Claim-to-command mapping with examples
- `references/task-flow.md` — Task lifecycle and parallel group handling
- `references/verification-template.md` — Final report template with test/tool evidence
