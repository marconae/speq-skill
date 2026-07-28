---
name: speq-plan-review
description: Adversarial plan-review method and challenge taxonomy — premortem framing, multi-axis challenge taxonomy, severity discipline, and findings output format. Triggered by plan-reviewer.
---

# Adversarial Plan Review

Build the case against approval, not for it.

## Method

**Premortem first.** Before scoring anything, write: "six months from now this plan failed catastrophically — why?" Produce 2-3 concrete failure stories. Route each into the taxonomy below. A failure story with no matching category means the taxonomy misses a category, not that the story does not count.

**No silent pass.** For every axis below, either raise a finding or write one line certifying "no objection — axis checked" with the evidence checked. Never skip an axis.

**Round 2 (if applicable):** first re-check every round-1 BLOCKER against the revised artifacts and the `[plan-review]` entries in `decision-log.md`. Confirm each is resolved, not reworded. Then do a fresh pass for new findings.

**Non-goal:** do not re-litigate decisions the user made in the clarifying interview. Challenge how the plan operationalizes those decisions, not the decisions themselves. If the user said "use approach X" and the plan uses X, check whether X is executed soundly, not whether X was the right call. A deviation the brief notes as authorized by an active project hook is settled the same way; do not raise it as a finding.

## Challenge Taxonomy

Tag every finding. Group findings by axis in the output.

### Intent fidelity: does the plan match what the user asked for?

- `[INTENT_DRIFT]`: the plan reinterprets or substitutes a different problem than the one asked. *Quote the original request; quote the plan line that diverges.*
- `[SCOPE_CREEP]`: work with no traceable user need: gold-plating, speculative flexibility, "while we're here" extras.
- `[SCOPE_REDUCTION]`: part of the ask dropped, deferred, or reframed as out-of-scope without user agreement.

### Feasibility: can this be built as described?

- `[EFFORT_MISESTIMATION]`: a task line hides more work than it states (planning fallacy).
- `[HIDDEN_DEPENDENCY]`: an unmodeled prerequisite, external system, or ordering constraint the plan does not surface.
- `[UNSTATED_ASSUMPTION]`: a load-bearing belief the plan never states or validates.
- `[NFR_IGNORED]`: security, performance, migration/rollback, or concurrency left untouched where the change plainly touches them.

### Requirement quality: are the spec deltas complete and unambiguous?

- `[AMBIGUOUS_REQUIREMENT]`: not testable as written; no concrete pass/fail test can be written from it.
- `[COMPLETENESS_GAP]`: missing edge case, error path, empty/boundary input for a described behavior.
- `[REQUIREMENT_CONFLICT]`: contradicts another delta in this plan, or an existing recorded spec (check via `/speq-cli`).

### Task breakdown: is the WBS correct and appropriately scoped?

- `[TRACEABILITY_GAP]`: a spec delta with no implementing task, or a task that implements nothing in scope.
- `[TASK_GRANULARITY]`: a task too large to verify as one unit, or a parallelization claim two "independent" tasks actually violate.
- `[CLUSTER_INCOHERENCE]`: tasks in one Parallelization group must share a spec delta or a source module, and overlapping `Knowledge` entries across groups are a consolidation signal, not a parallelism opportunity. Flag a group sliced by layer, a group whose tasks share no knowledge, or two groups the plan should have merged or sequenced.

### Design Depth: does the plan manage complexity well? (per `/speq-design-philosophy`)

- `[SHALLOW_DESIGN]`: a planned module/interface is shallow relative to the complexity it should hide.
- `[INFORMATION_LEAKAGE]`: a design decision (format, protocol, temporal split) reflected across multiple planned modules.
- `[TACTICAL_SHORTCUT]`: a tactical shortcut with no scheduled strategic follow-up.
- `[BOUNDARY_VIOLATION]`: planned business logic depends directly on a delivery mechanism, storage engine, or framework.

### Prose quality: does the writing meet `/speq-writing-guardrails`?

- `[PROSE_BLOAT]`: filler, repetition, unneeded hedging or preamble. Violates BLUF/terseness.
- `[PROSE_UNCLEAR]`: a sentence that is ambiguous, jargon-laden, or not understandable on one read.

Prose findings default to **ADVISORY**: style, not correctness. Escalate a `[PROSE_UNCLEAR]` to **BLOCKER** only when the unclear prose makes a requirement non-actionable. Tag that finding `[AMBIGUOUS_REQUIREMENT]` too; it is the same defect from two angles.

## Severity

- **BLOCKER**: violates user intent, or the plan is infeasible/untestable as written. Gates the plan; the orchestrator loops it back to `planner-agent`.
- **ADVISORY**: a real risk, tolerable if the human acknowledges it. Never blocks; surfaced in the orchestrator's final report only.

## Output Format

Write the findings document to `specs/_plans/<plan-name>/review/round-<N>.md` per `references/review-findings-template.md`, creating the `review/` directory if absent. Then return exactly one line and nothing else:

```
PLAN REVIEW round <N>: BLOCKERS: <n>, ADVISORY: <n>, INTENT: <n> — specs/_plans/<plan-name>/review/round-<N>.md
```

`INTENT` counts the Intent Fidelity BLOCKERs alone: a subset of `BLOCKERS`, matching the document's Summary block per the template's rules.

Never return the findings themselves as response text. `planner-agent` reads them from the file.

Every finding needs a location and a concrete `Fix:` imperative; vague objections are not actionable for the revision loop. On round 2, confirm-or-refute each round-1 BLOCKER by name before raising anything new.
