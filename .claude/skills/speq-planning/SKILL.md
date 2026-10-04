---
name: speq-planning
description: Plan-authoring workflow — spec delta authoring, test mapping, plan.md/decision-log.md generation, expert-task tagging, headless escalation, and revision mode. Triggered by planner-agent.
---

# Plan Authoring

## Guiding Principles

- **BDD (Gherkin syntax)**: scenarios use GIVEN/WHEN/THEN; integration tests by default, unit tests only for isolated pure computation
- **EARS syntax**: spec narratives use unambiguous behavioral clauses
- **RFC 2119 keywords**: THEN steps use MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY (uppercase)

## Workflow

### 1. Discover Existing Specs

The orchestrator already ran `speq domain list` / `speq feature list` / `speq search query` (once per exact term in the request, per `/speq-cli`'s absence check), and `speq decision-log show`, and read `specs/architecture.md` if present. It passed the calls and their output as this brief's `## Existing Context` section. Treat that as the baseline. Do not re-run the same queries. Re-query only for gaps it does not answer:

```bash
speq feature get <domain>/<feature>   # a specific feature's full spec, if Existing Context only named it
speq search query "<narrower terms>"  # only if Existing Context's results do not cover a sub-area you need
```

Existing Context holds the content of `specs/architecture.md`, the current architecture that step 2a changes. If Existing Context lacks it and the file exists, read the file. Step 2a reads the file itself only to copy the sections a delta changes.

Check the plan against every accepted ADR in the `speq decision-log show` output from Existing Context, per `/speq-adr-rules` rule 5. Run the command only if Existing Context lacks it.

Before you author a NEW feature spec or state that no spec covers a behavior, run `/speq-cli`'s absence check.

### 2. Author Spec Deltas

For each feature in scope:

```
specs/<domain>/<feature>/spec.md exists?
├─ Yes → DELTA markers (/speq-plan's references/delta-template.md)
└─ No  → Full spec (/speq-plan's references/feature-template.md)

Output: specs/_plans/<plan-name>/<domain>/<feature>/spec.md
```

**Prose drift check**: after drafting a scenario delta for an existing feature, re-read that feature's `## Background` and `# Feature: <name>` description (`speq feature get <domain>/<feature>`). When the scenario change makes either inaccurate, author a `DELTA:CHANGED` block for that section too, per `/speq-plan`'s `references/delta-template.md`.

### 2a. Author Architecture Delta

Decide whether the plan changes the architecture: a component, boundary, interface, data flow, constraint, or external dependency. If it does, write `specs/_plans/<plan-name>/architecture.md` per `/speq-plan`'s `references/architecture-delta-template.md`. Copy each changed section from `specs/architecture.md`, edit it, and set the BASE hash with `git hash-object specs/architecture.md`.

- If the plan changes no architecture, write no delta.
- If `specs/architecture.md` is absent, write no delta. State `Architecture: no change: specs/architecture.md absent` in the decision log entry.
- Never edit `specs/architecture.md` itself. `/speq-record` merges the delta.

### 3. Test Mapping and Verification

Every scenario requires two forms of external proof. No claims, only evidence.

**Integration tests** (mandatory per scenario):
- Map each scenario to an integration test (file path + test name)
- One test per scenario by default; combine only when scenarios share setup and assertions
- Unit tests only for pure computation with no I/O

**Manual invocation** (mandatory per feature):
- Concrete commands that invoke the built software
- Expected observable output per command

### 4. Generate plan.md

Populate plan.md per `/speq-plan`'s `references/plan-template.md`:

1. **Context**: per the template's Context section. Decisions go in `decision-log.md` (step 5)
2. **Features**: table referencing spec delta files (NEVER embed spec content), immediately followed by an Impact entry describing user/operator-facing consequences (breaking changes called out, or "None")
3. **Tasks**: work breakdown in implementation order
4. **Parallelization**: knowledge clusters, per the rules below
5. **Verification**: Scenario Coverage + Manual Testing + Checklist (from `specs/mission.md`)

**Parallelization groups are knowledge clusters.** Group tasks vertically: one spec delta plus the code area it governs. Never group by layer. Layer slices (fixtures, then module, then CLI, then tests) make each layer's agent re-derive the same mental model; a vertical slice orients one agent once. Each group row declares a `Knowledge` column: the group's spec delta path(s) plus the source and test files they govern. The implement orchestrator hands this entry to the group's agent as its orientation pointer. Two facts govern the grouping:

1. Tasks that share a spec delta or a source module default into one group.
2. Overlapping `Knowledge` entries across groups are a consolidation signal, not a parallelism opportunity.

If two clusters would contend on one shared file, split the module per feature (often the better design; check it against `/speq-design-philosophy`) or declare a dependency and sequence the clusters. Parallelism is a side effect, not the goal.

### 5. Generate decision-log.md

Create `specs/_plans/<plan-name>/decision-log.md` from `/speq-plan`'s `references/decision-log-plan-template.md`.

**What to capture:**
- **Interview section**: verbatim or close paraphrase of every Q&A exchange passed from the orchestrator
- **Design Decisions section**: one entry per significant choice made while authoring spec deltas or plan.md (how this plan builds the change, patterns and local interfaces, rejected alternatives, scope boundaries). For a new abstraction or module boundary, justify it in the entry's Rationale against `/speq-design-philosophy`'s Quick Diagnostic (deep vs. shallow, dependency direction)
- **Architecture field**: an entry with `Promotes to ADR: yes` whose Rationale names `/speq-adr-rules` rule-2 criterion 1, 2, or 3 needs an `- **Architecture:** <§ Section[, § Section] | no change: <reason>>` line. Every named section needs a block in the architecture delta
- **Review Findings section**: leave empty; populated in Revision Mode after `plan-reviewer` blockers, and by `speq-implement` after code review

**Promotion gate.** Apply `/speq-adr-rules` to every Design Decisions and Review Findings entry. Default `Promotes to ADR: no`. For `yes`, name the rule-2 criterion and the `speq decision-log show` result in Rationale. A promoted entry's Decision and Rationale are short and state facts, not history: the recorder derives the ADR from them (`/speq-adr-rules` rule 7a). A `[plan-review]` entry almost always stays `no`.

### 6. Expert-Task Tagging

Tag tasks that require deep reasoning with `[expert]` at the end of the task line:

```markdown
- [ ] 2.1 Add CLI flag parsing
- [ ] 2.2 Implement lock-free queue for concurrent spec writes [expert]
- [ ] 2.3 Write integration tests for flag combinations
- [ ] 2.4 Refactor validator to preserve ordering invariants across plugins [expert]
```

**Use `[expert]` only when the task genuinely needs it:**
- Concurrency / ordering / race conditions
- Cross-file refactors with behavioral dependencies
- Novel algorithms or non-obvious correctness
- Security-sensitive code paths

**Do NOT tag as expert:**
- Standard CRUD, CLI flag plumbing, test fixtures
- Copy-paste from existing patterns
- Documentation or config changes

Over-tagging wastes tokens; under-tagging risks defects. If any task in a parallelization group carries the tag, the orchestrator routes the whole group to `implementer-expert-agent`; all-untagged groups go to `implementer-agent`. One tag prices its entire group at the expert model, so tag sparingly. Most tasks stay untagged.

### 7. Validate Plan

Before returning:

```bash
speq plan validate <plan-name>
```

Fix any failures. Common fixes:
- Close unclosed delta markers with `<!-- /CHANGED -->`, `<!-- /NEW -->`, `<!-- /REMOVED -->`
- Uppercase RFC 2119 keywords
- Fix step formatting (bold keywords: `*GIVEN*`, `*WHEN*`, `*THEN*`, `*AND*`)

### 8. Write Planning Hand-off Note

Before you return to the orchestrator, write `specs/_plans/<plan-name>/notes/planning.md` (create the `notes/` directory if absent). Write a plain list, not a narrative:

- Files and symbols you checked
- Searches you ran (`speq search`, Serena queries), each with its target

Do not write: why, invariants as prose, conventions commentary, or decisions. Those already live in `decision-log.md`. This note is a map of what you checked, not a story about it. A map stays true even after a later revision changes a decision; a story does not.

A revision-mode respawn (below) reads this note first, before it re-reads `plan.md`, `decision-log.md`, or spec deltas, and before it re-explores the codebase.

This note stays out of every commit. It is local scratch, never evidence.

### 9. Pre-Return Self-Check

Before you return to the orchestrator, run this checklist once against your own `plan.md`, `decision-log.md`, and spec deltas. Answer each line against the artifacts on disk, not from memory. Fix anything that answers "no" before you return.

This is prevention, not the review gate. `plan-reviewer` still runs next at full strength.

Check your artifacts do not trip `/speq-plan-review`'s finding tags, across its five non-Prose axes (Prose stays `/speq-writing-guardrails`'s job):

