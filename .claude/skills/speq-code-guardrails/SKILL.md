---
name: speq-code-guardrails
description: TDD cycle and code quality guardrails — failing-test-first, evidence, dependency, error-handling, and test-quality rules. Triggered by /speq-implement, implementer-agent, implementer-expert-agent, and code-reviewer before any implementation or review work.
---

# Code Guardrails

**Clean Code** (Martin) TDD workflow and quality guardrails.

## Golden Rule

**No production code without a failing test first.** Before changing existing code that has no test, add one that captures its current behavior first.

## Evidence Rule

**No claim without evidence.** Run command, show output, then claim.

## Dependency Rule

**No new dependency without confirming the standard library or an already-installed dependency cannot do it first.**

## TDD Cycle (London School)

```
RED    → Write failing test, run it, show failure
GREEN  → Minimal code to pass, run test, show pass
REFACTOR → Clean up, run test + lint, show output
```

Run ONLY the test you created/changed — not the full suite.

## Tests

- Arrange, act, assert. One concept per test.
- The test name states the condition and the expected behavior.
- Independent and repeatable: no shared mutable state, no real clock, network, filesystem, or unseeded randomness.
- Cover every branch and every edge case: empty, single, maximum, off-by-one, transition, and each way the operation can fail.
- Assert observable behavior, never internal state.
- Pure logic is tested without doubles; orchestration is tested with doubles at its abstractions; adapters are tested against the real thing.
- Test code follows every rule in this document, same as production code.
- A skipped or ignored test is a defect. Fix it or delete it.

## Guiding Principles

| Principle | Meaning |
|-----------|---------|
| **KISS** | Simplest solution that works |
| **YAGNI** | Build for now, not hypotheticals |
| **DRY** | Extract duplication on the third occurrence, not the second (Rule of Three) |
| **Single Responsibility** (**SOLID**) | One function = one purpose |
| **Boy Scout** | Leave code cleaner than you found it |
| **Root Cause** | **Five Whys** — fix the source, not the symptom |

## Design

- Law of Demeter: talk only to immediate collaborators
- Dependency direction, module depth, and boundary placement: see `/speq-design-philosophy`

## Functions

- Small and focused
- Few arguments (≤3 ideal)
- No side effects
- No boolean flags — split into separate methods
- No selector arguments of any type — an argument that picks a branch means two functions
- One level of abstraction per function — decide or do the work, not both
- A function either mutates something or answers a question — never both in one call (Command-Query Separation)
- No output parameters — return the value
- Guard clauses first; one unindented main path
- No undocumented ordering contract between calls — if a second call requires a first, make it unreachable without it

## Errors

- Failure is signalled by the language's own error mechanism — never a sentinel value, magic number, or in-band absent value
- Absence is explicit — an empty collection or the language's optional type, never a stand-in for a value
- Every error states what was attempted, the input that failed, and the constraint violated
- Translate a third-party error into this module's own error type at the boundary — callers never handle a provider's error taxonomy
- Never discard an error, never catch broadly — handle at one level, let the rest propagate
- Errors signal exceptional conditions, not control flow
- The failure path is written test-first, like any other behavior

## Naming

- Descriptive, unambiguous, pronounceable
- Named constants over magic numbers
- No prefixes or type encodings
- Types are nouns, functions are verbs, predicates read as questions
- One word per concept across the codebase
- Name length matches scope — single letters only within a few lines
- No role-suffix names that state no responsibility (Manager, Processor, Handler, Data, Info, Util)
- A name reflects what a thing is for, never how it's built — keep transport, vendor, or storage format out of it
- Domain vocabulary over generic programming vocabulary

## Comments

- Public/interface methods: brief doc comment stating what the method promises — purpose, and design intent or rationale when that isn't self-evident from the signature
- Private methods: no comments
- No inline comments — code should be self-explanatory
- No work tracking (TODOs, FIXMEs, ticket refs)

## YAGNI Checks

- Abstraction (interface, generic type, configuration value) with one implementation/caller? Inline it — unless it's a seam over I/O, nondeterminism, or a third party, or the concrete choice is expected to change. Those seams stay.
- Feature flag or extension point nobody uses? Remove it.

## Code Smells

| Smell | Signal |
|-------|--------|
| Rigidity | Small changes cascade everywhere |
| Fragility | One change breaks unrelated code |
| Immobility | Can't reuse code elsewhere |
| Opacity | Hard to understand at a glance |

## Attribution

Concepts from Robert C. Martin's *Clean Code: A Handbook of Agile Software Craftsmanship*, adapted here.
