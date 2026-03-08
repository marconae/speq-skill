[speq-skill](../README.md) / [Docs](./index.md) / Semantic Anchors

---

# Semantic Anchors

## What are Semantic Anchors

Semantic anchors are named references to established methodologies, frameworks, and practices embedded directly in skill instructions. Instead of re-explaining a methodology from scratch, the skill names it — e.g., "London School TDD" or "BLUF" — giving the AI a precise, well-documented behavior to follow.

This works because LLMs have deep training data on these named practices. A single anchor like "Socratic Method" activates richer behavior than paragraphs of custom instruction.

For more background, see the Anthropic Cookbook entry on [semantic anchors](https://github.com/anthropics/anthropic-cookbook/tree/main/misc/semantic_anchors) and explore the [LLM-Coding/Semantic-Anchors](https://github.com/LLM-Coding/Semantic-Anchors) catalog of semantic anchors on GitHub.

## Anchor Reference

| Anchor | Skill | Category |
|--------|-------|----------|
| Information Foraging | speq-cli | Search strategy |
| Clean Code (Martin) | speq-code-guardrails | Code quality |
| London School TDD | speq-code-guardrails | Testing |
| SOLID | speq-code-guardrails | Design principles |
| Five Whys | speq-code-guardrails | Root cause analysis |
| Feynman Technique | speq-code-tools | Comprehension |
| Evidence Hierarchy | speq-ext-research | Research |
| Conventional Commits | speq-git-discipline | Version control |
| Work Breakdown Structure | speq-implement | Task decomposition |
| Pyramid Principle | speq-implement | Communication |
| BLUF (Bottom Line Up Front) | speq-implement | Reporting |
| Socratic Method | speq-mission | Interview |
| User Story Mapping (Patton) | speq-mission | Requirements |
| MECE Partitioning | speq-mission | Problem structuring |
| BDD (Gherkin syntax) | speq-plan | Specification |
| Socratic Method | speq-plan | Interview |
| MECE Partitioning | speq-plan | Problem structuring |
| EARS Syntax | speq-plan | Requirements |
| ADR (Nygard format) | speq-plan | Design decisions |
| Docs-as-Code | speq-record | Documentation |
| London School TDD | speq-implement (template) | Testing |
