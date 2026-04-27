# Plan: add-decision-log-validation

## Summary

Add structural validation for two flavors of decision log: extend `speq plan validate` to validate a plan's optional `decision-log.md` (lightweight per-plan format), and add a new `speq decision-log validate` command for the permanent `specs/decision-log.md` (ADR / Nygard format).

## Design

### Context

speq is gaining a decision-log workflow: each plan optionally records design decisions in `decision-log.md`, and `speq record` promotes selected entries into the permanent ADR file at `specs/decision-log.md`. Without validation, malformed decision logs would silently propagate into the permanent record. Two validators are needed because the two formats are intentionally different — the per-plan log is conversational and lightweight, the permanent log is strict ADR format.

- **Goals**
  - Catch structural issues in plan-level `decision-log.md` during `speq plan validate` before recording.
  - Validate the permanent `specs/decision-log.md` independently via `speq decision-log validate`.
  - Keep plan-level decision logs OPTIONAL (absence MUST NOT error).
  - Reuse the existing parser/rules/report module pattern from `src/validate/`.
- **Non-Goals**
  - Cross-validation between plan-level and permanent logs (e.g., verifying that a plan-log entry marked `Promotes to ADR: yes` actually appears in the permanent log).
  - Auto-fixing or rewriting decision logs.
  - Linking ADRs back to the plans that produced them (verification only — no graph walks).
  - Renumbering, reformatting, or otherwise mutating files.

### Decision

Introduce a single new module `src/validate/decision_log.rs` that exposes two entry points: `validate_plan_log(content: &str, plan_name: &str)` and `validate_permanent_log(content: &str)`. Both return a `DecisionLogValidationResult` with errors and warnings, modeled after the existing `ValidationResult` shape in `src/validate/report.rs`. The plan-level path is invoked from `src/plan.rs::validate_plan` if `decision-log.md` exists. The permanent-log path is invoked from a new `handle_decision_log_command` in `src/main.rs`, wired through a new `DecisionLog { command: DecisionLogCommands }` variant on `Commands` in `src/cli.rs`.

#### Architecture

```
                    ┌──────────────────────────┐
                    │  src/cli.rs (clap derive)│
                    └────────────┬─────────────┘
                                 │
                ┌────────────────┴────────────────┐
                │                                 │
        ┌───────▼────────┐               ┌────────▼──────────┐
        │ Plan { Validate}│               │ DecisionLog       │
        │  (existing)    │               │   { Validate }    │
        └───────┬────────┘               │  (new)            │
                │                        └────────┬──────────┘
                ▼                                 ▼
       ┌─────────────────┐              ┌─────────────────────┐
       │ src/plan.rs     │              │ src/main.rs         │
       │  validate_plan  │              │  handle_decision_   │
       │   ┌── if log ── │              │   log_command       │
       │   │  exists     │              └─────────┬───────────┘
       └───┼─────────────┘                        │
           │                                      │
           ▼                                      ▼
   ┌────────────────────────────────────────────────────┐
   │ src/validate/decision_log.rs (NEW)                 │
   │  - validate_plan_log(content, plan_name)           │
   │  - validate_permanent_log(content)                 │
   │  - DecisionLogValidationResult                     │
   │  - DecisionLogError / DecisionLogWarning           │
   └────────────────────────────────────────────────────┘
```

#### Patterns

| Pattern | Where | Why |
|---------|-------|-----|
| Line-oriented state machine | `decision_log.rs` parser | Logs are flat Markdown with section anchors; full AST traversal is overkill |
| `Result<T, ErrorEnum>` + `thiserror` | error types in `decision_log.rs` | Matches existing `ValidationError` style in `report.rs` |
| Optional file = silent success | `plan.rs` integration | Per-plan log is OPTIONAL by spec; absence MUST NOT trigger an error |
| Two parser functions, one module | `decision_log.rs` | The two formats share enough primitives (heading detection, field extraction) to live together but produce distinct error variants |

### Consequences

