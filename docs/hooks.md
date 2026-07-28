[speq-skill](../README.md) / [Docs](./index.md) / Hooks

---

# Project Hooks

Repo-local customization for `speq-skill`'s entry-point skills.

---

## Mechanism

Create `.speq/<name>-hook.md` in the project root. The matching entry-point skill reads it as its first workflow step, announces `Loaded project hook: .speq/<name>-hook.md`, and treats its content as authoritative instructions for that run.

## File naming

`<name>` is the skill's public command name without the `speq-`/`speq:` prefix:

| Skill | Hook file |
|---|---|
| `/speq:mission` | `.speq/mission-hook.md` |
| `/speq:plan` | `.speq/plan-hook.md` |
| `/speq:plan-pr` | `.speq/plan-pr-hook.md` |
| `/speq:implement` | `.speq/implement-hook.md` |
| `/speq:implement-pr` | `.speq/implement-pr-hook.md` |
| `/speq:record` | `.speq/record-hook.md` |
| `/speq:audit` | `.speq/audit-hook.md` |

Only the 7 entry-point skills read hooks directly. A skill that spawns sub-agents (e.g. `planner-agent`, `plan-reviewer`, `code-reviewer`) passes each one the hook's file path; the sub-agent reads it itself when relevant to its task.

Commit `.speq/` to the repo — hooks are project-level configuration. No frontmatter required.

## Behavior

- Hook content is authoritative: it can override any step of a skill's workflow, including the clarifying interview, the `plan-reviewer` loop, and TDD.
- Loading is always announced in the skill's output.
- `/speq:audit` lists active hook files as an informational check.
- `/speq:plan-pr` and `/speq:implement-pr` delegate whole steps to `/speq:plan`, `/speq:implement`, and `/speq:record`; each loads its own hook independently.

## Example

```markdown
<!-- .speq/plan-hook.md -->
Always write plan.md and decision-log.md prose in British English spelling.
Skip the Manual Testing section in Verification for internal tooling plans —
this team only ships internal CLIs, there's no external user to manually verify for.
```

`/speq:plan` announces `Loaded project hook: .speq/plan-hook.md`; `planner-agent` applies both instructions when authoring the plan.
