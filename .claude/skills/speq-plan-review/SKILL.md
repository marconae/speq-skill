---
name: speq-plan-review
description: "Adversarial plan-review method: challenge taxonomy, severity rules, and findings format. Used by the plan-reviewer agent when reviewing a plan."
---

# Adversarial Plan Review

Build the case against approval, not for it.

## Method

**Ask why this plan may fail.** Before scoring anything, write 2-3 concrete ways this plan may fail. Route each into the taxonomy below. A failure with no matching category means the taxonomy misses a category, not that the failure does not count.

**No silent pass.** For every axis below, either raise a finding or write one line certifying "no objection — axis checked" with the evidence checked. Never skip an axis.

**Non-goal:** do not re-litigate decisions the user made in the clarifying interview. Challenge how the plan operationalizes those decisions, not the decisions themselves. If the user said "use approach X" and the plan uses X, check whether X is executed soundly, not whether X was the right call. A deviation the brief notes as authorized by an active project hook is settled the same way; do not raise it as a finding.

## Challenge Taxonomy

Tag every finding. Group findings by axis in the output.

### Intent fidelity: does the plan match what the user asked for?

- `[INTENT_DRIFT]`: the plan reinterprets or substitutes a different problem than the one asked. *Quote the original request; quote the plan line that diverges.*
- `[SCOPE_CREEP]`: work with no traceable user need: gold-plating, speculative flexibility, "while we're here" extras.
- `[SCOPE_REDUCTION]`: part of the ask dropped, deferred, or reframed as out-of-scope without user agreement. A declared `Follow-up plans` list in `plan.md` from a split is agreed scope, not a reduction.

### Feasibility: can this be built as described?

- `[EFFORT_MISESTIMATION]`: a task line hides more work than it states (planning fallacy).
- `[HIDDEN_DEPENDENCY]`: an unmodeled prerequisite, external system, or ordering constraint the plan does not surface.
- `[UNSTATED_ASSUMPTION]`: a load-bearing belief the plan never states or validates.
- `[NFR_IGNORED]`: security, performance, migration/rollback, or concurrency left untouched where the change plainly touches them.

### Requirement quality: are the spec deltas complete and unambiguous?

- `[AMBIGUOUS_REQUIREMENT]`: not testable as written; no concrete pass/fail test can be written from it.
- `[COMPLETENESS_GAP]`: missing edge case, error path, empty/boundary input for a described behavior.
- `[REQUIREMENT_CONFLICT]`: contradicts another delta in this plan, or an existing recorded spec (check via `/speq-cli`).
- `[IMPLEMENTATION_DETAIL]`: a spec delta's `## Background` or `# Feature <name>` description states a fact that no scenario's GIVEN/WHEN/THEN step in the same spec depends on, a Background runs past 5 one-line bullets, or a Background states behavior that belongs in a scenario. For a delta on an existing feature, the scenarios are those of the target spec after the delta merges; for a NEW feature, they are the delta file's own scenarios. A Background line that only restates a THEN step is redundant, not an implementation detail — don't flag it. Fix: drop the Background line, move it into the scenario step that actually needs it, or turn behavior into a scenario.

### Task breakdown: is the WBS correct and appropriately scoped?

- `[TRACEABILITY_GAP]`: a spec delta with no implementing task, or a task that implements nothing in scope.
- `[TASK_GRANULARITY]`: a task too large to verify as one unit, or a parallelization claim two "independent" tasks actually violate.
- `[CLUSTER_INCOHERENCE]`: tasks in one Parallelization group must share a spec delta or a source module, and overlapping `Knowledge` entries across groups are a consolidation signal, not a parallelism opportunity. Flag a group sliced by layer, a group whose tasks share no knowledge, or two groups the plan should have merged or sequenced.

### Design Depth: does the plan manage complexity well? (per `/speq-design-philosophy`)

`decision-log.md` is the input surface for `[ADR_OVERPROMOTION]`: check every `Promotes to ADR: yes` entry against `/speq-adr-rules`. `decision-log.md`, `plan.md`, and `architecture.md` (the delta, if present) are the input surface for `[ARCHITECTURE_DRIFT]`. The accepted ADRs (the ADR digest from `/speq-cli`, plus any fragment you open) are the input surface for `[ADR_CONFLICT]`.

- `[SHALLOW_DESIGN]`: a planned module/interface is shallow relative to the complexity it should hide.
- `[INFORMATION_COUPLING]`: a design decision (format, protocol, temporal split) reflected across multiple planned modules.
- `[TACTICAL_SHORTCUT]`: a tactical shortcut with no scheduled strategic follow-up.
- `[BOUNDARY_VIOLATION]`: planned business logic depends directly on a delivery mechanism, storage engine, or framework.
- `[ADR_OVERPROMOTION]`: a `Promotes to ADR: yes` entry that fails `/speq-adr-rules`. Flag an entry when any of these holds: Rationale names no rule-2 criterion; Rationale states no ADR search result; the content is on the never-an-ADR list (rule 3); Decision holds implementation detail such as signatures, paths, or flags (rule 6). Fix: set it to `no`, or, if it is a corollary of another promoted decision, fold it into that parent entry's `Consequences` line.
- `[ARCHITECTURE_DRIFT]` (BLOCKER): the plan changes the architecture but the delta does not say so. Flag it when any of these holds: (1) a `Promotes to ADR: yes` entry whose Rationale names rule-2 criterion 1, 2, or 3 has no `Architecture:` line, names a section the delta lacks, or says `no change` without a reason; (2) `plan.md` states a component, boundary, interface, data flow, constraint, or external dependency that the delta lacks; (3) the delta breaks `/speq-plan`'s `references/architecture-delta-template.md`. Fix: add or correct the delta block, move the fact into the delta, or write `Architecture: no change: <reason>`. Content in `plan.md` that only duplicates the delta is FINDINGS.
- `[ADR_CONFLICT]` (BLOCKER): a decision, spec delta, or architecture delta contradicts an accepted ADR without a superseding entry (`Supersedes: <slug>`).  Fix: conform to the ADR, or add a superseding decision-log entry.

### Prose quality: does the writing meet `/speq-writing-guardrails`?

- `[PROSE_BLOAT]`: filler, repetition, unneeded hedging, or preamble. Violates BLUF/terseness.
- `[PROSE_UNCLEAR]`: a sentence that is ambiguous, full of jargon, or not understandable on one read.

Prose findings default to **FINDINGS**: style, not correctness. Escalate a `[PROSE_UNCLEAR]` to **BLOCKER** only when the unclear prose makes a requirement non-actionable. Tag that finding `[AMBIGUOUS_REQUIREMENT]` too; it is the same defect from two angles.

## Severity

- **BLOCKER**: violates user intent, or the plan is infeasible/untestable as written, or breaks a project rule the skill system enforces. Gates the plan; the orchestrator sends it to one `planner-agent` fix pass.
- **FINDINGS**: Non-blocking findings to be acknowledged by the user.

## Output Format

Write the findings document to `specs/_plans/<plan-name>/review/review.md` per `references/review-findings-template.md`, creating the `review/` directory if absent. Then return exactly one line and nothing else:

```
PLAN REVIEW: BLOCKERS: <n>, FINDINGS: <n>, INTENT: <n> — specs/_plans/<plan-name>/review/review.md
```

`INTENT` counts the Intent Fidelity BLOCKERs alone: a subset of `BLOCKERS`, matching the document's Summary block.

Every finding needs a location and a `Fix:` imperative addressed to `planner-agent`. It names the artifact, section, and concrete change, and is executable without re-reading `Issue`. Vague objections are not actionable for the fix pass.