- **Intent Fidelity**: no `[INTENT_DRIFT]` (a substituted or reinterpreted goal), `[SCOPE_CREEP]` (untraceable extras), or `[SCOPE_REDUCTION]` (a dropped ask with no user agreement).
- **Feasibility**: no `[EFFORT_MISESTIMATION]` (a task line that hides more work than it states), `[HIDDEN_DEPENDENCY]` (an unmodeled prerequisite), `[UNSTATED_ASSUMPTION]` (a load-bearing belief never stated), or `[NFR_IGNORED]` (security, performance, migration, or concurrency left untouched where the change touches it).
- **Requirement Quality**: no `[AMBIGUOUS_REQUIREMENT]` (not testable as written), `[COMPLETENESS_GAP]` (a missing edge case or error path), `[REQUIREMENT_CONFLICT]` (contradicts another delta or a recorded spec — check via `/speq-cli`), or `[IMPLEMENTATION_LEAKAGE]` (a Background or Feature description fact no scenario step depends on).
- **Task Breakdown**: no `[TRACEABILITY_GAP]` (a delta with no implementing task, or the reverse), `[TASK_GRANULARITY]` (a task too large to verify as one unit), or `[CLUSTER_INCOHERENCE]` (a Parallelization group sliced by layer, or overlapping `Knowledge` entries across groups).
- **Design Depth** (per `/speq-design-philosophy`): no `[SHALLOW_DESIGN]`, `[INFORMATION_LEAKAGE]` (a format or protocol decision reflected across modules), `[TACTICAL_SHORTCUT]` with no scheduled follow-up, `[BOUNDARY_VIOLATION]` (business logic depending directly on a delivery mechanism, storage engine, or framework), `[ADR_OVERPROMOTION]` (a `Promotes to ADR: yes` entry with no named criterion, no search result, never-an-ADR content, or implementation detail in Decision, per `/speq-adr-rules`), `[ARCHITECTURE_DRIFT]` (a qualifying ADR entry with no `Architecture:` line, a named section the delta lacks, or `no change` with no reason; a component, boundary, interface, data flow, constraint, or external dependency in `plan.md` that the delta lacks; or a delta that breaks the delta template), or `[ADR_CONFLICT]` (a decision, spec delta, or architecture delta that contradicts an accepted ADR with no superseding entry).

