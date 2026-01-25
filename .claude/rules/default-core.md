# Core Standards

## Verification Rule

**No claim without evidence.** Run command, show output, then claim.

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

## Naming

- Descriptive, unambiguous, pronounceable
- Named constants over magic numbers
- No prefixes or type encodings

## Functions

- Small and focused
- Few arguments (≤3 ideal)
- No side effects
- No boolean flags—split into separate methods

## Comments

- Public/interface methods: brief doc comment (purpose only)
- Private methods: no comments
- No inline comments
- No code explanations—code should be self-explanatory
- No work tracking (TODOs, FIXMEs, ticket refs)

## Code Smells

| Smell | Signal |
|-------|--------|
| Rigidity | Small changes cascade everywhere |
| Fragility | One change breaks unrelated code |
| Immobility | Can't reuse code elsewhere |
| Opacity | Hard to understand at a glance |
