[speq-skill](../README.md) / [Docs](./index.md) / Semantic Anchors

---

# Semantic Anchors

## What are semantic anchors

Semantic anchors are named references to established methodologies, frameworks, and practices, embedded directly in skill instructions. A skill does not re-explain a methodology from scratch. Instead, the skill names the methodology, for example "London School TDD" or "BLUF". The skill then relies on the training data that the model already has for that practice. A single anchor like "Socratic Method" activates more detailed behavior than paragraphs of custom instruction.

For more information, see the [LLM-Coding/Semantic-Anchors](https://github.com/LLM-Coding/Semantic-Anchors) catalog on GitHub.

## Anchor reference

| Anchor | Skill | Category |
|--------|-------|----------|
| Information Foraging | speq-cli | Search strategy |
| Clean Code (Martin) | speq-code-guardrails | Code quality |
| London School TDD | speq-code-guardrails | Testing |
| SOLID | speq-code-guardrails | Design principles |
| KISS | speq-code-guardrails | Design principles |
| DRY | speq-code-guardrails | Design principles |
| YAGNI | speq-code-guardrails | Design principles |
| Law of Demeter | speq-code-guardrails | Design principles |
| Boy Scout Rule | speq-code-guardrails | Code quality |
| Five Whys | speq-code-guardrails | Root cause analysis |
| Conventional Commits | speq-git-discipline | Version control |
| Work Breakdown Structure | speq-implement | Task decomposition |
| BLUF (Bottom Line Up Front) | speq-implement | Reporting |
| Socratic Method | speq-mission | Interview |
| User Story Mapping (Patton) | speq-mission | Requirements |
| MECE Partitioning | speq-mission | Problem structuring |
| Socratic Method | speq-plan | Interview |
| MECE Partitioning | speq-plan | Problem structuring |
| BDD (Gherkin syntax) | speq-planning | Specification |
| EARS Syntax | speq-planning | Requirements |
| RFC 2119 / 8174 | speq-planning | Requirements |
| ADR (Nygard format) | speq-planning | Design decisions |
| Premortem | speq-plan-review | Risk analysis |
| Devil's Advocate (diabolus advocatus) | plan-reviewer (agent) | Adversarial review |
| BLUF (Bottom Line Up Front) | speq-audit | Reporting |
| London School TDD | speq-implement (template) | Testing |
| ASD-STE100 (Simplified Technical English) | speq-writing-guardrails | Plain-language rules |
| BLUF / Inverted Pyramid | speq-writing-guardrails | Structure |
| RFC 2119 / 8174 | speq-writing-guardrails | Prose register |
| A Philosophy of Software Design (Ousterhout) | speq-design-philosophy | Design principles |
| Deep Modules | speq-design-philosophy | Design principles |
| Information Hiding | speq-design-philosophy | Design principles |
| Strategic vs Tactical Programming | speq-design-philosophy | Design principles |
| Dependency Rule (Clean Architecture / Martin) | speq-design-philosophy | Design principles |
| Rule of Three | speq-code-guardrails | Design principles |
| Command-Query Separation | speq-code-guardrails | Design principles |
