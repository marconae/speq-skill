---
name: speq-code-review
description: Code review tag taxonomy and findings output format — guardrail violations, dead code, test quality, bad comments, optimizations, YAGNI/over-engineering, error handling, and design depth. Triggered by code-reviewer.
---

# Code Review Taxonomy

Analyze each changed file for the categories below.

**Non-goal:** a deviation the brief notes as authorized by an active project hook (e.g. a skipped guardrail, a relaxed convention) is a settled, intentional choice — don't raise it as a finding under any category below.

## 1. Guardrail Violations

Per `/speq-code-guardrails`:
- `[TOO_MANY_ARGUMENTS]` — more than 3 arguments
- `[SIDE_EFFECT]` — function has side effects
- `[BOOLEAN_FLAG_PARAMETER]` — boolean flag parameter
- `[MAGIC_NUMBER]` — magic number without a named constant (standing in for a failure, it's `[SENTINEL_ERROR_VALUE]`, not this tag)
- `[MISSING_DOC_COMMENT]` — missing doc comment on a public interface
- `[INLINE_COMMENT]` — inline comment present (TODOs and other work-tracking comments are `[WORK_TRACKING_COMMENT]`, not this tag)
- `[SELECTOR_ARGUMENT]` — an argument (of any type, not just boolean) that picks which branch a function takes
- `[OUTPUT_PARAMETER]` — a value returned via a mutated argument instead of the return value
- `[MIXED_ABSTRACTION_LEVEL]` — a function mixes high-level orchestration with low-level detail
- `[COMMAND_QUERY_MIX]` — a single call both mutates something and hands back an answer
- `[WEASEL_NAME]` — a name that states no responsibility (Manager, Processor, Handler, Data, Info, Util)
- `[IMPLEMENTATION_IN_NAME]` — a name that bakes in a transport, vendor, or format instead of the abstraction

## 2. Dead Code

- `[UNUSED_FUNCTION]` — unused function or method
- `[UNREACHABLE_CODE]` — unreachable code path
- `[UNUSED_IMPORT]` — import not used
- `[UNUSED_VARIABLE]` — variable assigned but never read

## 3. Test Quality

Per `/speq-code-guardrails`' Tests section — tests are quality subjects, not only removal candidates:
- `[OBSOLETE_TEST]` — tests removed functionality
- `[DUPLICATE_TEST]` — duplicate test coverage
- `[ASSERTION_FREE_TEST]` — test always passes, no assertions
- `[VAGUE_TEST_NAME]` — test name doesn't state the condition and expected behavior
- `[NONDETERMINISTIC_TEST]` — test depends on real clock, network, filesystem, or unseeded randomness
- `[IMPLEMENTATION_COUPLED_TEST]` — test asserts internal state instead of observable behavior
- `[UNTESTED_ERROR_PATH]` — a failure path with no test
- `[MISSING_BOUNDARY_TEST]` — no test for empty, single, maximum, off-by-one, or transition input
- `[SKIPPED_TEST]` — test is skipped or ignored rather than fixed or deleted
- `[SUPPRESSED_WARNING]` — a lint or compiler warning is silenced instead of resolved

## 4. Bad Comments

- `[REDUNDANT_COMMENT]` — describes "what" not "why"
- `[OUTDATED_COMMENT]` — doesn't match the code
- `[COMMENTED_OUT_CODE]` — commented-out code block
- `[WORK_TRACKING_COMMENT]` — TODO, FIXME, ticket refs

## 5. Optimization Opportunities

The Evidence Rule applies here too — raise a finding in this category only with a measurement. Without one, the finding is `[UNMEASURED_OPTIMIZATION]` against the code that was optimized speculatively.

- `[PERFORMANCE_ISSUE]` — obvious performance issue
- `[UNNECESSARY_ALLOCATION]` — unnecessary allocation in a loop
- `[DUPLICATE_OPERATION]` — operation that repeats work already done
- `[UNMEASURED_OPTIMIZATION]` — a change justified as a performance optimization with no measurement behind it

## 6. YAGNI / Over-Engineering

Per `/speq-code-guardrails`'s YAGNI Checks:
- `[STANDARD_LIBRARY_DUPLICATE]` — logic that reimplements something the language's standard library already provides
- `[SHRINKABLE]` — same logic expressible in meaningfully fewer lines
- `[DEAD_FLEXIBILITY]` — a feature flag, extension point, or parameter that's never varied
- `[UNNEEDED_DEPENDENCY]` — a dependency added for something the standard library or an already-installed dependency already covers
- `[SPECULATIVE_ABSTRACTION]` — an interface, generic type, or configuration value with exactly one implementation or caller, and not a seam over I/O, nondeterminism, or a third party

## 7. Error Handling

- `[SENTINEL_ERROR_VALUE]` — a magic value or in-band signal stands in for an error instead of the language's own error mechanism
- `[CONTEXTLESS_ERROR]` — an error that doesn't state what was attempted, the input that failed, or the constraint violated
- `[SWALLOWED_ERROR]` — an error is discarded instead of handled or propagated
- `[BROAD_CATCH]` — a catch that's broader than the specific error it's meant to handle
- `[LEAKED_PROVIDER_ERROR]` — a third-party error type crosses a module boundary unwrapped
- `[ERROR_AS_CONTROL_FLOW]` — an error mechanism used for expected, non-exceptional flow

## 8. Design Depth

Per `/speq-design-philosophy`:
- `[SHALLOW_MODULE]` — learning the interface takes almost as much effort as the implementation behind it would, or classitis (many small modules named for a role, not a responsibility — a purely naming defect with no structural symptom is `[WEASEL_NAME]`, not this tag)
- `[INFORMATION_LEAKAGE]` — a single design choice (a format, a protocol, an execution-order split) shows up in more than one module and would need editing in both if it changed
- `[TACTICAL_SHORTCUT]` — a shortcut taken with no follow-up to invest in the design
- `[MISSING_DESIGN_INTENT]` — a public/interface comment states purpose but not the design intent or rationale a non-obvious abstraction needs
- `[BOUNDARY_VIOLATION]` — business logic names a delivery mechanism, storage engine, or framework directly
- `[IO_IN_BUSINESS_LOGIC]` — I/O performed directly inside business logic instead of through an injected abstraction
- `[AMBIENT_STATE_READ]` — environment or global state read in place instead of injected
- `[LEAKED_BOUNDARY_TYPE]` — a framework, storage, or third-party type crosses a module boundary
- `[DEPENDENCY_CYCLE]` — a cycle in the module dependency graph
- `[SELF_CONSTRUCTED_DEPENDENCY]` — a module constructs its own concrete dependency instead of receiving it
- `[PROVIDER_SHAPED_ABSTRACTION]` — an abstraction shaped around a provider's API instead of the consumer's own vocabulary
- `[FEATURE_ENVY]` — a function reaches into another module's data more than its own

## Output Format

```markdown
# Code Review Findings

## Summary
- Files reviewed: N
- Total findings: M
- By category: Violations (X), Dead Code (Y), Tests (Z), Comments (W), Optimizations (V), YAGNI (U), Error Handling (T), Design Depth (S)

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

Every tag across all 8 categories is delegated to `implementer-agent`/`implementer-expert-agent` exactly like any other finding — no special-casing. Tag the resulting fix task `[expert]` if removing the dependency/abstraction, or fixing a dependency-direction/boundary violation, has cross-file or subtle-correctness implications.
