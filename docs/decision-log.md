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

## Plan-level decision log

`planner-agent` creates it automatically during `/speq:plan`, capturing the interview Q&A and design choices in a conversational format.

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

## Permanent decision log

Lives at `specs/_decision/`, one committed fragment file per plan: `specs/_decision/NNN-<plan-name>.md`. `recorder-agent` writes a fragment during `/speq:record` for every entry marked `Promotes to ADR: yes` in the plan log.

### Why fragments and slugs

Each plan writes its own fragment, so parallel plans never conflict on the same file. ADRs are identified by a stable, kebab-case slug instead of a sequential number, so promoting a new ADR never requires editing or renumbering an older one.

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
- `recorder-agent` writes only the new fragment for the plan it is recording; it never edits another fragment.
- `recorder-agent` sets `**Supersedes:** <slug>` on the new ADR as a one-way forward pointer — a superseded ADR keeps `**Status:** Accepted`. The two-way `Status: Superseded by <slug>` form applies to hand-authored or migrated entries.

### Validate vs. show

Two commands operate on `specs/_decision/`:

| Command | Reads | Writes | Purpose |
|---------|-------|--------|---------|
| `speq decision-log validate` | `specs/_decision/*.md` | Nothing | Structure, required fields, status vocabulary, and slug-reference checks |
| `speq decision-log show` | `specs/_decision/*.md` | Nothing (stdout only) | Prints the assembled `# Architecture Decision Records` view on demand |

`show` prints an assembled view and writes nothing to disk. It orders fragments by their numeric `NNN-` prefix, breaking ties by filename; ADRs within a fragment print in the order they were authored.

The `NNN-` prefix is a record-time sequence number, not a date or global identity: two plans recorded in parallel on different branches can produce the same prefix. Slugs identify ADRs; `show`'s filename tie-break keeps the printed order deterministic regardless.

An absent or empty `specs/_decision/` directory is valid — `validate` passes and `show` prints only the header.

Run either command from the project root:

```bash
speq decision-log validate
speq decision-log show
```

---

## Workflow integration

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

## Design traceability and human creative input

The plan-level log's `Decision / Alternatives / Rationale` fields record the creative choices made during development. The permanent log preserves them as a timestamped, immutable archive across the project's lifetime.

> [!IMPORTANT]
> *This is not legal advice. Consult qualified counsel for copyright questions specific to your jurisdiction and use case.*
