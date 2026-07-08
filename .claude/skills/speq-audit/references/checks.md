# Audit Checks — Detection and Remediation

Detection recipes, thresholds, and remediation procedures for each `speq-audit` check. All paths are relative to the project root. Run every command with the local repo build convention (`speq …`).

## Contents
- [1. Spec structure](#1-spec-structure)
- [2. Feature specs](#2-feature-specs)
- [3. Decision log](#3-decision-log)
- [4. `_recorded` gitignored](#4-_recorded-gitignored)
- [5. Mission sync](#5-mission-sync)
- [6. Unrecorded plans](#6-unrecorded-plans)
- [7. Recorded-folder naming](#7-recorded-folder-naming)
- [8. Library thresholds](#8-library-thresholds)
- [9. Reserved-dir gitignore](#9-reserved-dir-gitignore)
- [10. Git hygiene](#10-git-hygiene)
- [11. Active-plan validity](#11-active-plan-validity)
- [Remediation: decision-log migration](#remediation-decision-log-migration)
- [Remediation: domain/feature restructure](#remediation-domainfeature-restructure)

Reserved top-level names under `specs/` (NOT domains): `_plans/`, `_recorded/`, `_decision/`, `mission.md`, `.gitignore`.

## 1. Spec structure
**Detect:** `find specs -name spec.md`. Every result MUST match `specs/<domain>/<feature>/spec.md` — exactly two path segments between `specs/` and `spec.md`. Skip anything under `_plans/`, `_recorded/`, `_decision/`. A `spec.md` at depth 1 (`specs/<x>/spec.md`) or depth 3+ is non-conforming; so is a `.md` directly under a domain dir, or a feature dir with no `spec.md`.
**Signal:** `✗` listing each offending path. Remediate via [domain/feature restructure](#remediation-domainfeature-restructure).

## 2. Feature specs
**Detect:** `speq feature validate` (whole library). Exit non-zero or an error list = fail.
**Signal:** `✓ 0 errors` or `✗ N errors` (name the first few features). Not auto-fixed — report the validator's messages; the user fixes the spec prose.

## 3. Decision log
**Detect:** if `specs/decision-log.md` exists → OLD format (`✗`, offer migration). Else run `speq decision-log validate` over `specs/_decision/` (absent/empty passes). 
**Signal:** `✗ old specs/decision-log.md (N ADRs)`, `✗ validate failed: <msg>`, or `✓`. Remediate the old-format case via [decision-log migration](#remediation-decision-log-migration).

## 4. `_recorded` gitignored
**Detect:** `specs/.gitignore` exists and contains a line matching `/_recorded` or `_recorded`.
**Signal:** `✓` or `✗ missing from specs/.gitignore`.
**Remediate (inline, on Yes):** append `/_recorded` to `specs/.gitignore` (create the file if absent). Rationale: `_recorded/` is archival churn — deliberately untracked.

## 5. Mission sync
Delegated to `audit-agent` (see SKILL.md Phase 3). The agent returns (a) library domains/features unmentioned in the mission, (b) mission capabilities with no backing spec.
**Signal:** `✓ in sync`, `⚠ N features unmentioned · M capabilities unbacked`, or `✗ no mission.md`.
**Remediate (on Yes):** spawn `/speq-mission` seeded with the two lists so its interview targets those gaps. Never edit `mission.md` here.

## 6. Unrecorded plans
**Detect:** for each dir in `specs/_plans/*/`: a `verification-report.md` present = implementation done but NOT archived → **unrecorded** (`✗`, flag it). A `plan.md` with no `verification-report.md` = un-implemented (note, not stale). An `open-questions.md` = intentionally blocked (note). `speq plan list` enumerates active plans.
**Signal:** `✗ N unrecorded: <names>` / `✓ none`.
**Remediate:** point to `/speq-record <plan>` (record owns archival; do not archive here).

## 7. Recorded-folder naming
**Detect:** each entry in `specs/_recorded/*` SHOULD match `^\d{3}-<plan>`. Legacy `YYYY-MM-DD-<plan>` or bare `<plan>` names are inconsistent. `_recorded/` is gitignored, so this is cosmetic/local.
**Signal:** `⚠ N legacy names` / `✓`.
**Remediate (inline, on Yes):** `mv` each legacy folder to the next `NNN-<plan>` (NNN = count of existing entries, zero-padded, assigned in existing chronological order). Low risk — the dir is gitignored.

## 8. Library thresholds
**Detect:** per feature, count `### Scenario:` blocks (>10 = over). Per domain, count feature dirs (>8 = over). These mirror the thresholds `speq record` escalates on.
**Signal:** `⚠ <domain>/<feature>: N scenarios` and/or `⚠ <domain>: N features`, else `✓ max X scenarios · Y features`.
**Remediate:** advisory only — recommend `/speq-plan` to split a fat feature/domain. Never auto-split (creative/structural).

## 9. Reserved-dir gitignore
**Detect:** in `specs/.gitignore` (and the repo root `.gitignore`), `_decision/` and `_plans/` MUST NOT be ignored (they hold the permanent decision record and pending work — both tracked). Only `_recorded` may be ignored.
**Signal:** `✗ _plans wrongly ignored` / `✗ _decision wrongly ignored`, else `✓ _decision, _plans tracked`.
**Remediate (inline, on Yes):** remove the offending `_decision`/`_plans` line from the gitignore.

## 10. Git hygiene
**Detect:** `git status --short specs/` — any output = dirty. (If not a git repo, mark `— n/a`.)
**Signal:** `⚠ dirty (N files)` / `✓ specs/ clean`.
**Remediate:** advisory — recommend committing or stashing before applying structural fixes, so remediation diffs stay clean.

## 11. Active-plan validity
**Detect:** `speq plan validate <plan>` for each dir under `specs/_plans/`.
**Signal:** `✓ all valid` / `✗ <plan>: <msg>`.
**Remediate:** report the validator message; the user fixes the plan.

---

## Remediation: decision-log migration
Replay the recorder's promotion mapping (from `recorder-agent.md`) to convert an OLD single `specs/decision-log.md` (sequential `## ADR-NNN` blocks) into NEW per-plan fragments. Because the old file predates per-plan attribution, group all ADRs into ONE migration fragment unless each block names its originating plan.

Delegate to a spawned worker (Yes required):
1. Parse each `## ADR-NNN: <Title>` block.
2. For each: **Title** = heading; **ID** = kebab-case slug of the title, UNIQUE across all fragments; **Plan** = the block's plan (or the repo/migration name); **Status** = `Accepted` (or `Deprecated` / `Superseded by <slug>` if the old block said so); **Supersedes** = if the block referenced `ADR-NNN`, convert to that ADR's new slug (one-way forward pointer). Map Context/Decision/Options Considered/Consequences straight across. **Drop all dates.**
3. Write `specs/_decision/NNN-<name>.md` — NNN = (count of existing `specs/_decision/` files) + 1, zero-padded; H1 `# Decisions: <name>`; one `## ADR: <Title>` per block in original order.
4. Delete `specs/decision-log.md`.
5. Validate: `speq decision-log validate` (MUST pass — slugs unique, every `Supersedes`/`Superseded by` resolves).

## Remediation: domain/feature restructure
Highest-risk fix. Never auto-run.
1. Build the concrete move list: for each non-conforming `spec.md`, propose its target `specs/<domain>/<feature>/spec.md` path (infer domain/feature from the current path or the `# Feature:` heading).
2. Show the full `mv` list and confirm with `AskUserQuestion`.
3. On Yes, spawn a worker to `mkdir -p` the targets and `mv` each `spec.md` (and sibling files) into place.
4. Validate: `speq feature validate` — MUST pass. If it fails, stop and report; do not guess.
