---
name: speq-writing-guardrails
description: Dense prose rules for speq artifacts and GitHub text: sentence caps, BLUF, consistent vocabulary, evidence, and no em dashes. Triggered by /speq-mission, /speq-audit, /speq-implement, /speq-plan-pr, /speq-implement-pr, planner-agent, plan-reviewer, and recorder-agent.
---

# Writing Guardrails

Prose anchored in **BLUF / Inverted Pyramid** structure and **ASD-STE100** controlled-language discipline. Follow these concepts for governed prose. Write for readers with varied English proficiency. Make each sentence clear on the first reading.

## Scope

- Govern: prose in `plan.md`, `spec.md`, `mission.md`, decision logs, verification reports, GitHub PRs, issues, and comments.
- Leave unchanged: Gherkin, Background bullets, tables, ASCII diagrams, delta markers, and validator-owned RFC keyword casing.
- PR text: lead with tradeoffs and boundaries; tie claims to diffs, files, tests, or command output.

## Rules

- Classify each passage as procedural or descriptive.
- One sentence states one idea. Split a sentence joined by `and`, `which`, or `while` into two.
- Procedural: imperative, one instruction per sentence, maximum 20 words.
- Descriptive: simple tense, one fact per sentence, one topic per paragraph, maximum 25 words and six sentences per paragraph.
- Put the conclusion first. Make headings summarize their sections. Make each section stand alone.
- Use active voice and name the actor. Use verbs, not nominalizations.
- Put conditions before commands: `If the build fails, read the log.`
- Use complete grammar. Keep articles and `that`. Use no perfect or progressive forms.
- Use no semicolons or contractions.
- Replace weak requirements: `should` → `MUST` or a fact. Replace hypothetical modals with conditions. Keep uncertainty when evidence requires it.
- Use `MUST`, `SHOULD`, and `MAY` only as RFC 2119 / 8174 normative keywords.
- Label destructive instructions: `WARNING` for injury, `CAUTION` for data or equipment damage. State the consequence after the instruction.
- Use one term, meaning, word class, and verb for each concept. Repeat nouns when pronouns lose the referent.
- Remove filler, hype, vague claims, hedges, escape hatches, and `and/or`. Quantify vague terms.
- Use one verb per concept: `check`, `run`, and `show`, or another chosen set.
- Replace: `utilize` → `use`; `in order to` → `to`; `prior to` → `before`; `ensure` → `make sure that`; `facilitate` → `help`; `e.g.` → `for example`; `i.e.` → `that is`; `etc.` → named items; `functionality` → `function` or `feature`; `out of the box` → `by default`; `under the hood` → `internally`.
- Keep technical names, product names, identifiers, commands, flags, paths, config keys, code, quoted errors, and numbers with units unchanged. Count each as one word.
- Do not claim project behavior without evidence. Scope unsupported claims, label inferences, and name untested areas.
- Ban em dashes. Use a comma, period, colon, parentheses, or a new sentence.

## Delivery Check

- Count procedural and descriptive sentences against their caps.
- Scan for weak modals, contractions, semicolons, and em dashes.
- Move each condition before its command.
- Check vocabulary consistency, active voice, actors, filler, and unsupported claims.
- Rerun the checks after every correction.

## External Boundaries

- These rules are informed by ASD-STE100 Simplified Technical English. They do not claim ASD-STE100 compliance.
- Do not reproduce the ASD-STE100 standard, controlled dictionary, official examples, logos, or branding.
- ASD-STE100 is a registered trademark of ASD. This skill is not affiliated with or endorsed by ASD or STEMG.
- RFC 2119 and RFC 8174 identify normative-keyword conventions. This skill reproduces no RFC text.
