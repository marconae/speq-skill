[speq-skill](../README.md) / [Docs](./index.md) / Workflow

---

# Workflow Guide

A one-time `/speq:mission` to setup, then a repeating Plan → Implement → Record cycle.

```
/speq:mission → specs/mission.md, specs/architecture.md  (once per project)
                       │
/speq:plan    →  /speq:implement  →  /speq:record     (repeat)
```

## Set up the project

Run `/speq:mission` once per project.

- Interviews you about the project's purpose
- Writes `specs/mission.md` and `specs/architecture.md`
- For an existing codebase, drafts the architecture from the code. You confirm or correct it

## Plan a feature, change or fix

Run `/speq:plan <intent>`.

- Interviews you about the change
- Writes a plan in `specs/_plans/<plan-name>/`: `plan.md`, spec deltas, `decision-log.md`, and an optional `architecture.md` delta
- Has a reviewer challenge the plan before it reaches you
- Shows any proposed ADR, so you can drop it

## Implement it

Run `/speq:implement <plan-name>`.

- Splits the plan into tasks and hands them to sub-agents
- Reviews the code the sub-agents wrote
- Runs the build, tests, and lint checks
- Writes `verification-report.md`

## Record it

Run `/speq:record <plan-name>`.

- Merges the spec deltas into `specs/`
- Accepts the proposed ADRs
- Merges the architecture delta, if the plan has one
- Archives the plan to `specs/_recorded/NNN-<plan-name>/`

## Check the library

Run `/speq:audit` to check structure, ADRs, mission drift, and unrecorded plans.

## Good to know

Plan names follow `<verb>-<scope>`, with the verb `add`, `change`, `remove`, `refactor`, or `fix`.

`/speq:record` stops and asks you when a feature has more than 10 scenarios or a domain has more than 8 features. It never reorganizes without your decision.

For how to add features and start greenfield or brownfield projects, see the [FAQ](./faq.md). To run plan and implement unattended on a branch with a PR, see [Headless PR Pipeline](./headless-workflow.md). For decisions and ADRs, see [Decision Log](./decision-log.md). For model choices, see [Model Routing](./model-routing.md).
