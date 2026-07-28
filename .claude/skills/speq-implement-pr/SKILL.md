---
name: speq-implement-pr
description: "Headless follow-up to /speq-plan-pr. Continues a plan on its feat/plan-name branch and runs it end-to-end: implements via /speq-implement, bumps the version, commits and pushes, runs the real test suites, records only if green, then opens/updates the PR and marks it ready. Resumes from the PR Lifecycle checkpoint in the plan's tasks.md if a prior run was cut off mid-flight. Arg: plan name, PR number, or branch name."
model: sonnet
---

# Spec Implementer, headless (Orchestrator)

You are a thin orchestrator on top of `speq-implement`. Run the pipeline end-to-end in one session:
- Continue a plan on its existing `feat/<plan-name>` branch and drive it to a ready PR through the existing skills, run unchanged.
- Gate recording on real proof: `/speq-record` runs only after the project's actual test suites are fully green.
- Survive involuntary interruption: write a checkpoint mark after each phase, so a fresh invocation resumes from the correct phase if this session is cut off (usage-limit reset, crash). The checkpoint is crash recovery, never a reason to stop deliberately.

The checkpoint schema, writer table, entry-dispatch table, red path, gate/resume messages, and driver contract live in `references/checkpoint-protocol.md`. Read it before step 2. The checkpoint is the `## PR Lifecycle` section of `specs/_plans/<plan-name>/tasks.md`. You write every lifecycle mark except `recorded` (written by `recorder-agent`, verified by you).

Three phases, run consecutively in this session:

| Phase | Work | Then |
|-------|------|------|
| A | blocker check → `/speq-implement` → version bump + build → commit + push evidence | continue to B |
| B | real test suites → `/speq-record` (green only) | continue to C |
| C | `ship-ready` → verification comment → `pr-ready` mark | final report (terminal) |

Workflow rules:
- Delegate implementation to `/speq-implement` and spec merge to `/speq-record`. Run every git/`gh` action yourself, per `/speq-git-operations`. Your own work: resolve the branch, gate, bump the version, write checkpoint marks, sequence those calls, and execute the git/gh operations.
- Advance only when the current step succeeds. Halt and report on the first failed or blocked step.
- Reuse the one `feat/<plan-name>` branch and PR that `speq-plan-pr` created.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli`: spec discovery, to resolve plan names
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text
- `/speq-git-operations`: the git/gh operation-to-command mapping, safety rules, and return formats — you run every operation directly

`speq-implement` and `speq-record` invoke their own required skills.

## Workflow

### 0. Load Project Hook (orchestrator)

Check for `.speq/implement-pr-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/implement-pr-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue, no mention.

### 1. Resolve Target + Branch

`$1` empty → ask the caller which plan. Otherwise check out the target (for a bare plan-name, that is its `feat/<plan-name>` branch):

```
Run — operation: checkout (per /speq-git-operations)
  target: <$1 — a plan-name (→ feat/<plan-name>), PR number, or branch name>
```

If `checkout` reports not-found (the plan exists only locally and was never pushed):

```
Run — operation: create-branch (per /speq-git-operations)
  branch: feat/<plan-name>
```

### 2. Checkpoint Dispatch (orchestrator)

Read the checkpoint and enter at the phase the entry-dispatch table in `references/checkpoint-protocol.md` selects. When the table calls for it, pre-create `specs/_plans/<plan-name>/tasks.md` with only the H1 and the `## PR Lifecycle` section, and mark `[x] resolved`. The dispatch decides where execution enters, not where it stops: from the entry phase, run each remaining phase in order in this same session. If dispatch finds marks already set (a prior run was cut off), report which marks are `[x]` and which phase this run resumes into, then continue normally.

### Phase A: Implement + Commit

**A1. Blocker check**: read `specs/_plans/<plan-name>/open-questions.md`. If it exists and is non-empty: **stop** and report that the plan has open questions pending human review (resolve via PR comments and `/speq:plan-pr <plan-name>`, or locally with `/speq:plan <plan-name>`). This is the human-in-the-loop point.

**A2. Implement**: invoke `/speq-implement <plan-name>` and let it run to completion, unchanged, reused as-is. It fills `tasks.md` below the lifecycle section, spawns the implementer agents, runs `code-reviewer`, and produces `verification-report.md`. When it returns and `verification-report.md` is present, mark `[x] implemented`.

**A3. Bump version**: bump the workspace version per the plan's `workspace/version` spec delta if it specifies one. Otherwise apply the conventional next version per Conventional Commits semantics (`feat` → minor bump; a purely `fix` plan → patch). Run a build to keep the lockfile in sync, redirecting its output to a log: `mkdir -p target && <build-command> > target/speq-build.log 2>&1`. Branch on the exit code. If a report quotes output, quote at most `tail -n 30` of the log. Then mark `[x] version-bumped`.

**A4. Commit evidence**: commit and push now, so the evidence artifacts reach git history before `/speq-record`'s archive `mv` moves the plan directory out of tracked space, and so the branch survives workspace loss:

```
Run — operation: commit (per /speq-git-operations)
  paths: implementation files, version bump, specs/_plans/<plan-name>/
         (the whole plan directory — not an itemized subset, so new
         artifacts ride along automatically — except
         specs/_plans/<plan-name>/notes/planning.md, which stays out of
         every commit)
  message: <type>(<scope>): implement <plan-name>    # type + scope per speq-plan-pr's PR-title derivation rule

Run — operation: push (per /speq-git-operations)
```

