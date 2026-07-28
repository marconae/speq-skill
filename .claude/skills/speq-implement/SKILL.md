---
name: speq-implement
description: "Orchestrate implementation of a reviewed plan: task breakdown, TDD sub-agents, code review, and verification report. Use when the user asks to implement, build, or execute a plan under specs/_plans/ — after /speq-plan, before /speq-record. Arg: <plan-name>."
model: sonnet
---

# Spec Implementer (Orchestrator)

Orchestrate implementation of the plan in `specs/_plans/<plan-name>`. Get the plan name from the user prompt. If none is given, ask.

Sub-agents do the heavy work. Each pins its own model and effort in its frontmatter:

| Sub-agent | When used |
|-----------|-----------|
| `implementer-agent` | Groups with no `[expert]` task (default) |
| `implementer-expert-agent` | Groups with at least one `[expert]` task |
| `code-reviewer` | Final review of all changed files |

## Required Skills

Invoke before starting:
- `/speq-cli`: spec discovery
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text

Do not invoke coding skills (`/speq-code-tools`, `/speq-ext-research`, `/speq-code-guardrails`) yourself: the orchestrator never writes code. Sub-agents invoke their own required skills.

## Orchestrator Role

- Create and maintain `tasks.md` for persistence.
- Spawn sub-agents for parallel task groups.
- Update task status after each sub-agent completes.
- Never implement directly. Delegate all coding work.
- Rotate sub-agents to keep context windows fresh.

**Rotation rule:** sub-agents checkpoint after every 2-3 tasks (expert: 1-2). When a sub-agent has completed `max_tasks_per_agent` (default 5) tasks, or returns `ROTATION NEEDED`: read tasks.md for current state, note the completed tasks from the return, and spawn a fresh agent of the SAME type with the remaining tasks. Repeat until the group is complete.

**Rotation hand-off:** the outgoing agent writes a hand-off note to `specs/_plans/<plan-name>/notes/<group-letter>.md` (its own duty, per its Early Termination section). `<group-letter>` is the group's letter only — the token before the `:` in its Parallelization-table `Group` entry (group `A: plan-log validation` → `notes/A.md`), never the full group name and never a slug of it. Add one line to the fresh agent's brief: `Orientation: read specs/_plans/<plan-name>/notes/<group-letter>.md first`. If the note is absent, omit the line. The note is working state inside the plan directory: `/speq-record`'s archive step removes it, and it is never committed evidence.

## Workflow

### Phase 0: Load Project Hook (orchestrator)

Check for `.speq/implement-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/implement-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue, no mention.

Add a `Project Hook:` line (path only, not content) to every sub-agent brief below. `implementer-agent`, `implementer-expert-agent`, and `code-reviewer` read the hook themselves from that path.

### Phase 1: Load Plan

**Open-questions gate:** read `specs/_plans/<plan-name>/open-questions.md`. If it exists and is non-empty: **stop** and report that the plan has unresolved open questions pending human answers (resolve with `/speq-plan <plan-name>`, or via PR comments and `/speq-plan-pr <plan-name>` for a headless plan). This is the human-in-the-loop point, the same gate as `speq-implement-pr`'s blocker check.

```
Read: specs/_plans/<plan-name>/plan.md
```

Extract: feature specs, implementation tasks, parallelization groups, verification commands.

### Phase 2: Create Tasks

Decompose the plan into a **Work Breakdown Structure** in `specs/_plans/<plan-name>/tasks.md`.

**Lifecycle guard:** if `tasks.md` exists and contains a `## PR Lifecycle` section, `speq-implement-pr` pre-created it as its checkpoint (per its `references/checkpoint-protocol.md`). Preserve that section verbatim at the top and write the `## Phase N` sections below it. Never edit `## PR Lifecycle`: its writers are fixed by that protocol. Standalone runs get no lifecycle section. Create the file as below.

**Format:**

```markdown
# Tasks: <plan-name>

## Phase 2: Implementation (Group A)
- [ ] 2.1 <task from plan>
- [ ] 2.2 <task from plan> [expert]

## Phase 2: Implementation (Group B)
- [ ] 2.3 <task from plan>

## Phase 3: Verification
- [ ] 3.1 Run test suite
- [ ] 3.2 Run linter
```

**Status markers:**
- `[ ]` pending
- `[~]` started
- `[x]` completed

**Difficulty tags:**
- `[expert]`: tagged by `planner-agent` during planning. Routes the task's whole group to `implementer-expert-agent` (Phase 3 routing rule). Preserve the tag through every status transition.
- untagged: a group with only untagged tasks routes to `implementer-agent`.

