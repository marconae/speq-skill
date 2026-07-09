---
name: speq-git-operations
description: The git/gh operation-to-command mapping and return formats for git-agent — the sole write-permitted actor in this system. Triggered by git-agent.
---

# Git Operations

Unlike other skills in this system, this one documents *write* commands — that's intentional. `git-agent` is the sole component permitted to write git history or touch a remote; this skill is its exclusive execution reference and does not apply to any other agent.

## `create-branch`
Params: `branch` (exact name), `base` (optional; default: repository default branch).
`git fetch`, then create `branch` off `base` and check it out.
Returns:
```
Branch: <branch> (created off <base>)
```

## `checkout`
Params: `target` (a branch name, a PR number, `#N`, or a PR URL).
Resolve and check out: PR number/URL → `gh pr checkout <target>`; a branch that exists locally or remotely → check it out (`git fetch` first if remote-only). If `target` matches nothing, report not-found — create nothing.
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
Params: none — pushes the current branch (`-u origin <branch>` on first push).
Returns:
```
Pushed: <branch>
```

## `create-pr`
Params: `title`, `body`, `draft` (bool), `base` (optional; default: default branch), `head` (optional; default: current branch).
If a PR already exists for `head`, do not recreate it — return it unchanged. Otherwise `gh pr create` with the given title and body, adding `--draft` when `draft` is true.
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
Params: `target` (PR or issue number/URL; default: PR for current branch), `since` (optional ISO timestamp — return only comments and reviews posted at or after it).
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
