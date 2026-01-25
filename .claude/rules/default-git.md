---
name: Version Control
description: Git guardrails and commit conventions
---

# Version Control

## Git Guardrails

**READ ONLY.** Inspect state only. Never write.

### Allowed

```bash
git status    git diff    git log    git show    git branch
```

### Forbidden

`add` `commit` `push` `pull` `fetch` `merge` `rebase` `cherry-pick` `checkout` `switch` `restore` `reset` `revert` `tag`

**No exceptions. User controls all git writes.**

**You must not use `git stash`**

### Before Completing Work

```bash
git status   # Verify expected files changed
git diff     # Review actual changes
```

## Commit Conventions

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
| `refactor` | Restructuring |
| `test` | Tests |
| `docs` | Documentation |
| `spec` | Spec changes |
| `chore` | Maintenance |

**Breaking:** `!` after type or `BREAKING CHANGE:` footer → MAJOR version
