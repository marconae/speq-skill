[speq-skill](../README.md) / [Docs](./index.md) / Decision Log

---

# Decision Log

The decision log records design choices made during planning:

- why the team chose a direction for the plan
- which alternatives it considered
- which decisions are durable enough for a permanent Architecture Decision Record (ADR)

There are two distinct formats:

| Format | Location | Purpose |
|--------|----------|---------|
| Plan-level log | `specs/_plans/<plan-name>/decision-log.md` | Lightweight notes during planning |
| Permanent log | `specs/_decision/NNN-<plan-name>.md` (one fragment per plan) | Curated ADR archive |

ADRs are rare. The `## Design` section of `plan.md` is not an ADR: it describes how one plan builds its change.

---

## Plan-level decision log

`planner-agent` creates the plan-level decision log automatically during `/speq:plan`. The log records the interview questions and answers, and the design choices. It uses a conversational format.

### Promotion gate

`/speq:adr-rules` owns the rules. In short:

- The default is `Promotes to ADR: no`. The expected count of `yes` entries per plan is zero. Two or more get re-judged.
- `yes` needs one criterion: the decision affects multiple components or teams, sets a long-lived constraint, is a major technology, API, persistence, security, or deployment choice, or rejects a plausible alternative. The entry's Rationale names the criterion.
- Search first. Run `speq decision-log show` and state the result in Rationale. To change an existing ADR, write a new one with a `Supersedes:` line.
- Some content is never an ADR: naming, file placement, one-off fixes, scope trims, implementation detail, and plan-specific workarounds. Conventions go to `CLAUDE.md`, `AGENTS.md`, a `.speq/*-hook.md` file, or mission Constraints. HOW this plan builds goes to `plan.md` `## Design`. WHAT the system is goes to `specs/architecture.md`, changed through an architecture delta. An ADR records why. `specs/architecture.md` records what is.
- One ADR records one decision. Decision holds no signatures, paths, or flags.
- A corollary of an already-promoted decision is not its own entry. It is a bullet in that parent entry's `Consequences` line.

### Vocabulary

| Term | Meaning |
|------|---------|
| ADR | A permanent record in `specs/_decision/` of one durable decision |
| Design section | The `## Design` part of `plan.md`. It is not an ADR |
| Decision-log entry | A note in a plan's `decision-log.md`. Only `Promotes to ADR: yes` entries can become ADRs |
| ADR candidate | An entry marked `Promotes to ADR: yes`. It is the proposal, shown when the plan is ready |
| Accepted | An ADR written by `/speq:record`. Recording the plan is the acceptance |

### Format

```markdown
# Decision Log: <plan-name>

## Interview

**Q:** <question the agent asked>
**A:** <your answer>

## Design Decisions

### [N] <short title>

- **Decision:** What was decided
- **Alternatives:** What was considered and not chosen (may read `none`)
- **Rationale:** Why this direction
- **Consequences:** (optional) Effects, trade-offs, or corollary decisions folded in here — omitted when there are none
- **Supersedes:** (optional) Slug of the ADR this decision replaces
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

The permanent decision log lives at `specs/_decision/`. It stores one committed fragment file per plan: `specs/_decision/NNN-<plan-name>.md`. `recorder-agent` writes a fragment during `/speq:record` for every entry marked `Promotes to ADR: yes` in the plan log. It writes each ADR with `Status: Accepted`.

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

#### Full form vs. short form

`### Options Considered` and `### Consequences` each have an independent trigger: `recorder-agent` emits `### Options Considered` only when the plan-level entry's Alternatives names a real rejected option, and `### Consequences` only when the entry carries a Consequences line. It never infers either section from prose elsewhere in the entry.

- **Full form**: both optional sections present — field block, `### Context`, `### Decision`, `### Options Considered`, `### Consequences`.
- **Short form**: neither trigger fires (Alternatives is empty/`none` and there is no Consequences line) — the ADR is just the field block, `### Context`, and `### Decision`.
- `recorder-agent` sets `**Supersedes:** <slug>` on the new ADR as a one-way forward pointer. A superseded ADR keeps `**Status:** Accepted`. The two-way form, `Status: Superseded by <slug>`, applies only to hand-authored or migrated entries.

### Status lifecycle

A decision-log entry with `Promotes to ADR: yes` is a proposal. The plan skill prints it as an ADR candidate. `/speq:record` is the acceptance and writes `Status: Accepted`. To drop a proposal, set the entry to `Promotes to ADR: no` and run `/speq:plan` again. In a headless run, comment on the draft PR and run `/speq:plan-pr` again with the PR number. Running `/speq:implement-pr` accepts the candidates on that PR.

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
  └─ recorder-agent accepts entries marked "Promotes to ADR: yes"
     as Status: Accepted in a new specs/_decision/NNN-<plan-name>.md fragment

speq decision-log show
  └─ assembles every specs/_decision/*.md fragment into one
     # Architecture Decision Records view, printed to stdout
```

---

## Design traceability and human creative input

The `Decision / Alternatives / Rationale` fields of the plan-level log record the creative choices made during development. The permanent log preserves these choices as a timestamped, immutable archive of the project.

> [!IMPORTANT]
> *This is not legal advice. Qualified counsel can answer copyright questions specific to your jurisdiction and use case.*
