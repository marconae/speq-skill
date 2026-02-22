---
name: speq-git-discipline
description: Enforce Git discipline for speq-skills.
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

## Security

Never commit secrets (API keys, passwords, credentials, tokens).

## Commit Conventions

When user asks for a commit message, use this format:

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
