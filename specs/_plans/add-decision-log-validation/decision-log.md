# Decision Log: add-decision-log-validation

Date: 2026-04-27

## Interview

**Q:** What format does the plan-level decision-log.md use?
**A:** Light conversational format — NOT full ADR. The file has H1 `# Decision Log: <plan-name>`, a `Date: YYYY-MM-DD` line, and at least one of `## Interview`, `## Design Decisions`, `## Review Findings`. Each decision entry has `- **Decision:**`, `- **Alternatives:**`, `- **Rationale:**`, `- **Promotes to ADR:** yes/no`.

**Q:** What format does the permanent specs/decision-log.md use?
**A:** ADR (Nygard) format — single file with H1 `# Architecture Decision Records`, one or more `## ADR-NNN: <Title>` headings (sequential, no gaps, starting at 001). Each ADR requires `**Date:**`, `**Plan:**`, `**Status:**`, `### Context`, `### Decision`. Status must be `Accepted`, `Superseded by ADR-NNN`, or `Deprecated`. `### Options Considered` and `### Consequences` are optional.

**Q:** How should the CLI surface validation for decision logs?
**A:** Two separate concerns. `speq plan validate <plan-name>` is EXTENDED to validate `decision-log.md` in the plan dir if present (non-breaking — the file is optional, absence is not an error). New command `speq decision-log validate` validates `specs/decision-log.md`.

**Q:** Should the plan-level decision-log.md be required?
**A:** No. It is optional. If absent, `speq plan validate` succeeds without mentioning it. If present, it must pass validation.

## Design Decisions

### [1] One module hosting both validators

- **Decision:** Place both `validate_plan_log` and `validate_permanent_log` in a single new module `src/validate/decision_log.rs`.
- **Alternatives:** Split into `plan_log.rs` and `permanent_log.rs` modules.
- **Rationale:** The two parsers share line-scanning helpers (heading detection, `**Field:**` extraction, section delimiters). A single module with two clearly-named entry points minimizes duplication and keeps the conceptual surface small.
- **Promotes to ADR:** yes

### [2] Line-oriented parsing instead of pulldown-cmark AST

- **Decision:** Implement validation as a line state machine, not via Markdown AST traversal.
- **Alternatives:** Use `pulldown-cmark` (already a dependency) and walk events.
- **Rationale:** Decision logs use a small, regular surface (H1, H2, H3, bullets, bold-prefixed key-value lines). Line scanning is more direct, easier to surface line numbers from, and avoids dragging the entire AST through validation paths.
- **Promotes to ADR:** yes

### [3] Plan-log validation embedded in `validate_plan` (no flag)

- **Decision:** When `decision-log.md` exists in a plan directory, `speq plan validate` validates it automatically. No opt-in flag.
- **Alternatives:** Add `--include-decision-log` / `--skip-decision-log` flags.
- **Rationale:** Keeps `speq plan validate` a single source of truth for plan structural integrity. The file is optional, so users who don't want validation simply don't author one. Flag proliferation is the larger evil.
- **Promotes to ADR:** no

### [4] Invalid `Promotes to ADR` value yields warning, not error

- **Decision:** When a decision entry has `Promotes to ADR: maybe` (or any value other than `yes`/`no`), emit a warning and exit 0.
- **Alternatives:** Hard error and non-zero exit.
- **Rationale:** Aligns with existing tolerance in the validator (e.g., lowercase keywords yield warnings, not errors). The decision log is a workflow artifact; surfacing the issue without blocking iteration matches the system's tone.
- **Promotes to ADR:** yes

### [5] Plan-level decision-log.md remains OPTIONAL

- **Decision:** Absence of `decision-log.md` in a plan directory MUST NOT trigger any error or warning.
- **Alternatives:** Require it and warn/error on absence; require for plans containing design ADRs.
- **Rationale:** Backward compatibility — existing plans authored before this feature shipped have no decision log. Many small plans (refactors, bug fixes) never need one. Mandating one would create busy-work without proportional benefit.
- **Promotes to ADR:** yes

## Review Findings

<!-- Populated by speq-implement after code review. -->
