# Decision Log: <plan-name>

<!--
STRUCTURAL TEMPLATE - DO NOT COPY-PASTE
Generate actual content from the clarifying interview and plan design.
Capture the interview as one short line per Q&A.
Promotion gate: /speq-adr-rules. Default "Promotes to ADR: no".
-->

## Interview

<!-- One line per exchange in the clarifying interview, a few words each. -->

- Q: <question asked> → A: <user answer>

## Design Decisions

<!-- Usually empty or a few entries. Log a choice only when a reviewer or implementer cannot infer it from the deltas and tasks. -->

### [1] <Short decision title>

- **Decision:** What was chosen.
- **Rationale:** Why this choice.
- **Promotes to ADR:** no

<!-- Optional fields, add only when they carry a fact:
- **Alternatives:** A plausible alternative that was rejected, and why.
- **Consequences:** Effects, trade-offs, or corollary decisions.
-->

<!-- Rare: an entry that promotes to an ADR. Most plans have none.

### [2] <Short decision title>

- **Decision:** What was chosen.
- **Rationale:** Why this choice. Name the /speq-adr-rules criterion and the search result.
- **Supersedes:** <slug of the ADR this replaces> (optional; only when this decision replaces an existing ADR)
- **Architecture:** <§ Section[, § Section] | no change: <reason>> (required when Rationale names /speq-adr-rules rule-2 criterion 1, 2, or 3. Each named section needs a block in architecture.md)
- **Promotes to ADR:** yes
-->

## Review Findings

<!-- Review findings that changed a design decision or settled a HUMAN finding: plan-review findings (prefix title "[plan-review]"), populated by speq-plan/speq-plan-pr after plan-reviewer resolves a blocker. MECHANICAL fixes get no entry. -->

### [1] <Finding title>

- **Finding:** What the reviewer identified.
- **Direction change:** How the implementation was adjusted.
- **Promotes to ADR:** yes / no
