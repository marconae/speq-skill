# Verification Report: add-decision-log-validation

**Generated:** 2026-04-27

## Verdict

| Result | Details |
|--------|---------|
| **PASS** | All 175 tests pass; build, lint, format, and manual tests all clean |

| Check | Status |
|-------|--------|
| Build | ✓ |
| Tests | ✓ |
| Lint | ✓ |
| Format | ✓ |
| Scenario Coverage | ✓ |
| Manual Tests | ✓ |

## Test Evidence

### Test Results

| Type | Run | Passed | Failed |
|------|-----|--------|--------|
| Unit (decision_log module) | 20 | 20 | 0 |
| Unit (plan, validate, other) | 92 | 92 | 0 |
| Integration (plan_validate) | 25 | 25 | 0 |
| Integration (decision_log_validate) | 7 | 7 | 0 |
| Integration (other) | 31 | 31 | 0 |

### Manual Tests

| Test | Command | Result |
|------|---------|--------|
| plan validate with decision-log.md | `./target/debug/speq plan validate add-decision-log-validation` | ✓ Passed, exit 0 |
| decision-log validate (valid) | `cd tests/fixtures/decision_log_validate/valid && ../../../../target/debug/speq decision-log validate` | ✓ "Permanent decision log validation passed.", exit 0 |
| decision-log validate (missing) | `cd tests/fixtures/decision_log_validate/missing && ../../../../target/debug/speq decision-log validate` | ✓ "decision-log.md not found", exit 1 |

## Tool Evidence

### Linter

```
Checking speq-skill v0.3.0 (/home/ferris/code/speq-skill)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s
```
Zero warnings, zero errors.

### Formatter

`cargo fmt` applied minor style fixes to `src/plan.rs` and `src/main.rs`. `cargo fmt --check` clean after apply.

## Scenario Coverage

| Domain | Feature | Scenario | Test Location | Test Name | Passes |
|--------|---------|----------|---------------|-----------|--------|
| cli | plan-validate | Validate plan with valid decision-log.md passes | `tests/plan_validate.rs` | `decision_log::plan_with_valid_decision_log_passes` | Pass |
| cli | plan-validate | Validate plan without decision-log.md still passes | `tests/plan_validate.rs` | `decision_log::plan_without_decision_log_passes` | Pass |
| cli | plan-validate | Validate plan with decision-log missing Date field | `tests/plan_validate.rs` | `decision_log::plan_decision_log_missing_date_fails` | Pass |
| cli | plan-validate | Validate plan with decision-log having no sections | `tests/plan_validate.rs` | `decision_log::plan_decision_log_no_sections_fails` | Pass |
| cli | plan-validate | Validate plan with decision-log having invalid Promotes to ADR value | `tests/plan_validate.rs` | `decision_log::plan_decision_log_bad_promote_warns` | Pass |
| cli | plan-validate | Validate plan with decision-log having wrong H1 heading | `tests/plan_validate.rs` | `decision_log::plan_decision_log_bad_h1_fails` | Pass |
| cli | decision-log-validate | Validate a well-formed permanent decision log | `tests/decision_log_validate.rs` | `valid_permanent_log_passes` | Pass |
| cli | decision-log-validate | Validate when permanent decision log is missing | `tests/decision_log_validate.rs` | `missing_permanent_log_fails` | Pass |
| cli | decision-log-validate | Validate fails on non-sequential ADR numbers | `tests/decision_log_validate.rs` | `non_sequential_adr_numbers_fail` | Pass |
| cli | decision-log-validate | Validate fails on ADR missing required field | `tests/decision_log_validate.rs` | `adr_missing_required_field_fails` | Pass |
| cli | decision-log-validate | Validate fails on invalid Status value | `tests/decision_log_validate.rs` | `invalid_status_value_fails` | Pass |
| cli | decision-log-validate | Validate fails when ADRs do not start at 001 | `tests/decision_log_validate.rs` | `adr_must_start_at_001` | Pass |
| cli | decision-log-validate | Validate passes when optional ADR sections are absent | `tests/decision_log_validate.rs` | `optional_sections_absent_passes` | Pass |

## Notes

- `speq plan validate add-decision-log-validation` produces two warnings from the plan's own `decision-log.md`. These are expected — the document's Interview and Design Decisions sections contain template prose that textually references `Promotes to ADR: yes/no` and `Promotes to ADR: maybe`. The validator correctly surfaces them as warnings (exit 0) since they are not valid `yes`/`no` values. This is correct behavior by design.
