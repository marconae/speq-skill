[speq-skill](../README.md) / [Docs](./index.md) / Model Routing

---

# Model Routing

speq-skill routes work between a main session, workflow skills, and specialist sub-agents. The generated Claude and Codex plugin artifacts fix this routing.

---

## Principle

> Orchestration is cheap. Reasoning is expensive.

Workflow skills coordinate the work. They gather context, ask clarifying questions, confirm preconditions, and dispatch specialist roles. Planning, expert implementation, and review use the heavier reasoning tier. An error in these steps compounds in later steps.

---

## Claude routing

| Skill or agent | Model | Effort | Notes |
|----------------|-------|--------|-------|
| `/speq:plan` | `sonnet` | inherited | Thin orchestration |
| `/speq:implement` | `sonnet` | inherited | Thin orchestration |
| `/speq:record` | `sonnet` | inherited | Thin orchestration |
| `/speq:mission` | inherited | inherited | Interactive bootstrap |
| `/speq:plan-pr` | `sonnet` | inherited | Thin orchestration (headless) |
| `/speq:implement-pr` | `sonnet` | inherited | Thin orchestration (headless) |
| `/speq:audit` | `sonnet` | inherited | Thin orchestration (health check) |
| Utility skills | inherited | inherited | Reference material for the caller |
| `planner-agent` | `opus` | `xhigh` | Spec deltas, ADRs, task decomposition |
| `plan-reviewer` | `opus` | `xhigh` | Adversarial plan review |
| `implementer-agent` | `sonnet` | `high` | Standard implementation tasks |
| `implementer-expert-agent` | `opus` | `xhigh` | Tasks tagged `[expert]` |
| `code-reviewer` | `opus` | `xhigh` | Adversarial implementation review |
| `audit-agent` | `opus` | `high` | Mission ↔ spec-library sync |
| `recorder-agent` | `sonnet` | `medium` | Deterministic spec merge and archive |

`/speq:plan-pr` and `/speq:implement-pr` run every git/`gh` operation directly, per `/speq:git-operations`, at their own orchestrator row above. No separate agent tier exists for git/GitHub operations.

---

## Codex routing

| Skill or agent | Model | Effort | Notes |
|----------------|-------|--------|-------|
| `/speq:plan` | `gpt-5.4` | `medium` | Thin orchestration |
| `/speq:implement` | `gpt-5.4` | `medium` | Thin orchestration |
| `/speq:record` | `gpt-5.4` | `medium` | Thin orchestration |
| `/speq:mission` | inherited | inherited | Interactive bootstrap |
| `/speq:plan-pr` | `gpt-5.4` | `medium` | Thin orchestration (headless) |
| `/speq:implement-pr` | `gpt-5.4` | `medium` | Thin orchestration (headless) |
| `/speq:audit` | `gpt-5.4` | `medium` | Thin orchestration (health check) |
| Utility skills | inherited | inherited | Reference material for the caller |
| `planner-agent` | `gpt-5.5` | `xhigh` | Spec deltas, ADRs, task decomposition |
| `plan-reviewer` | `gpt-5.5` | `xhigh` | Adversarial plan review |
| `implementer-agent` | `gpt-5.4` | `high` | Standard implementation tasks |
| `implementer-expert-agent` | `gpt-5.5` | `xhigh` | Tasks tagged `[expert]` |
| `code-reviewer` | `gpt-5.5` | `xhigh` | Adversarial implementation review |
| `audit-agent` | `gpt-5.5` | `high` | Mission ↔ spec-library sync |
| `recorder-agent` | `gpt-5.4` | `medium` | Deterministic spec merge and archive |

`/speq:plan-pr` and `/speq:implement-pr` run every git/`gh` operation directly, per `/speq:git-operations`, at their own orchestrator row above. No separate agent tier exists for git/GitHub operations.

---

## Expert task tagging

`planner-agent` marks tasks that need deep reasoning with the tag `[expert]`:

```markdown
- [ ] 2.1 Add CLI flag parsing
- [ ] 2.2 Implement lock-free queue for concurrent spec writes [expert]
```

When `/speq:implement` processes a task group:

- Untagged tasks route to `implementer-agent`
- `[expert]` tasks route to `implementer-expert-agent`
- When expert tasks establish invariants that standard tasks depend on, they run first

Use `[expert]` for concurrency, subtle correctness, cross-file refactors, novel algorithms, and security-sensitive work. Avoid it for routine CLI plumbing, fixtures, docs, or straightforward use of existing patterns.

---

## Packaging

The checked-in source currently lives in `.claude/skills` and `.claude/agents`. `scripts/plugin/build.sh` treats that as the shared source and generates platform-specific outputs:

- Claude artifacts under `dist/marketplace/plugins/speq-skill`
- Codex artifacts under `dist/marketplace/codex/plugins/speq-skill`
- Codex marketplace manifest under `dist/marketplace/codex/.agents/plugins/marketplace.json`

Claude-generated output exposes `/speq:*` skills. Codex-generated output exposes the same workflows behind the `$` trigger. Generation also rewrites Codex-specific prompt text.
