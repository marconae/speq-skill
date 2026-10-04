# Plan Review Findings Template

`plan-reviewer` writes this document to `specs/_plans/<plan-name>/review/round-<N>.md` and returns only a one-line verdict. Consumers: `planner-agent` (revision mode reads the BLOCKER findings and executes each `Fix:` line), the orchestrator's PR report (Advisory section), and human PR reviewers.

## Rules

1. Emit all six axis sections in taxonomy order — Intent Fidelity, Feasibility, Requirement Quality, Task Breakdown, Design Depth, Prose Quality. An axis with no findings gets exactly one certification line — never omit an axis.
2. Every finding: a `#### [TAG] SEVERITY` heading plus `Location`, `Issue`, `Fix`. A BLOCKER also carries `Escalation: HUMAN | MECHANICAL` (per `/speq-plan-review`'s Escalation Class section). ADVISORY findings never carry it — they never escalate.
3. `Fix:` is an imperative instruction addressed to `planner-agent` — name the artifact, section, and concrete change. It must be executable without re-reading `Issue`.
4. The Summary block carries four counts: total blockers, total advisories, `Intent Fidelity blockers`, and `Human-escalation blockers` — the number of BLOCKER findings tagged `Escalation: HUMAN` (Intent Fidelity blockers included). The verdict line's `INTENT:` and `HUMAN:` fields are the third and fourth counts; both must match the Summary exactly.
5. Round 2 only: open with `## Round-1 Blocker Recheck`, listing each round-1 BLOCKER as `Resolved:` or `Not resolved:` (with evidence) before any new findings. A `Not resolved:` entry restates that finding's `Escalation:` and `Fix:` verbatim from round 1 (refined only if round 2 learned something that changes the concrete instruction) — round-2.md must be self-sufficient for the orchestrator and `planner-agent`; neither re-reads round-1.md for this.
6. Confirm-only round 2 (orchestrator passed `Plan Size: small`): the document holds only `## Summary` and `## Round-1 Blocker Recheck`. Omit every axis section. The Summary still carries `Human-escalation blockers not resolved:`, matching the verdict's `HUMAN:` field.

## Skeleton

```markdown
# Plan Review Findings: <plan-name> (round <N>)

## Summary
- Axes checked: 6/6
- Total findings: <M> (Blockers: <X>, Advisory: <Y>)
- Intent Fidelity blockers: <I>
- Human-escalation blockers: <H>

## Round-1 Blocker Recheck   <!-- round 2 only -->
- Resolved: [<TAG>] <finding title> — <evidence>
- Not resolved: [<TAG>] <finding title> — <evidence>
  - Escalation: <HUMAN|MECHANICAL>   <!-- carried from round 1, refined only if round 2 changes the instruction -->
  - Fix: <imperative instruction to planner-agent>   <!-- carried from round 1, refined only if round 2 changes the instruction -->

## Intent Fidelity
[no objection — axis checked: <evidence>]

#### [<TAG>] <BLOCKER|ADVISORY>
- Location: <artifact> § <section>
- Issue: <defect, quoting the offending line(s)>
- Fix: <imperative instruction to planner-agent>
- Escalation: <HUMAN|MECHANICAL>   <!-- BLOCKER only -->

## Feasibility
...

## Requirement Quality
...

## Task Breakdown
...

## Design Depth
...

## Prose Quality
...
```

## Confirm-Only Skeleton (round 2, `Plan Size: small` only)

```markdown
# Plan Review Findings: <plan-name> (round 2, confirm-only)

## Summary
- Mode: confirm-only (Plan Size: small)
- Round-1 blockers: <T> total — <R> resolved, <U> not resolved
- Intent Fidelity blockers not resolved: <I>
- Human-escalation blockers not resolved: <H>

## Round-1 Blocker Recheck
- Resolved: [<TAG>] <finding title> — <evidence>
- Not resolved: [<TAG>] <finding title> — <evidence>
  - Escalation: <HUMAN|MECHANICAL>   <!-- carried from round 1 -->
  - Fix: <imperative instruction to planner-agent>   <!-- carried from round 1 -->
```

## Example

```markdown
## Intent Fidelity

#### [SCOPE_REDUCTION] BLOCKER
- Location: plan.md § Out of Scope
- Issue: the user asked for "retry with backoff on all outbound calls"; plan.md defers backoff to "a follow-up plan" without user agreement
- Fix: Delete the backoff bullet from plan.md § Out of Scope, add a task "implement exponential backoff for outbound HTTP calls" to plan.md § Tasks, and add a DELTA:NEW scenario covering retry exhaustion to the http-client spec delta
- Escalation: HUMAN — a dropped ask needs the requester's agreement, not the reviewer's

## Feasibility
[no objection — axis checked: task 2.3's dependency verified at the pinned version]

## Requirement Quality

#### [REQUIREMENT_CONFLICT] BLOCKER
- Location: specs/_plans/add-x/cli/record/spec.md § Scenarios
- Issue: this delta's "Reject unknown anchor" scenario contradicts the recorded spec's existing "Unknown anchors are ignored" scenario; both apply to the same input
- Fix: Add a DELTA:CHANGED block replacing the recorded "Unknown anchors are ignored" scenario with wording consistent with this delta's new behavior
- Escalation: MECHANICAL — resolved by reading the two conflicting scenarios; no judgment call needed
```
