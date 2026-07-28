# Checkpoint Protocol (PR Lifecycle)

The headless pipeline runs as short sessions, not one long session. Each session reads the checkpoint, runs exactly one phase, updates the checkpoint, and stops. Session termination is the point: minimized sub-agent returns only slow transcript growth, and the API re-bills the accumulated transcript on every turn — only ending the session breaks that scaling. `speq-implement-pr` cites this file; if `speq-plan-pr` later gets the same split, it cites this file too instead of restating it.

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

- Markers reuse the tasks.md convention verbatim: `[ ]` pending, `[~]` attempted, `[x]` done.
- Lifecycle entries are deliberately unnumbered (no `<phase>.<index>`) — a second guard, beyond the section heading, against task-dispatch logic treating them as work items.
- `tested-green` encodes the red path: `[ ]` not attempted; `[x]` all suites green; `[~]` attempted and failing, annotated in line — `- [~] tested-green — red: <failed suites> (logs: target/speq-<suite>.log)`. A later green rerun replaces the line with `- [x] tested-green`.
- Only the writers in the table below touch this section. Implementer agents never edit it; `/speq-implement`'s Context Recovery never scans it.

## Writer Table

Every mark is orchestrator-written except `recorded`:

| Mark | Writer | When | Location |
|---|---|---|---|
| `resolved` | `implement-pr` orchestrator | checkout/branch succeeded; pre-create `tasks.md` with the section if absent | `specs/_plans/<plan-name>/tasks.md` |
| `implemented` | orchestrator | `/speq-implement` returned and `verification-report.md` is present | same |
| `version-bumped` | orchestrator | version bump + build done | same |
| `tested-green` | orchestrator | after the suites — `[x]` green, `[~]` red per the marker rule | same |
| `recorded` | `recorder-agent` (`/speq-spec-merge` Finalize, right after the archive `mv`); the orchestrator verifies on `/speq-record` return and writes the mark if absent | archive step | `specs/_recorded/NNN-<plan-name>/tasks.md` |
| `pr-ready` | orchestrator | `ship-ready` succeeded | `specs/_recorded/*-<plan-name>/tasks.md` |

Pre-creation closes the existence gap: `tasks.md` normally appears only in `/speq-implement` Phase 2, but `resolved` must be checkpointed before that. Create the file with only the H1 and the `## PR Lifecycle` section; `/speq-implement` preserves the section verbatim and writes its `## Phase N` sections below it.

## Entry Dispatch

Run this immediately after checkout — the checkpoint lives on the branch:

```
specs/_plans/<plan-name>/tasks.md exists?
├─ Yes → read ## PR Lifecycle
│   ├─ section absent → legacy in-flight plan: prepend the section, mark [x] resolved → Phase A
│   ├─ version-bumped not [x] → Phase A (partial [~] task groups resume via
│   │                            /speq-implement's existing Context Recovery)
│   ├─ version-bumped [x], tested-green [ ] → Phase B
│   └─ tested-green [~] (red) → red path below
├─ No, but specs/_recorded/*-<plan-name>/tasks.md exists →
│   ├─ recorded [x], pr-ready not [x] → Phase C
│   └─ pr-ready [x] → already done: report the PR state, stop
└─ Neither → new run: specs/_plans/<plan-name>/plan.md exists?
    ├─ Yes → pre-create tasks.md (H1 + ## PR Lifecycle, [x] resolved) → Phase A
    └─ No  → report "no such plan" and stop (implement-pr never plans)
```

The `specs/_recorded/*-<plan-name>/` glob matches both canonical `NNN-<plan-name>` and legacy `YYYY-MM-DD-<plan-name>` archive names.

**Red path:** on `[~] tested-green`, re-run the test suites only — assume a human or a prior session fixed the failure out of band. Green → replace the line with `[x] tested-green` and continue Phase B from the record step. Red again → update the annotation, report, stop. Never re-enter `/speq-implement` from the red path; new implementation work is a distinct, explicit invocation.

## Stop Messages

Each non-terminal phase ends with a hard stop. The model's rule is absolute: STOP — do not begin the next phase in this session, even though you could. The human's options below are information for the report, never license to continue on your own:

```
Phase <A|B> complete for <plan-name>.
Checkpoint: <tasks.md path> § PR Lifecycle (<last mark written>).
Interactive: continue in this session by re-prompting, or /clear then re-invoke
/speq-implement-pr <plan-name> for a fresh context.
Headless: the driver starts a fresh session for the next phase.
```

## Durability and Driver Contract

`specs/_recorded/` is gitignored, so the `recorded` and `pr-ready` marks are local-workspace state, never remote state. The driver contract is same-working-directory session restarts: fresh session, same checkout. This constraint is not new — Phase A's working tree must survive until its commit anyway. Fresh-clone drivers are out of scope.

One `claude -p` invocation no longer yields a ready PR. The driver loops: relaunch on each phase-complete report until `pr-ready`, a blocked report, or a red report. Document this loop wherever the driver is configured.
