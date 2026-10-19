# PR Body Template

Used by `/speq-plan-pr` as the draft PR body.

## Skeleton

```markdown
## Current State
<1-2 sentences: what is broken, missing, or true today, plain language>

## What Changes
<1-2 sentences: the mechanism>
- <concrete change>
- <concrete change>

## Impact                                    <!-- omit if plan.md's ## Impact says "None" -->
<trimmed to what a user must know to decide — not the full plan.md text>

ADR candidates: <none | titles of the `Promotes to ADR: yes` entries>

Architecture: <none | none (specs/architecture.md absent) | § Components (changed), § Deployment (new)>
```

## Rules

- **`ADR candidates` is one line.** Write `none` when no entry promotes, which is the usual case. Otherwise list the entry titles. Each title is a proposed ADR.
- **`Architecture` is one line.** Write `none` when the plan has no architecture delta, or `none (specs/architecture.md absent)` in a repo without that file. Otherwise list each changed section with `changed`, `new`, or `removed`. Never paste section content. The delta file holds it.
- **`Current State` / `What Changes` replace a task-list dump.** State the problem, then the mechanism that fixes it. Never a step-by-step of what got implemented — that is `tasks.md`'s job, not the PR body's.
- **`Impact` is plan.md's own `## Impact` section, trimmed**, not copied verbatim: keep only the bullets a user must weigh to approve. Omit the whole section when plan.md says "None".
- **The body is a summary, never a second copy.** The plan files are in the PR diff. If a fact matters enough to state, it belongs in `Current State`/`What Changes`/`Impact`.
