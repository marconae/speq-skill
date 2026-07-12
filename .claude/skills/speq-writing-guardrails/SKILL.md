---
name: speq-writing-guardrails
description: Prose guardrails for speq artifacts and GitHub PRs, issues, and comments — front-loaded (BLUF), terse, unambiguous writing anchored in established style and requirements standards. Triggered by /speq-mission, /speq-audit, /speq-implement, /speq-plan-pr, /speq-implement-pr, planner-agent, plan-reviewer, and recorder-agent.
---

# Writing Guardrails

Prose anchored in **BLUF / Inverted Pyramid**, **Strunk & White**, **INCOSE GtWR**, **ISO/IEC/IEEE 29148**, and **RFC 2119 / 8174**.

## Scope

**Governed (free prose — fix these):**
- `plan.md` — Summary, Goals, Non-Goals, Migration, Testing, Dead-Code prose
- `spec.md` — the Feature-description line under `# Feature:`
- `mission.md` — Problem, Architecture, Constraints prose
- decision-log — Rationale and Finding prose
- verification-report — Notes and Review-Summary bullets
- every GitHub PR, issue, and comment body and title

**Not governed (leave alone):** Gherkin scenarios, Background bullets, tables, ASCII diagrams, delta markers, RFC-2119 keyword casing (validator-owned).

## Structure first — BLUF / Inverted Pyramid

Anchored in **NN/g F-pattern** and **Anthropic context-engineering**.

- **Lead with the conclusion.** State the decision or outcome in the first sentence.
- **One idea per paragraph.** Split a paragraph that carries two.
- **Write statement headings that form a scan path.** Prefer `## Rebuild the index` over `## Migration`.
- **Make each section self-contained.** Agents retrieve sections out of order.

## Cut verbosity — "Omit needless words"

Anchored in **Strunk & White** and **Zinsser**.

- **Cap sentences at 25 words; paragraphs at 3–7 lines.** The Summary two-sentence cap is hard.
- **Start statements with a verb.** Delete `you can`, `there is`, `it is worth noting`.
- **Cut adverbs, qualifiers, and hedges.** Delete `very`, `for now`, and non-committal time hedges.
- **Ban filler:** `basically`, `simply`, `obviously`, `just`, `actually`.

## Kill ambiguity — INCOSE GtWR + ISO/IEC/IEEE 29148

Target: unambiguous, singular, verifiable, complete.

- **Quantify vague terms.** Write `within 2.0 s`, not `fast`.
- **Delete escape clauses:** `as appropriate`, `where possible`, `etc.`, and the `/` in `and/or`.
- **Use one term per concept.** Do not alternate names for one thing.
- **Use active voice and name the actor.** Write `speq record merges the delta`.
- **Repeat the noun over a distant `it` or `this`.** Pronouns lose their referent across sentences.

## Word swaps

| Write | Not |
|-------|-----|
| use | utilize |
| to | in order to |
| because | due to the fact that |
| now | at this point in time |
| can | has the ability to |
| some | a number of |
| if | in the event that |

## Register: descriptive vs normative — RFC 2119 / 8174

- **Descriptive prose** — warm, concise, present tense.
- **Normative statements** — ALL-CAPS `MUST` / `SHOULD` / `MAY`, used sparingly.
- **Lowercase `should` is non-binding.** Only the ALL-CAPS keyword carries obligation.
