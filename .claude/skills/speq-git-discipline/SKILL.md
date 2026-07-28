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

Committing these puts the plan's evidence trail into the PR's history before `/speq-record`'s archive `mv specs/_plans/<plan-name> specs/_recorded/NNN-<plan-name>` removes the directory from tracked space. By default, `specs/_recorded/` is itself gitignored — its contents then live only in the local workspace, with the evidence trail already preserved in history by the earlier plan-directory commits rather than by tracking the archive. That's a default, not a rule this workflow enforces: a project may choose to track `_recorded/` too, in which case the archived plan simply lands in history a second time at the archive step.

`/speq-audit`'s gitignore-hygiene checks default to this split (`_plans`/`_decision` tracked, `_recorded` ignored) and, like every other audit finding, only offer to align a project that's drifted from it — never force it.

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
