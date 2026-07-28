---
name: speq-git-discipline
description: Read-only git guardrails and the Conventional Commits reference — no history writes, verify changes before completing. Triggered by planner-agent, implementer-agent, implementer-expert-agent, and recorder-agent. The headless orchestrators speq-plan-pr and speq-implement-pr are the sole exception: they write git history directly and use /speq-git-operations instead of this skill.
---

# Git Discipline

Git read-only guardrails. User controls all git writes.

## Allowed Commands

```bash
git status    git diff    git log    git show    git branch
```

## Forbidden Commands

```
add    commit    push    pull    fetch
merge  rebase    cherry-pick
checkout    switch    restore
reset    revert    stash    tag
```

**No exceptions.**

## Before Completing Work

Verify expected changes:

```bash
git status   # Check files changed
git diff     # Review actual changes
```

## Tracked vs Gitignored Spec Paths

- Everything under `specs/_plans/<plan-name>/` is tracked and committed with the plan directory, never gitignored. This includes the evidence artifacts:
  - `open-questions.md` (present only while the plan is blocked)
  - `review/` (plan-review findings, `round-<N>.md`)
  - `review-findings.md` (code-review findings)
  - `tasks.md` and `verification-report.md`
- These commits put the evidence trail into history before `/speq-record` archives the plan (`mv specs/_plans/<plan-name> specs/_recorded/NNN-<plan-name>`).
- `specs/_recorded/` is gitignored by default. This is a default, not an enforced rule: a project can track `_recorded/`, and the archived plan then lands in history a second time.
- `/speq-audit`'s gitignore-hygiene checks default to this split (`_plans`/`_decision` tracked, `_recorded` ignored). They only offer to align a drifted project, never force it.

## Security

Never commit secrets (API keys, passwords, credentials, tokens).

## Commit Conventions

Follow the **Conventional Commits** specification:

```
<type>[scope]: <description>

[body]

[footer]
```

| Type | Use |
|------|-----|
| `feat` | New feature (MINOR) |
| `fix` | Bug fix (PATCH) |
| `perf` | Performance |
| `refactor` | Restructure, same behavior |
| `test` | Tests |
| `docs` | Documentation |
| `spec` | Spec changes |
| `chore` | Maintenance |

**Breaking changes:** Add `!` after type or `BREAKING CHANGE:` footer → MAJOR version
