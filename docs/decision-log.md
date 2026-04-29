[speq-skill](../README.md) / [Docs](./index.md) / Decision Log

---

# Decision Log

The decision log captures design choices made during planning — why a direction was taken, what alternatives were considered, and which decisions are significant enough to become permanent Architecture Decision Records (ADRs).

There are two distinct formats:

| Format | Location | Purpose |
|--------|----------|---------|
| Plan-level log | `specs/_plans/<plan-name>/decision-log.md` | Lightweight notes during planning |
| Permanent log | `specs/decision-log.md` | Curated ADR archive |

---

## Plan-level Decision Log

Created automatically by `planner-agent` during `/speq:plan`. It captures the interview Q&A and design choices in a conversational format.

### Format

```markdown
# Decision Log: <plan-name>

Date: YYYY-MM-DD

## Interview

**Q:** <question the agent asked>
**A:** <your answer>

## Design Decisions

### [N] <short title>

- **Decision:** What was decided
- **Alternatives:** What was considered and not chosen
- **Rationale:** Why this direction
- **Promotes to ADR:** yes / no

## Review Findings

<!-- Populated by speq-implement after code review. -->
```

### Rules

- The file is **optional**. If absent, `speq plan validate` passes without mention.
- If present, the H1 must match the plan name exactly: `# Decision Log: <plan-name>`
- A `Date:` line is required.
- At least one of `## Interview`, `## Design Decisions`, or `## Review Findings` must be present.
- `Promotes to ADR:` accepts `yes` or `no`. Any other value produces a warning.

### Validation

```bash
speq plan validate <plan-name>
```

Decision log errors are reported alongside delta spec errors. Decision log warnings are shown on success.

---

## Permanent Decision Log

Lives at `specs/decision-log.md`. Built incrementally by `recorder-agent` during `/speq:record`: entries marked `Promotes to ADR: yes` in the plan log are promoted here as sequential ADRs.

### Format

```markdown
# Architecture Decision Records

## ADR-001: <Title>

**Date:** YYYY-MM-DD
**Plan:** <plan-name>
**Status:** Accepted

### Context

Why this decision was needed.

### Decision

What was decided.

### Options Considered

(optional) What was considered and not chosen.

### Consequences

(optional) Trade-offs and follow-on effects.
```

### Rules

- H1 must be exactly `# Architecture Decision Records`.
- ADR headings follow `## ADR-NNN: <Title>` — sequential, no gaps, starting at `ADR-001`.
- Required fields per ADR: `**Date:**`, `**Plan:**`, `**Status:**`, `### Context`, `### Decision`.
- `**Status:**` must be one of: `Accepted`, `Superseded by ADR-NNN`, `Deprecated`.
- `### Options Considered` and `### Consequences` are optional.

### Validation

```bash
speq decision-log validate
```

Run from the project root. Reads `specs/decision-log.md`. Reports errors for structural violations; exits non-zero on failure.

---

## Workflow Integration

```
/speq:plan
  └─ planner-agent creates specs/_plans/<plan-name>/decision-log.md

/speq:implement
  └─ code-reviewer populates ## Review Findings in decision-log.md

/speq:record
  └─ recorder-agent promotes entries marked "Promotes to ADR: yes"
     into specs/decision-log.md as the next ADR
```

---

## Design Traceability and Human Creative Input

The plan-level log's `Decision / Alternatives / Rationale` structure creates a structured record of the human creative choices made during development. The permanent log provides a timestamped, immutable archive of those choices across the project's lifetime.

> *This is not legal advice. Consult qualified counsel for copyright questions specific to your jurisdiction and use case.*
