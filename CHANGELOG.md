# Changelog

## 0.26.0

- Improved semantic search and performance of index building.

## 0.25.0

- `speq search query` prints a notice on stderr when it builds a missing index first.
- Agents read the accepted ADRs before they plan; ADR text stays short.
- `/speq:implement-pr` keeps rotation hand-off notes (`notes/`) out of its commits.
- Plan review treats a plan as small when its verb is `fix`, its decision log has no design decisions, and it has no architecture delta.

## 0.23.0

- ADRs are rare by default. A new gate (`/speq:adr-rules`) admits a decision only with a named criterion. Planning lists the candidates, and recording a plan accepts them.
- `/speq:audit` removes existing ADRs that fail the gate, after you confirm.
- Skills keep the current architecture in `specs/architecture.md`. Plans change it through an architecture delta, and `/speq:record` merges it.
- `/speq:mission` drafts `specs/architecture.md` from the code in existing projects and writes it to a draft file. You review the draft and confirm before mission writes `specs/architecture.md`.

## 0.22.0

- Stricter ADR promotion: workflow decisions default to `no`, minor decisions record as short ADRs. Plan review flags implementation detail in plans and ADRs that should not exist.
- `/speq:audit` checks for noise ADRs and removes them after you confirm.

## 0.21.0

- `plan-reviewer` tags every BLOCKER `HUMAN` or `MECHANICAL`. Only `HUMAN` findings reach you after round 2. The verdict line gains a `HUMAN:` count, which breaks anything that parses it.
- Round 2 of plan review runs only when round 1 raised a `HUMAN` blocker.
- PR bodies follow one template (Current State, What Changes, Impact, details). A PR comment appears only for an unresolved `HUMAN` finding. `/speq:implement-pr` posts a structured verification summary.

## 0.20.0

- `DELTA:CHANGED` can target a feature's `## Background` or `# Feature:` description. `speq plan validate` and `speq record` reject invalid uses. `speq record` writes `notes/prose-realignment.md` when it changes text.

## 0.19.0

- `speq-plan-pr` and `speq-implement-pr` start faster.
- Round 2 of plan review shrinks to a blocker recheck for small plans.
- Planning reuses the orchestrator's context and hands off through `notes/planning.md`.

## 0.16.0

- Reviewers write full findings to `review/round-<N>.md` and `review-findings.md` and return a one-line verdict, which gains an `INTENT` count.
- `speq-implement` stops when `open-questions.md` is not empty, like `speq-implement-pr`.
- `speq-implement-pr` commits evidence artifacts before recording, so they reach git history.

## 0.15.0

- New `speq-design-philosophy` skill. Stricter `speq-code-guardrails` and `speq-code-review` (error handling, design depth, test quality). 
- Plan review gains a Design Depth axis.

## 0.14.0

- `plan.md` gains a required `## Impact` section. `/speq:plan-pr` puts it in the draft PR body. Advisory findings and design decisions post as a PR comment. 
- `/speq:implement-pr` posts a verification summary comment.

## 0.13.0

- Project hooks: `.speq/<name>-hook.md` files override any step of the entry-point skills. `/speq:audit` lists active hooks. See [docs/hooks.md](./docs/hooks.md).

## 0.12.0

- New `plan-reviewer` reviews plans adversarially before implementation. Blockers send the planner back for up to 2 rounds, then ask you (interactive) or list `OPEN QUESTIONS:` (headless). Advisory findings appear in the plan-ready report.

## 0.11.0

- New `/speq:audit`: a read-only health check of spec structure, validators, the decision log, mission sync, unrecorded plans, gitignore hygiene, and library thresholds. It checks `mission.md` against the specs and offers fixes after you confirm.

## 0.10.0

- ADRs move from one `specs/decision-log.md` to one fragment per plan, `specs/_decision/NNN-<plan-name>.md`, each with a kebab-case `**ID:**`. The old log was migrated.
- New `speq decision-log show`. `speq decision-log validate` checks the whole directory.
- Plans archive to `specs/_recorded/NNN-<plan-name>/`. ADRs drop the `**Date:**` field.

## 0.9.0

- New `speq:writing-guardrails` for speq artifacts and PR, issue, and comment text. Every prose-writing component loads it.

## 0.8.2

- `speq-plan-pr` always leaves the PR as a draft. Only `speq-implement-pr` marks it ready.

## 0.8.1

- PR titles follow `<type>(<scope>): <slug>`. The PR stays a draft until the implementation is pushed.

## 0.8.0

- Code review flags over-engineering (YAGNI), and the code guardrails gain a dependency rule.

## 0.7.0

- New `speq-plan-pr` and `speq-implement-pr`: headless planning and implementation on a `feat/<plan-name>` branch with a PR.
- Headless planning lists `OPEN QUESTIONS:` when a decision needs a human.

## 0.4.0

- Codex support: the installer generates a Codex plugin next to the Claude Code one, registers a Codex marketplace, and installs skills into `$CODEX_HOME/skills`. Skills stay `/speq:*` on both. Serena and Context7 are declared for Codex.

## 0.3.1

- New `speq decision-log validate` for the ADR format. `speq plan validate` checks an optional `decision-log.md`.
- Planning writes a plan decision log, and recording promotes curated entries to the permanent log.

## 0.3.0

- Planning and recording run in dedicated sub-agents. Tasks tagged `[expert]` go to a new expert implementer.

## 0.2.9

- Record rejects mismatched or unclosed delta markers.
- Local cache fallback when the system cache is not writable. `SPEQ_CACHE_DIR` overrides the cache location.

## 0.2.4

- New `plan list` command.

## 0.2.2

- Initial release.
