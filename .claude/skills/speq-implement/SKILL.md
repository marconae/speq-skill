---
name: speq-implement
description: "Orchestrate implementation of a reviewed plan: task breakdown, TDD sub-agents, code review, and verification report. Use when the user asks to implement, build, or execute a plan under specs/_plans/ — after /speq-plan, before /speq-record. Arg: <plan-name>."
model: sonnet
---

# Spec Implementer (Orchestrator)

Orchestrate implementation of the plan in `specs/_plans/<plan-name>`. Get the plan name from the user prompt; ask if none specified.

Orchestration (reading tasks.md, dispatching sub-agents, verifying results) is tool-call heavy but not reasoning-heavy. The sub-agents do the heavy lifting — each pins its own model and effort in its frontmatter:

| Sub-agent | When used |
|-----------|-----------|
| `implementer-agent` | Groups with no `[expert]` task (default) |
| `implementer-expert-agent` | Groups containing at least one `[expert]` task |
| `code-reviewer` | Final review of all changed files |

## Required Skills

Invoke before starting:
- `/speq-cli` — Spec discovery
- `/speq-writing-guardrails` — Prose style for artifacts and GitHub text

Do not invoke coding skills (`/speq-code-tools`, `/speq-ext-research`, `/speq-code-guardrails`) yourself: the orchestrator never writes code, so that text only dilutes attention in the most expensive session. Sub-agents (`implementer-agent`, `implementer-expert-agent`, `code-reviewer`) invoke their own required skills.

## Orchestrator Role

The main agent acts as **orchestrator**:
- Creates and maintains `tasks.md` for persistence
- Spawns sub-agents for parallel task groups
- Updates task status after sub-agent completion
- Never implements directly — delegates all coding work
- Rotates sub-agents to keep context windows fresh

**Rotation rule:** sub-agents checkpoint after every 2-3 tasks (expert: 1-2). When a sub-agent has completed `max_tasks_per_agent` (default 5) tasks, or returns `ROTATION NEEDED`, read tasks.md for current state, note the completed tasks from the sub-agent's return, and spawn a fresh agent of the SAME type with the remaining tasks. Continue until the group is complete.

**Rotation hand-off:** the outgoing agent leaves a hand-off note at `specs/_plans/<plan-name>/notes/<group-letter>.md`, where `<group-letter>` is the group's letter only — the token before the `:` in its Parallelization-table `Group` entry (group `A: plan-log validation` → `notes/A.md`), never the full group name and never a slug of it (its own duty, per its Early Termination section). Add one line to the fresh agent's brief: `Orientation: read specs/_plans/<plan-name>/notes/<group-letter>.md first`. If the note is absent, omit the line. The note transfers the outgoing agent's mental model — without it, the fresh agent rebuilds the same orientation from cold files. The note lives inside the plan directory, so `/speq-record`'s archive step removes it with the rest of the plan; it is working state, never committed evidence.

## Workflow

### Phase 0: Load Project Hook (orchestrator)

Check for `.speq/implement-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/implement-hook.md". Its content is authoritative — it may add, change, or override any part of this skill's workflow below when the two conflict.
- **Absent:** continue normally, no mention.

Note it (not its full content) as a `Project Hook:` line in every sub-agent brief below — `implementer-agent`, `implementer-expert-agent`, and `code-reviewer` read it themselves from that path when noted.

### Phase 1: Load Plan

**Open-questions gate:** read `specs/_plans/<plan-name>/open-questions.md`. If it exists and is non-empty → **stop** and report that the plan has unresolved open questions pending human answers (resolve with `/speq-plan <plan-name>`, or via PR comments and `/speq-plan-pr <plan-name>` for a headless plan). The human-in-the-loop point lives exactly here — same gate as `speq-implement-pr`'s blocker check.

```
Read: specs/_plans/<plan-name>/plan.md
```

Extract: feature specs, implementation tasks, parallelization groups, verification commands.

### Phase 2: Create Tasks

Decompose the plan into a **Work Breakdown Structure** in `specs/_plans/<plan-name>/tasks.md`.

**Lifecycle guard:** if `tasks.md` already exists and contains a `## PR Lifecycle` section, `speq-implement-pr` pre-created it as its checkpoint (per its `references/checkpoint-protocol.md`). Preserve that section verbatim at the top and write the `## Phase N` sections below it. Never edit `## PR Lifecycle` — its writers are fixed by that protocol. Run standalone, `tasks.md` gets no lifecycle section; create the file as below.

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
- `[expert]` — tagged by `planner-agent` during planning. Routes the task's whole group to `implementer-expert-agent` (Phase 3 routing rule). Preserve the tag through every status transition.
- untagged — a group with only untagged tasks routes to `implementer-agent`. Most tasks are untagged.

