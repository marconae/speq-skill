---
name: code-guardrails
description: |
  TDD workflow and code quality guardrails for implementation.
  Use for: writing production code, implementing features, fixing bugs.
  Triggers: implement, code, TDD, test-driven, write code, fix bug.
  Core rule: No production code without failing test first.
  Evidence rule: No claim without running command and showing output.
---

# Code Guardrails

TDD workflow and code quality guardrails.

## Golden Rule

**No production code without a failing test first.**

## Evidence Rule

**No claim without evidence.** Run command, show output, then claim.

## TDD Cycle

```
RED    → Write failing test, run it, show failure
GREEN  → Minimal code to pass, run test, show pass
REFACTOR → Clean up, run test + lint, show output
```

Run ONLY the test you created/changed — not the full suite.

## Guiding Principles

| Principle | Meaning |
|-----------|---------|
| **KISS** | Simplest solution that works |
| **YAGNI** | Build for now, not hypotheticals |
| **DRY** | Extract duplication, don't copy-paste |
| **Single Responsibility** | One function = one purpose |
| **Boy Scout** | Leave code cleaner than you found it |
| **Root Cause** | Fix the source, not the symptom |

## Design

- Config at high levels, behavior at low levels
- Polymorphism over conditionals
- Dependency injection for testability
- Law of Demeter: talk only to immediate collaborators

## Functions

- Small and focused
- Few arguments (≤3 ideal)
- No side effects
- No boolean flags — split into separate methods

## Naming

- Descriptive, unambiguous, pronounceable
- Named constants over magic numbers
- No prefixes or type encodings

## Comments

- Public/interface methods: brief doc comment (purpose only)
- Private methods: no comments
- No inline comments — code should be self-explanatory
- No work tracking (TODOs, FIXMEs, ticket refs)

## Code Smells

| Smell | Signal |
|-------|--------|
| Rigidity | Small changes cascade everywhere |
| Fragility | One change breaks unrelated code |
| Immobility | Can't reuse code elsewhere |
| Opacity | Hard to understand at a glance |
