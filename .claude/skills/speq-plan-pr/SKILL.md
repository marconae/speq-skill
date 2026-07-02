---
name: speq-plan-pr
description: Headless, non-interactive version of speq-plan. Plans a feature without a live interview, commits the result to a feat/plan-name branch, and opens a PR. If a decision genuinely needs a human, it persists the partial plan and open questions and asks in a PR comment instead of blocking. Arg — plan name, feature intent text, PR number, or branch name.
model: sonnet
---

# Spec Planner, headless (Orchestrator)

This skill is a **thin orchestrator**, thinner than `speq-plan`: it has no
live user to interview. It gathers whatever context it can, delegates all
planning reasoning to `planner-agent` (same sub-agent `speq-plan` uses, told
to operate in headless mode), and delegates every git/PR mechanic to
`git-pr-agent`. The orchestrator itself never runs a git or `gh` command and
never authors spec content — it only decides which sub-agent to call next and
interprets what came back.

**Why this exists:** `speq-plan` blocks on `AskUserQuestion` for its
interview — correct for a human driving a live session, but it can't run
unattended. This skill moves the human-in-the-loop point from "blocks a chat
session" to "blocks on a PR comment," so autonomous callers can plan a
feature and hand off async when a decision genuinely needs a person.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli` — Spec discovery and search

`planner-agent` invokes its own required skills (`/speq-code-tools`,
`/speq-ext-research`, `/speq-cli`, `/speq-git-discipline`) independently.
`git-pr-agent` invokes its own (`/speq-cli`). Don't duplicate those lists
here — the orchestrator's job is to call the right sub-agent, not to know
what it does internally.

## Workflow

### 1. Resolve Mode (delegate)

```python
Task(
  subagent_type="git-pr-agent",
  description="Resolve plan-pr target",
  prompt="""
## Mode
resolve-target

## Input
<the raw argument the caller passed>

## Your Task
Classify the input (PR number/URL, existing branch, or plan-name), check out
or create feat/<plan-name> accordingly, and report plan/PR/open-questions
state per the resolve-target output format.
"""
)
```

If the input didn't match anything existing, treat it as free-text feature
intent for a brand-new plan and derive `<plan-name>` the same way `speq-plan`
does:

| Verb | When |
|------|------|
| `add` | New feature |
| `change` | Modify existing |
| `remove` | Deprecate/delete |
| `refactor` | Restructure, same behavior |
| `fix` | Bug or spec mismatch |

Pattern: `<verb>-<feature-scope>[-<qualifier>]`

### 2. Gather Answers — resume only (delegate)

If step 1 reported unresolved `open-questions.md`, delegate to
`git-pr-agent` (`mode: fetch-answers`) to pull PR comments and review
comments as plain-text Q&A. This stands in for the interview `speq-plan`
would otherwise run live.

### 3. Discovery (orchestrator)

Same lightweight calls `speq-plan` makes — enough context to brief
`planner-agent`, not a full exploration:

```bash
speq domain list
speq feature list
speq search query "<relevant terms>"
```

### 4. Delegate to planner-agent

Same shape `speq-plan` uses, with headless framing added:

```python
Task(
  subagent_type="planner-agent",
  description="Plan <plan-name> (headless)",
  prompt="""
## Plan Name
<plan-name>

## Interview Mode
headless

## User Intent
<feature intent text, or "see resume Q&A below">

## Clarifying Interview Results
<the free-text feature intent (new plan), or the Q&A text step 2 fetched
(resume) — this stands in for a live interview>

## Existing Context
<output of relevant `speq search` / `speq feature get` calls>

## External Research
none — agent to research as needed

## Your Task
Produce spec deltas and plan.md per your normal workflow. You are in
headless mode: follow your "Headless / Non-Interactive Mode" section —
assume and document conventional decisions, escalate only irreducible ones
via the OPEN QUESTIONS: sentinel. Tag deep-reasoning tasks with [expert].

Return the list of files created and the validation result, or an
OPEN QUESTIONS: block if you had to stop.
"""
)
```

### 5. Branch on the Result (delegate)

**Clean return** (no `OPEN QUESTIONS:` sentinel):

1. Confirm `speq plan validate <plan-name>` passes.
2. Delegate to `git-pr-agent`: `commit-and-push` (the plan directory, message
   `spec(plan): <plan-name>`) → `open-or-update-pr` (`draft: false`, title
   `spec(plan): <plan-name>`, body summarizing the plan's Features table and
   task count, ending "Ready for implementation — run
   `/speq:implement-pr <plan-name>`").
3. If this was a resume of a previously-blocked plan, also delegate
   `mark-resolved`.

**`OPEN QUESTIONS:` returned:**

Delegate to `git-pr-agent`: `post-questions` with the question list. It
writes `open-questions.md`, flags `plan.md` as blocked, commits, pushes, and
opens/updates the PR as draft with the questions posted as a comment.

### 6. Report (orchestrator)

Tell the caller whether the plan is ready or blocked, and the PR link either
way.

## Spec Hierarchy (reference)

```
specs/
├── <domain>/<feature>/spec.md            # Permanent
├── _plans/<plan-name>/                   # Active
│   └── open-questions.md                 # Present only while blocked
└── _recorded/<plan-name>/                # Archived
```

## Work Split (reference)

| Step | Performed by | Why |
|------|--------------|-----|
| Mode resolution, discovery, coordination | This skill (pins Sonnet) | Tool-call heavy, reasoning light |
| Spec delta authoring, ADR, task decomposition, assume-vs-escalate calls | `planner-agent` sub-agent | Reasoning-heavy; defects here compound through implementation |
| Branch, commit, push, PR create/comment | `git-pr-agent` sub-agent | Mechanical; keeps git/gh detail out of both orchestrators |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Orchestrator runs `git`/`gh` itself | That's `git-pr-agent`'s entire job |
| Treating headless mode as "never ask" | Irreducible decisions still must escalate — see `planner-agent`'s headless section |
| Silently dropping open questions on resume | Always re-fetch and forward PR answers before re-planning |
| Opening a second PR for the same plan | One branch/PR per plan — `git-pr-agent` reuses the existing one |
