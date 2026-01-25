# TDD Cycle Checklist

Per-iteration checklist for implementing a single scenario or behavior.

## Before Starting

- [ ] Read the scenario from feature spec
- [ ] Search codebase for existing implementation
- [ ] Understand what needs to be tested

## RED Phase

| Step | Action | Verify |
|------|--------|--------|
| 1 | Write ONE test for ONE behavior | Test is focused, name is clear |
| 2 | Run the test | Command executes without errors |
| 3 | Verify test FAILS | Failure is for expected reason (missing code, not typo) |
| 4 | Show output | Fresh output displayed in response |

### RED Checklist

- [ ] Test name describes the behavior being tested
- [ ] Test has single clear assertion
- [ ] Test failure message is meaningful
- [ ] Ran test and showed output
- [ ] Failure is because feature doesn't exist (not syntax error)

## GREEN Phase

| Step | Action | Verify |
|------|--------|--------|
| 1 | Write SIMPLEST code to pass | No extra features |
| 2 | Run the test | Test passes |
| 3 | Show output | Fresh output displayed in response |

**Note:** Sub-agents run only the changed test. Full suite runs in verification phase.

### GREEN Checklist

- [ ] Implementation is minimal (just enough to pass)
- [ ] No "improvements" or "nice-to-haves" added
- [ ] Test passes
- [ ] Ran test and showed output

## REFACTOR Phase

| Step | Action | Verify |
|------|--------|--------|
| 1 | Improve code structure | Remove duplication, improve names |
| 2 | Run the test | Still passing |
| 3 | Run linter | No warnings |
| 4 | Show output | Fresh output displayed in response |

### REFACTOR Checklist

- [ ] No new behavior added
- [ ] Duplication removed
- [ ] Names are clear and descriptive
- [ ] Test still passes
- [ ] Linter passes
- [ ] Ran test + lint and showed output

## Iteration Complete

After completing RED-GREEN-REFACTOR for one behavior:

- [ ] Scenario is covered by test
- [ ] Code is clean and minimal
- [ ] All evidence shown in response
- [ ] Ready for next behavior or task completion

## Common Mistakes

| Mistake | Correction |
|---------|------------|
| Test passes immediately | Delete test, check assumptions |
| Multiple behaviors in one test | Split into separate tests |
| Code written before test | Delete code, write test first |
| "Should pass" without running | Run command, show output |
| Refactoring adds behavior | Undo, keep refactor pure |
