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
- [12. Project hooks](#12-project-hooks)
- [13. ADR noise](#13-adr-noise)
- [14. Architecture file](#14-architecture-file)
- [15. Serena](#15-serena)
- [Remediation: decision-log migration](#remediation-decision-log-migration)
- [Remediation: domain/feature restructure](#remediation-domainfeature-restructure)
- [Remediation: ADR noise removal](#remediation-adr-noise-removal)

Reserved top-level names under `specs/` (NOT domains): `_plans/`, `_recorded/`, `_decision/`, `mission.md`, `architecture.md`, `.gitignore`.

## 1. Spec structure
**Detect:** `find specs -name spec.md`. Every result MUST match `specs/<domain>/<feature>/spec.md`: exactly two path segments between `specs/` and `spec.md`. Skip anything under `_plans/`, `_recorded/`, `_decision/`. Non-conforming: a `spec.md` at depth 1 (`specs/<x>/spec.md`) or depth 3+, a `.md` directly under a domain dir, or a feature dir with no `spec.md`.
**Signal:** `✗` listing each offending path. Remediate via [domain/feature restructure](#remediation-domainfeature-restructure).

## 2. Feature specs
**Detect:** `speq feature validate` (whole library). Exit non-zero or an error list = fail.
**Signal:** `✓ 0 errors` or `✗ N errors` (name the first few features). Not auto-fixed: report the validator's messages. The user fixes the spec prose.

## 3. Decision log
**Detect:** if `specs/decision-log.md` exists → OLD format (`✗`, offer migration). Else run `speq decision-log validate` over `specs/_decision/` (absent/empty passes). 
**Signal:** `✗ old specs/decision-log.md (N ADRs)`, `✗ validate failed: <msg>`, or `✓`. Remediate the old-format case via [decision-log migration](#remediation-decision-log-migration).

## 4. `_recorded` gitignored
**Detect:** `specs/.gitignore` exists and contains a line matching `/_recorded` or `_recorded`.
**Signal:** `✓` or `✗ missing from specs/.gitignore`.
**Remediate (inline, on Yes):** append `/_recorded` to `specs/.gitignore` (create the file if absent). Rationale: `_recorded/` is archival churn, deliberately untracked.

## 5. Mission sync
Delegated to `audit-agent` (see SKILL.md Phase 3). The agent returns (a) library domains/features unmentioned in the mission, (b) mission capabilities with no backing spec.
**Signal:** `✓ in sync`, `⚠ N features unmentioned · M capabilities unbacked`, or `✗ no mission.md`.
**Remediate (on Yes):** spawn `/speq-mission` seeded with the two lists so its interview targets those gaps. Never edit `mission.md` here.

## 6. Unrecorded plans
**Detect:** for each dir in `specs/_plans/*/`: `verification-report.md` present = implemented but NOT archived → **unrecorded** (`✗`, flag it). `plan.md` with no `verification-report.md` = un-implemented (note, not stale). `open-questions.md` present = intentionally blocked (note). `speq plan list` enumerates active plans.
**Signal:** `✗ N unrecorded: <names>` / `✓ none`.
**Remediate:** point to `/speq-record <plan>` (record owns archival; do not archive here).

## 7. Recorded-folder naming
**Detect:** each entry in `specs/_recorded/*` SHOULD match `^\d{3}-<plan>`. Legacy `YYYY-MM-DD-<plan>` or bare `<plan>` names are inconsistent. `_recorded/` is gitignored, so this is cosmetic/local.
**Signal:** `⚠ N legacy names` / `✓`.
**Remediate (inline, on Yes):** `mv` each legacy folder to the next `NNN-<plan>` (NNN = count of existing entries, zero-padded, assigned in existing chronological order). Low risk: the dir is gitignored.

## 8. Library thresholds
**Detect:** per feature, count `### Scenario:` blocks (>10 = over). Per domain, count feature dirs (>8 = over). These mirror the thresholds `speq record` escalates on.
**Signal:** `⚠ <domain>/<feature>: N scenarios` and/or `⚠ <domain>: N features`, else `✓ max X scenarios · Y features`.
**Remediate:** advisory only: recommend `/speq-plan` to split an over-threshold feature/domain. Never auto-split (creative/structural).

## 9. Reserved-dir gitignore
**Detect:** in `specs/.gitignore` (and the repo root `.gitignore`), `_decision/` and `_plans/` MUST NOT be ignored (both tracked: they hold the permanent decision record and pending work). Only `_recorded` may be ignored.
**Signal:** `✗ _plans wrongly ignored` / `✗ _decision wrongly ignored`, else `✓ _decision, _plans tracked`.
**Remediate (inline, on Yes):** remove the offending `_decision`/`_plans` line from the gitignore.

## 10. Git hygiene
**Detect:** `git status --short specs/` — any output = dirty. (If not a git repo, mark `— n/a`.)
**Signal:** `⚠ dirty (N files)` / `✓ specs/ clean`.
**Remediate:** advisory: recommend a commit or stash before structural fixes, so remediation diffs stay clean.

## 11. Active-plan validity
**Detect:** `speq plan validate <plan>` for each dir under `specs/_plans/`.
**Signal:** `✓ all valid` / `✗ <plan>: <msg>`.
**Remediate:** report the validator message; the user fixes the plan.

## 12. Project hooks
**Detect:** list any `.speq/*-hook.md` files present in the repo root.
**Signal:** informational only, no `✓`/`✗`/`⚠`: `N hooks active: <filenames>` or `none`. Never a finding, never remediated. It only surfaces that custom behavior is in effect.

## 13. ADR noise
Delegated to `adr-audit-agent` (see SKILL.md Phase 3b). The agent reads every `specs/_decision/*.md` fragment and returns one verdict per ADR: `KEEP`, `NOISE-PROCESS`, `NOISE-LOCAL`, `NOISE-COROLLARY`, `NOISE-DUPLICATE`, `STALE`, or `UNSURE`. The agent file defines the tags. `/speq-adr-rules` defines the rules they apply.
**Detect:** skip (`— n/a`) when `specs/_decision/` holds no fragment, or when the old `specs/decision-log.md` exists (migrate first).
**Signal:** `✓ N ADRs, no noise`, `⚠ M of N noise · S stale · U unsure`, or `— n/a`.
**Remediate (on Yes, `NOISE-*` only):** [ADR noise removal](#remediation-adr-noise-removal). `STALE` and `UNSURE` are report-only: the user edits the ADR or plans a change.

## 14. Architecture file
**Detect:** check `specs/architecture.md` against the rules in `/speq-plan`'s `references/architecture-template.md`. Read-only, no CLI command exists for it.
1. Missing: the file does not exist.
2. Old sections: `specs/architecture.md` is missing and `specs/mission.md` still holds `## Architecture`, `## External Dependencies`, or technical or performance constraint lines.
3. Bad structure: the first line is not `# Architecture`, a canonical `##` section (Overview, Components, Data Flow, Interfaces, Constraints, External Dependencies) is missing, duplicated, or out of order, the body has a `###` heading, a table, or a prose paragraph, or an empty canonical section lacks `- None`.
**Signal:** `✓ present, N sections`, `✗ missing`, `✗ missing, mission.md holds the old sections`, or `✗ bad structure: <first violation with line>`.
**Remediate (on Yes):** missing or old sections: spawn `/speq-mission`, which runs the one-time migration interview. Bad structure: report only. The user fixes the file or plans the change with `/speq-plan`. Never edit `specs/architecture.md` here.

## 15. Serena
**Detect:** call `ToolSearch` with the query `serena`. A match is any tool whose name contains `serena`. Read-only. Serena is optional. The skills use it when it exists and work without it, so a missing server is only a hint. Context7 is optional and is not checked.
**Signal:** `✓ serena found`, `ℹ not installed` with the Detail `skills work without it, see docs/mcp-servers.md`, or `— n/a` when the host has no `ToolSearch`, for example Codex.
**Remediate:** none. A hint is not a finding. It does not count toward `N findings` and does not appear in the numbered remediations.

---

## Remediation: decision-log migration
Replay the ADR promotion mapping (from `/speq-spec-merge`, section "Promote ADRs to Permanent Decision Log") to convert an OLD single `specs/decision-log.md` (sequential `## ADR-NNN` blocks) into NEW per-plan fragments. Because the old file predates per-plan attribution, group all ADRs into ONE migration fragment unless each block names its originating plan.

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

## Remediation: ADR noise removal
Removal loses a decision if the verdict is wrong. Show the full verdict list first. The user can strike slugs from it. Delegate the rest to a spawned worker (Yes required). For each confirmed `NOISE-*` ADR:
1. `NOISE-COROLLARY`: add the ADR's decision as one sentence to the `### Consequences` of its `Parent` ADR. Create the section when the parent is short form. Add no dates.
2. Delete the ADR block from its fragment: from its `## ADR:` heading up to the next `## ADR:` heading or the end of the file.
3. When a fragment has no ADR block left, delete the fragment file. Leave the `NNN-` prefixes of the other fragments unchanged.
4. Validate: `speq decision-log validate` MUST pass. If it fails, undo this step's edits, report the validator message, and stop. Do not guess.
5. Report each removed slug and the fold target of each corollary. Do not commit: the user reviews the diff.
