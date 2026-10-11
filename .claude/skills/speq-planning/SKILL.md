---
name: speq-planning
description: Plan-authoring workflow — spec delta authoring, test mapping, plan.md/decision-log.md generation, expert-task tagging, headless escalation, and revision mode. Use when authoring a plan.
---

# Plan Authoring

## Guiding Principles

- **BDD (Gherkin syntax)**: scenarios use GIVEN/WHEN/THEN; integration tests by default, unit tests only for isolated pure computation
- **RFC 2119 keywords**: THEN steps use MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY (uppercase)

## Workflow

### 0. Read Budget

Context is re-sent on every turn, so every byte you read costs again on each later call. Find before you read:

- Run `speq feature list` to see which feature is described where. Then read only the needed lines (`Read` with offset and limit, or `speq feature get '<domain>/<feature>/<scenario>'`).
- Do not read a file over about 8 KB whole. Read long docs (`testing.md`, `architecture.md`) by heading.
- Do not read a file twice. Write what you need to `notes/planning.md` the first time (create it now, append as you go, and consult it instead of re-reading).
- This applies to `cat` and `sed` in Bash too. Never `Read` a tool-output file the harness saved for you in full. Use offset and limit, or search it.
- For a branch diff, run `git diff --stat` first, then `git diff -- <path>` for the files that matter.
- Quote only the lines of an external spec that the plan cites.
- A project hook (`.speq/plan-hook.md` or `.speq/plan-pr-hook.md`) can require specific reads. The hook wins.

### 1. Discover Existing Specs

The orchestrator already ran `speq domain list` and `speq feature list`, named the features that relate to the request (per `/speq-cli`'s absence check), ran the ADR digest (`/speq-cli`), and named `specs/architecture.md` if present. It passed the calls and their output as this brief's `## Existing Context` section. Treat that as the baseline. Do not re-run the same queries. Re-query only for gaps it does not answer:

```bash
speq feature get <domain>/<feature>   # a specific feature's full spec, if Existing Context only named it
```

Existing Context names `specs/architecture.md`, the current architecture that step 2a changes. Read it once by heading (list the headings, then read the sections the plan touches), and keep what you need in `notes/planning.md`. Step 2a copies only the sections a delta changes.

Check the plan against every accepted ADR in the ADR digest from Existing Context, per `/speq-adr-rules` rule 5. Read the full fragment of any ADR the plan touches. Build the digest only if Existing Context lacks it.

Before you author a NEW feature spec or state that no spec covers a behavior, run `/speq-cli`'s absence check.

### 2. Author Spec Deltas

For each feature in scope:

```
specs/<domain>/<feature>/spec.md exists?
├─ Yes → DELTA markers (/speq-plan's references/delta-template.md)
└─ No  → Full spec (/speq-plan's references/feature-template.md)

Output: specs/_plans/<plan-name>/<domain>/<feature>/spec.md
```

**Prose drift check**: after drafting a scenario delta for an existing feature, check that feature's `## Background` and `# Feature: <name>` description, reusing what you read the first time; never re-read per scenario (read once with `speq feature get <domain>/<feature>` if you have not). When the scenario change makes either inaccurate, author a `DELTA:CHANGED` block for that section too, per `/speq-plan`'s `references/delta-template.md`.

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

Write each artifact once, then change it with `Edit`. Do not re-read a file you wrote this run. Draft per feature and do not restate spec delta text in `plan.md` or `decision-log.md`.

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
- **Interview section**: when `decision-log.md` already has it (interactive `speq-plan` writes it), keep it unchanged and do not copy it elsewhere. Otherwise (headless), write one line per Q&A from the brief, `- Q: <question> → A: <answer>`, a few words each; omit the section when the brief holds only a feature intent and no Q&A
- **Design Decisions section**: log a choice only when a reviewer or implementer cannot infer it from the deltas and tasks: a plausible alternative you rejected, or a constraint that is not obvious. Expect 0 to 3 entries. Never log naming, file placement, task order, or anything the deltas already state. Alternatives and Consequences are optional. For a new abstraction or module boundary, justify it in the entry's Rationale against `/speq-design-philosophy`'s Quick Diagnostic (deep vs. shallow, dependency direction)
- **Architecture field**: an entry with `Promotes to ADR: yes` whose Rationale names `/speq-adr-rules` rule-2 criterion 1, 2, or 3 needs an `- **Architecture:** <§ Section[, § Section] | no change: <reason>>` line. Every named section needs a block in the architecture delta
- **Review Findings section**: leave empty; populated in Revision Mode only for fixes that changed a design decision or settled a HUMAN finding

**Promotion gate.** Apply `/speq-adr-rules` to every Design Decisions and Review Findings entry. Default `Promotes to ADR: no`. For `yes`, name the rule-2 criterion and the ADR digest result in Rationale. A promoted entry's Decision and Rationale are short and state facts, not history: the recorder derives the ADR from them (`/speq-adr-rules` rule 7a). A `[plan-review]` entry almost always stays `no`.

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

### 8. Finalize Planning Hand-off Note

You started `specs/_plans/<plan-name>/notes/planning.md` in step 0 and appended to it while exploring. Before you return to the orchestrator, make sure it is a plain list, not a narrative:

- Files and symbols you checked
- Lookups you ran (speq CLI calls, Serena queries), each with its target

Do not write: why, invariants as prose, conventions commentary, or decisions. Those already live in `decision-log.md`. This note is a map of what you checked, not a story about it. A map stays true even after a later revision changes a decision; a story does not.

A revision-mode respawn (below) reads this note first, before it re-reads `plan.md`, `decision-log.md`, or spec deltas, and before it re-explores the codebase.

This note stays out of every commit. It is local scratch, never evidence.

### 9. Pre-Return Self-Check

Before you return to the orchestrator, run this checklist once against your own `plan.md`, `decision-log.md`, and spec deltas. Answer each line against your own writes in this run. Do not re-read the files. Fix anything that answers "no" before you return.

This is prevention, not the review gate. `plan-reviewer` still runs next at full strength.

Check your artifacts do not trip `/speq-plan-review`'s finding tags, across its five non-Prose axes (Prose stays `/speq-writing-guardrails`'s job):