If the plan tagged no tasks and a task clearly needs expert reasoning (for example concurrency, a cross-file refactor, a novel algorithm), you MAY add `[expert]` when materializing tasks.md. Tag sparingly: over-tagging wastes tokens.

Also create runtime tasks:
```
For each task in tasks.md:
  TaskCreate(subject, description, activeForm)
```

### Phase 3: Implement (Orchestrated)

For each parallel group in plan's `## Parallelization`:

1. **Route the whole group by its hardest task.** Any `[expert]` task routes the whole group to `implementer-expert-agent`. Otherwise the whole group goes to `implementer-agent`. One agent per group. Never split a group by tag: a group is one knowledge cluster, and each extra agent rebuilds the same mental model, which costs more than the model-price difference.
2. **Mark started**: update tasks.md, `[ ]` → `[~]`.
3. **Spawn one sub-agent for the group** with the matching invocation template below. If the plan's Parallelization table has a `Knowledge` column, copy the group's entry into the brief's `Knowledge:` line.
4. **Await completion**: the sub-agent returns results or a rotation signal.
5. **Handle rotation**: apply the Rotation rule and Rotation hand-off above.
6. **Mark completed**: update tasks.md, `[~]` → `[x]` (preserve the `[expert]` tag).
7. **Update TaskTools**: `TaskUpdate(taskId, status: "completed")`.
8. **Next group**: proceed once its dependencies are complete.

**Standard subagent invocation** (group has no `[expert]` task):

```
Delegate to implementer-agent — Implement <group-name>

## Your Tasks (the whole group)

{group_task_list}

## Context

- Plan: specs/_plans/{plan_name}/plan.md
- Tasks file: specs/_plans/{plan_name}/tasks.md
- Knowledge: <the group's Knowledge entry from the plan's Parallelization table — read these spec deltas and files first; omit this line if the plan has no Knowledge column>
- Orientation: read specs/_plans/{plan_name}/notes/<group-letter>.md first <rotation respawns only; <group-letter> is the group's letter only, the token before ":" in its Group entry — e.g. notes/A.md; omit otherwise>
- Update tasks.md after each task completion (preserve task numbering)
- Report checkpoint after every 2-3 tasks
- Project Hook: <if active, ".speq/implement-hook.md — read it and apply it"; otherwise omit this line>
```

**Expert subagent invocation** (group contains at least one `[expert]` task):

```
Delegate to implementer-expert-agent — Implement <group-name>

## Your Tasks (the whole group — routed to you for its [expert] tasks)

{group_task_list}

## Context

- Plan: specs/_plans/{plan_name}/plan.md
- Tasks file: specs/_plans/{plan_name}/tasks.md
- Knowledge: <the group's Knowledge entry from the plan's Parallelization table — read these spec deltas and files first; omit this line if the plan has no Knowledge column>
- Orientation: read specs/_plans/{plan_name}/notes/<group-letter>.md first <rotation respawns only; <group-letter> is the group's letter only, the token before ":" in its Group entry — e.g. notes/A.md; omit otherwise>
- The untagged tasks in the list are yours too — the group routes as one unit
- Preserve the [expert] tag when updating status markers
- Checkpoint after every 1-2 tasks (expert tasks are heavier)
- Report key reasoning / invariants applied
- Project Hook: <if active, ".speq/implement-hook.md — read it and apply it"; otherwise omit this line>
```

### Phase 4: Code Review

Review all changed files after implementation completes. Code review runs ONCE per implementation: after fix tasks complete, proceed to Phase 5 (its checks verify the fixes). Do not respawn `code-reviewer` for a second round.

