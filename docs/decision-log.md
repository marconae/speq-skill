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

`/speq:adr-rules` owns the rules. In short, the default is `no`, a `yes` needs a named criterion and a prior `speq decision-log show` search, and one ADR records one decision without paths, signatures, or flags.

## Plan-level log

```markdown
# Decision Log: <plan-name>

## Interview

**Q:** <question>
**A:** <answer>

## Design Decisions

### [N] <short title>

- **Decision:** What was decided
- **Alternatives:** What was rejected (may read `none`)
- **Rationale:** Why
- **Consequences:** (optional) Effects and folded-in corollaries
- **Supersedes:** (optional) Slug of the ADR this replaces
- **Promotes to ADR:** yes / no
```

The file is optional. If present, `speq plan validate` requires the H1 `# Decision Log: <plan-name>` and at least one `##` section. A `Promotes to ADR:` value other than `yes` or `no` produces a warning.

## Permanent log

```markdown
# Decisions: <plan-name>

## ADR: <Title>

**ID:** <slug>
**Plan:** <plan-name>
**Status:** Accepted
**Supersedes:** <slug>

### Context
### Decision
### Options Considered
### Consequences
```

- `**ID:**` is a kebab-case slug, unique across all fragments. Slugs identify ADRs, not numbers.
- `**Status:**` is `Accepted`, `Deprecated`, or `Superseded by <slug>`.
- `**Supersedes:**`, `### Options Considered`, and `### Consequences` are optional. `recorder-agent` writes the last two only when the plan entry has real content for them.
- To change an ADR, record a new one with `**Supersedes:**`. The old fragment is never edited.
- Each plan writes its own fragment, so parallel plans do not conflict. The `NNN-` prefix is a record-time sequence number, not an identity.

## Commands

```bash
speq decision-log validate   # structure, required fields, status values, slug references
speq decision-log show       # prints every fragment as one "Architecture Decision Records" view
```

Both read `specs/_decision/` and write nothing. An absent or empty directory is valid. `speq plan validate <plan-name>` checks the plan-level log.
