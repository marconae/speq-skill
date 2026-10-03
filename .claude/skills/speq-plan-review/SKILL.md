---
name: speq-plan-review
description: Adversarial plan-review method and challenge taxonomy — premortem framing, multi-axis challenge taxonomy, severity discipline, and findings output format. Triggered by plan-reviewer.
---

# Adversarial Plan Review

Build the case against approval, not for it.

## Method

**Premortem first.** Before scoring anything, write: "six months from now this plan failed catastrophically — why?" Produce 2-3 concrete failure stories. Route each into the taxonomy below. A failure story with no matching category means the taxonomy misses a category, not that the story does not count.

**No silent pass.** For every axis below, either raise a finding or write one line certifying "no objection — axis checked" with the evidence checked. Never skip an axis.

**Round 2 (if applicable):** first re-check every round-1 BLOCKER against the revised artifacts and the `[plan-review]` entries in `decision-log.md`. Confirm each is resolved, not reworded, listing each as `Resolved:` or `Not resolved:` per the output template.

- **`Plan Size: full`** (the orchestrator's default when it omits the field): then do a fresh pass for new findings, across all six axes, same as round 1.
- **`Plan Size: small`**: skip the fresh pass. The findings document holds only `## Summary` and `## Round-1 Blocker Recheck` — no axis sections. The orchestrator classifies the plan as `small` from artifacts already on disk (`fix`-verb plan name, empty `## Design Decisions` in `decision-log.md`, no `architecture.md` in the plan directory) and passes it as an explicit `Plan Size:` field in the round-2 respawn prompt.

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
- `[IMPLEMENTATION_LEAKAGE]`: a spec delta's `## Background` or `# Feature <name>` description states a fact that no scenario's GIVEN/WHEN/THEN step in the same spec depends on. For a delta on an existing feature, the scenarios are those of the target spec after the delta merges; for a NEW feature, they are the delta file's own scenarios. A Background line that only restates a THEN step is redundant, not leakage — don't flag it. Fix: drop the Background line, or move it into the scenario step that actually needs it.

### Task breakdown: is the WBS correct and appropriately scoped?

- `[TRACEABILITY_GAP]`: a spec delta with no implementing task, or a task that implements nothing in scope.
- `[TASK_GRANULARITY]`: a task too large to verify as one unit, or a parallelization claim two "independent" tasks actually violate.
- `[CLUSTER_INCOHERENCE]`: tasks in one Parallelization group must share a spec delta or a source module, and overlapping `Knowledge` entries across groups are a consolidation signal, not a parallelism opportunity. Flag a group sliced by layer, a group whose tasks share no knowledge, or two groups the plan should have merged or sequenced.

### Design Depth: does the plan manage complexity well? (per `/speq-design-philosophy`)

`decision-log.md` is the input surface for `[ADR_OVERPROMOTION]`: check every `Promotes to ADR: yes` entry against `/speq-adr-rules`. `decision-log.md`, `plan.md`, and `architecture.md` (the delta, if present) are the input surface for `[ARCHITECTURE_DRIFT]`.

- `[SHALLOW_DESIGN]`: a planned module/interface is shallow relative to the complexity it should hide.
- `[INFORMATION_LEAKAGE]`: a design decision (format, protocol, temporal split) reflected across multiple planned modules.
- `[TACTICAL_SHORTCUT]`: a tactical shortcut with no scheduled strategic follow-up.
- `[BOUNDARY_VIOLATION]`: planned business logic depends directly on a delivery mechanism, storage engine, or framework.
- `[ADR_OVERPROMOTION]`: a `Promotes to ADR: yes` entry that fails `/speq-adr-rules`. Flag an entry when any of these holds: Rationale names no rule-2 criterion; Rationale states no `speq decision-log show` search result; the content is on the never-an-ADR list (rule 3); Decision holds implementation detail such as signatures, paths, or flags (rule 6). Fix: set it to `no`, or, if it is a corollary of another promoted decision, fold it into that parent entry's `Consequences` line. Escalation: `MECHANICAL`.
- `[ARCHITECTURE_DRIFT]` (BLOCKER): the plan changes the architecture but the delta does not say so. Flag it when any of these holds: (1) a `Promotes to ADR: yes` entry whose Rationale names rule-2 criterion 1, 2, or 3 has no `Architecture:` line, names a section the delta lacks, or says `no change` without a reason; (2) `plan.md` states a component, boundary, interface, data flow, constraint, or external dependency that the delta lacks; (3) the delta breaks `/speq-plan`'s `references/architecture-delta-template.md`. Fix: add or correct the delta block, move the fact into the delta, or write `Architecture: no change: <reason>`. Escalation: `MECHANICAL`. Content in `plan.md` that only duplicates the delta is ADVISORY.

### Prose quality: does the writing meet `/speq-writing-guardrails`?

- `[PROSE_BLOAT]`: filler, repetition, unneeded hedging or preamble. Violates BLUF/terseness.
- `[PROSE_UNCLEAR]`: a sentence that is ambiguous, jargon-laden, or not understandable on one read.

Prose findings default to **ADVISORY**: style, not correctness. Escalate a `[PROSE_UNCLEAR]` to **BLOCKER** only when the unclear prose makes a requirement non-actionable. Tag that finding `[AMBIGUOUS_REQUIREMENT]` too; it is the same defect from two angles.

## Severity

- **BLOCKER**: violates user intent, or the plan is infeasible/untestable as written, or breaks a project rule the skill system enforces (the ADR rules, the Background rule). Gates the plan; the orchestrator loops it back to `planner-agent`.
- **ADVISORY**: a real risk, tolerable if the human acknowledges it. Never blocks; surfaced in the orchestrator's final report only.

## Escalation Class (BLOCKER only)

Tag every BLOCKER `Escalation: HUMAN` or `Escalation: MECHANICAL`. This decides two things downstream, both in the orchestrator: whether round 2 runs at all (only if round 1 raised a `HUMAN` finding — an all-`MECHANICAL` round 1 skips straight to a fix-and-validate pass and ships), and what happens if a finding is still open once review is done: `HUMAN` findings reach the user (via `AskUserQuestion` or an `OPEN QUESTIONS:` PR comment); `MECHANICAL` ones get one direct fix from `planner-agent`, no further review round, no human interruption unless that fix itself fails.

`HUMAN` — same bar as `/speq-planning`'s headless escalation rule, applied here to a review finding instead of a planning choice:
- Irreversible, or changes what the feature does for a user
- Genuinely incompatible architectural designs with no clear winner
- Security or compliance consequence
- A load-bearing fact that neither the plan's own artifacts nor the codebase can settle, and whose falsity would change what the feature does for a user (e.g., "this closes the leak" resting on a claim nothing in this repo verifies)

A fact any of the plan's own artifacts, the codebase, or the recorded spec library *can* settle is `MECHANICAL`, however tedious checking it is — checking is not judgment. Don't stretch the fourth bullet to cover it: a stale test-name citation, a scenario that contradicts another scenario, or a tooling gap the plan's own text already describes are all things `plan-reviewer` can verify itself, not things it must ask about.

`MECHANICAL` — default. Anything resolvable by reading the plan's own artifacts, no external judgment call needed: a spec delta that contradicts another delta or a recorded spec, a citation that doesn't match the real test suite, a missing delta for a location the plan itself says changed, an inconsistent task placement, an unimplementable tooling reference, an assumption the plan can verify against the codebase itself (add the verification as the `Fix:`, don't escalate the question). Every Intent Fidelity BLOCKER is `HUMAN` by definition — a substituted or dropped ask is never something the reviewer resolves unilaterally.

Justify `HUMAN` in the finding's `Issue:` line. Don't default to `HUMAN` because a finding is hard to fix; default to `MECHANICAL` unless it actually requires a judgment call only the requester can make.

## Output Format

Write the findings document to `specs/_plans/<plan-name>/review/round-<N>.md` per `references/review-findings-template.md`, creating the `review/` directory if absent. Then return exactly one line and nothing else:

```
PLAN REVIEW round <N>: BLOCKERS: <n>, ADVISORY: <n>, INTENT: <n>, HUMAN: <n> — specs/_plans/<plan-name>/review/round-<N>.md
```

`INTENT` counts the Intent Fidelity BLOCKERs alone: a subset of `BLOCKERS`, matching the document's Summary block per the template's rules. `HUMAN` counts every BLOCKER tagged `Escalation: HUMAN` (Intent Fidelity BLOCKERs included — they are always `HUMAN`); the rest of `BLOCKERS` are `MECHANICAL` and derivable as `BLOCKERS - HUMAN`.

When round 2 ran confirm-only (`Plan Size: small`), append ` [confirm-only]` right after the round number, so the orchestrator's report can name which mode ran:

```
PLAN REVIEW round 2 [confirm-only]: BLOCKERS: <n>, ADVISORY: 0, INTENT: <n>, HUMAN: <n> — specs/_plans/<plan-name>/review/round-2.md
```

`ADVISORY` is always `0` on a confirm-only round: it ran no axis pass that could surface or re-surface one. The orchestrator reads ADVISORY findings from round 1's file instead. Omit the `[confirm-only]` marker for round 1 and for a full round 2.

Never return the findings themselves as response text. `planner-agent` reads them from the file.

Every finding needs a location and a concrete `Fix:` imperative; vague objections are not actionable for the revision loop. On round 2, confirm-or-refute each round-1 BLOCKER by name before raising anything new.
