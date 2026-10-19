[speq-skill](../README.md) / [Docs](./index.md) / PR Pipeline

---

# PR Pipeline

Plan and build a change as a GitHub pull request. It suits teams that already iterate on PRs: the plan, the questions, and the review happen directly within the pull request.

```
/speq:plan-pr <intent>  →  interview, then draft PR
                                   │
                    you comment on the PR
                                   │
              /speq:plan-pr <PR number>  →  plan revised from your comments
                                   │
                                   ▼
             /speq:implement-pr <name>  →  same PR, updated and marked ready
```

## Why use it

- The team reviews the plan as a PR before any code exists
- Comments on the PR become feedback for the next planning run
- Plan, code, and recorded specs end up in one PR, with one history

## Plan on a branch

Run `/speq:plan-pr <intent>`.

- Runs the same clarifying interview and review as `/speq:plan`
- Creates the branch `feat/<plan-name>`, commits the plan, and opens a draft PR

## Revise from PR comments

Comment on the PR: on the conversation, in a review, or inline on any changed file. Then run `/speq:plan-pr <PR number>` again.

- It reads the new comments as your feedback on the changed files and revises the plan
- It commits and pushes to the same PR, and replies with what it resolved and what it could not

## Build and ship

Run `/speq:implement-pr <plan-name>`.

- Implements the plan on the same branch and pushes to the same PR
- Runs the real test suites. It records the specs only if they pass
- Marks the PR ready for review
- Auto splits the spec library during `/speq:record`
- If recording fails, it stops with `Blocked: record failed: <reason>`

## Resume

- If a run stops, for example at a usage limit, run `/speq:implement-pr <plan-name>` again. It picks up the plan's `tasks.md` and continues where the work stopped

## Architecture changes

- When the plan changes the architecture, the PR body carries one `Architecture:` line with the changed sections
