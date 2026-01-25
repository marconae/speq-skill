# Evidence Requirements

Mapping of claims to required evidence with examples of sufficient vs insufficient evidence.

## Evidence Table

| Claim | Required Command | Sufficient Evidence | Insufficient Evidence |
|-------|-----------------|---------------------|----------------------|
| "Test fails (RED)" | Run test command | Fresh output showing failure for expected reason | "It should fail", previous output |
| "Test passes (GREEN)" | Run test command | Fresh output showing 0 failures | "It passed earlier", assumption |
| "All tests pass" | Run full test suite | Complete suite output, 0 failures | Single test output, previous run |
| "Lint clean" | Run lint command | Output showing 0 errors/warnings | "I ran it", partial check |
| "Build succeeds" | Run build command | Exit code 0, no errors | "Build should work" |
| "Code formatted" | Run format check | Output showing no changes needed | "I formatted it" |
| "Coverage ≥80%" | Run coverage command | Coverage report showing percentage | "Coverage is good" |
| "Bug fixed" | Run reproducing test | Test that failed now passes | "Code changed" |

## Evidence Examples

### Good Evidence (RED)

```
$ cargo test test_new_feature
running 1 test
test test_new_feature ... FAILED

failures:
    test_new_feature: not yet implemented

test result: FAILED. 0 passed; 1 failed
```

✓ Shows test ran, shows failure, shows reason

### Bad Evidence (RED)

> "The test should fail because the function doesn't exist yet."

✗ No command run, no output shown

### Good Evidence (GREEN)

```
$ cargo test test_new_feature
running 1 test
test test_new_feature ... ok

test result: ok. 1 passed; 0 failed

$ cargo test
running 47 tests
...
test result: ok. 47 passed; 0 failed
```

✓ Shows specific test passes, shows all tests pass

### Bad Evidence (GREEN)

> "Tests pass now."

✗ No output, can't verify claim

### Good Evidence (Verification Checklist)

```
$ cargo build
   Compiling myproject v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)

$ cargo test
running 47 tests
test result: ok. 47 passed; 0 failed

$ cargo clippy -- -D warnings
    Finished dev target(s)

$ cargo fmt --check
(no output - already formatted)
```

✓ Each command run, each output shown, all pass

### Bad Evidence (Verification Checklist)

> "Build, tests, lint, and format all pass."

✗ No commands, no output, no proof

## Stop Words

When about to use these words, STOP and run verification:

| Word/Phrase | Action |
|-------------|--------|
| "should" | Run command to prove |
| "probably" | Run command to verify |
| "seems to" | Run command to confirm |
| "I think" | Run command to know |
| "Great!" | Full verification first |
| "Perfect!" | Full verification first |
| "Done!" | Show final evidence |

## Evidence Workflow

```
1. Make claim in your head
2. STOP - what command proves this?
3. Run the command
4. Read the output carefully
5. Does output actually prove the claim?
6. If yes → state claim with output
7. If no → investigate and fix
```

## Verification Timing

| When | Verify |
|------|--------|
| After writing test | Test fails for right reason |
| After writing code | Test passes, all tests pass |
| After refactoring | All tests still pass, lint clean |
| Before marking task done | Full verification checklist |
| Before claiming completion | All phases verified |
