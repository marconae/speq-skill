[speq-skill](../README.md) / [Docs](./index.md) / Decision Log

---

# Decision Log

The decision log captures design choices made during planning — why a direction was taken, what alternatives were considered, and which decisions are significant enough to become permanent Architecture Decision Records (ADRs).

There are two distinct formats:

| Format | Location | Purpose |
|--------|----------|---------|
| Plan-level log | `specs/_plans/<plan-name>/decision-log.md` | Lightweight notes during planning |
| Permanent log | `specs/_decision/NNN-<plan-name>.md` (one fragment per plan) | Curated ADR archive |

---

## Plan-level Decision Log

Created automatically by `planner-agent` during `/speq:plan`. It captures the interview Q&A and design choices in a conversational format.

### Format

```markdown
# Decision Log: <plan-name>

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

<!-- [plan-review]-prefixed entries: populated by planner-agent after resolving a plan-reviewer blocker.
     Unprefixed entries: populated by speq-implement after code review. -->
```

### Rules

- The file is **optional**. If absent, `speq plan validate` passes without mention.
- If present, the H1 must match the plan name exactly: `# Decision Log: <plan-name>`
- At least one of `## Interview`, `## Design Decisions`, or `## Review Findings` must be present.
- `Promotes to ADR:` accepts `yes` or `no`. Any other value produces a warning.

### Validation

```bash
speq plan validate <plan-name>
```

Decision log errors are reported alongside delta spec errors. Decision log warnings are shown on success.

---

## Permanent Decision Log

Lives at `specs/_decision/`, one committed fragment file per plan: `specs/_decision/NNN-<plan-name>.md`. `recorder-agent` writes a fragment during `/speq:record` for every entry marked `Promotes to ADR: yes` in the plan log. Different plans write different files, so parallel plans never conflict on the same text.

Each ADR carries a stable, kebab-case `**ID:**` slug instead of a sequential number. `Supersedes:` and `Status: Superseded by <slug>` reference that slug, so promoting a new ADR never requires editing or renumbering an older one.

### Format

```markdown
# Decisions: <plan-name>

## ADR: <Title>

**ID:** <slug>
**Plan:** <plan-name>
**Status:** Accepted
**Supersedes:** <superseded-slug>

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

- Each fragment's H1 is exactly `# Decisions: <plan-name>`.
- ADR headings follow `## ADR: <Title>` — no numbering, no ordering requirement between fragments.
- Required fields per ADR: `**ID:**`, `**Plan:**`, `**Status:**`, `### Context`, `### Decision`.
- `**ID:**` is a kebab-case slug, unique across every file in `specs/_decision/`.
- `**Status:**` must be one of: `Accepted`, `Deprecated`, `Superseded by <slug>`.
- `**Supersedes:**`, `### Options Considered`, and `### Consequences` are optional.
- `recorder-agent` writes only the new fragment for the plan it is recording — it never edits another fragment.

`recorder-agent` records a one-way forward pointer — `**Supersedes:** <slug>` — on the new ADR only, and never edits the superseded fragment. This forward pointer is the source of truth: a superseded ADR authored through the automated flow keeps `**Status:** Accepted`. The two-way `Status: Superseded by <slug>` form stays valid for hand-authored or migrated entries; the recorder never back-edits a prior fragment.

### Validate vs. Show

Two commands operate on `specs/_decision/`:

| Command | Reads | Writes | Purpose |
|---------|-------|--------|---------|
| `speq decision-log validate` | `specs/_decision/*.md` | Nothing | Structure, required fields, status vocabulary, and slug-reference checks |
| `speq decision-log show` | `specs/_decision/*.md` | Nothing (stdout only) | Prints the assembled `# Architecture Decision Records` view on demand |

`show` never writes a merged file — there is no single permanent-log file to keep in sync. It orders fragments by their numeric `NNN-` prefix, breaking ties by filename; ADRs within a fragment print in the order they were authored.

The `NNN-` prefix is a record-time sequence number, not a date, and is not a global identity — two plans recorded in parallel can legitimately produce the same prefix on different branches. That is a cosmetic tie, not a conflict: slugs, not prefixes, identify ADRs, and `show`'s filename tie-break makes the resulting order deterministic either way.

An absent or empty `specs/_decision/` directory is valid — `validate` passes and `show` prints only the header.

Run either command from the project root:

```bash
speq decision-log validate
speq decision-log show
```

---

## Workflow Integration

```
/speq:plan
  └─ planner-agent creates specs/_plans/<plan-name>/decision-log.md
  └─ plan-reviewer challenges the plan; planner-agent logs resolved
     blockers as "[plan-review]"-prefixed ## Review Findings entries

/speq:implement
  └─ code-reviewer populates ## Review Findings in decision-log.md

/speq:record
  └─ recorder-agent promotes entries marked "Promotes to ADR: yes"
     into a new specs/_decision/NNN-<plan-name>.md fragment

speq decision-log show
  └─ assembles every specs/_decision/*.md fragment into one
     # Architecture Decision Records view, printed to stdout
```

---

## Design Traceability and Human Creative Input

The plan-level log's `Decision / Alternatives / Rationale` structure creates a structured record of the human creative choices made during development. The permanent log provides a timestamped, immutable archive of those choices across the project's lifetime.

> *This is not legal advice. Consult qualified counsel for copyright questions specific to your jurisdiction and use case.*
