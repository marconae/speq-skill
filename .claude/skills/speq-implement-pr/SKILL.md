---
name: speq-implement-pr
description: "Builds a planned feature on its feat/plan-name branch as a follow-up to /speq-plan-pr: implements, bumps the version, runs the real test suites, records the specs if green, comments the verification, and marks the PR ready. Run again to resume. Arg: plan name, PR number, or branch name."
model: sonnet
---

# Spec Implementer, PR flow (Orchestrator)

You are a thin orchestrator on top of `speq-implement`. Drive a plan on its existing `feat/<plan-name>` branch to a ready PR through the existing skills, run unchanged:
- `/speq-implement` builds and verifies. `/speq-record` merges the specs. You run every git/`gh` action yourself, per `/speq-git-operations`.
- Record only after the project's real test suites are green.
- Stop on the first failed step and report it. Running the skill again resumes the work.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli`: spec discovery, to resolve plan names
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text
- `/speq-git-operations`: the git/gh operation-to-command mapping, safety rules, and return formats. You run every operation directly

`speq-implement` and `speq-record` invoke their own required skills.

## Workflow

### 0. Load Project Hook

Check for `.speq/implement-pr-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/implement-pr-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue without mention.

### 1. Resolve Target

`$1` empty → ask the user which plan. Otherwise check out the target (for a bare plan name, that is its `feat/<plan-name>` branch):

```
Run — operation: checkout (per /speq-git-operations)
  target: <$1: a plan name (→ feat/<plan-name>), PR number, or branch name>
```

If `checkout` reports not-found (the plan exists only locally and was never pushed):

```
Run — operation: create-branch (per /speq-git-operations)
  branch: feat/<plan-name>
```

If `specs/_recorded/*-<plan-name>/` exists and `specs/_plans/<plan-name>/` does not, the plan is already recorded: skip to step 6.

### 2. Implement

If `verification-report.md` exists in the plan directory and every task in `tasks.md` is `[x]`, skip this step.

Otherwise invoke `/speq-implement <plan-name>` and let it run to completion. If `tasks.md` exists, it resumes from the first incomplete task.

### 3. Bump Version

Skip this step if the version file already differs from the default branch.

Read the tech stack in `specs/mission.md` and bump the version in that stack's version file (for example `Cargo.toml` for Rust, `package.json` for Node, `pyproject.toml` for Python). Apply the conventional next version per Conventional Commits: `feat` → minor, a purely `fix` plan → patch, a breaking change → major. Do not take the version from the plan's spec deltas. Run a build to keep the lockfile in sync, redirecting its output to a log: `mkdir -p target && <build-command> > target/speq-build.log 2>&1`. Branch on the exit code. If a report quotes output, quote at most `tail -n 30` of the log.

### 4. Commit Evidence

Commit and push now, so the evidence artifacts reach git history before `/speq-record` archives the plan out of tracked space, and so the branch survives workspace loss. Derive `<type>(<scope>)` per `/speq-plan-pr`'s PR-title derivation:

```
Run — operation: commit (per /speq-git-operations)
  paths: implementation files, version bump, and the whole specs/_plans/<plan-name>/
         directory, not an itemized subset, except specs/_plans/<plan-name>/notes/
         (local working state, never committed)
  message: <type>(<scope>): implement <plan-name>

Run — operation: push (per /speq-git-operations)
```

### 5. Test and Record

1. **Run the suites.** Run the project's real test suites (per `specs/mission.md § Commands`, typically an integration suite and an end-to-end suite), redirecting each suite's output to a log: `mkdir -p target && <suite-command> > target/speq-<suite>.log 2>&1`. Judge green or red by exit code. If any suite is red, report the failed suites with at most `tail -n 30` of the relevant log, **stop**, and leave the plan unrecorded. After the user fixes the code, run this skill again.
2. **Record.** Invoke `/speq-record <plan-name>`. If it asks its library-threshold split question, answer yes (split). Recording accepts the ADR candidates on the plan PR. Your run is the user's go-ahead, so do not stop for an ADR question. If it returns `Recording failed: <reason>` (for example a stale architecture delta base), **stop** and report it. After the user fixes the delta or the plan, run this skill again.
3. Note the `Archive:` path `/speq-record` returns.

### 6. Ship

1. **Ship.** One composite call. Step 4 already committed the implementation and evidence, so this commit covers only what `/speq-record` produced: the merge results and the archive's removal of the plan directory. Derive `<type>(<scope>)` and `<slug>` per `/speq-plan-pr`'s PR-title derivation. Do not rewrite the PR body `speq-plan-pr` set. Use `Implements <plan-name>.` as the body, which applies only if no PR exists yet:

   ```
   Run — operation: ship-ready (per /speq-git-operations)
     paths: permanent-spec merges (specs/<domain>/...), specs/architecture.md,
            specs/_decision/ additions, deletion of specs/_plans/<plan-name>/
     message: <type>(<scope>): record <plan-name>
     title: <type>(<scope>): <slug>
     body: Implements <plan-name>.
   ```

   `ship-ready` reuses the draft PR `speq-plan-pr` opened (or opens one if the plan was only implemented locally) and marks it ready. A commit with nothing to commit is a no-op, so running this step again is safe.

2. **Comment.** Post the verification summary. Build it from `<archive-path>/verification-report.md`, where `<archive-path>` is the `Archive:` path from step 5 or, when step 1 skipped ahead, the match of `specs/_recorded/*-<plan-name>/`. Compose it per `/speq-writing-guardrails` and `/speq-implement`'s `references/implementation-summary-template.md`:

   ```
   Run — operation: comment-pr (per /speq-git-operations)
     body: <the composed verification summary>
   ```

3. Report the finished PR and leave it for the user's review. Never merge it.

## Spec Hierarchy (reference)

```
specs/
├── <domain>/<feature>/spec.md            # Permanent (after record)
├── architecture.md                       # Permanent, changed by the recorder applying the plan's architecture delta
├── _plans/<plan-name>/                   # Active until recorded: committed whole at step 4 except notes/
│   ├── architecture.md                   # Architecture delta, present only when the plan changes the architecture
│   ├── tasks.md                          # Created by speq-implement
│   ├── review-findings.md                # Created by code-reviewer
│   ├── review/review.md                  # Created by plan-reviewer
│   ├── notes/<group-letter>.md           # Rotation hand-off notes, created by implementer agents; local, never committed
│   └── verification-report.md            # Created by speq-implement
└── _recorded/NNN-<plan-name>/            # Archived by speq-record (gitignored by default)
```

## Work Split (reference)

| Step | Performed by |
|------|--------------|
| Target resolution, version bump, coordination | This skill (pins Sonnet) |
| Task breakdown, coding, review | `speq-implement` |
| Spec merge and archive | `speq-record` |
| Branch, commit, push, PR create/update | This skill, directly, per `/speq-git-operations` |
