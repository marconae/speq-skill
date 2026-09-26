# PR Body Template

Used by `/speq-plan-pr` (draft, blocked) and `/speq-implement-pr` (ready) as the PR body — and by `/speq-plan`, which prints the same skeleton as terminal output for the human running it interactively, instead of posting it anywhere. Same content either way; only the destination differs. A human reading it gets what they need in one pass — the full trail is one click (or one more line) away, never pasted inline.

On a terminal print, drop the `<details>`/`<summary>` HTML — it's a GitHub folding mechanic with no terminal equivalent — and print its file list as a plain line instead. Everything else in the skeleton stays identical.

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

<details>
<summary>Full detail — spec, decisions, test evidence</summary>

`plan.md` · `architecture.md` · `decision-log.md` · `review/round-*.md`

</details>

Test plan
- [x] <checklist item>
- [ ] <checklist item>
```

## Rules

- **`ADR candidates` is one line.** Write `none` when no entry promotes, which is the usual case. Otherwise list the entry titles. Each title is a proposed ADR. Running `/speq-implement-pr` accepts them. To reject one, comment on the PR and run `/speq-plan-pr` again with the entry set to `Promotes to ADR: no`. After recording, `/speq-implement-pr` replaces the line with the accepted slugs.

- **`Architecture` is one line.** Write `none` when the plan has no architecture delta, or `none (specs/architecture.md absent)` in a repo without that file. Otherwise list each changed section with `changed`, `new`, or `removed`. Never paste section content. The delta file holds it. After recording, `/speq-implement-pr` rewrites the line as `Architecture: merged into specs/architecture.md: <sections>`.

- **`Current State` / `What Changes` replace a task-list dump.** State the problem, then the mechanism that fixes it. Never a step-by-step of what got implemented — that is `tasks.md`'s job, not the PR body's.
- **`Impact` is plan.md's own `## Impact` section, trimmed**, not copied verbatim: keep only the bullets a user must weigh to approve. Omit the whole section when plan.md says "None".
- **The `<details>` block is a pointer, never a second copy.** File names/paths only — `plan.md`, `architecture.md` (only when present), `decision-log.md`, `review/round-*.md`. If a fact matters enough to state, it belongs in `Current State`/`What Changes`/`Impact` above the fold, not buried here.
- **Test plan checklist stays inline, uncollapsed.** Checkboxes are the one thing a reviewer scans fastest as-is; folding them costs more than it saves.
- **Blocked path (`flag-blocked`):** same skeleton. State the open question as its own line with a link to `open-questions.md`/`review/round-N.md` for the why — never paste the reviewer's `Issue`/`Fix` text into the body.

## Worked Example

Real change, from a small terminal-app project: switching how session state is saved to disk. Before (the shape a verbose plan produces without this template — an `## Impact` section copied from `plan.md` verbatim, several bullets covering every angle, the one thing a reviewer must actually decide on buried in the middle):

> ## Impact
> Persistence changes for users. The state file format and location both change.
> - The editor buffer is still saved and restored across sessions. Unaffected.
> - Defined variables are no longer written to the state file; they are recomputed from the buffer on load instead. A variable's saved value is gone; only the expression that produced it survives.
> - The `serde`/`serde_json` dependency is removed from the build.
> - [... 2 more bullets on file format and location ...]

After, per this template (the one thing a user needs to know — their old session silently disappears once, no error, no migration — is the first line under Impact, not the third bullet):

```markdown
## Current State
Session state (buffer lines and defined variables) is saved as JSON to `~/.crabculator/state.json`, via `serde`.

## What Changes
State now saves as plain text to `~/.crabculator/state.txt`, one buffer line per file line. Variables are no longer persisted — they're recomputed by evaluating the buffer on load.
- Drops the `serde`/`serde_json` dependency entirely
- New file name and format; the old file is never read again, not migrated

## Impact
Breaking change for anyone with an existing `state.json`: on first launch after this change, their saved buffer and variables are silently gone — no error, no migration, the app just starts empty.
- A variable's *saved value* does not survive; only the expression that computed it does, and only if that expression is still in the buffer
- No corruption risk: the old `state.json` is simply never looked at again, not misread as plain text

<details>
<summary>Full detail — spec, decisions, test evidence</summary>

`plan.md` · `decision-log.md` · `review/round-1.md`

</details>

Test plan
- [x] `test_save_state_saves_buffer_lines`, `test_state_persistence_special_characters` — new format round-trips correctly
- [x] `state_file_ends_with_state_txt` — path/filename updated everywhere
- [ ] Manual check: upgrading over an existing `state.json` starts with an empty buffer, no crash (pending)
```
