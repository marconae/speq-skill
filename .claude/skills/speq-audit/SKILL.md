---
name: speq-audit
description: Audit a speq project's health — spec-library structure, feature/decision-log/plan validation, ADR noise, mission-to-spec sync, unrecorded plans, and gitignore hygiene — then guide fixes. Use when the user asks to audit, health-check, doctor, lint, or sanity-check the specs or repo, or after cloning or inheriting a speq project.
model: sonnet
---

# Spec Auditor (Orchestrator)

Thin orchestrator. It runs read-only health checks over a speq project, delegates two judgment checks (mission ↔ spec-library sync to `audit-agent`, ADR noise and accuracy to `adr-audit-agent`), prints a BLUF summary, and offers to fix each finding. **Always ask before a fix.**

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli` — spec discovery and the `validate` commands
- `/speq-writing-guardrails` — prose style for the summary

The `audit-agent` and `adr-audit-agent` sub-agents invoke `/speq-cli` themselves. **Read `references/checks.md`** for the per-check detection recipes, thresholds, and remediation procedures.

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
13. ADR noise — delegated to `adr-audit-agent` (Phase 3)
14. Architecture file — `specs/architecture.md` present, structurally valid, and mission.md free of the old architecture sections
15. Serena — `ToolSearch` finds Serena tools; hint only, Serena is optional

Reuse the CLI (no new commands): `speq feature validate`, `speq decision-log validate`, `speq plan validate`, `speq plan list`, `speq domain list`, `speq feature list`.

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

#### 3b. ADR review to adr-audit-agent

```
Delegate to adr-audit-agent — Apply the ADR gate from /speq-adr-rules to every existing ADR

## Context
- Decision records: specs/_decision/*.md (read every fragment in full)

## Your Task
Read every ADR. Return one verdict per ADR (KEEP, NOISE-*, STALE, UNSURE) with its
evidence, per your workflow. Advisory only — do NOT edit or delete any file.

Project Hook: <if active, ".speq/audit-hook.md — read it and apply it"; otherwise omit this line>
```

Skip the delegation and mark the check `— n/a` when `specs/_decision/` holds no fragment. When the old `specs/decision-log.md` still exists, skip it too and note "migrate first": the migration puts every ADR in one fragment, and the review runs on the migrated result.

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
| ADR noise (adr-audit-agent)           | ⚠ | 2 of 12 noise · 1 stale · 1 unsure |
| _recorded gitignored                  | ✗ | missing from specs/.gitignore |
| Reserved dirs tracked                 | ✓ | _decision, _plans tracked |
| mission.md ↔ spec library             | ⚠ | 2 features unmentioned · 1 capability unbacked |
| Architecture file                     | ✗ | specs/architecture.md missing |
| Unrecorded plans                      | ✗ | 1: add-export-command |
| Recorded-folder naming                | ⚠ | 3 legacy names |
| Library thresholds                    | ✓ | max 8 scenarios · 5 features |
| Git hygiene                           | ✓ | specs/ clean |
| Serena (code tools, optional)         | ℹ | not installed, skills work without it, see docs/mcp-servers.md |
| Project hooks                         | — | 1 active: plan-hook.md |

## ADR review  (only when the agent flags a noise, stale, or unsure ADR)
| Slug | Verdict | Reason | Parent / Evidence |
|------|---------|--------|-------------------|
| pin-plan-scratch-dir | NOISE-PROCESS | where one plan keeps scratch files | specs/cli/plan/spec.md:40 |
| retry-flag-naming | NOISE-COROLLARY | follows from add-retry | add-retry |
| use-line-scanner | STALE | scanner replaced by regex | src/scan.rs:41 |
| cap-index-size | UNSURE | cap may be a hard limit | - |

## Recommended actions  (I ask before each change)
1. Migrate specs/decision-log.md → specs/_decision/ fragments (7 ADRs → slugs)
2. Add `/_recorded` to specs/.gitignore
3. Record the finished plan → /speq-record add-export-command
4. Reconcile mission.md → /speq-mission (seeded): features `cli/export`, `cli/import` unmentioned; capability "Diff specs" unbacked
5. Remove 2 noise ADRs (fold `retry-flag-naming` into `add-retry` first)
6. Create specs/architecture.md → /speq-mission (migration interview seeded from the old mission.md sections)
```

A clean project prints `✓ healthy` and omits the ADR review and actions sections. Print the ADR review table with every non-`KEEP` verdict, in the agent's own words. Do not soften or re-judge it.

### Phase 5: Remediate (orchestrator — each finding gated)

For each actionable finding, ask with `AskUserQuestion` (**Yes / No / Skip**). Apply per `references/checks.md`. **Never act without a Yes.** After an applied fix, re-run the affected validator and reprint its one-line result. Routing:

| Finding | Remediation |
|---------|-------------|
| `_recorded` not ignored · wrong reserved-dir ignored · legacy `_recorded/` name | Apply inline on Yes (edit `specs/.gitignore` / `mv` the folder) |
| Old `decision-log.md` · non-conforming domain/feature layout | Delegate to a spawned worker on Yes (see `references/checks.md`); re-validate |
| Unrecorded plan | Point to `/speq-record <plan>` |
| Over-threshold domain/feature | Recommend `/speq-plan` (structural — not auto-fixed) |
| ADR noise (`NOISE-*`) | Show the list first; the user can strike slugs. On Yes, spawn a worker to remove the rest per `references/checks.md` (fold each corollary into its parent first), then run `speq decision-log validate` |
| Stale or unsure ADR | Report only. The user edits the ADR, or runs `/speq-plan` when the decision changed |
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
| ADR read, noise and accuracy verdicts | `adr-audit-agent` sub-agent (pins Fable) | Judgment-heavy. A wrong removal loses a real decision |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Modifying files during Phase 2 | Audit is read-only until the user confirms |
| Applying a fix without a Yes | Every remediation is user-gated |
| Editing `mission.md` directly | `/speq-mission` owns that file |
| Judging ADR noise in the orchestrator | Only `adr-audit-agent` verdicts count. The orchestrator relays them |
| Removing an ADR that another ADR references | It breaks `speq decision-log validate`. The agent keeps chain members |
| Auto-restructuring domains or thresholds | Reorganization is a user decision |
| Reporting "fast"/"clean" without counts | Quantify findings (N features, N scenarios) |
