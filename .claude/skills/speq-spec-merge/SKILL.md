---
name: speq-spec-merge
description: Delta-merge procedure — DELTA marker semantics, library-organization thresholds, and ADR-promotion field mapping. Triggered by recorder-agent.
---

# Spec Merge

## Load Plan Context

```
Read: specs/_plans/<plan-name>/plan.md
List: specs/_plans/<plan-name>/**/spec.md
```

Run `speq feature list` to see the current permanent spec library.

## Apply Deltas

For each delta spec at `specs/_plans/<plan-name>/<domain>/<feature>/spec.md`:

```
specs/<domain>/<feature>/spec.md exists?
├─ No  → Copy delta (strip markers)
└─ Yes → Merge using markers below
```

| Marker | Action |
|--------|--------|
| `DELTA:NEW` | Append scenario |
| `DELTA:CHANGED` | Replace scenario with same name |
| `DELTA:REMOVED` | Delete scenario with same name |

After each merge:
1. Strip all `<!-- DELTA:* -->` markers
2. Validate: `speq feature validate <domain>/<feature>`
3. If validation fails, stop and report. Do not guess fixes

## Check Library Thresholds

After all merges:

| Metric | Threshold | Action |
|--------|-----------|--------|
| Scenarios per spec | >10 | Return to orchestrator for user decision |
| Domain features | >8 | Return to orchestrator for user decision |

**Never assume:** library reorganization is a user decision. Return a concrete question to the orchestrator.

## Promote ADRs to Permanent Decision Log

Read `specs/_plans/<plan-name>/decision-log.md` (if it exists).

For each entry where `Promotes to ADR: yes`:

1. Convert to ADR format using `/speq-plan`'s `references/decision-log-permanent-template.md`:
   - **Title** — from the decision entry heading
   - **ID** — a kebab-case slug derived from the title; MUST be unique across every file in `specs/_decision/`
   - **Plan** — `<plan-name>`
   - **Status** — `Accepted`
   - **Supersedes** (optional) — if the entry names an earlier decision it replaces, set this to that decision's existing ADR slug
   - **Context** — synthesized from the entry's Rationale + Alternatives
   - **Decision** — from the entry's Decision bullet
   - **Options Considered** — from the entry's Alternatives bullet
   - **Consequences** — brief inference from Rationale

2. Write ONE new fragment file `specs/_decision/NNN-<plan-name>.md`:
   - NNN = (count of existing files in `specs/_decision/`) + 1, zero-padded to 3 digits
   - H1: `# Decisions: <plan-name>`
   - Emit one `## ADR: <Title>` block per promoted entry, in decision-log order
   - MUST NOT edit any other file in `specs/_decision/` — a supersede reference is a one-way pointer from the new ADR's `**Supersedes:**` field to the target slug

3. Validate: `speq decision-log validate`

If `decision-log.md` is absent or has no "Promotes to ADR: yes" entries, skip silently.

## Finalize

1. Final validation: `speq feature validate`
2. Archive: `mv specs/_plans/<plan-name> specs/_recorded/NNN-<plan-name>`, where NNN = (count of existing entries in `specs/_recorded/`) + 1, zero-padded to 3 digits
3. If `specs/_recorded/NNN-<plan-name>/tasks.md` contains a `## PR Lifecycle` section, set `- [x] recorded` there. Section absent → skip silently (the plan did not run through the headless pipeline). Writing the mark in the same step as the `mv` keeps the crash window minimal — the actor that archives records that it archived.

If a threshold is exceeded, return BEFORE archiving and ask the orchestrator to clarify with the user.

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Merging without running validator | Broken specs may land |
| Assuming split/domain reorganization | User must decide |
| Rewriting scenario wording during merge | Recording is a mechanical operation |
| Leaving DELTA markers | Pollutes permanent specs |