If the plan did not tag any tasks but you encounter a task that clearly warrants expert reasoning (e.g. concurrency, cross-file refactor, novel algorithm), you MAY add `[expert]` when materializing tasks.md. Do this sparingly — over-tagging wastes tokens.

Also create runtime tasks:
```
For each task in tasks.md:
  TaskCreate(subject, description, activeForm)
```

### Phase 3: Implement (Orchestrated)

For each parallel group in plan's `## Parallelization`:

1. **Route the whole group by its hardest task** — if any task in the group carries `[expert]`, the whole group goes to `implementer-expert-agent`; otherwise the whole group goes to `implementer-agent`. One agent per group. Never split a group by tag: a group is one knowledge cluster, and two agents in it each rebuild the same mental model (skills, plan, specs, source files) — that duplicated orientation costs more than the model-price difference on the untagged tasks.
2. **Mark started** — Update tasks.md: `[ ]` → `[~]`
3. **Spawn one subagent for the group** — use the matching invocation template below. If the plan's Parallelization table has a `Knowledge` column, copy the group's entry into the brief's `Knowledge:` line — it names the spec deltas and files the agent reads first, replacing a cold search
4. **Await completion** — The sub-agent returns with results or a rotation signal
5. **Handle rotation** — Apply the Rotation rule and Rotation hand-off above: fresh agent of the SAME type, remaining group tasks, orientation line pointing at `notes/<group-letter>.md` (the group's letter only, per the hand-off rule)
6. **Mark completed** — Update tasks.md: `[~]` → `[x]` (preserve `[expert]` tag)
7. **Update TaskTools** — `TaskUpdate(taskId, status: "completed")`
8. **Next group** — Proceed to next parallel group once its dependencies are complete

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

After implementation completes, review all changed files. Code review runs ONCE per implementation — after fix tasks complete, proceed to Phase 5 (its checks verify the fixes); do not respawn `code-reviewer` for a second round.

1. **Collect changed files** — `git diff --name-only <base>` for tracked changes plus `git ls-files --others --exclude-standard` for new files. Implementation work is uncommitted at this point, so diff against the working tree — a commit-range diff (`<base>...HEAD`) would miss all of it.
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
3. **Process findings** — Branch on the two counts in the verdict; if both are 0, go to Phase 5. One agent applies the whole fix pass, routed by its hardest finding — the findings cluster on the files just written, and a second agent there re-orients into the same code and can collide with the first.
   - **standard > 0, expert == 0** — spawn `implementer-agent`:
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
   - **expert > 0** — spawn `implementer-expert-agent` with the same brief shape, naming section `## Expert fixes` — and also `## Standard fixes` when standard > 0. It tags only the tasks derived from `## Expert fixes` with `[expert]`.
4. **Proceed to verification** — Phase 5 verifies all tests pass

### Phase 5: Verification

#### 5a. Automated Checks

Execute commands from plan's `## Verification > Checklist`, redirecting each command's output to a log: `mkdir -p target && <command> > target/speq-<suite>.log 2>&1`, then branch on the exit code. If a report quotes output, quote at most `tail -n 30` of the log — raw build/test output never lands verbatim in the transcript.

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

Generate using `references/verification-template.md`. Structure the report **BLUF (Bottom Line Up Front)**: lead with pass/fail verdict and summary before evidence details. Fill the Verdict table's `Code review` row from Phase 4's verdict line — that row is how the counts survive into a later session's condensed PR comment, which quotes the Verdict table.

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

The two report lines are the run's machine-readable handoff: a headless caller folds the code-review line into its condensed PR comment, and the report path is what `/speq-record` gates on.

## Context Recovery

If context is lost or compacted:

1. Read `specs/_plans/<plan-name>/tasks.md`
2. Identify incomplete tasks (`[ ]` or `[~]`) — scan only `## Phase N` sections, never `## PR Lifecycle` (its unnumbered entries are `speq-implement-pr` checkpoints, not work items)
3. Resume from first incomplete task
4. Continue orchestration workflow

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
| Orchestrator invokes coding skills | Same reason — that text dilutes the most expensive session for work it never does |
| Splitting one group between two agents by tag | The group is one knowledge cluster; each extra agent re-derives the same mental model |
| Proceeding past a non-empty open-questions.md | The plan is blocked on human answers — implement only after they're resolved |
| Dropping the `[expert]` tag on a status flip | The tag must survive `[ ]` → `[~]` → `[x]` |
| Marking `[x]` without a sub-agent completion return | Only verified completions are done |
| A second code-review round | Review runs once; Phase 5 verifies the fixes |
| Skipping the verification report | `/speq-record` gates on it |
