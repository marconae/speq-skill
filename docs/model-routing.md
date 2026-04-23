[speq-skill](../README.md) / [Docs](./index.md) / Model Routing

---

# Model Routing

speq-skill routes work between a main session, workflow skills, and dedicated specialist sub-agents. The routing is designed so that expensive reasoning is applied only where it is required. This guide explains the architecture and the conventions the plugin uses. It is grounded in the current [Claude Code sub-agents documentation](https://docs.claude.com/en/docs/claude-code/sub-agents).

---

## Architecture at a Glance

```
User prompt
   │
   ▼
┌──────────────────────────────┐
│  Main session                │  runs in the configured model
│  (invokes a workflow skill)  │
└──────────────┬───────────────┘
               │
               ▼
┌───────────────────────────────┐
│  Workflow skill (orchestrator)│  pins Sonnet in frontmatter,
│  /speq:plan                   │  overriding the session model
│  /speq:implement              │  while the skill is active
│  /speq:record                 │
└──────────────┬────────────────┘
               │ spawns
               ▼
┌──────────────────────────────┐
│  Sub-agent (specialist)      │  runs in its own context window
│  planner-agent               │  with its own model and effort
│  implementer-agent           │  (declared in frontmatter)
│  implementer-expert-agent    │
│  code-reviewer               │
│  recorder-agent              │
└──────────────────────────────┘
```

Two layers do the actual work:

- **Workflow skills** are thin orchestrators. They gather context, ask the user clarifying questions, verify preconditions, and dispatch sub-agents. They do not themselves perform spec authoring, implementation, or review.
- **Sub-agents** are specialists. Each one pins its own `model` and `effort` in frontmatter and runs in an isolated context window with a focused system prompt.

---

## Guiding Principle

> Orchestration is cheap. Reasoning is expensive.

Spec authoring, implementation, and adversarial code review are the steps where a wrong choices accumulate in bad results downstream. This is why these steps pin the reasoning-heavy tier. More mechanical steps, e.g. applying delta markers, or recording, do not and thus are pinned with ligher and faster models.

This matches Anthropic's published orchestrator/worker pattern: a lean coordinator delegating to specialist workers. See [Building effective agents](https://www.anthropic.com/research/building-effective-agents) and [How we built our multi-agent research system](https://www.anthropic.com/engineering/multi-agent-research-system).

---

## Skills and Model Pinning

Claude Code lets a skill's frontmatter pin `model:` and `effort:`; the pin overrides the session's setting for the duration of the skill and then reverts. speq-skill uses this so orchestration is cheap and predictable regardless of which model the user has selected for their session.

- **Orchestrator skills** (`/speq:plan`, `/speq:implement`, `/speq:record`) pin `model: sonnet` in frontmatter. Orchestration is tool-call heavy and reasoning-light — reading tasks.md, partitioning task groups, and spawning sub-agents does not benefit from a premium tier.
- **Utility skills** (`/speq:code-tools`, `/speq:cli`, `/speq:git-discipline`, `/speq:ext-research`, `/speq:code-guardrails`) do not pin a model. They are reference material injected into the calling context and inherit the model of whatever skill or agent invoked them.
- **Sub-agents** pin their own model and effort in frontmatter, so planning, expert implementation, and review run on the reasoning-heavy tier regardless of the orchestrator's model. The session choice does not affect output quality of those steps.

The user's `/model` selection therefore controls only the thin layer outside of speq-skill — the main session prompt before a workflow skill takes over, and any follow-up after. Cost and quality inside the speq workflow are determined by the pins in the skill and sub-agent frontmatter.

---

## Sub-Agent Catalog

Each sub-agent pins `model` and `effort` in its frontmatter. The table below describes the role; the actual values live in `.claude/agents/*.md` so they can evolve with the model lineup without churning the documentation.

| Sub-agent | Spawned by | Tier | Why |
|-----------|------------|------|-----|
| `planner-agent` | `/speq:plan` | reasoning-heavy | Architectural tradeoffs, ADR authoring, MECE decomposition of features into tasks. Defects here compound through every downstream implementation task. |
| `implementer-agent` | `/speq:implement` | standard | Default for coding tasks. Handles the majority of implementation work: feature additions, bug fixes, test writing, routine refactors. |
| `implementer-expert-agent` | `/speq:implement` | reasoning-heavy | Reserved for tasks tagged `[expert]` in `tasks.md`. See [Expert task tagging](#expert-task-tagging). |
| `code-reviewer` | `/speq:implement` | reasoning-heavy | Adversarial review requires holding two large artifacts (spec and implementation) in mind and surfacing non-obvious defects. |
| `recorder-agent` | `/speq:record` | mechanical | Applies delta markers, validates specs, archives the plan. No reasoning premium needed. |

All five agents ship with the plugin and are registered in `plugin.json`.

### Context Isolation

Per the Claude Code sub-agent docs, each sub-agent "runs in its own context window with a custom system prompt, specific tool access, and independent permissions." This means:

- A sub-agent's context does not consume the orchestrator's tokens.
- Long implementation runs can rotate sub-agents to refresh context without losing the orchestration state held by `/speq:implement`.
- A failing sub-agent does not contaminate sibling sub-agents or the orchestrator.

---

## Expert Task Tagging

`planner-agent` identifies tasks that require deep reasoning and marks them with `[expert]` at the end of the task line in `tasks.md`:

```markdown
- [ ] 2.1 Add CLI flag parsing
- [ ] 2.2 Implement lock-free queue for concurrent spec writes [expert]
- [ ] 2.3 Write integration tests for flag combinations
- [ ] 2.4 Refactor validator to preserve ordering invariants [expert]
```

When `/speq:implement` processes a task group, it partitions by tag:

- Untagged tasks → `implementer-agent`
- `[expert]`-tagged tasks → `implementer-expert-agent`

When both exist in the same group and touch disjoint files, they run in parallel. Otherwise expert tasks run first, because they often establish invariants the standard tasks rely on.

### When to Tag `[expert]`

Tag a task as `[expert]` only when it genuinely needs deeper reasoning:

- Concurrency, ordering, or race conditions
- Cross-file refactors with behavioral dependencies
- Novel algorithms or non-obvious correctness
- Security-sensitive code paths

### When NOT to Tag `[expert]`

- Standard CRUD or CLI flag plumbing
- Test fixtures and scaffolding
- Copy-paste from existing patterns in the codebase
- Documentation or configuration changes

Over-tagging wastes tokens; under-tagging risks defects. Most tasks in a typical plan should be untagged.

---

## Effort Levels

Each sub-agent declares an `effort` level in frontmatter. Claude Code honors this per sub-agent; it overrides the session's effort for the duration of that sub-agent's run, then reverts.

Roughly:

- Reasoning-heavy sub-agents (`planner-agent`, `implementer-expert-agent`, `code-reviewer`) run at the highest effort tier. This is where extended thinking pays for itself.
- `implementer-agent` runs at a standard effort tier — enough for solid code, without burning thinking budget on straightforward tasks.
- `recorder-agent` runs at a lower effort tier — recording is deterministic file surgery.

See the Claude Code [sub-agent docs](https://docs.claude.com/en/docs/claude-code/sub-agents) for the full frontmatter schema including `model`, `effort`, and `tools`.

---

## Summary

- Orchestrator workflow skills (`/speq:plan`, `/speq:implement`, `/speq:record`) pin `model: sonnet` so orchestration is cheap regardless of the session's `/model` selection.
- Workflow skills do not author specs or write code directly — they delegate to specialist sub-agents.
- Sub-agents pin their own `model` and `effort` in frontmatter and run in isolated context windows.
- `[expert]` tags in `tasks.md` route individual tasks to `implementer-expert-agent` instead of `implementer-agent`.
- Actual model and effort values live next to the skill and sub-agent definitions in `.claude/` so the framework is insulated from model-version churn.
