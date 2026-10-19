---
name: speq-audit
description: "Audits a speq project's health: specs, decision logs, plans, mission sync, unrecorded plans, and gitignore hygiene. Use when asked to audit, health-check, or check a project build with speq-skill."
model: sonnet
---

# Spec Auditor (Orchestrator)

Thin orchestrator. It runs read-only health checks over a speq project, delegates the mission ↔ spec-library sync check to `audit-agent`, judges the ADRs itself, prints a summary, and offers to fix each finding. **Always ask before a fix.**

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli` — spec discovery and the `validate` commands
- `/speq-writing-guardrails` — prose style for the summary

Each check in `references/checks.md` names the skill that owns its rules. Invoke that skill instead of working from memory.

The `audit-agent` sub-agent invokes `/speq-cli` itself. **Read `references/checks.md`** for the per-check detection recipes, thresholds, and remediation procedures.

## Workflow

### Phase 0: Load Project Hook (orchestrator)

Check for `.speq/audit-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/audit-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue normally, no mention.

Note it (not its full content) as a `Project Hook:` line in each sub-agent brief below.

### Phase 1: Preconditions (orchestrator)

```
Check: specs/ directory exists?
├─ Yes → proceed
└─ No  → STOP: "No spec library found. Run /speq-mission to bootstrap."
```

### Phase 2: Run checks (orchestrator, READ-ONLY)

Run every check in `references/checks.md`, recording a `✓` / `✗` / `⚠` and a one-line detail for each. The Serena check records `ℹ` (hint) instead of `⚠`. A hint is not a finding, so it never counts toward `N findings` and never gets a remediation. **Modify nothing in this phase.** The checks:

Layout
1. Spec structure — every `spec.md` is at `specs/<domain>/<feature>/spec.md`
2. Feature specs — `speq feature validate`
3. Library thresholds — scenarios per spec, features per domain

Decisions
4. Decision log — old `specs/decision-log.md` vs `specs/_decision/`; `speq decision-log validate`
5. ADR review — judged by the orchestrator (Phase 3b)

Plans
6. Active-plan validity — `speq plan validate <plan>` per active plan
7. Unrecorded plans — `specs/_plans/*/` with a `verification-report.md`
8. Recorded-folder naming — `_recorded/*` follows `NNN-<plan>`

Git ignores
9. `_recorded` ignored — `specs/.gitignore` contains `/_recorded`
10. Reserved dirs tracked — `_decision` and `_plans` are not ignored

Mission and architecture
11. Mission sync — delegated to `audit-agent` (Phase 3a)
12. Architecture file — `specs/architecture.md` present and valid, mission.md free of the old sections

Info
13. Project hooks — informational only
14. Serena — hint only, Serena is optional

### Phase 3: Delegate the judgment checks

Send both delegations in one message so they run concurrently.

#### 3a. Mission sync to audit-agent

```
Delegate to audit-agent — Verify mission ↔ spec library

## Context
- Mission: specs/mission.md
- Inventory: run `speq domain list` and `speq feature list`

## Your Task
Diff the mission against the real spec library. Return two lists: (a) domains/features
present in the library but NOT reflected in the mission's Core Capabilities / Domain
Glossary; (b) mission capabilities with NO backing spec. Advisory only —
do NOT edit mission.md.

Project Hook: <if active, ".speq/audit-hook.md — read it and apply it"; otherwise omit this line>
```

If `specs/mission.md` is absent, skip the delegation and mark the check `✗ (no mission.md)`.

#### 3b. ADR review (orchestrator)

Invoke `/speq-adr-rules`, then follow check 5 in `references/checks.md` to judge every ADR. Advisory only: edit nothing until the user says Yes.

Skip the review and mark the check `— n/a` when `specs/_decision/` holds no fragment. When the old `specs/decision-log.md` still exists, skip it too and note "migrate first": the migration puts every ADR in one fragment, and the review runs on the migrated result.

### Phase 4: Print the summary (orchestrator)

