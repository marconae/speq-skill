[speq-skill](../README.md) / [Docs](./index.md) / Decision Log

---

# Decision Log

The decision log records why a plan chose its design. It has two parts: a plan-level log for every plan, and permanent ADR fragments for the rare durable decision.

| Part | Location | Purpose |
|------|----------|---------|
| Plan-level log | `specs/_plans/<plan-name>/decision-log.md` | Interview answers and design choices of one plan |
| Permanent log | `specs/_decision/NNN-<plan-name>.md` | One fragment per plan, holding its accepted ADRs |

An ADR (Architecture Decision Record) is one durable decision. The decisions of one plan live in that plan's `decision-log.md`. Only the durable ones become ADRs. The `## Context` section of `plan.md` is a bullet list of the problem. It holds no decisions.

---

## Lifecycle

```
/speq:plan     planner-agent writes decision-log.md. An entry marked
               "Promotes to ADR: yes" is an ADR candidate.
               plan-reviewer logs resolved blockers as "[plan-review]" entries.

/speq:record   recorder-agent writes each candidate as an ADR with
               Status: Accepted into specs/_decision/NNN-<plan-name>.md.
               Recording the plan is the acceptance.
```

To drop a candidate, set the entry to `Promotes to ADR: no` and run `/speq:plan` again. In a headless run, comment on the draft PR and run `/speq:plan-pr` again.

## Promotion rules

`/speq:adr-rules` owns the rules. In short, the default is `no`, a `yes` needs a named criterion and a prior `speq decision-log show` search.

## Plan-level log

```markdown
# Decision Log: add-prose-delta-anchors

## Interview

**Q:** Should a delta need new marker syntax to target a Background?
**A:** No. Wrap the existing heading.

## Design Decisions

### [1] Anchor a delta block by its first non-empty line

- **Decision:** A delta block's anchor is its first non-empty line.
- **Alternatives:** Scan the whole block for the first heading
- **Rationale:** A heading quoted inside the prose can take over the anchor.
- **Consequences:** Plans need no new marker syntax.
- **Promotes to ADR:** yes
```

`Alternatives` may read `none`. `Consequences` and `Supersedes` (the slug of the ADR this replaces) are optional.

The file is optional. If present, `speq plan validate` verifies its structure.

## Permanent log

A fragment is one Markdown file. This one comes from the plan `add-prose-delta-anchors`:

```markdown
# Decisions: add-prose-delta-anchors

## ADR: Anchor a delta block by its first non-empty line

**ID:** anchor-first-nonempty-line
**Plan:** add-prose-delta-anchors
**Status:** Accepted

### Context

A delta block must name the section it targets without ambiguity.

### Decision

A delta block's anchor is its first non-empty line. A Background or description delta wraps the existing heading in place, with no new marker syntax.

### Options Considered

- Scan the whole block for the first heading: rejected, a heading quoted inside the prose can take over the anchor.

### Consequences

- Plans need no new marker syntax to target a Background or description.
```

Each ADR needs `**ID:**`, `**Plan:**`, `**Status:**`, `### Context`, and `### Decision`. A later plan can replace this ADR by adding a line `**Supersedes:** anchor-first-nonempty-line` to its own ADR. The old fragment is never edited. `### Options Considered` and `### Consequences` are optional.

## Commands

```bash
speq decision-log validate   # structure, required fields, status values, slug references
speq decision-log show       # prints every fragment as one "Architecture Decision Records" view
```

Both read `specs/_decision/` and write nothing. An absent or empty directory is valid. `speq plan validate <plan-name>` checks the plan-level log.
