[speq-skill](../README.md) / [Docs](./index.md) / Headless PR Pipeline

---

# Headless PR Pipeline

Plan and build a change as a GitHub pull request. It suits teams that already iterate on PRs: the plan, the questions, and the review happen directly within the pull request.

```
/speq:plan-pr <intent>  →  draft PR (with open questions, if blocked)
                                   │
                         team replies on the PR
                                   │
                                   ▼
             /speq:implement-pr <name>  →  same PR, updated and marked ready
```

## Why use it

- The team reviews the plan as a PR before any code exists
- Open questions arrive as PR comments, so anyone can answer, not only the person who started the run
- Plan, code, and recorded specs end up in one PR, with one history
- It runs unattended, so you can start it and come back to a PR

## Plan on a branch

Run `/speq:plan-pr <intent>`.

- Creates the branch `feat/<plan-name>` and opens a draft PR
- Works without an interview. Where `/speq:plan` would ask you, it takes a documented default, or posts an open question on the PR
- Names the PR like a conventional commit, for example `add-search-candle` becomes `feat(search): add search candle`

## Answer questions on the PR

When a decision needs input, the plan stops and asks.

- It asks for decisions
- The PR stays a draft, and the questions appear as a comment
- Reply on the PR to answer, then run `/speq:plan-pr <plan-name>` again. It reads your replies as the answers
- `/speq:implement-pr` refuses to continue while questions are open

## Build and ship

Run `/speq:implement-pr <plan-name>`.

- Implements the plan on the same branch and pushes to the same PR
- Runs the real test suites. It records the specs only if they pass
- Marks the PR ready for review
- Says yes to the library-split question of `/speq:record`
- If recording fails, it stops with `Blocked: record failed: <reason>`

## Recover from interruptions

- Each phase leaves a mark in the plan's `tasks.md`
- If a run stops, for example at a usage limit or a crash, run `/speq:implement-pr <plan-name>` again in the same working directory. It resumes at the phase that was not finished

## Architecture changes

- When the plan changes the architecture, the PR body carries one `Architecture:` line with the changed sections
- After recording, the line reads `Architecture: merged into specs/architecture.md: <sections>`

## Git access

- Both skills run every branch, commit, push, and PR operation themselves, per `/speq:git-operations`
