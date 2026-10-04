---
name: speq-spec-merge
description: Delta-merge procedure — DELTA marker semantics, library-organization thresholds, and ADR-promotion field mapping. Triggered by recorder-agent.
---

# Spec Merge

## Load Plan Context

```
Read: specs/_plans/<plan-name>/plan.md
List: specs/_plans/<plan-name>/**/spec.md
Read: specs/_plans/<plan-name>/architecture.md (if it exists)
```

Run `speq feature list` to see the current permanent spec library.

If `architecture.md` exists in the plan, run steps 1 and 2 of "Apply Architecture Delta" before any feature merge.

## Apply Deltas

For each delta spec at `specs/_plans/<plan-name>/<domain>/<feature>/spec.md`:

```
specs/<domain>/<feature>/spec.md exists?
├─ No  → Copy delta (strip markers)
└─ Yes → Merge using markers below
```

A delta block's anchor is the first non-empty line inside the block. `### Scenario: <name>` and `## Background` match exactly; `# Feature: <name>` matches by the prefix `# Feature`, so a description delta MAY rename the feature.

**Scenario-anchor markers** (`### Scenario: <name>` wrapped in the marker):

| Marker | Action |
|--------|--------|
| `DELTA:NEW` | Append scenario |
| `DELTA:CHANGED` | Replace scenario with same name |
| `DELTA:REMOVED` | Delete scenario with same name |

**Prose-anchor markers** (`## Background` or `# Feature: <name>` wrapped in the marker):

| Marker | Action |
|--------|--------|
| `DELTA:CHANGED` | Replace the whole section (heading plus body) in place. Any line the block omits is deleted from the permanent spec |
| `DELTA:NEW` | Rejected — Background and the description are required sections that always exist |
| `DELTA:REMOVED` | Rejected — removing a required section produces an invalid spec |

After each merge:
1. Strip all `<!-- DELTA:* -->` markers
2. Validate: `speq feature validate <domain>/<feature>`
3. If validation fails, stop and report. Do not guess fixes

## Apply Architecture Delta

The optional file `specs/_plans/<plan-name>/architecture.md` changes `specs/architecture.md`. The `speq` CLI does not see it, so you apply it by hand. Its format is defined in `/speq-plan`'s `references/architecture-delta-template.md`. The target format is defined in `/speq-plan`'s `references/architecture-template.md`.

1. **Pre-check (read-only), before any feature merge.**
   - No delta file: report `Architecture: no delta` and skip the rest of this section.
   - Otherwise check: the H1 is `# Architecture Delta: <plan-name>`, DELTA markers are balanced, each block's first non-empty line is `## <Section>` and is its only `##` line, no two blocks share an anchor, and only legal kinds are used (NEW and REMOVED on non-canonical sections only).
   - Check that `specs/architecture.md` exists, that each CHANGED and REMOVED anchor exists in it, and that no NEW anchor exists in it.
   - Check the line rules: outside blocks only the H1, the BASE comment, and blank lines. Inside blocks only `- ` bullets, fenced blocks, and blank lines. No `###`, no tables, no prose paragraphs.
2. **Base check.** Run `git hash-object specs/architecture.md` and compare it with the BASE comment. If they match, continue. If they differ, run `git cat-file -p <BASE>` and compare each CHANGED and REMOVED section with the same section in the current file. A differing section, or a missing blob, stops the record with a stale-base error. Both commands are read-only and fit `/speq-git-discipline`. A stale BASE would overwrite another plan's changes.
3. **Feature merges.** Run "Apply Deltas" as usual.
4. **Merge.** Build the merged text in memory. CHANGED replaces the heading and body up to the next `##`, and any line the block omits is deleted. NEW appends the section at the end. REMOVED deletes the section. Strip all DELTA markers. Write the file once.
5. **Post-check.** The H1 is `# Architecture`. The six canonical sections appear once each, in order. No `##` heading is duplicated. No DELTA text is left. The line rules hold. On failure, restore the original text, report the failure, and do not archive.
6. **Re-entry.** Check this before step 1. A threshold respawn runs this section again after the delta was applied. The delta is applied when each CHANGED section in `specs/architecture.md` already equals its block body, each NEW section exists with its block body, and each REMOVED section is gone. In that case skip steps 1, 2, 4, and 5, run step 3, and report `Architecture: already merged`.
7. **Continue** with the thresholds, ADR promotion, and Finalize. The `mv` in Finalize archives the delta with the plan.

