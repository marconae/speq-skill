---
name: speq-writing-guardrails
description: Nine prose rules covering conclusion-first order, one idea per sentence, named actors, consistent names, plain explanations, no filler, neutral tone, current results only, and no em dashes. Triggered by /speq-mission, /speq-audit, /speq-implement, /speq-plan-pr, /speq-implement-pr, planner-agent, plan-reviewer, recorder-agent, architecture-agent, and code-reviewer.
---

# Writing Guardrails

Follow these rules for governed prose.

## Scope

- Govern: prose in `plan.md`, `spec.md`, `mission.md`, `architecture.md` (project and plan level), decision logs, ADR fragments in `specs/_decision/`, `review-findings.md`, verification reports, implementation summaries, GitHub PRs, issues, and comments.
- Leave unchanged: Gherkin, Background bullets, tables, ASCII diagrams, delta markers, and validator-owned RFC keyword casing.

## Rules

1. Lead with the conclusion. State the decision, result, or finding first. Give reasons afterward.
2. Express one idea per sentence. Split sentences that make separate claims.
3. Name the actor. State who or what performs each action.
4. Name each thing once and reuse that name. Do not switch to a synonym for the same thing later in the text.
5. Assume the reader does not know the method, concept, technology, or system. Explain unfamiliar concepts in simple language before using specialized terms.
6. Remove filler, hedges, superlatives, intensifiers, hyperbole, and promotional language. Use measurable evidence when making claims about scale or quality.
7. Use neutral, matter-of-fact language. Avoid colloquial, metaphorical, dramatic, and rhetorical language.
8. State the current result. Do not narrate attempts, revisions, or reasoning history unless that history is relevant.
9. Do not use em dashes. Use a period, comma, or colon.
