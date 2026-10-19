---
name: speq-plan-pr
description: "Plans a feature through the same clarifying interview as /speq-plan, then creates the feat/plan-name branch, commits the plan, and opens a draft pull request. Arg: feature intent text, plan name, PR number, or branch name."
model: sonnet
---

# Spec Planner, PR flow (Orchestrator)

This skill runs `/speq-plan` and ships the result as a draft PR. It writes no plan content itself. `planner-agent` and `plan-reviewer` do that, through `/speq-plan`.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli`: spec discovery
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text
- `/speq-git-operations`: the git/gh operation-to-command mapping, safety rules, and return formats.

## Workflow

### 0. Load Project Hook

Check for `.speq/plan-pr-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/plan-pr-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue without mention.

### 1. Resolve Target

If the argument is a plan name, PR number, or branch, check it out:

```
Run — operation: checkout (per /speq-git-operations)
  target: <the raw argument>
```

- **Found, with a PR:** go to step 3 (revise).
- **Not found, or the argument is feature intent text:** go to step 2 (new plan).

### 2. New Plan

1. Run `/speq-plan` steps 0 to 6: discovery, the clarifying interview, the plan name, `planner-agent`, and the one review with its fix pass. Skip its step 7.
2. Create the branch. The plan files are uncommitted, so they carry over:
   ```
   Run — operation: create-branch (per /speq-git-operations)
     branch: feat/<plan-name>
   ```
3. Ship the draft, per [Ship](#ship).

### 3. Revise from PR Comments

Run this when step 1 found a PR for the target.

1. Read the feedback:
   ```
   Run — operation: read-comments (per /speq-git-operations)
     since: <timestamp of the last `spec(plan)` commit on this branch>
   ```
2. Treat every returned comment as user feedback on the files the PR changed: the PR conversation, review summaries, and inline comments on any file. Write them to `specs/_plans/<plan-name>/notes/pr-feedback.md`, one entry per comment with its file and line when it has them. `notes/` is never committed.
3. If no comment is new, report that and stop.
4. Respawn `planner-agent` with the path to `notes/pr-feedback.md`, per `/speq-planning`'s Revision Mode. If a comment is ambiguous, ask the user.
5. Run `/speq-plan` step 6 on the revised plan.
6. Commit and push, and answer on the PR:
   ```
   Run — operation: commit (per /speq-git-operations)
     paths: the plan directory, excluding specs/_plans/<plan-name>/notes/
     message: spec(plan): address PR feedback for <plan-name>

   Run — operation: push (per /speq-git-operations)

   Run — operation: comment-pr (per /speq-git-operations)
     comment_body: one line per comment, `Resolved` or `Could not resolve` with the reason
   ```

### Ship

Commit the plan and open the draft PR with one composite call:

```
Run — operation: ship-draft (per /speq-git-operations)
  paths: the plan directory, excluding specs/_plans/<plan-name>/notes/
  message: spec(plan): <plan-name>
  title: <the derived <type>(<scope>): <slug>>
  body: per references/pr-body-template.md, ending "Draft pending implementation."
```

Derive the PR title from `<plan-name>` as `<type>(<scope>): <slug>`:
- **type**: `add`/`change` → `feat`, `remove` → `chore`, `refactor` → `refactor`, `fix` → `fix`; `chore` for an unparseable name.
- **scope**: the token after the verb.
- **slug**: the `<plan-name>` with hyphens turned into spaces.

Example: `add-plan-export` ⇒ `feat(plan): add plan export`. With no scope token, emit `<type>: <slug>`.

The PR stays a draft. `speq-implement-pr` is the only skill that marks it ready.

### Report

Give the PR link. Print `plan.md`'s `## Impact` section, the `ADR candidates:` line, and the `Architecture:` line from the PR body. Mention the non-blocking findings from `specs/_plans/<plan-name>/review/review.md`, read from the file, not from memory. If the fix pass ran, name it in one line ("N review findings fixed"). Tell the user to review the PR, comment on it, and run `/speq-plan-pr <PR number>` again to apply the comments, or `/speq-implement-pr <plan-name>` to build.
