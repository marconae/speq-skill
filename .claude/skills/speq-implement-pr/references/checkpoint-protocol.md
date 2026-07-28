# Checkpoint Protocol (PR Lifecycle)

The headless pipeline runs end-to-end in one session, but a session can be cut off involuntarily — a usage-limit reset mid-run (it happened in production on PRs #229/#232/#236), a crash, a killed process. The checkpoint exists for that case: each phase writes its mark on completion, so a fresh invocation reads the marks and resumes from the correct phase instead of redoing or skipping work — the same way `/speq-implement`'s Context Recovery reads `tasks.md` to resume. The checkpoint is interruption resilience, not a deliberate session boundary: under normal conditions one invocation runs Phase A through C without stopping. `speq-implement-pr` cites this file; if `speq-plan-pr` later gets the same resilience, it cites this file too instead of restating it.

## Lifecycle Schema

The checkpoint is a `## PR Lifecycle` section at the top of `specs/_plans/<plan-name>/tasks.md` — the file that already survives context loss and resumes production runs. There is no separate state file: a second source of truth can disagree with the first at exactly the crash boundaries checkpointing exists for.

```markdown
# Tasks: <plan-name>

## PR Lifecycle
- [ ] resolved
- [ ] implemented
- [ ] version-bumped
- [ ] tested-green
- [ ] recorded
- [ ] pr-ready
```

Rules:

- Markers reuse the tasks.md convention for pending/done — `[ ]` pending, `[x]` done — but `tested-green` uses `[!]` for its red state, not `[~]`: `[~]` already means "in progress" for `## Phase N` task entries elsewhere in this same file, and reusing it here would make one marker mean two different things depending on which section you're reading.
- Lifecycle entries are deliberately unnumbered (no `<phase>.<index>`) — a second guard, beyond the section heading, against task-dispatch logic treating them as work items.
- `tested-green` encodes the red path: `[ ]` not attempted; `[x]` all suites green; `[!]` attempted and failing, annotated in line — `- [!] tested-green — red: <failed suites> (logs: target/speq-<suite>.log)`. A later green rerun replaces the line with `- [x] tested-green`.
- Only the writers in the table below touch this section. Implementer agents never edit it; `/speq-implement`'s Context Recovery never scans it.

## Writer Table

Every mark is orchestrator-written except `recorded`:

| Mark | Writer | When | Location |
|---|---|---|---|
| `resolved` | `implement-pr` orchestrator | checkout/branch succeeded; pre-create `tasks.md` with the section if absent | `specs/_plans/<plan-name>/tasks.md` |
| `implemented` | orchestrator | `/speq-implement` returned and `verification-report.md` is present | same |
| `version-bumped` | orchestrator | version bump + build done | same |
| `tested-green` | orchestrator | after the suites — `[x]` green, `[!]` red per the marker rule | same |
| `recorded` | `recorder-agent` (`/speq-spec-merge` Finalize, right after the archive `mv`); the orchestrator verifies on `/speq-record` return and writes the mark if absent | archive step | `specs/_recorded/NNN-<plan-name>/tasks.md` |
| `pr-ready` | orchestrator | `ship-ready` succeeded | `specs/_recorded/*-<plan-name>/tasks.md` |

Pre-creation closes the existence gap: `tasks.md` normally appears only in `/speq-implement` Phase 2, but `resolved` must be checkpointed before that. Create the file with only the H1 and the `## PR Lifecycle` section; `/speq-implement` preserves the section verbatim and writes its `## Phase N` sections below it.

## Entry Dispatch

Run this immediately after checkout — the checkpoint lives on the branch. The dispatch selects where execution enters; the run then continues through the remaining phases in the same session:

```
specs/_plans/<plan-name>/tasks.md exists?
├─ Yes → read ## PR Lifecycle
│   ├─ section absent → legacy in-flight plan: prepend the section, mark [x] resolved → Phase A
│   ├─ version-bumped not [x] → Phase A (partial [~] task groups resume via
│   │                            /speq-implement's existing Context Recovery)
│   ├─ version-bumped [x], tested-green [ ] → Phase B
│   └─ tested-green [!] (red) → red path below
├─ No, but specs/_recorded/*-<plan-name>/tasks.md exists →
│   ├─ recorded [x], pr-ready not [x] → Phase C
│   └─ pr-ready [x] → already done: report the PR state, stop
└─ Neither → new run: specs/_plans/<plan-name>/plan.md exists?
    ├─ Yes → pre-create tasks.md (H1 + ## PR Lifecycle, [x] resolved) → Phase A
    └─ No  → report "no such plan" and stop (implement-pr never plans)
```

The `specs/_recorded/*-<plan-name>/` glob matches both canonical `NNN-<plan-name>` and legacy `YYYY-MM-DD-<plan-name>` archive names.

**Red path:** on `[!] tested-green`, re-run the test suites only — assume a human or a prior session fixed the failure out of band. Green → replace the line with `[x] tested-green` and continue Phase B from the record step. Red again → update the annotation, report, stop. Never re-enter `/speq-implement` from the red path; new implementation work is a distinct, explicit invocation.

## Gate and Resume Messages

The pipeline halts only at genuine gates, never between phases. Two gates exist: a non-empty `open-questions.md` at Phase A entry (human answers required), and a red test suite in Phase B (`[!] tested-green` — a human fixes the code; blind retries are out of scope). A gate report states the gate, the checkpoint state, and the resume path:

```
Blocked: <open questions | red suites: <names>> for <plan-name>.
Checkpoint: <tasks.md path> § PR Lifecycle (<last mark written>).
Re-invoke /speq-implement-pr <plan-name> once <the questions are answered | the failure is fixed>.
```

When a fresh session's entry dispatch finds marks already set — a prior run was cut off involuntarily — report on entry which marks are `[x]` and which phase this run resumes into, then continue normally to the end.

## Durability and Driver Contract

`specs/_recorded/` is gitignored, so the `recorded` and `pr-ready` marks are local-workspace state, never remote state. Resumption assumes same-working-directory restarts: fresh session, same checkout. This constraint is not new — Phase A's working tree must survive until its commit anyway. Fresh-clone drivers are out of scope.

One `claude -p` invocation normally yields a ready PR — the driver needs no phase loop. If a run is cut off involuntarily, a fresh invocation (headless relaunch, or interactive re-prompt) resumes correctly from the checkpoint in `tasks.md`.
