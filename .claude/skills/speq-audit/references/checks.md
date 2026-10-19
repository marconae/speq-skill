# Audit Checks

Reserved names under `specs/` (not domains): `_plans/`, `_recorded/`, `_decision/`, `mission.md`, `architecture.md`, `.gitignore`.

## 1. Spec structure
**Detect:** `find specs -name spec.md`, skipping reserved dirs. Each result must sit at `specs/<domain>/<feature>/spec.md`. Offenders: a `spec.md` at another depth, a `.md` directly under a domain dir, a feature dir with no `spec.md`.
**Signal:** `✓` or `✗` listing each offending path.
**Remediate:** [domain/feature restructure](#remediation-domainfeature-restructure).

## 2. Feature specs
**Detect:** `speq feature validate` (call `/speq-cli`).
**Signal:** `✓ 0 errors` or `✗ N errors` naming the first few features.
**Remediate:** none. Report the validator messages. The user fixes the spec.

## 3. Library thresholds
**Detect:** compare each feature and domain against the thresholds in `/speq-spec-merge` ("Check Library Thresholds"). Count `### Scenario:` blocks per feature and feature dirs per domain.
**Signal:** `⚠ <domain>/<feature>: N scenarios`, `⚠ <domain>: N features`, or `✓ max X scenarios · Y features`.
**Remediate:** advisory. Recommend `/speq-plan` to split. Never auto-split.

## 4. Decision log
**Detect:** `specs/decision-log.md` exists = old format. Otherwise `speq decision-log validate` (absent or empty passes).
**Signal:** `✗ old specs/decision-log.md (N ADRs)`, `✗ validate failed: <msg>`, or `✓`.
**Remediate (old format, on Yes):** [decision-log migration](#remediation-decision-log-migration).

## 5. ADR review
Run by the orchestrator (SKILL.md Phase 3b). Call `/speq-adr-rules` first.

**Detect:** skip (`— n/a`) when `specs/_decision/` holds no fragment, or when the old `specs/decision-log.md` exists (migrate first). Otherwise read every fragment in full and judge each ADR by the rules in `/speq-adr-rules`.
- **Noise:** fails the rules. Remove it. Fold a corollary into its parent's `### Consequences`.
- **Wordy:** a real decision with more text than rule 7a allows. Trim it.
- **Keep** everything else. When in doubt, keep. Never remove an ADR named in another's `Supersedes` or `Superseded by`.

**Signal:** `✓ N ADRs, clean` or `⚠ M noise · T wordy of N ADRs`, each listed with a one-line reason and `file:line` evidence.
**Remediate (on Yes):** [ADR cleanup](#remediation-adr-cleanup).

## 6. Active-plan validity
**Detect:** `speq plan validate <plan>` for each plan in `speq plan list`.
**Signal:** `✓ all valid` or `✗ <plan>: <msg>`.
**Remediate:** none. The user fixes the plan.

## 7. Unrecorded plans
**Detect:** a `specs/_plans/*/` dir with `verification-report.md` is implemented but not archived. `plan.md` alone is un-implemented. `open-questions.md` means blocked on purpose. Note both, flag neither.
**Signal:** `✗ N unrecorded: <names>` or `✓ none`.
**Remediate:** point to `/speq-record <plan>`. Never archive here.

## 8. Recorded-folder naming
**Detect:** each `specs/_recorded/*` entry should match `^\d{3}-<plan>`. Legacy `YYYY-MM-DD-<plan>` or bare names are inconsistent. The dir is gitignored, so this is cosmetic.
**Signal:** `⚠ N legacy names` or `✓`.
**Remediate (inline, on Yes):** `mv` each legacy folder to the next `NNN-<plan>` (NNN = count of entries plus one, zero-padded, in existing chronological order).

## 9. `_recorded` ignored
**Detect:** `specs/.gitignore` has a `/_recorded` or `_recorded` line.
**Signal:** `✓` or `✗ missing from specs/.gitignore`.
**Remediate (inline, on Yes):** append `/_recorded` (create the file if absent).

## 10. Reserved dirs tracked
**Detect:** neither `specs/.gitignore` nor the root `.gitignore` ignores `_decision/` or `_plans/`. Only `_recorded` may be ignored.
**Signal:** `✗ _plans wrongly ignored`, `✗ _decision wrongly ignored`, or `✓ _decision, _plans tracked`.
**Remediate (inline, on Yes):** remove the offending line.

## 11. Mission sync
Delegated to `audit-agent` (SKILL.md Phase 3a). It returns (a) library domains and features the mission does not mention, (b) mission capabilities with no backing spec.
**Signal:** `✓ in sync`, `⚠ N features unmentioned · M capabilities unbacked`, or `✗ no mission.md`.
**Remediate (on Yes):** spawn `/speq-mission` seeded with the two lists. Never edit `mission.md`.

## 12. Architecture file
**Detect:** read-only, no CLI command exists. Check `specs/architecture.md` against the Rules in `/speq-plan`'s `references/architecture-template.md`. Also flag the old layout: the file is missing and `specs/mission.md` still holds `## Architecture`, `## External Dependencies`, or technical or performance constraint lines.
**Signal:** `✓ present, N sections`, `✗ missing`, `✗ missing, mission.md holds the old sections`, or `✗ bad structure: <first violation with line>`.
**Remediate (on Yes):** missing or old layout: spawn `/speq-mission` for its migration interview. Bad structure: report only. The user fixes the file or plans the change with `/speq-plan`. Never write `specs/architecture.md` here.

## 13. Project hooks
**Detect:** list `.speq/*-hook.md` files in the repo root.
**Signal:** informational, no `✓`/`✗`/`⚠`: `N hooks active: <filenames>` or `none`. Never a finding.

## 14. Serena
**Detect:** `ToolSearch` with the query `serena`. Any tool name containing `serena` is a match. Serena is optional and Context7 is not checked.
**Signal:** `✓ serena found`, `ℹ not installed` (Detail: `skills work without it, see docs/mcp-servers.md`), or `— n/a` when the host has no `ToolSearch`, for example Codex.
**Remediate:** none. A hint is not a finding and does not count toward `N findings`.

---

## Remediation: decision-log migration
Delegate to a spawned worker (Yes required). Convert the old single `specs/decision-log.md` (`## ADR-NNN` blocks) to per-plan fragments with the field mapping in `/speq-spec-merge` ("Promote ADRs to Permanent Decision Log"). The old file has no per-plan attribution, so put all ADRs in one fragment `specs/_decision/NNN-<name>.md` (NNN = existing fragment count plus one, zero-padded, H1 `# Decisions: <name>`) unless a block names its plan. Convert each `ADR-NNN` reference to the target's new slug. Drop all dates. Delete `specs/decision-log.md`, then run `speq decision-log validate`. It MUST pass.

## Remediation: domain/feature restructure
Always ask. Never auto-run.
1. Build the move list: for each offending `spec.md`, propose `specs/<domain>/<feature>/spec.md`, inferring domain and feature from the path or the `# Feature:` heading.
2. Show the full `mv` list and confirm.
3. On Yes, spawn a worker to `mkdir -p` and `mv` each `spec.md` with its sibling files.
4. Run `speq feature validate`. It MUST pass. On failure, stop and report.

## Remediation: ADR cleanup
Show the list first. The user can strike slugs. Delegate the rest to a spawned worker (Yes required).
1. Noise: delete the ADR block (its `## ADR:` heading up to the next one or the file end). Delete a fragment left empty. Leave other `NNN-` prefixes unchanged.
2. Corollary: add its decision as one sentence to the parent's `### Consequences` first.
3. Wordy: rewrite to the shape in `/speq-adr-rules` rules 6 to 7a. Keep the heading, `**ID:**`, `**Plan:**`, `**Status:**`, and supersede links.
4. Run `speq decision-log validate`. It MUST pass. On failure, undo and report.
5. Do not commit. The user reviews the diff.
