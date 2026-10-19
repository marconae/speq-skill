# Plan Review Findings Template

`plan-reviewer` writes this document to `specs/_plans/<plan-name>/review/review.md` and returns only a one-line verdict. Consumers: `planner-agent` (fix pass reads the BLOCKER findings and executes each `Fix:` line), the orchestrator's PR report (Findings section), and human PR reviewers.

## Rules

1. Emit all six axis sections in taxonomy order: Intent Fidelity, Feasibility, Requirement Quality, Task Breakdown, Design Depth, Prose Quality.
2. Every finding: a `#### [TAG] SEVERITY` heading plus `Location`, `Issue`, `Fix`.
3. The Summary block carries three counts: total blockers, total advisories, and `Intent Fidelity blockers`. The verdict line's `INTENT:` field is the third count and must match the Summary exactly.

## Skeleton

```markdown
# Plan Review Findings: <plan-name>

## Summary
- Axes checked: 6/6
- Total findings: <M> (Blockers: <X>, Findings: <Y>)
- Intent Fidelity blockers: <I>

## Intent Fidelity
[no objection — axis checked: <evidence>]

#### [<TAG>] <BLOCKER|FINDINGS>
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

## Example

```markdown
## Intent Fidelity

#### [SCOPE_REDUCTION] BLOCKER
- Location: plan.md § Out of Scope
- Issue: the user asked for "retry with backoff on all outbound calls"; plan.md defers backoff to "a follow-up plan" without user agreement
- Fix: Delete the backoff bullet from plan.md § Out of Scope, add a task "implement exponential backoff for outbound HTTP calls" to plan.md § Tasks, and add a DELTA:NEW scenario covering retry exhaustion to the http-client spec delta

## Feasibility
[no objection — axis checked: task 2.3's dependency verified at the pinned version]

## Requirement Quality

#### [REQUIREMENT_CONFLICT] BLOCKER
- Location: specs/_plans/add-x/cli/record/spec.md § Scenarios
- Issue: this delta's "Reject unknown anchor" scenario contradicts the recorded spec's existing "Unknown anchors are ignored" scenario; both apply to the same input
- Fix: Add a DELTA:CHANGED block replacing the recorded "Unknown anchors are ignored" scenario with wording consistent with this delta's new behavior
```
