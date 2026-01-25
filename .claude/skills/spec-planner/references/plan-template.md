# Plan: <plan-name>

## Summary

One-paragraph description of what this plan accomplishes.

## Features

| Feature | Status | Spec |
|---------|--------|------|
| <feature-name> | NEW / CHANGED / REMOVED | `<path>/spec.md` |

Status values:
- **NEW** — Feature doesn't exist yet
- **CHANGED** — Modifying existing feature behavior
- **REMOVED** — Deprecating/deleting feature

## Requirements

<!-- Optional: High-level requirements if not fully captured in feature specs -->

| Requirement | Details |
|-------------|---------|
| ... | ... |

## Dependencies

<!-- Optional: External dependencies, libraries, or prerequisite work -->

## Migration

<!-- Optional: For changes affecting existing data/structure -->

| Current | New |
|---------|-----|
| ... | ... |

## Implementation Tasks

1. Task description
2. Task description
3. ...

## Parallelization

<!-- Optional: Tasks that can run concurrently -->

| Parallel Group | Tasks |
|----------------|-------|
| Group A | Task 1, Task 2 |
| Group B | Task 3, Task 4 |

Sequential dependencies:
- Group A → Group B (B depends on A)

## Dead Code Removal

<!-- Required: Identify obsolete code to remove -->

| Type | Location | Reason |
|------|----------|--------|
| Function | `<path>` | Replaced by X |
| Test | `<path>` | Tests removed feature |
| Module | `<path>` | No longer used |

## Verification

**Principle:** Integration Test > Unit Test. Evidence before claims.

Run commands from `specs/mission.md § Commands`:

1. **Test** — ALL tests pass (unit, integration, e2e)
2. **Coverage** — ~80% code coverage (use command from `specs/mission.md § Tech Stack`)
3. **Lint** — No errors or warnings (use tooling, agents MUST NOT format manually)
4. **Format** — No changes (use tooling, agents MUST NOT format manually)
5. **Dead Code** — Removed all obsolete code identified above

### Evidence Requirements

| Claim | Required Evidence |
|-------|-------------------|
| "Tests pass" | Fresh test run output showing 0 failures |
| "Coverage ~80%" | Fresh coverage command output showing percentage |
| "Lint clean" | Linter output showing 0 errors/warnings |
| "Formatted" | Formatter output showing no changes |
| "Feature works" | Integration test covering all scenarios |

**Rule:** No claim without running the command and showing output in this session.

### Code Coverage

Coverage command is defined in `specs/mission.md § Tech Stack`. Example:

```bash
# Rust
cargo tarpaulin --out Html

# Node.js
npm run test:coverage

# Python
pytest --cov=src --cov-report=term
```

Target: ~80% line coverage. Evidence MUST be fresh command output showing coverage percentage.

### Scenario Verification

Each feature spec scenario MUST be verified:

| Scenario | Test Type | Test Location |
|----------|-----------|---------------|
| <scenario-name> | Integration / Unit | `<test-path>` |

- **Integration test** — Default for all scenarios
- **Unit test** — Only if scenario is isolated and small
- **Documentation** — Never sufficient alone

A feature is complete when ALL scenarios have passing integration tests.
