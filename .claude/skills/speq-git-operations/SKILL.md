---
name: speq-git-operations
description: The git/gh operation-to-command mapping and return formats for git-agent — the sole write-permitted actor in this system. Triggered by git-agent.
---

# Git Operations

This skill documents write commands for `git-agent` only, the sole component permitted to write git history or touch a remote. It applies to no other agent.

## `create-branch`
Params: `branch` (exact name), `base` (optional; default: repository default branch).
`git fetch`, then create `branch` off `base` and check it out.
Returns:
```
Branch: <branch> (created off <base>)
```

## `checkout`
Params: `target` (a branch name, a PR number, `#N`, or a PR URL).
Resolve and check out: PR number/URL → `gh pr checkout <target>`; a branch that exists locally or remotely → check it out (`git fetch` first if remote-only). If `target` matches nothing, report not-found. Create nothing.
Returns:
```
Branch: <branch> (checked out)
PR: <none | #N (draft|ready) — <url>>
```

## `commit`
Params: `paths` (files/globs to stage), `message` (exact commit message).
`git add <paths>`, `git commit -m "<message>"`. Nothing staged or nothing changed is a no-op, not an error.
Returns:
```
Committed: <sha> "<subject>"
```
or
```
Nothing to commit
```

## `push`
Params: none. Pushes the current branch (`-u origin <branch>` on first push).
Returns:
```
Pushed: <branch>
```

## `create-pr`
Params: `title`, `body`, `draft` (bool), `base` (optional; default: default branch), `head` (optional; default: current branch).
If a PR already exists for `head`, do not recreate it. Return it unchanged. Otherwise `gh pr create` with the given title and body, adding `--draft` when `draft` is true.
Returns:
```
PR: #N (draft|ready) — <url> (created | already existed)
```

## `ready-pr`
Params: `pr` (optional; default: PR for current branch).
`gh pr ready <pr>` when it is a draft; no-op if already ready.
Returns:
```
PR: #N (ready) — <url>
```

## `comment-pr`
Params: `pr` (optional; default: PR for current branch), `body` (exact comment text).
`gh pr comment <pr> --body "<body>"`.
Returns:
```
Comment: <url>
```

## `create-issue`
Params: `title`, `body`, `labels` (optional).
`gh issue create` with the given fields.
Returns:
```
Issue: #N — <url>
```

## `comment-issue`
Params: `issue` (number or URL), `body` (exact comment text).
`gh issue comment <issue> --body "<body>"`.
Returns:
```
Comment: <url>
```

## `read-comments`
Params: `target` (PR or issue number/URL; default: PR for current branch), `since` (optional ISO timestamp: return only comments and reviews posted at or after it).
`gh pr view --json comments,reviews` or `gh issue view --json comments` (or `gh api`). Concatenate each body as plain text with author and timestamp. An empty result is valid.
Returns:
```
Comments found: N
<author> @ <timestamp>:
<body>
...
```
or
```
Comments found: 0
```

## Composite operations

Each composite is one operation with a sequence fixed by this skill. The caller supplies parameters, never the sequence. "Nothing to commit" at the commit step is a no-op: continue with the remaining steps. Stop at the first failing step. Report which steps completed and which step failed.

## `ship-draft`
Params: `paths` (files/globs to stage), `message` (exact commit message), `title`, `body`.
Sequence: `commit`(paths, message) → `push` → `create-pr`(draft: true, title, body).
Returns:
```
Committed: <sha|no-op> / Pushed: <ref> / PR: #<n> (draft) — <url>
```

## `ship-ready`
Params: `paths` (files/globs to stage), `message` (exact commit message), `title`, `body`.
Sequence: `commit`(paths, message) → `push` → `create-pr`(draft: true, title, body) → `ready-pr`.
Returns:
```
Committed: <sha|no-op> / Pushed: <ref> / PR: #<n> (draft) — <url> / Ready: true
```

## `flag-blocked`
Params: `paths` (files/globs to stage), `message` (exact commit message), `title`, `body`, `comment_body` (exact comment text).
Sequence: `commit`(paths, message) → `push` → `create-pr`(draft: true, title, body) → `comment-pr`(comment_body).
Returns:
```
Committed: <sha|no-op> / Pushed: <ref> / PR: #<n> (draft) — <url> / Commented: <comment-url>
```
