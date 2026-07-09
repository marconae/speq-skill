---
name: speq-code-review
description: Code review tag taxonomy and findings output format — guardrail violations, dead code, obsolete tests, bad comments, optimizations, YAGNI/over-engineering. Triggered by code-reviewer.
---

# Code Review Taxonomy

Analyze each changed file for the categories below.

**Non-goal:** a deviation the brief notes as authorized by an active project hook (e.g. a skipped guardrail, a relaxed convention) is a settled, intentional choice — don't raise it as a finding under any category below.

## 1. Guardrail Violations

Per `/speq-code-guardrails`:
- `[TOO_MANY_ARGUMENTS]` — more than 3 arguments
- `[SIDE_EFFECT]` — function has side effects
- `[BOOLEAN_FLAG_PARAMETER]` — boolean flag parameter
- `[MAGIC_NUMBER]` — magic number without a named constant
- `[MISSING_DOC_COMMENT]` — missing doc comment on a public interface
- `[INLINE_COMMENT]` — inline comment present (TODOs and other work-tracking comments are `[WORK_TRACKING_COMMENT]`, not this tag)

## 2. Dead Code

- `[UNUSED_FUNCTION]` — unused function or method
- `[UNREACHABLE_CODE]` — unreachable code path
- `[UNUSED_IMPORT]` — import not used
- `[UNUSED_VARIABLE]` — variable assigned but never read

## 3. Obsolete Tests

- `[OBSOLETE_TEST]` — tests removed functionality
- `[DUPLICATE_TEST]` — duplicate test coverage
- `[ASSERTION_FREE_TEST]` — test always passes, no assertions

## 4. Bad Comments

- `[REDUNDANT_COMMENT]` — describes "what" not "why"
- `[OUTDATED_COMMENT]` — doesn't match the code
- `[COMMENTED_OUT_CODE]` — commented-out code block
- `[WORK_TRACKING_COMMENT]` — TODO, FIXME, ticket refs

## 5. Optimization Opportunities

- `[PERFORMANCE_ISSUE]` — obvious performance issue
- `[UNNECESSARY_ALLOCATION]` — unnecessary allocation in a loop
- `[DUPLICATE_OPERATION]` — operation that repeats work already done

## 6. YAGNI / Over-Engineering

Per `/speq-code-guardrails`'s YAGNI Checks:
- `[STANDARD_LIBRARY_DUPLICATE]` — logic that reimplements something the language's standard library already provides
- `[SHRINKABLE]` — same logic expressible in meaningfully fewer lines
- `[DEAD_FLEXIBILITY]` — a feature flag, extension point, or parameter that's never varied
- `[UNNEEDED_DEPENDENCY]` — a dependency added for something the standard library or an already-installed dependency already covers
- `[SPECULATIVE_ABSTRACTION]` — an interface, generic type, or configuration value with exactly one implementation or caller

## Output Format

```markdown
# Code Review Findings

## Summary
- Files reviewed: N
- Total findings: M
- By category: Violations (X), Dead Code (Y), Tests (Z), Comments (W), Optimizations (V), YAGNI (U)

## Findings

### path/to/module

#### [TOO_MANY_ARGUMENTS] Function has too many arguments
- Location: line 42
- Issue: `process_data(a, b, c, d, e, f)` has 6 arguments
- Suggestion: Create a config struct

#### [UNUSED_FUNCTION] Unused function
- Location: line 87
- Issue: `old_helper()` has no callers
- Suggestion: Remove function

### path/to/module_test

#### [OBSOLETE_TEST] Tests removed functionality
- Location: line 15
- Issue: `test_old_feature` tests deleted code
- Suggestion: Remove test

#### [STANDARD_LIBRARY_DUPLICATE] Custom function reimplements a standard library operation
- Location: line 55
- Issue: `dedup_items(...)` reimplements the language's built-in deduplication operation
- Suggestion: Replace with the standard library's deduplication function

#### [SPECULATIVE_ABSTRACTION] Interface with a single implementation
- Location: line 90
- Issue: `Storage` interface has exactly one implementation, `FileStorage`
- Suggestion: Inline `FileStorage`; reintroduce the interface if a second implementation appears
```

## Routing

Every tag across all 6 categories is delegated to `implementer-agent`/`implementer-expert-agent` exactly like any other finding — no special-casing. Tag the resulting fix task `[expert]` if removing the dependency/abstraction has cross-file or subtle-correctness implications.
