---
name: speq-git-discipline
description: Read-only git guardrails and the Conventional Commits reference — no history writes, verify changes before completing. Triggered by planner-agent, implementer-agent, implementer-expert-agent, and recorder-agent; git-agent alone is exempt and uses /speq-git-operations instead.
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

Everything under `specs/_plans/<plan-name>/` is tracked and committed with the plan directory — never gitignored. That includes the run's evidence artifacts alongside the plan itself:

- `open-questions.md` — present only while the plan is blocked
- `review/` — plan-review findings (`round-<N>.md`)
- `review-findings.md` — code-review findings
- `tasks.md` and `verification-report.md`

Committing these puts the plan's evidence trail into the PR's history before `/speq-record`'s archive `mv specs/_plans/<plan-name> specs/_recorded/NNN-<plan-name>` removes the directory from tracked space. `specs/_recorded/` itself stays gitignored — its contents are local-workspace state, preserved in history by the earlier plan-directory commits, not by tracking the archive.

`/speq-audit`'s gitignore-hygiene checks verify exactly this split: `_plans` and `_decision` tracked, only `_recorded` ignored.

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