1. **Collect changed files**: `git diff --name-only <base>` for tracked changes plus `git ls-files --others --exclude-standard` for new files. Implementation work is uncommitted at this point, so diff against the working tree. A commit-range diff (`<base>...HEAD`) would miss all of it.
2. **Spawn code-reviewer agent:**
   ```
   Delegate to code-reviewer — Review implementation quality

   ## Changed Files

   {changed_files_list}

   ## Context

   - Plan: specs/_plans/{plan_name}/plan.md
   - Review for: guardrail violations, dead code, test quality, bad comments, optimizations, YAGNI/over-engineering, error handling, design depth
   - Write findings to specs/_plans/{plan_name}/review-findings.md per your output format, partitioned into `## Standard fixes` and `## Expert fixes`; return only the one-line verdict.
   - Project Hook: <if active, ".speq/implement-hook.md — read it and apply it"; otherwise omit this line>
   ```
   It returns one line: `CODE REVIEW: <n> findings — standard: <n>, expert: <n> — <path>`. You never see the findings themselves.
3. **Process findings**: branch on the two counts in the verdict. If both are 0, go to Phase 5. One agent applies the whole fix pass, routed by its hardest finding: the findings cluster on the files just written, and a second agent there re-orients into the same code and can collide with the first.
   - **standard > 0, expert == 0**: spawn `implementer-agent`:
     ```
     Delegate to implementer-agent — Apply standard review fixes

     ## Your Assignment (fix-task mode)

     Read specs/_plans/{plan_name}/review-findings.md, section `## Standard fixes`.
     Append one fix task per finding to specs/_plans/{plan_name}/tasks.md under a
     `## Phase 4: Review Fixes` group, deriving each task line from the finding's
     `Fix:` field, then execute them.

     ## Context

     - Plan: specs/_plans/{plan_name}/plan.md
     - Project Hook: <if active, ".speq/implement-hook.md — read it and apply it"; otherwise omit this line>
     ```
   - **expert > 0**: spawn `implementer-expert-agent` with the same brief shape, naming section `## Expert fixes`, and also `## Standard fixes` when standard > 0. It tags only the tasks derived from `## Expert fixes` with `[expert]`.
4. **Proceed to Phase 5.**

### Phase 5: Verification

#### 5a. Automated Checks

Execute commands from plan's `## Verification > Checklist`. Redirect each command's output to a log: `mkdir -p target && <command> > target/speq-<suite>.log 2>&1`, then branch on the exit code. If a report quotes output, quote at most `tail -n 30` of the log. Raw build or test output never lands verbatim in the transcript.

- Build → exit 0
- Test → 0 failures
- Lint → 0 errors
- Format → no changes

#### 5b. Scenario Coverage Audit

Cross-reference plan's `## Verification > Scenario Coverage` against test results:

- Every listed scenario → corresponding test exists and passes
- Flag any scenario without a passing test as incomplete

#### 5c. Manual Verification

Execute each step from plan's `## Verification > Manual Testing`:

- Run documented commands against the built software
- Capture actual output as evidence for the verification report
- Mark pass/fail per feature

Update tasks.md verification tasks as completed.

### Phase 6: Verification Report

Generate using `references/verification-template.md`. Structure the report **BLUF (Bottom Line Up Front)**: verdict and summary first, evidence after. Fill the Verdict table's `Code review` row from Phase 4's verdict line: a later session's condensed PR comment quotes the Verdict table, and that row is how the counts survive.

Save to: `specs/_plans/<plan-name>/verification-report.md`

### Phase 7: Completion

```
✓ All tasks in tasks.md marked [x]
✓ Code review passed (or findings fixed)
✓ Verification passed
✓ Report generated

Code review: <n> findings — <n> fixed
Verification report: specs/_plans/<plan-name>/verification-report.md

Ready for: /speq-record <plan-name>
```

The two report lines are the run's machine-readable handoff: a headless caller folds the code-review line into its condensed PR comment, and `/speq-record` gates on the report path.

## Context Recovery

If context is lost or compacted:

1. Read `specs/_plans/<plan-name>/tasks.md`
2. Find incomplete tasks (`[ ]` or `[~]`). Scan only `## Phase N` sections, never `## PR Lifecycle` (its unnumbered entries are `speq-implement-pr` checkpoints, not work items)
3. Resume from the first incomplete task
4. Continue the orchestration workflow

## References

| File | Use When |
|------|----------|
| `references/tdd-cycle-checklist.md` | Sub-agent TDD reference |
| `references/task-flow.md` | Task lifecycle management |
| `references/verification-template.md` | Phase 6 report generation |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Orchestrator writes code directly | All coding is delegated to sub-agents |
| Orchestrator invokes coding skills | Same reason: that text dilutes the most expensive session for work it never does |
| Splitting one group between two agents by tag | The group is one knowledge cluster; each extra agent re-derives the same mental model |
| Proceeding past a non-empty open-questions.md | The plan is blocked on human answers |
| Dropping the `[expert]` tag on a status flip | The tag must survive `[ ]` → `[~]` → `[x]` |
| Marking `[x]` without a sub-agent completion return | Only verified completions are done |
| A second code-review round | Review runs once; Phase 5 verifies the fixes |
| Skipping the verification report | `/speq-record` gates on it |
