[speq-skill](../README.md) / [Docs](./index.md) / Decision Log

---

# Decision Log

The decision log records design choices made during planning:

- why the team chose a direction for the plan
- which alternatives it considered
- which decisions are significant enough for a permanent Architecture Decision Record (ADR)

There are two distinct formats:

| Format | Location | Purpose |
|--------|----------|---------|
| Plan-level log | `specs/_plans/<plan-name>/decision-log.md` | Lightweight notes during planning |
| Permanent log | `specs/_decision/NNN-<plan-name>.md` (one fragment per plan) | Curated ADR archive |

---

## Plan-level decision log

`planner-agent` creates the plan-level decision log automatically during `/speq:plan`. The log records the interview questions and answers, and the design choices. It uses a conversational format.

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

- The file is **optional**. If the file is absent, `speq plan validate` passes without a message.
- If the file is present, the H1 must match the plan name exactly: `# Decision Log: <plan-name>`.
- At least one of `## Interview`, `## Design Decisions`, or `## Review Findings` must be present.
- `Promotes to ADR:` accepts `yes` or `no`. Any other value produces a warning.

### Validation

```bash
speq plan validate <plan-name>
```

`speq plan validate` reports decision log errors alongside delta spec errors. It prints decision log warnings on success.

---

## Permanent decision log

The permanent decision log lives at `specs/_decision/`. It stores one committed fragment file per plan: `specs/_decision/NNN-<plan-name>.md`. `recorder-agent` writes a fragment during `/speq:record` for every entry marked `Promotes to ADR: yes` in the plan log.

### Why fragments and slugs

Each plan writes its own fragment, so parallel plans never conflict on the same file. A stable, kebab-case slug identifies each ADR instead of a sequential number. When `recorder-agent` promotes a new ADR, it does not need to edit or renumber an older one.

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

- The H1 of each fragment is exactly `# Decisions: <plan-name>`.
- ADR headings follow the form `## ADR: <Title>`. Fragments need no numbering and no fixed order between them.
- Each ADR must have these fields: `**ID:**`, `**Plan:**`, `**Status:**`, `### Context`, and `### Decision`.
- `**ID:**` is a kebab-case slug. It must be unique across every file in `specs/_decision/`.
- `**Status:**` must be one of: `Accepted`, `Deprecated`, `Superseded by <slug>`.
- `**Supersedes:**`, `### Options Considered`, and `### Consequences` are optional.
- `recorder-agent` writes only the new fragment for the plan that it records. It never edits another fragment.
- `recorder-agent` sets `**Supersedes:** <slug>` on the new ADR as a one-way forward pointer. A superseded ADR keeps `**Status:** Accepted`. The two-way form, `Status: Superseded by <slug>`, applies only to hand-authored or migrated entries.

### Validate vs. show

Two commands operate on `specs/_decision/`:

| Command | Reads | Writes | Purpose |
|---------|-------|--------|---------|
| `speq decision-log validate` | `specs/_decision/*.md` | Nothing | Structure, required fields, status vocabulary, and slug-reference checks |
| `speq decision-log show` | `specs/_decision/*.md` | Nothing (stdout only) | Prints the assembled `# Architecture Decision Records` view on demand |

`show` prints an assembled view and writes nothing to disk. It orders fragments by their numeric `NNN-` prefix and breaks ties by filename. Within a fragment, ADRs print in the order that they were written.

The `NNN-` prefix is a record-time sequence number, not a date or a global identity. Two plans recorded in parallel on different branches can produce the same prefix. Slugs identify each ADR. Even when two prefixes match, the filename tie-break in `show` keeps the printed order deterministic.

An absent or empty `specs/_decision/` directory is valid. `validate` passes, and `show` prints only the header.

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

The `Decision / Alternatives / Rationale` fields of the plan-level log record the creative choices made during development. The permanent log preserves these choices as a timestamped, immutable archive of the project.

> [!IMPORTANT]
> *This is not legal advice. Qualified counsel can answer copyright questions specific to your jurisdiction and use case.*