Lead with the verdict (BLUF), then the checks table, then numbered remediations. End each remediation with the concrete next-step command. Tables are exempt from prose guardrails. Keep the Summary line terse. Use this format:

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
| ADR review                            | ⚠ | 2 noise · 1 wordy of 12 ADRs |
| _recorded gitignored                  | ✗ | missing from specs/.gitignore |
| Reserved dirs tracked                 | ✓ | _decision, _plans tracked |
| mission.md ↔ spec library             | ⚠ | 2 features unmentioned · 1 capability unbacked |
| Architecture file                     | ✗ | specs/architecture.md missing |
| Unrecorded plans                      | ✗ | 1: add-export-command |
| Recorded-folder naming                | ⚠ | 3 legacy names |
| Library thresholds                    | ✓ | max 8 scenarios · 5 features |
| Serena (code tools, optional)         | ℹ | not installed, skills work without it, see docs/mcp-servers.md |
| Project hooks                         | — | 1 active: plan-hook.md |

## Recommended actions  (I ask before each change)
1. Migrate specs/decision-log.md → specs/_decision/ fragments (7 ADRs → slugs)
2. Add `/_recorded` to specs/.gitignore
3. Record the finished plan → /speq-record add-export-command
4. Reconcile mission.md → /speq-mission (seeded): features `cli/export`, `cli/import` unmentioned; capability "Diff specs" unbacked
5. Clean up 3 ADRs: remove 2 noise, trim 1 wordy
6. Create specs/architecture.md → /speq-mission (migration interview seeded from the old mission.md sections)
```

A clean project prints `✓ healthy` and omits the actions section. List each noisy or wordy ADR under its action.

### Phase 5: Remediate (orchestrator — each finding gated)

For each actionable finding, ask the user (**Yes / No / Skip**). Apply per `references/checks.md`. **Never act without a Yes.** After an applied fix, re-run the affected validator and reprint its one-line result. Routing:

| Finding | Remediation |
|---------|-------------|
| `_recorded` not ignored · wrong reserved-dir ignored · legacy `_recorded/` name | Apply inline on Yes (edit `specs/.gitignore` / `mv` the folder) |
| Old `decision-log.md` · non-conforming domain/feature layout | Delegate to a spawned worker on Yes (see `references/checks.md`); re-validate |
| Unrecorded plan | Point to `/speq-record <plan>` |
| Over-threshold domain/feature | Recommend `/speq-plan` (structural — not auto-fixed) |
| ADR noise or wordy ADR | Show the list first; the user can strike slugs. On Yes, spawn a worker for the ADR cleanup in `references/checks.md` |
| Missing architecture file · mission.md still holds the old architecture sections | On Yes, spawn `/speq-mission`. It runs the one-time migration interview. Never write `specs/architecture.md` directly |
| Architecture file with bad structure | Report the violations. The user fixes the file, or plans the change with `/speq-plan` |
| Mission drift | On Yes, spawn `/speq-mission` **seeded** with the audit-agent's inconsistency lists; never edit `mission.md` directly |

### Phase 6: Close (orchestrator)

Print the final status and any remaining manual next steps.

## Work Split (reference)

| Step | Performed by | Why |
|------|--------------|-----|
| CLI validators, filesystem/structure checks, summary, remediation gates | This skill (pins Sonnet) | Mechanical + conversational |
| Mission ↔ spec-library semantic diff | `audit-agent` sub-agent | Reasoning-heavy cross-referencing |
| ADR review | This skill, by the `/speq-adr-rules` rules | Rules are explicit. A wrong removal loses a real decision, so bias to keep |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Editing `mission.md` directly | `/speq-mission` owns that file |
| Judging ADR noise from titles, age, or a sample | Read every fragment in full. Every ADR gets judged with evidence |
| Removing an ADR that another ADR references | It breaks `speq decision-log validate`. Keep chain members |
| Reporting "fast"/"clean" without counts | Quantify findings (N features, N scenarios) |
