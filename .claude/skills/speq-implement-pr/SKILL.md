---
name: speq-implement-pr
description: "Headless follow-up to speq-plan-pr. Continues on a plan's feat/plan-name branch, runs speq-implement, bumps the version, runs the real test suites, records only if green, then pushes and opens/updates a PR. Arg: plan name, PR number, or branch name."
model: sonnet
---

# Spec Implementer, headless (Orchestrator)

This skill is a **thin orchestrator**, layered on top of `speq-implement`
the same way `speq-plan-pr` layers on `speq-plan`. It resolves which branch
to work on, runs the existing implement/record skills unchanged, gates
`speq-record` on real test results, and delegates every git/PR mechanic to
`git-pr-agent`. It never runs a git or `gh` command itself and never
implements code itself — `speq-implement` already owns that.

Stop and report on any step's failure — do not proceed to the next step.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli` — spec discovery, to resolve plan names

`speq-implement` and `speq-record` invoke their own required skills when
this orchestrator calls them. `git-pr-agent` invokes its own (`/speq-cli`).

## Workflow

### 1. Resolve Target + Branch (delegate)

`$1` empty → ask the caller which plan. Otherwise delegate:

```python
Task(
  subagent_type="git-pr-agent",
  description="Resolve implement-pr target",
  prompt="""
## Mode
resolve-target

## Input
<$1 — a plan-name, PR number, or branch name>

## Your Task
Check out the right branch: a PR checkout, an existing feat/<plan-name>
branch (local or remote), or create feat/<plan-name> fresh off the default
branch if the plan only exists locally and was never pushed. Report
plan/PR/open-questions state per the resolve-target output format.
"""
)
```

### 2. Blocker Check (orchestrator)

If step 1 reports unresolved `open-questions.md` → **stop**. Report that the
plan has open questions pending human review (resolve via PR comments and
`/speq:plan-pr <plan-name>`, or locally with `/speq:plan <plan-name>`); do
not implement an unresolved plan.

### 3. Implement

Invoke `/speq-implement <plan-name>` and let it run to completion —
unchanged, reused as-is. It creates/updates `tasks.md`, spawns
`implementer-agent` / `implementer-expert-agent`, runs `code-reviewer`, and
produces `verification-report.md`.

### 4. Bump Version (orchestrator)

Bump the workspace version per the plan's `workspace/version` spec delta if
it specifies one; otherwise apply the conventional next version per
Conventional Commits semantics (this plan's changes are `feat` → minor bump,
unless the plan is purely a `fix` → patch). Run a build to keep the lockfile
in sync.

### 5. Test + Record Gate (orchestrator)

Run the project's real test suites (per `specs/mission.md § Commands` —
typically an integration suite and an end-to-end suite). Only if **all**
suites are fully green, invoke `/speq-record <plan-name>` — and if it raises
its library-threshold split question, **answer yes** automatically (split)
so a headless run never stalls waiting for that decision. If any suite
fails, **stop here**, report the failures, and do not record.

### 6. PR (delegate)

```python
Task(
  subagent_type="git-pr-agent",
  description="Push implementation and open/update PR for <plan-name>",
  prompt="""
## Mode
commit-and-push

## Plan Name
<plan-name>

## Paths
<implementation files, version bump, verification-report.md>

## Message
feat(<scope>): implement <plan-name>

## Your Task
Commit and push per the commit-and-push output format.
"""
)
```

then:

```python
Task(
  subagent_type="git-pr-agent",
  description="Open/update PR for <plan-name>",
  prompt="""
## Mode
open-or-update-pr

## Plan Name
<plan-name>

## Draft
false

## Title
<plan-name>

## Body
Summary of the implementation diff and both test-suite results
(integration + e2e), plus the /speq:record outcome.

## Your Task
Open or update per the open-or-update-pr output format.
"""
)
```

Because this is the same `feat/<plan-name>` branch `speq-plan-pr` used, this
adds commits to the existing PR (or opens one, if the plan was only ever
implemented locally). Leave the PR for human review — never merge it.

## Spec Hierarchy (reference)

```
specs/
├── <domain>/<feature>/spec.md            # Permanent (after record)
├── _plans/<plan-name>/                   # Active until recorded
│   ├── tasks.md                          # Created by speq-implement
│   └── verification-report.md            # Created by speq-implement
└── _recorded/<plan-name>/                # Archived by speq-record
```

## Work Split (reference)

| Step | Performed by | Why |
|------|--------------|-----|
| Target resolution, gating, coordination | This skill (pins Sonnet) | Tool-call heavy, reasoning light |
| Task breakdown, coding, review | `speq-implement` (unchanged) | Already the right split — not duplicated here |
| Spec merge, archive | `speq-record` (unchanged) | Already the right split — not duplicated here |
| Branch, commit, push, PR create/update | `git-pr-agent` sub-agent | Mechanical; keeps git/gh detail out of both orchestrators |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Orchestrator runs `git`/`gh` itself | That's `git-pr-agent`'s entire job |
| Recording without both suites green | Implementation not proven |
| Implementing a plan with open questions | The human-in-the-loop point is exactly there — don't skip it |
| Opening a second PR for the same plan | Same `feat/<plan-name>` branch as `speq-plan-pr` — always update, never duplicate |
| Merging the PR | Never this skill's call to make |
