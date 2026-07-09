[speq-skill](../README.md) / [Docs](./index.md) / Model Routing

---

# Model Routing

speq-skill routes work between a main session, workflow skills, and specialist sub-agents. The routing is hardcoded in generated Claude and Codex plugin artifacts for `0.4.0`; a dynamic routing config is deferred to a later version.

---

## Principle

> Orchestration is cheap. Reasoning is expensive.

Workflow skills coordinate the work: they gather context, ask clarifying questions, verify preconditions, and dispatch specialist roles. Planning, expert implementation, and review use the heavier reasoning tier because mistakes there compound downstream.

---

## Claude Routing

| Skill or agent | Model | Effort | Notes |
|----------------|-------|--------|-------|
| `/speq:plan` | `sonnet` | inherited | Thin orchestration |
| `/speq:implement` | `sonnet` | inherited | Thin orchestration |
| `/speq:record` | `sonnet` | inherited | Thin orchestration |
| `/speq:mission` | inherited | inherited | Interactive bootstrap |
| `/speq:plan-pr` | `sonnet` | inherited | Thin orchestration — headless |
| `/speq:implement-pr` | `sonnet` | inherited | Thin orchestration — headless |
| `/speq:audit` | `sonnet` | inherited | Thin orchestration — health check |
| Utility skills | inherited | inherited | Reference material for the caller |
| `planner-agent` | `opus` | `xhigh` | Spec deltas, ADRs, task decomposition |
| `plan-reviewer` | `opus` | `xhigh` | Adversarial plan review |
| `implementer-agent` | `sonnet` | `high` | Standard implementation tasks |
| `implementer-expert-agent` | `opus` | `xhigh` | Tasks tagged `[expert]` |
| `code-reviewer` | `opus` | `xhigh` | Adversarial implementation review |
| `audit-agent` | `opus` | `high` | Mission ↔ spec-library sync |
| `recorder-agent` | `sonnet` | `medium` | Deterministic spec merge and archive |
| `git-agent` | `sonnet` | `medium` | Generic git/GitHub operations — no reasoning premium |

---

## Codex Routing

| Skill or agent | Model | Effort | Notes |
|----------------|-------|--------|-------|
| `/speq:plan` | `gpt-5.4` | `medium` | Thin orchestration |
| `/speq:implement` | `gpt-5.4` | `medium` | Thin orchestration |
| `/speq:record` | `gpt-5.4` | `medium` | Thin orchestration |
| `/speq:mission` | inherited | inherited | Interactive bootstrap |
| `/speq:plan-pr` | `gpt-5.4` | `medium` | Thin orchestration — headless |
| `/speq:implement-pr` | `gpt-5.4` | `medium` | Thin orchestration — headless |
| `/speq:audit` | `gpt-5.4` | `medium` | Thin orchestration — health check |
| Utility skills | inherited | inherited | Reference material for the caller |
| `planner-agent` | `gpt-5.5` | `xhigh` | Spec deltas, ADRs, task decomposition |
| `plan-reviewer` | `gpt-5.5` | `xhigh` | Adversarial plan review |
| `implementer-agent` | `gpt-5.4` | `high` | Standard implementation tasks |
| `implementer-expert-agent` | `gpt-5.5` | `xhigh` | Tasks tagged `[expert]` |
| `code-reviewer` | `gpt-5.5` | `xhigh` | Adversarial implementation review |
| `audit-agent` | `gpt-5.5` | `high` | Mission ↔ spec-library sync |
| `recorder-agent` | `gpt-5.4` | `medium` | Deterministic spec merge and archive |
| `git-agent` | `gpt-5.4` | `medium` | Generic git/GitHub operations — no reasoning premium |

---

## Expert Task Tagging

`planner-agent` marks tasks requiring deep reasoning with `[expert]`:

```markdown
- [ ] 2.1 Add CLI flag parsing
- [ ] 2.2 Implement lock-free queue for concurrent spec writes [expert]
```

When `/speq:implement` processes a task group:

- Untagged tasks route to `implementer-agent`
- `[expert]` tasks route to `implementer-expert-agent`
- Expert tasks run first when they establish invariants that standard tasks depend on

Use `[expert]` for concurrency, subtle correctness, cross-file refactors, novel algorithms, and security-sensitive work. Avoid it for routine CLI plumbing, fixtures, docs, or straightforward use of existing patterns.

---

## Packaging

The checked-in source currently lives in `.claude/skills` and `.claude/agents`. `scripts/plugin/build.sh` treats that as the shared source and generates platform-specific outputs:

- Claude artifacts under `dist/marketplace/plugins/speq-skill`
- Codex artifacts under `dist/marketplace/codex/plugins/speq-skill`
- Codex marketplace manifest under `dist/marketplace/codex/.agents/plugins/marketplace.json`

Both generated outputs expose `/speq:*` skills. Claude-specific workflow syntax is translated out of the Codex output during generation.