- **Intent Fidelity**: no `[INTENT_DRIFT]` (a substituted or reinterpreted goal), `[SCOPE_CREEP]` (untraceable extras), or `[SCOPE_REDUCTION]` (a dropped ask with no user agreement).
- **Feasibility**: no `[EFFORT_MISESTIMATION]` (a task line that hides more work than it states), `[HIDDEN_DEPENDENCY]` (an unmodeled prerequisite), `[UNSTATED_ASSUMPTION]` (a load-bearing belief never stated), or `[NFR_IGNORED]` (security, performance, migration, or concurrency left untouched where the change touches it).
- **Requirement Quality**: no `[AMBIGUOUS_REQUIREMENT]` (not testable as written), `[COMPLETENESS_GAP]` (a missing edge case or error path), `[REQUIREMENT_CONFLICT]` (contradicts another delta or a recorded spec — check via `/speq-cli`), or `[IMPLEMENTATION_LEAKAGE]` (a Background or Feature description fact no scenario step depends on).
- **Task Breakdown**: no `[TRACEABILITY_GAP]` (a delta with no implementing task, or the reverse), `[TASK_GRANULARITY]` (a task too large to verify as one unit), or `[CLUSTER_INCOHERENCE]` (a Parallelization group sliced by layer, or overlapping `Knowledge` entries across groups).
- **Design Depth** (per `/speq-design-philosophy`): no `[SHALLOW_DESIGN]`, `[INFORMATION_LEAKAGE]` (a format or protocol decision reflected across modules), `[TACTICAL_SHORTCUT]` with no scheduled follow-up, `[BOUNDARY_VIOLATION]` (business logic depending directly on a delivery mechanism, storage engine, or framework), `[ADR_OVERPROMOTION]` (a `Promotes to ADR: yes` entry with no named criterion, no search result, never-an-ADR content, or implementation detail in Decision, per `/speq-adr-rules`), `[ARCHITECTURE_DRIFT]` (a qualifying ADR entry with no `Architecture:` line, a named section the delta lacks, or `no change` with no reason; a component, boundary, interface, data flow, constraint, or external dependency in `plan.md` that the delta lacks; or a delta that breaks the delta template), or `[ADR_CONFLICT]` (a decision, spec delta, or architecture delta that contradicts an accepted ADR with no superseding entry).

Open `/speq-plan-review` for a tag's full definition if you are unsure it applies.

## Headless / Non-Interactive Mode

Only if the orchestrator's prompt states `Interview Mode: headless`: read `/speq-planning`'s `references/headless-mode.md` and follow it.

## Revision Mode

Only if the orchestrator respawns you with the path to a `plan-reviewer` findings file: read `/speq-planning`'s `references/revision-mode.md` and follow it. Read `notes/planning.md` first.
