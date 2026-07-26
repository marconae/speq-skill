---
name: speq-plan-review
description: Adversarial plan-review method and challenge taxonomy — premortem framing, multi-axis challenge taxonomy, severity discipline, and findings output format. Triggered by plan-reviewer.
---

# Adversarial Plan Review

Build the case against approval, not for it. A plan that survives review should survive because it's actually sound, not because the reviewer was polite.

## Method

**Premortem first.** Before scoring anything, write: "six months from now this plan failed catastrophically — why?" Produce 2-3 concrete failure stories. Route each one into the taxonomy below; a failure story with no matching category means the taxonomy is missing a category, not that it doesn't count.

**No silent pass.** For every axis below, either raise a finding or write one line certifying "no objection — axis checked" with the evidence checked. Never skip an axis silently — that's how a defect slides through unreviewed.

**Round 2 (if applicable):** re-check every round-1 BLOCKER against the reviser's diff first — confirm each is actually resolved, not reworded — before doing a fresh pass for new ones.

**Non-goal:** do not re-litigate decisions the user already made in the clarifying interview. Challenge how the plan *operationalizes* those decisions, not the decisions themselves. If the user said "use approach X" and the plan uses approach X, that's settled — check whether X is executed soundly, not whether X was the right call. A deviation the brief notes as authorized by an active project hook is settled the same way — don't raise it as a finding.

## Challenge Taxonomy

Tag every finding, group findings by axis in the output.

### Intent fidelity — does the plan match what the user actually asked for?

- `[INTENT_DRIFT]` — the plan quietly reinterprets or substitutes a different problem than the one asked. *Quote the original request; quote the plan line that diverges.*
- `[SCOPE_CREEP]` — work with no traceable user need: gold-plating, speculative flexibility, "while we're here" extras.
- `[SCOPE_REDUCTION]` — part of the ask silently dropped, deferred, or reframed as out-of-scope without the user agreeing.

### Feasibility — can this actually be built as described?

- `[EFFORT_MISESTIMATION]` — a task line hides more work than it states (planning fallacy); which task is secretly three tasks?
- `[HIDDEN_DEPENDENCY]` — an unmodeled prerequisite, external system, or ordering constraint the plan doesn't surface.
- `[UNSTATED_ASSUMPTION]` — a load-bearing belief the plan never states or validates; what happens if it's false?
- `[NFR_IGNORED]` — security, performance, migration/rollback, or concurrency left untouched where the change plainly touches them.

### Requirement quality — are the spec deltas complete and unambiguous?

- `[AMBIGUOUS_REQUIREMENT]` — not testable as written; you can't write a concrete pass/fail test from it right now.
- `[COMPLETENESS_GAP]` — missing edge case, error path, empty/boundary input for a described behavior.
- `[REQUIREMENT_CONFLICT]` — contradicts another delta in this plan, or an existing recorded spec (check via `/speq-cli`).

### Task breakdown — is the WBS correct and appropriately scoped?

- `[TRACEABILITY_GAP]` — a spec delta with no implementing task, or a task that implements nothing in scope.
- `[TASK_GRANULARITY]` — a task too large to verify as one unit, or a parallelization claim two "independent" tasks actually violate.

### Design Depth — does the plan manage complexity well? (per `/speq-design-philosophy`)

- `[SHALLOW_DESIGN]` — a planned module/interface is shallow relative to the complexity it should hide.
- `[INFORMATION_LEAKAGE]` — a design decision (format, protocol, temporal split) reflected across multiple planned modules.
- `[TACTICAL_SHORTCUT]` — the plan takes a tactical shortcut with no scheduled strategic follow-up.
- `[BOUNDARY_VIOLATION]` — planned business logic depends directly on a delivery mechanism, storage engine, or framework.

### Prose quality — does the writing meet `/speq-writing-guardrails`?

- `[PROSE_BLOAT]` — prose not required to make the point: filler, repetition, unneeded hedging or preamble. Violates BLUF/terseness.
- `[PROSE_UNCLEAR]` — a sentence that's ambiguous, jargon-laden, or not understandable on one read.

Prose findings default to **ADVISORY** — they're style, not correctness. Escalate a `[PROSE_UNCLEAR]` to **BLOCKER** only when the unclear prose makes a requirement non-actionable; tag that finding `[AMBIGUOUS_REQUIREMENT]` too, since it's the same defect from two angles.

## Severity

- **BLOCKER** — violates user intent, or the plan is infeasible/untestable as written. Gates the plan; the orchestrator loops it back to `planner-agent`.
- **ADVISORY** — a real risk, tolerable if the human acknowledges it. Never blocks; surfaced in the orchestrator's final report only.

## Output Format

```markdown
# Plan Review Findings: <plan-name> (round <N>)

## Summary
- Axes checked: 6/6
- Total findings: M (Blockers: X, Advisory: Y)

## Intent Fidelity
[no objection — axis checked: <evidence>]
OR
#### [INTENT_DRIFT] BLOCKER
- Location: plan.md § Summary
- Issue: <what the user asked vs what the plan does>
- Fix: <concrete correction>

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

Every finding needs a location and a concrete fix suggestion — vague objections aren't actionable for the revision loop. On round 2, confirm-or-refute each round-1 BLOCKER by name before raising anything new.
