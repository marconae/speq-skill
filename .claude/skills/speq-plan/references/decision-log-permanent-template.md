# Decisions: <plan-name>

<!-- One fragment per plan. Add one ## ADR block per promoted decision below. Rules: /speq-adr-rules. -->
<!-- ID is a kebab-case slug, unique across every file in specs/_decision. -->
<!-- Supersedes is optional — set it only when this ADR replaces an earlier one. -->

<!--
Two shapes. Never infer either optional section from the entry's prose — emit each
only from what the plan-level decision-log.md entry actually carries:

- Full form: both optional sections present.
  - ### Options Considered ⇐ the entry's Alternatives names a real rejected option
    (not `none` and not empty).
  - ### Consequences ⇐ the entry carries a Consequences line.
- Short form: field block + ### Context + ### Decision, nothing else.
  - Applies when the entry's Alternatives is empty/`none` AND the entry carries no
    Consequences line. Omit both ### Options Considered and ### Consequences entirely
    — do not emit empty or placeholder sections.

The two triggers are independent: an entry can have Options Considered without
Consequences, or the reverse.
-->

## ADR: <Title>

**ID:** <kebab-case-slug>
**Plan:** <plan-name>
**Status:** Accepted <!-- written by /speq-record; recording is the acceptance -->
**Supersedes:** <superseded-slug> <!-- optional; only when this ADR supersedes another; value = the superseded ADR's slug -->

### Context

What problem or situation prompted this decision.

### Decision

The chosen approach.

### Options Considered

<!-- Optional — include only when the entry's Alternatives names a real rejected option. -->

| Option | Verdict |
|--------|---------|
| <chosen option> | ✓ Chosen — <brief rationale> |
| <other option> | ✗ Rejected — <brief reason> |

### Consequences

<!-- Optional — include only when the entry carries a Consequences line. -->

What becomes easier or harder as a result.

<!--
Short-form example (Alternatives: none, no Consequences line):

## ADR: <Title>

**ID:** <kebab-case-slug>
**Plan:** <plan-name>
**Status:** Accepted

### Context

What problem or situation prompted this decision.

### Decision

The chosen approach.
-->