| Decision | Alternatives Considered | Rationale |
|----------|------------------------|-----------|
| One module with two entry points | Separate `plan_log.rs` and `permanent_log.rs` modules | The two parsers share line-scanning helpers and live in the same conceptual domain; one module with clearly named functions is simpler |
| Reuse line-iteration approach over `pulldown-cmark` AST | Full Markdown AST traversal | Decision logs use a small, regular surface (H1, H2, bullets, bold-prefixed fields). Line scanning is more direct and less error-prone for these constraints |
| Plan-log validation embedded inside `validate_plan` | Separate CLI flag for plan log | Keeps `speq plan validate` a single source of truth; no extra friction for users |
| `Promotes to ADR` invalid value yields a warning, not an error | Hard error | Aligns with existing tolerance for soft-failures (lowercase keywords) — surfaces the issue without blocking |

## Features

| Feature | Status | Spec |
|---------|--------|------|
| cli/plan-validate | CHANGED | `cli/plan-validate/spec.md` |
| cli/decision-log-validate | NEW | `cli/decision-log-validate/spec.md` |

## Dependencies

- No new external crates. Existing `thiserror` and standard library suffice.

## Implementation Tasks

1. **Test fixtures (plan-level decision log)**
   1.1 Create `tests/fixtures/plan_validate/with-decisions/` with valid `plan.md` + valid `decision-log.md`.
   1.2 Create `tests/fixtures/plan_validate/decisions-no-date/` (decision log missing `Date:` line).
   1.3 Create `tests/fixtures/plan_validate/decisions-no-sections/` (decision log with H1 + Date but no `##` sections).
   1.4 Create `tests/fixtures/plan_validate/decisions-bad-promote/` (decision log entry with `Promotes to ADR: maybe`).
   1.5 Create `tests/fixtures/plan_validate/decisions-bad-h1/` (decision log H1 differs from plan name).

2. **Test fixtures (permanent decision log)**
   2.1 Create `tests/fixtures/decision_log_validate/valid/` with `specs/decision-log.md` containing ADR-001 fully populated.
   2.2 Create `tests/fixtures/decision_log_validate/missing/` with no `decision-log.md`.
   2.3 Create `tests/fixtures/decision_log_validate/non-sequential/` with ADR-001 then ADR-003.
   2.4 Create `tests/fixtures/decision_log_validate/missing-field/` with ADR-001 missing `**Status:**`.
   2.5 Create `tests/fixtures/decision_log_validate/invalid-status/` with `**Status:** Pending`.
   2.6 Create `tests/fixtures/decision_log_validate/non-start-001/` where the first ADR is ADR-002.
   2.7 Create `tests/fixtures/decision_log_validate/optional-absent/` with valid ADR-001 lacking Options Considered + Consequences.

3. **Decision log module**
   3.1 Create `src/validate/decision_log.rs` with `DecisionLogValidationResult`, `DecisionLogError` (thiserror), and `DecisionLogWarning` types. [expert]
   3.2 Implement `validate_plan_log(content: &str, plan_name: &str) -> DecisionLogValidationResult` — H1 match, Date line present, at least one of the three sections, scan decision entries for invalid `Promotes to ADR` values. [expert]
   3.3 Implement `validate_permanent_log(content: &str) -> DecisionLogValidationResult` — H1 match, parse all `## ADR-NNN: <Title>` headings, verify sequential numbering starting at 001, verify each ADR has all required fields (Date, Plan, Status, Context, Decision), verify Status value, allow optional sections to be absent. [expert]
   3.4 Register `pub mod decision_log;` in `src/validate/mod.rs`.

4. **Plan validation integration**
   4.1 In `src/plan.rs`, after the existing delta-spec loop, check for `plan_dir.join("decision-log.md")`; if present, read it and call `decision_log::validate_plan_log`.
   4.2 Surface decision-log errors via `result.add_error(...)` (one error per failure) and warnings via a new `decision_log_warnings: Vec<String>` field on `PlanValidationResult` (or reuse existing warnings mechanism — pick whichever fits cleanest).
   4.3 Update `handle_plan_command` in `src/main.rs` to print decision-log errors/warnings alongside existing output.

5. **CLI wiring for new command**
   5.1 Add `DecisionLog { command: DecisionLogCommands }` to `Commands` in `src/cli.rs`.
   5.2 Add `DecisionLogCommands::Validate` subcommand (no args).
   5.3 Add `handle_decision_log_command` in `src/main.rs` that reads `specs/decision-log.md`, calls `validate_permanent_log`, prints errors/warnings, returns appropriate ExitCode.