**A5. Continue**: proceed directly into Phase B in this same session.

### Phase B: Test + Record

**B1. Run suites**: run the project's real test suites (per `specs/mission.md § Commands`, typically an integration suite and an end-to-end suite), redirecting each suite's output to a log: `mkdir -p target && <suite-command> > target/speq-<suite>.log 2>&1`. Judge green/red by exit code. When reporting failures, quote at most `tail -n 30` of the relevant log.
- **All green** → mark `[x] tested-green`, continue with B2.
- **Any suite red** → mark `- [!] tested-green — red: <failed suites> (logs: target/speq-<suite>.log)`, report the failures, and **stop**. Leave the plan unrecorded.
- On red-path re-entry (`[!]` at dispatch), re-run the suites only, per the protocol. Never re-enter `/speq-implement` from here.

**B2. Record**: invoke `/speq-record <plan-name>`. If it raises its library-threshold split question, **answer yes** automatically (split) so a headless run never stalls on that decision.

**B3. Verify the recorded mark**: parse the `Archive:` path from `/speq-record`'s return. Check that `<archive-path>/tasks.md` has `- [x] recorded`. Write the mark yourself if it is absent (covers a stale `recorder-agent`).

**B4. Continue**: proceed directly into Phase C in this same session.

### Phase C: Ship

**C1. Ship**: one composite call. Phase A already committed the implementation and evidence artifacts, so this commit covers only what `/speq-record` produced: the merge results and the archive's removal of the plan directory:

```
Run — operation: ship-ready (per /speq-git-operations)
  paths: permanent-spec merges (specs/<domain>/...), specs/_decision/ additions,
         deletion of specs/_plans/<plan-name>/
  message: <type>(<scope>): record <plan-name>    # type + scope per speq-plan-pr's PR-title derivation rule
  title: <type>(<scope>): <slug>    # same derivation rule as speq-plan-pr
  body: summary of the implementation diff, both test-suite results (integration + e2e), and the /speq:record outcome
```

`ship-ready`'s create-pr step returns the draft PR `speq-plan-pr` opened (or opens one if the plan was only implemented locally), and its ready-pr step marks it ready.

**C2. Comment**: post the verification summary. Compose the comment body per `speq-writing-guardrails`' PR-facing content rule before calling `comment-pr`:

```
Run — operation: comment-pr (per /speq-git-operations)
  body: condensed verification summary — the Verdict table and Notes from
        <archive-path>/verification-report.md (the specs/_recorded/NNN-<plan-name>
        path /speq-record reported), plus "Full evidence:
        specs/_plans/<plan-name>/verification-report.md (committed in this
        branch's implementation commit)".
        Do not duplicate the Tool Evidence / Scenario Coverage tables —
        they are already in the branch's history.
```

**C3. Finish**: mark `[x] pr-ready` at `specs/_recorded/*-<plan-name>/tasks.md`, then report the finished PR and leave it for human review. Re-entry after a crash between C1 and C3 is safe: `ship-ready`'s commit no-ops on nothing-to-commit, and its create-pr/ready-pr steps reuse the existing PR.

## Spec Hierarchy (reference)

```
specs/
├── <domain>/<feature>/spec.md            # Permanent (after record)
├── _plans/<plan-name>/                   # Active until recorded — committed whole at Phase A, not as an itemized subset
│   ├── tasks.md                          # Pre-created here (§ PR Lifecycle checkpoint); WBS filled by speq-implement
│   ├── review-findings.md                # Created by code-reviewer
│   ├── review/round-N.md                 # Created by plan-reviewer
│   ├── notes/<group-letter>.md           # Rotation hand-off notes, created by implementer agents
│   └── verification-report.md            # Created by speq-implement
└── _recorded/NNN-<plan-name>/            # Archived by speq-record (gitignored by default)
```

## Work Split (reference)

| Step | Performed by | Why |
|------|--------------|-----|
| Target resolution, gating, checkpoint marks, coordination | This skill (pins Sonnet) | Tool-call heavy, reasoning light |
| Task breakdown, coding, review | `speq-implement` (unchanged) | Already the right split |
| Spec merge, archive, `recorded` mark | `speq-record` (unchanged) | Already the right split |
| Branch, commit, push, PR create/update | This skill, directly, per `/speq-git-operations` | No separate agent hop — you already have direct git/gh tool access |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Treating `## PR Lifecycle` entries as work items | They are checkpoints; they are unnumbered exactly so task dispatch skips them |
| Letting a sub-agent write lifecycle marks | Marks are orchestrator-written, except `recorded` (recorder-agent, per the writer table) |
| Proceeding past a non-empty open-questions.md | The human-in-the-loop gate lives at A1 |
| Recording with any suite red | `/speq-record` runs only on fully green suites |
| Skipping the A4 commit | Evidence artifacts silently never reach git history once `/speq-record`'s archive `mv` moves them out of tracked space |
| Re-entering `/speq-implement` on the red path | The red path re-runs suites only; new implementation work is an explicit, separate invocation |
| Spawning a sub-agent for git/gh work | No agent hop needed — you already have direct tool access and composed the content; a spawn only adds latency |
| Merging the PR | The pipeline ends at a ready PR; a human merges |
