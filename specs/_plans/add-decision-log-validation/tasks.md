# Tasks: add-decision-log-validation

## Group A: Test Fixtures (parallel with Group B)

- [x] 1.1 Create `tests/fixtures/plan_validate/with-decisions/` (valid plan.md + valid decision-log.md)
- [x] 1.2 Create `tests/fixtures/plan_validate/decisions-no-date/` (decision log missing Date: line)
- [x] 1.3 Create `tests/fixtures/plan_validate/decisions-no-sections/` (decision log with H1 + Date but no ## sections)
- [x] 1.4 Create `tests/fixtures/plan_validate/decisions-bad-promote/` (decision log with `Promotes to ADR: maybe`)
- [x] 1.5 Create `tests/fixtures/plan_validate/decisions-bad-h1/` (decision log H1 differs from plan name)
- [x] 2.1 Create `tests/fixtures/decision_log_validate/valid/` (specs/decision-log.md with ADR-001 fully populated)
- [x] 2.2 Create `tests/fixtures/decision_log_validate/missing/` (no decision-log.md)
- [x] 2.3 Create `tests/fixtures/decision_log_validate/non-sequential/` (ADR-001 then ADR-003)
- [x] 2.4 Create `tests/fixtures/decision_log_validate/missing-field/` (ADR-001 missing **Status:**)
- [x] 2.5 Create `tests/fixtures/decision_log_validate/invalid-status/` (**Status:** Pending)
- [x] 2.6 Create `tests/fixtures/decision_log_validate/non-start-001/` (first ADR is ADR-002)
- [x] 2.7 Create `tests/fixtures/decision_log_validate/optional-absent/` (valid ADR-001 lacking Options Considered + Consequences)

## Group B: Decision Log Module (parallel with Group A)

- [x] 3.1 Create `src/validate/decision_log.rs` with DecisionLogValidationResult, DecisionLogError (thiserror), DecisionLogWarning types [expert]
- [x] 3.2 Implement `validate_plan_log(content, plan_name)` — H1 match, Date, sections, Promotes to ADR [expert]
- [x] 3.3 Implement `validate_permanent_log(content)` — H1, ADR headings, sequential numbering, required fields, Status value [expert]
- [x] 3.4 Register `pub mod decision_log;` in `src/validate/mod.rs`

## Group C: Integration (depends on Group B)

- [x] 4.1 In `src/plan.rs`, check for decision-log.md and call `decision_log::validate_plan_log` if present
- [x] 4.2 Surface decision-log errors/warnings in PlanValidationResult
- [x] 4.3 Update `handle_plan_command` in `src/main.rs` to print decision-log errors/warnings
- [x] 5.1 Add `DecisionLog { command: DecisionLogCommands }` to `Commands` in `src/cli.rs`
- [x] 5.2 Add `DecisionLogCommands::Validate` subcommand (no args)
- [x] 5.3 Add `handle_decision_log_command` in `src/main.rs`

## Group D: Integration Tests (depends on Groups A, B, C)

- [x] 6.1 Extend `tests/plan_validate.rs` with one test per new plan-level decision log scenario
- [x] 6.2 Create `tests/decision_log_validate.rs` with one test per permanent log scenario

## Group E: Verification (depends on Group D)

- [x] 7.1 Run `cargo fmt && cargo clippy` — zero errors/warnings
- [x] 7.2 Run `cargo test` — all tests pass
- [x] 7.3 Run `./target/debug/speq plan validate add-decision-log-validation` — passes
