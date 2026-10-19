---
name: speq-planning
description: "Plan-authoring workflow for the planner-agent: spec deltas, test mapping, plan.md, decision-log.md, expert tagging, and revision mode."
---

# Plan Authoring

## Guiding Principles

- **BDD (Gherkin syntax)**: scenarios use GIVEN/WHEN/THEN; integration tests by default, unit tests only for isolated pure computation
- **RFC 2119 keywords**: THEN steps use MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY (uppercase)

## Rules

- **Read-only investigation**: planning reads existing code, tests, specs, and docs. It never builds the software, runs a test suite, starts Docker or an e2e run, or launches the software. Plan the commands to run; do not run them.
- **Short Background**: `## Background` is a concise summary of the context every scenario shares, 3-5 one-line bullets at most. It never encodes features, behavior, or requirements; each of those is a scenario (or a step in one). When a change adds behavior, add or change a scenario, not a Background line. When you rewrite a Background that has grown past this, cut it down in the same `DELTA:CHANGED` block.
- **Architecture by delta**: never edit `specs/architecture.md`; `/speq-record` merges the delta. When the plan changes no architecture, write no delta.
- **Proof is tests and manual runs**: `speq` commands (`speq plan validate`, `speq feature get`) check the plan, not the product, so never list them as test or manual evidence.
- **One copy**: never embed spec content in `plan.md` or `decision-log.md`; reference the delta files. Write each artifact once, then change it with `Edit`, and do not re-read a file you wrote this run.
- **Specs describe the software, not its tests**: a spec delta states features, behavior, and constraints of the software. Never encode e2e tests, test suites, Docker setups, or the test environment as scenarios or Background; they say how to verify the software, not what it does. `specs/mission.md` encodes how to test (suites, commands, environment), so reference it from the plan's Verification section.
- **Local notes**: `notes/planning.md` is local scratch. It is never committed and never evidence.

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

- If `specs/architecture.md` is absent, write no delta. State `Architecture: no change: specs/architecture.md absent` in the decision log entry.

### 3. Test Mapping and Verification

Every scenario requires two forms of external proof. No claims, only evidence.

**Integration tests** (mandatory per scenario):
- Map each new or changed scenario to an integration test (file path + test name)
- List the existing tests that the change or the new tests make redundant, for removal under plan.md's Dead Code Removal
- One test per scenario by default; combine only when scenarios share setup and assertions
- Unit tests only for pure computation with no I/O

**Manual invocation** (mandatory per feature):
- Concrete commands that invoke the built software
- Expected observable output per command

### 4. Generate plan.md

Draft per feature. Populate plan.md per `/speq-plan`'s `references/plan-template.md`:

1. **Context**: per the template's Context section. Decisions go in `decision-log.md` (step 5)
2. **Features**: table referencing spec delta files, immediately followed by an Impact entry describing user/operator-facing consequences (breaking changes called out, or "None")
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
- **Interview section**: when `decision-log.md` already has it, keep it unchanged and do not copy it elsewhere
- **Design Decisions section**: log a choice only when a reviewer or implementer cannot infer it from the deltas and tasks: a plausible alternative you rejected, or a constraint that is not obvious. Expect 0 to 3 entries. Never log naming, file placement, task order, or anything the deltas already state. Alternatives and Consequences are optional. For a new abstraction or module boundary, justify it in the entry's Rationale against `/speq-design-philosophy`'s Quick Diagnostic (deep vs. shallow, dependency direction)
- **Architecture field**: an entry with `Promotes to ADR: yes` whose Rationale names `/speq-adr-rules` rule-2 criterion 1, 2, or 3 needs an `- **Architecture:** <§ Section[, § Section] | no change: <reason>>` line. Every named section needs a block in the architecture delta
- **Review Findings section**: leave empty; populated in Revision Mode only for fixes that changed a design decision

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

**Do not tag as expert:**
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

### 9. Pre-Return Self-Check

Before you return to the orchestrator, check your own `plan.md`, `decision-log.md`, and spec deltas once, against your own writes in this run.

This is prevention, not the review gate. `plan-reviewer` still runs next at full strength.

Check your artifacts against `/speq-plan-review`'s five non-Prose axes (Intent Fidelity, Feasibility, Requirement Quality, Task Breakdown, Design Depth) and its finding tags (load `/speq-plan-review` if they are not in context). Align prose with `/speq-writing-guardrails`. Fix anything the reviewer would flag.

## Revision Mode

Only if the orchestrator respawns you with the path to a feedback file (`plan-reviewer` findings or PR comments): read `notes/planning.md` first, then `/speq-planning`'s `references/revision-mode.md` and follow it.