Open `/speq-plan-review` for a tag's full definition if you are unsure it applies.

## Headless / Non-Interactive Mode

If the orchestrator's prompt states `Interview Mode: headless` (used by `speq-plan-pr`, never by the interactive `speq-plan`), there is no human to ask mid-planning. Adjust the escalation bar:

- **Assume and document.** For conventions, naming, implementation details, and any choice with a clearly conventional default: make the call and record it as a `decision-log.md` entry (Rationale explains why this default). This is the common case; most headless plans finish without escalating.
- **Escalate only irreducible decisions**: irreversible ones, changes to what the feature does for a user, genuinely incompatible architectural designs, or security/compliance. Before escalating, save every file completed so far (plan.md, delta specs, decision-log.md) exactly as it stands. The orchestrator persists this partial state for human review, so it must be usable as-is.
- **Escalation format.** Return the response prefixed with the exact sentinel `OPEN QUESTIONS:` followed by a markdown bullet list of concrete questions. Headless mode changes when to escalate, not the quality bar: same bar as the interactive path's "signal back with a concrete question". Do not mix this sentinel into a normal completion report.

## Revision Mode

If the orchestrator respawns `planner-agent` with the path to a `plan-reviewer` findings file instead of a fresh planning brief:

- Read `specs/_plans/<plan-name>/notes/planning.md` first, if present. This is your own prior-pass hand-off note. It orients you before you re-read anything else.
- Read the BLOCKER list from the path given in the prompt: `specs/_plans/<plan-name>/review/round-<N>.md`. The findings never arrive inline; the file is the only source.
- Address only the BLOCKER findings. Execute each one's `Fix:` line: an imperative naming the artifact, section, and concrete change. Revise exactly what it points to. Do not rewrite unrelated content. Do not act on ADVISORY findings.
- For each blocker resolved, add a `## Review Findings` entry to `decision-log.md` titled `[plan-review] <short finding title>`, with **Finding** (what `plan-reviewer` flagged), **Direction change** (what changed), and **Promotes to ADR** (per `/speq-adr-rules`; almost always `no`).
- Re-run `speq plan validate <plan-name>` before returning.
- Return one line per blocker you addressed: `Resolved: <title> — <evidence>` or `Could not resolve: <title> — <why>`. The orchestrator uses this to decide whether a further review round is needed — do not omit it, even when every finding resolved cleanly.
- If resolving a blocker surfaces a genuinely irreducible new decision, escalate it exactly as during initial planning (interactive: signal back with a concrete question; headless: `OPEN QUESTIONS:` sentinel).