Failure format: `Recording failed: architecture delta: architecture.md:<line>: <rule>`.

Report line: `Architecture: § Components (CHANGED), § Deployment (NEW)`, or `Architecture: no delta`. List the lines each CHANGED block removed, so the user can spot an accidental deletion.

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
   - **Status** — `Accepted`. Recording the plan is the acceptance, per `/speq-adr-rules`
   - **Supersedes** (optional) — if the entry carries a `Supersedes` line, copy that slug. Check that it names an existing ADR slug
   - **Context** — one or two sentences from the entry's Rationale: the situation that forces the decision, stated as present fact. No history of how the plan reached it, per `/speq-adr-rules` rule 7a. Do not copy Rationale or Alternatives prose
   - **Decision** — from the entry's Decision bullet, in one to three sentences
   - **Options Considered** — emit this section ONLY when the entry's Alternatives names a real rejected option (not `none`, not empty). Never infer it from prose elsewhere in the entry
   - **Consequences** — emit this section ONLY from the entry's own Consequences line, when present. Never infer it from Rationale or any other field
   - The entry's `Architecture` field is not mapped into the ADR. It states which sections of `specs/architecture.md` the plan changes, and the merge in "Apply Architecture Delta" already applied them
   - When the entry carries neither (Alternatives is empty/`none` and there is no Consequences line), the assembled ADR is short-form: field block + `### Context` + `### Decision` only — no `### Options Considered`, no `### Consequences`

2. Write ONE new fragment file `specs/_decision/NNN-<plan-name>.md`:
   - NNN = (count of existing files in `specs/_decision/`) + 1, zero-padded to 3 digits
   - H1: `# Decisions: <plan-name>`
   - Emit one `## ADR: <Title>` block per promoted entry, in decision-log order
   - MUST NOT edit any other file in `specs/_decision/` — a supersede reference is a one-way pointer from the new ADR's `**Supersedes:**` field to the target slug

3. Validate: `speq decision-log validate`

If `decision-log.md` is absent or has no "Promotes to ADR: yes" entries, skip silently.

## Finalize

1. Final validation: `speq feature validate`. If an architecture delta was applied, confirm the post-check of "Apply Architecture Delta" passed
2. Archive: `mv specs/_plans/<plan-name> specs/_recorded/NNN-<plan-name>`, where NNN = (count of existing entries in `specs/_recorded/`) + 1, zero-padded to 3 digits
3. If `specs/_recorded/NNN-<plan-name>/tasks.md` contains a `## PR Lifecycle` section, set `- [x] recorded` there. Section absent → skip silently (the plan did not run through the headless pipeline). Writing the mark in the same step as the `mv` keeps the crash window minimal — the actor that archives records that it archived.

If a threshold is exceeded, return BEFORE archiving and ask the orchestrator to clarify with the user.

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Rewriting scenario wording during merge | Recording is a mechanical operation |
| `DELTA:CHANGED` or `DELTA:REMOVED` naming a scenario absent from the target spec | Rejected — `record` errors instead of silently merging nothing |
| Heading left outside the marker, so the block's first line is not a recognized anchor | Rejected — `record` cannot tell what the block targets |
| Two delta blocks of one file sharing an anchor, whatever their marker kinds | Rejected — the merged result would depend on block order; write one block carrying the final text |
| Writing `Status: Proposed` on promotion | Rejected — `/speq-record` accepts the ADR, per `/speq-adr-rules` |
| `DELTA:NEW` or `DELTA:REMOVED` on a canonical architecture section | Rejected — canonical sections always exist; use `DELTA:CHANGED` |
| Editing `specs/architecture.md` beyond the delta | Recording is a mechanical operation |
