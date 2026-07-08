# Decisions: plan-a

## ADR: Use line scanner

**ID:** use-line-scanner
**Plan:** plan-a
**Status:** Accepted

### Context

Field detection must anchor to the line start. A prose mention of `**ID:**` or `**Status:**` inside backticks describes the format and MUST NOT be parsed as a real field.

### Decision

Match `**Status:**` and `**ID:**` only when the trimmed line begins with the marker, so this backtick-quoted `**Status:**` reference does not override the real `Accepted` value above.
