# Changelog

## 0.23.0

- ADRs are rare by default. A new gate (`/speq:adr-rules`) admits a decision only with a named criterion. Planning lists the candidates, and recording a plan accepts them.
- `/speq:audit` removes existing ADRs that fail the gate, after you confirm.
- The "project-wide process convention" ADR override is removed.

## 0.22.0

- Serena starts through a `serena` command that the installer sets up with `uv tool install` (optional, skipped when `uv` is missing).
- Stricter ADR promotion: workflow decisions default to `no`, minor decisions record as short ADRs. `plan-reviewer` flags `[IMPLEMENTATION_LEAKAGE]` and `[ADR_OVERPROMOTION]`.
- `/speq:audit` checks ADR noise through the new `adr-audit-agent` and removes noise ADRs after you confirm.
- `speq-writing-guardrails` is cut to nine rules.

## 0.21.0

- `plan-reviewer` tags every BLOCKER `HUMAN` or `MECHANICAL`. Only `HUMAN` findings reach you after round 2. The verdict line gains a `HUMAN:` count, which breaks anything that parses it.
- Round 2 of plan review runs only when round 1 raised a `HUMAN` blocker.
- PR bodies follow one template (Current State, What Changes, Impact, details). A PR comment appears only for an unresolved `HUMAN` finding. `/speq:implement-pr` posts a structured verification summary.

## 0.20.0

- `DELTA:CHANGED` can target a feature's `## Background` or `# Feature:` description. `speq plan validate` and `speq record` reject invalid uses. `speq record` writes `notes/prose-realignment.md` when it changes text.

## 0.19.0

- `speq-plan-pr` and `speq-implement-pr` run git and `gh` operations directly. `git-agent` is retired, so pipelines start faster.
- Round 2 of plan review shrinks to a blocker recheck for small plans.
- `planner-agent` reuses the orchestrator's context, self-checks before review, and hands off through `notes/planning.md`.

## 0.18.0

- Shorter agent and skill prose.

## 0.16.0

- Reviewers write full findings to `review/round-<N>.md` and `review-findings.md` and return a one-line verdict, which gains an `INTENT` count.
- `speq-implement` stops when `open-questions.md` is not empty, like `speq-implement-pr`.
- Fix: `speq-implement-pr` now commits evidence artifacts before recording, so they reach git history.
- Fix: `build.sh` no longer rewrites file-path citations into nonexistent paths.

## 0.15.0

- New `speq-design-philosophy` skill. Stricter `speq-code-guardrails` and `speq-code-review` (error handling, design depth, test quality). Plan review gains a Design Depth axis.

## 0.14.0

- `plan.md` gains a required `## Impact` section. `/speq:plan-pr` puts it in the draft PR body. Advisory findings and design decisions post as a PR comment. `/speq:implement-pr` posts a verification summary comment.

## 0.13.3

- Fix: the installer provisions the embedding model into the macOS cache directory, so `speq search query` finds it.

## 0.13.2

- Fix: skill names no longer double the namespace (`speq:speq:<skill>`).

## 0.13.1

- Search indexing is 38% faster on 1,000 scenarios, with unchanged ranking.
- Clearer skill and agent descriptions for more reliable auto-invocation.

## 0.13.0

- Project hooks: `.speq/<name>-hook.md` files override any step of the entry-point skills. `/speq:audit` lists active hooks. See [docs/hooks.md](./docs/hooks.md).

## 0.12.0

- New `plan-reviewer` reviews plans adversarially before implementation. Blockers send the planner back for up to 2 rounds, then ask you (interactive) or list `OPEN QUESTIONS:` (headless). Advisory findings appear in the plan-ready report.

## 0.11.0

- New `/speq:audit`: a read-only health check of spec structure, validators, the decision log, mission sync, unrecorded plans, gitignore hygiene, and library thresholds. It offers confirmed fixes. `audit-agent` checks `mission.md` against the specs.

## 0.10.0

- ADRs move from one `specs/decision-log.md` to one fragment per plan, `specs/_decision/NNN-<plan-name>.md`, each with a kebab-case `**ID:**`. The old log was migrated.
- New `speq decision-log show`. `speq decision-log validate` checks the whole directory.
- Plans archive to `specs/_recorded/NNN-<plan-name>/`. ADRs drop the `**Date:**` field.

## 0.9.0

- New `speq:writing-guardrails` for speq artifacts and PR, issue, and comment text. Every prose-writing component loads it.

## 0.8.2

- `git-pr-agent` becomes the general `git-agent`. `speq-plan-pr` always leaves the PR as a draft. Only `speq-implement-pr` marks it ready.

## 0.8.1

- PR titles follow `<type>(<scope>): <slug>`. The PR stays a draft until the implementation is pushed.

## 0.8.0

- `code-reviewer` gains a YAGNI and over-engineering category, and every category uses `[TAG]` markers. `speq-code-guardrails` gains a dependency rule and YAGNI checks.

## 0.7.0

- New `speq-plan-pr` and `speq-implement-pr`: headless planning and implementation on a `feat/<plan-name>` branch with a PR.
- `planner-agent` can escalate in headless mode through an `OPEN QUESTIONS:` sentinel.

## 0.6.0

- Pre-built binaries for Linux x86_64 and ARM64, macOS, and Windows. The installer tries them first and falls back to a source build.

## 0.5.1

- Fix: the installer skipped `main` when piped through `curl | bash`.

## 0.5.0

- Embedding inference is pure Rust and needs no separate runtime, also on Intel Mac. The installer downloads the model on every install. Missing model files fail with a clear error.

## 0.4.2

- Fix: MCP servers registered twice.

## 0.4.1

- Fix: Serena starts in the current project (`--project-from-cwd`).

## 0.4.0

- Codex support: the installer generates a Codex plugin next to the Claude Code one, registers a Codex marketplace, and installs skills into `$CODEX_HOME/skills`. Skills stay `/speq:*` on both. Serena and Context7 are declared for Codex.

## 0.3.1

- New `speq decision-log validate` for the ADR format. `speq plan validate` checks an optional `decision-log.md`.
- `planner-agent` writes the plan decision log. `recorder-agent` promotes curated entries to the permanent log.

## 0.3.0

- `speq-plan` and `speq-record` become thin orchestrators over `planner-agent` and `recorder-agent`.
- New `implementer-expert-agent` for tasks tagged `[expert]`. Model and effort are pinned per sub-agent.

## 0.2.9

- Record rejects mismatched or unclosed delta markers.
- Local cache fallback when the system cache is not writable. `SPEQ_CACHE_DIR` overrides the cache location.

## 0.2.8

- Builds on Intel Mac. Semantic anchors in skills and docs.

## 0.2.7

- Semantic anchors in skills and docs.

## 0.2.5

- Fix: RFC 2119 keyword matching at word boundaries.
- New curl-pipeable uninstaller.

## 0.2.4

- New `plan list` command. MCP config moves into the plugin.
- Fix: the installer exited when the Rust toolchain was missing.

## 0.2.2

- Initial release.
