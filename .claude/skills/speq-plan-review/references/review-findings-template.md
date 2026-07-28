# Plan Review Findings Template

`plan-reviewer` writes this document to `specs/_plans/<plan-name>/review/round-<N>.md` and returns only a one-line verdict. Consumers: `planner-agent` (revision mode reads the BLOCKER findings and executes each `Fix:` line), the orchestrator's PR report (Advisory section), and human PR reviewers.

## Rules

1. Emit all six axis sections in taxonomy order — Intent Fidelity, Feasibility, Requirement Quality, Task Breakdown, Design Depth, Prose Quality. An axis with no findings gets exactly one certification line — never omit an axis.
2. Every finding: a `#### [TAG] SEVERITY` heading plus `Location`, `Issue`, `Fix`.
3. `Fix:` is an imperative instruction addressed to `planner-agent` — name the artifact, section, and concrete change. It must be executable without re-reading `Issue`.
4. The Summary block carries three counts: total blockers, total advisories, and `Intent Fidelity blockers` — the number of BLOCKER findings under the Intent Fidelity axis alone. The verdict line's `INTENT:` field is that third count; it must match the Summary exactly.
5. Round 2 only: open with `## Round-1 Blocker Recheck`, listing each round-1 BLOCKER as `Resolved:` or `Not resolved:` (with evidence) before any new findings.
6. Confirm-only round 2 (orchestrator passed `Plan Size: small`): the document holds only `## Summary` and `## Round-1 Blocker Recheck`. Omit every axis section. The verdict's `ADVISORY:` count is always `0` for this document. The orchestrator reads round 1's advisories from `review/round-1.md` instead, since this round ran no fresh pass that could surface or supersede them.

## Skeleton

```markdown
# Plan Review Findings: <plan-name> (round <N>)

## Summary
- Axes checked: 6/6
- Total findings: <M> (Blockers: <X>, Advisory: <Y>)
- Intent Fidelity blockers: <I>

## Round-1 Blocker Recheck   <!-- round 2 only -->
- Resolved: [<TAG>] <finding title> — <evidence>
- Not resolved: [<TAG>] <finding title> — <evidence>

## Intent Fidelity
[no objection — axis checked: <evidence>]

#### [<TAG>] <BLOCKER|ADVISORY>
- Location: <artifact> § <section>
- Issue: <defect, quoting the offending line(s)>
- Fix: <imperative instruction to planner-agent>

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

## Round-1 Blocker Recheck
- Resolved: [<TAG>] <finding title> — <evidence>
- Not resolved: [<TAG>] <finding title> — <evidence>
```

## Verdict Line

After writing the document, return exactly one line and nothing else:

```
PLAN REVIEW round <N>: BLOCKERS: <n>, ADVISORY: <n>, INTENT: <n> — specs/_plans/<plan-name>/review/round-<N>.md
```

## Example

```markdown
## Intent Fidelity

#### [SCOPE_REDUCTION] BLOCKER
- Location: plan.md § Out of Scope
- Issue: the user asked for "retry with backoff on all outbound calls"; plan.md defers backoff to "a follow-up plan" without user agreement
- Fix: Delete the backoff bullet from plan.md § Out of Scope, add a task "implement exponential backoff for outbound HTTP calls" to plan.md § Tasks, and add a DELTA:NEW scenario covering retry exhaustion to the http-client spec delta

## Feasibility
[no objection — axis checked: task 2.3's dependency verified at the pinned version]
```
