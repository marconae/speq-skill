[speq-skill](../README.md) / [Docs](./index.md) / Hooks

---

# Project Hooks

Repository-specific instructions for the entry-point skills of `speq-skill`.

---

## Mechanism

Create `.speq/<name>-hook.md` in the project root. The matching entry-point skill reads the hook file as its first workflow step. Then the skill announces `Loaded project hook: .speq/<name>-hook.md`. The skill treats the hook content as authoritative instructions for that run.

## File naming

`<name>` is the public command name of the skill, without the `speq-`/`speq:` prefix:

| Skill | Hook file |
|---|---|
| `/speq:mission` | `.speq/mission-hook.md` |
| `/speq:plan` | `.speq/plan-hook.md` |
| `/speq:plan-pr` | `.speq/plan-pr-hook.md` |
| `/speq:implement` | `.speq/implement-hook.md` |
| `/speq:implement-pr` | `.speq/implement-pr-hook.md` |
| `/speq:record` | `.speq/record-hook.md` |
| `/speq:audit` | `.speq/audit-hook.md` |

Only the 7 entry-point skills read hooks directly. A skill that spawns sub-agents, for example `planner-agent`, `plan-reviewer`, or `code-reviewer`, passes the hook file path to each sub-agent. When the file is relevant to the task of the sub-agent, the sub-agent reads the file itself.

Commit `.speq/` to the repository. Hook files are project-level configuration and need no frontmatter.

## Behavior

- Hook content is authoritative. It can override any step of a skill workflow: the clarifying interview, the `plan-reviewer` loop, or TDD.
- The skill always announces when it loads a hook file.
- `/speq:audit` lists active hook files as an informational check.
- `/speq:plan-pr` and `/speq:implement-pr` delegate whole steps to `/speq:plan`, `/speq:implement`, and `/speq:record`. Each of these skills loads its own hook file independently.

## Example

```markdown
<!-- .speq/plan-hook.md -->
Always write plan.md and decision-log.md prose in British English spelling.
Skip the Manual Testing section in Verification for internal tooling plans —
this team only ships internal CLIs, there's no external user to manually verify for.
```

`/speq:plan` announces `Loaded project hook: .speq/plan-hook.md`. When `planner-agent` authors the plan, it applies both instructions.