6. **Integration tests**
   6.1 Extend `tests/plan_validate.rs` with one test per new plan-level scenario, using fixtures from task 1.
   6.2 Create `tests/decision_log_validate.rs` with one test per scenario in `cli/decision-log-validate`, using fixtures from task 2.

7. **Verification**
   7.1 Run `cargo fmt && cargo clippy` — zero errors/warnings.
   7.2 Run `cargo test` — all tests pass.
   7.3 Run `./target/debug/speq plan validate add-decision-log-validation` — passes.

## Parallelization

| Parallel Group | Tasks |
|----------------|-------|
| Group A | 1 (plan fixtures), 2 (permanent fixtures) |
| Group B | 3 (decision_log module) |
| Group C | 4 (plan integration), 5 (CLI wiring) |
| Group D | 6 (integration tests) |
| Group E | 7 (verification) |

Sequential dependencies:
- Group A → Group D (tests need fixtures)
- Group B → Group C (integration depends on module)
- Group B → Group D (tests exercise the module via the CLI)
- Group C → Group D (integration tests run the CLI)
- Group D → Group E

## Dead Code Removal

| Type | Location | Reason |
|------|----------|--------|
| (none) | — | Pure addition; no existing code is replaced or rendered obsolete |

## Verification

### Scenario Coverage

| Scenario | Test Type | Test Location | Test Name |
|----------|-----------|---------------|-----------|
| Validate plan with valid decision-log.md passes | Integration | `tests/plan_validate.rs` | `plan_with_valid_decision_log_passes` |
| Validate plan without decision-log.md still passes | Integration | `tests/plan_validate.rs` | `plan_without_decision_log_passes` |
| Validate plan with decision-log missing Date field | Integration | `tests/plan_validate.rs` | `plan_decision_log_missing_date_fails` |
| Validate plan with decision-log having no sections | Integration | `tests/plan_validate.rs` | `plan_decision_log_no_sections_fails` |
| Validate plan with decision-log having invalid Promotes to ADR value | Integration | `tests/plan_validate.rs` | `plan_decision_log_bad_promote_warns` |
| Validate plan with decision-log having wrong H1 heading | Integration | `tests/plan_validate.rs` | `plan_decision_log_bad_h1_fails` |
| Validate a well-formed permanent decision log | Integration | `tests/decision_log_validate.rs` | `valid_permanent_log_passes` |
| Validate when permanent decision log is missing | Integration | `tests/decision_log_validate.rs` | `missing_permanent_log_fails` |
| Validate fails on non-sequential ADR numbers | Integration | `tests/decision_log_validate.rs` | `non_sequential_adr_numbers_fail` |
| Validate fails on ADR missing required field | Integration | `tests/decision_log_validate.rs` | `adr_missing_required_field_fails` |
| Validate fails on invalid Status value | Integration | `tests/decision_log_validate.rs` | `invalid_status_value_fails` |
| Validate fails when ADRs do not start at 001 | Integration | `tests/decision_log_validate.rs` | `adr_must_start_at_001` |
| Validate passes when optional ADR sections are absent | Integration | `tests/decision_log_validate.rs` | `optional_sections_absent_passes` |

### Manual Testing

| Feature | Command | Expected Output |
|---------|---------|-----------------|
| cli/plan-validate (with log) | `./target/debug/speq plan validate add-decision-log-validation` | Reports `validation passed`, exits 0 (this plan ships with a valid decision-log.md) |
| cli/plan-validate (no log) | `./target/debug/speq plan validate <some-plan-without-decision-log>` | Reports `validation passed`, no mention of decision-log.md, exits 0 |
| cli/decision-log-validate (valid) | `cd tests/fixtures/decision_log_validate/valid && ../../../../target/debug/speq decision-log validate` | Reports `Permanent decision log validation passed`, exits 0 |
| cli/decision-log-validate (missing) | `cd tests/fixtures/decision_log_validate/missing && ../../../../target/debug/speq decision-log validate` | Reports `decision-log.md not found`, exits non-zero |

### Checklist

| Step | Command | Expected |
|------|---------|----------|
| Build | `cargo build` | Exit 0 |
| Test | `cargo test` | 0 failures |
| Lint | `cargo clippy` | 0 errors/warnings |
| Format | `cargo fmt --check` | No changes |
| Plan validation | `./target/debug/speq plan validate add-decision-log-validation` | Exit 0 |
