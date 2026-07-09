---
name: speq-audit
description: Audit a speq project's health — spec-library structure, feature/decision-log/plan validation, mission-to-spec sync, unrecorded plans, and gitignore hygiene — then guide fixes. Use when the user asks to audit, health-check, doctor, lint, or sanity-check the specs or repo, or after cloning or inheriting a speq project.
model: sonnet
---

# Spec Auditor (Orchestrator)

This skill is a **thin orchestrator**. It runs read-only health checks over a speq project, delegates the one reasoning-heavy check (mission ↔ spec-library sync) to the `audit-agent`, prints a BLUF summary, and offers to fix each finding — **always asking first**. Auditing is mostly mechanical (CLI validators + filesystem checks); only mission sync needs reasoning.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli` — spec discovery and the `validate` commands
- `/speq-writing-guardrails` — prose style for the summary

The `audit-agent` sub-agent invokes `/speq-cli` itself. **Read `references/checks.md`** for the per-check detection recipes, thresholds, and remediation procedures — the workflow below is the skeleton; the recipes live there.

## Workflow

### Phase 0: Load Project Hook (orchestrator)

Check for `.speq/audit-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/audit-hook.md". Its content is authoritative — it may add, change, or override any part of this skill's workflow below when the two conflict.
- **Absent:** continue normally, no mention.

Note it (not its full content) as a `Project Hook:` line in the `audit-agent` brief below.

### Phase 1: Preconditions (orchestrator)

```
Check: specs/ directory exists?
├─ Yes → proceed
└─ No  → STOP: "No spec library found. Run /speq-mission to bootstrap."
```

### Phase 2: Run checks (orchestrator, READ-ONLY)

Run every check in `references/checks.md`, recording a `✓` / `✗` / `⚠` and a one-line detail for each. **Modify nothing in this phase.** The checks:

1. Spec structure — every `spec.md` is at `specs/<domain>/<feature>/spec.md`
2. Feature specs — `speq feature validate`
3. Decision log — old `specs/decision-log.md` vs new `specs/_decision/`; `speq decision-log validate`
4. `_recorded` gitignored — `specs/.gitignore` contains `/_recorded`
5. Mission sync — delegated to `audit-agent` (Phase 3)
6. Unrecorded plans — `specs/_plans/*/` with a `verification-report.md`
7. Recorded-folder naming — `_recorded/*` follows `NNN-<plan>`
8. Library thresholds — >10 scenarios/spec, >8 features/domain
9. Reserved-dir gitignore — `_decision`/`_plans` tracked; only `_recorded` ignored
10. Git hygiene — `git status --short specs/` is clean
11. Active-plan validity — `speq plan validate <plan>` per active plan
12. Project hooks — informational only; list any `.speq/*-hook.md` present

Reuse the CLI (no new commands): `speq feature validate`, `speq decision-log validate`, `speq plan validate`, `speq plan list`, `speq domain list`, `speq feature list`.

### Phase 3: Delegate mission sync to audit-agent

```
Delegate to audit-agent — Verify mission ↔ spec library

## Context
- Mission: specs/mission.md
- Inventory: run `speq domain list` and `speq feature list`

## Your Task
Diff the mission against the real spec library. Return two lists: (a) domains/features
present in the library but NOT reflected in the mission's Core Capabilities / Domain
Glossary / Architecture; (b) mission capabilities with NO backing spec. Advisory only —
do NOT edit mission.md.

Project Hook: <if active, ".speq/audit-hook.md — read it and apply it"; otherwise omit this line>
```

If `specs/mission.md` is absent, skip the delegation and mark the check `✗ (no mission.md)`.

### Phase 4: Print the summary (orchestrator)

Lead with the verdict (BLUF), then the checks table, then numbered remediations each ending in the concrete next-step command. Tables are exempt from prose guardrails; the Summary line is terse. Use this format:

```
# speq:audit — <project>

| Result | Summary |
|--------|---------|
| **✓ healthy** _or_ **⚠ N findings** | <one-line BLUF: what's wrong, most important first> |

## Checks
| Check                                 | Status | Detail |
|---------------------------------------|--------|--------|
| Spec structure (<domain>/<feature>)   | ✓ | 4 domains · 11 features |
| Feature specs (feature validate)      | ✓ | 0 errors |
| Decision log format                   | ✗ | old specs/decision-log.md (7 ADRs) |
| _recorded gitignored                  | ✗ | missing from specs/.gitignore |
| Reserved dirs tracked                 | ✓ | _decision, _plans tracked |
| mission.md ↔ spec library             | ⚠ | 2 features unmentioned · 1 capability unbacked |
| Unrecorded plans                      | ✗ | 1: add-export-command |
| Recorded-folder naming                | ⚠ | 3 legacy names |
| Library thresholds                    | ✓ | max 8 scenarios · 5 features |
| Git hygiene                           | ✓ | specs/ clean |
| Project hooks                         | — | 1 active: plan-hook.md |

## Recommended actions  (I ask before each change)
1. Migrate specs/decision-log.md → specs/_decision/ fragments (7 ADRs → slugs)
2. Add `/_recorded` to specs/.gitignore
3. Record the finished plan → /speq-record add-export-command
4. Reconcile mission.md → /speq-mission (seeded): features `cli/export`, `cli/import` unmentioned; capability "Diff specs" unbacked
```

A clean project prints `✓ healthy` and omits the actions section.

### Phase 5: Remediate (orchestrator — each finding gated)

For each actionable finding, ask with `AskUserQuestion` (**Yes / No / Skip**). Apply per `references/checks.md`. **Never act without a Yes.** After an applied fix, re-run the affected validator and reprint its one-line result. Routing:

| Finding | Remediation |
|---------|-------------|
| `_recorded` not ignored · wrong reserved-dir ignored · legacy `_recorded/` name | Apply inline on Yes (edit `specs/.gitignore` / `mv` the folder) |
| Old `decision-log.md` · non-conforming domain/feature layout | Delegate to a spawned worker on Yes (see `references/checks.md`); re-validate |
| Unrecorded plan | Point to `/speq-record <plan>` |
| Over-threshold domain/feature | Recommend `/speq-plan` (structural — not auto-fixed) |
| Mission drift | On Yes, spawn `/speq-mission` **seeded** with the audit-agent's inconsistency lists; never edit `mission.md` directly |

### Phase 6: Close (orchestrator)

Print the final status and any remaining manual next steps.

## Work Split (reference)

| Step | Performed by | Why |
|------|--------------|-----|
| CLI validators, filesystem/structure checks, summary, remediation gates | This skill (pins Sonnet) | Mechanical + conversational |
| Mission ↔ spec-library semantic diff | `audit-agent` sub-agent | Reasoning-heavy cross-referencing |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Modifying files during Phase 2 | Audit is read-only until the user confirms |
| Applying a fix without a Yes | Every remediation is user-gated |
| Editing `mission.md` directly | `/speq-mission` owns that file |
| Auto-restructuring domains or thresholds | Reorganization is a user decision |
| Reporting "fast"/"clean" without counts | Quantify findings (N features, N scenarios) |
