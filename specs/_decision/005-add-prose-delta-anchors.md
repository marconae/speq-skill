# Decisions: add-prose-delta-anchors

## ADR: Anchor is the first non-empty line of a delta block; a prose delta wraps the skeleton heading in place

**ID:** anchor-first-nonempty-line
**Plan:** add-prose-delta-anchors
**Status:** Accepted

### Context

A delta block's anchor must identify the section it targets without ambiguity. A whole-block scan for a heading lets a heading-like line inside the prose hijack the anchor.

### Decision

`extract_anchor` reads only the first non-empty line of a delta block. `### Scenario: <name>` and `## Background` match exactly. `# Feature` matches by prefix, matching the leniency of the spec parser at `src/validate/parser.rs:189-190`. A Background or description delta places the marker directly around the `## Background` or `# Feature: <name>` heading the delta file already carries, with no duplicate heading and no new marker syntax.

### Options Considered

| Option | Verdict |
|--------|---------|
| First non-empty line, exact or prefix match | ✓ Chosen — every existing scenario block already starts with its `### Scenario:` heading, so current behavior is preserved |
| Scan every line of the block for the first heading | ✗ Rejected — cannot distinguish the block's own heading from a heading quoted inside the prose |

### Consequences

Anchor detection is unambiguous and needs no new syntax. A delta author places the target heading on the first line of the block. A `DELTA:CHANGED` block on `## Background` or `# Feature: <name>` deletes any line it omits, so authors must copy the current section before editing it.

## ADR: plan validate checks the matrix only, never anchor presence

**ID:** plan-validate-skips-anchor-presence
**Plan:** add-prose-delta-anchors
**Status:** Accepted

### Context

Whether an anchor exists in the target spec can be known only by reading that spec, and another plan may rewrite the target spec before this plan reaches record.

### Decision

`validate_delta_anchors` applies the target-independent rules only, rejecting reading the target spec at validate time because that would read a snapshot that can go stale before record runs. The record command checks anchor presence in the target spec. A delta file that adds a scenario with `DELTA:NEW` and edits it with `DELTA:CHANGED` on the same anchor is an error under this same rule.

## ADR: Recording is all or nothing across the plan

**ID:** record-all-or-nothing-across-plan
**Plan:** add-prose-delta-anchors
**Status:** Accepted

### Context

Turning three previously silent merge cases into errors is unsafe unless the write set is atomic. Otherwise a later error leaves an earlier permanent spec already overwritten, and a re-run after the fix would double-apply or fail.

### Decision

`record_plan` runs in three phases. It checks every delta spec's anchor rules and merges every delta into an in-memory `Vec<(PathBuf, String)>`, then calls `fs::write` only after every check and merge succeeds.

### Options Considered

| Option | Verdict |
|--------|---------|
| Three-phase check, merge, then write | ✓ Chosen — a failing delta leaves every permanent spec unchanged, so a re-run applies each delta exactly once |
| Write each merged spec inside the loop, stop on the first error | ✗ Rejected — leaves earlier specs already overwritten when a later delta fails |

### Consequences

A rejected plan can be fixed and re-recorded safely. The guarantee stops before the write, note-write, and archive phases. A file-system error in those phases is outside the guarantee.
