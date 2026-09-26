# Decision Log: <plan-name>

<!--
STRUCTURAL TEMPLATE - DO NOT COPY-PASTE
Generate actual content from the clarifying interview and plan design.
Capture interview Q&A verbatim or close paraphrase.
Promotion gate: default "Promotes to ADR: no". The expected count of "yes" entries is zero.
Mark "yes" only when a criterion in /speq-adr-rules applies. Name the criterion and the
`speq decision-log show` search result in Rationale. Never promote content on the
never-an-ADR list, and keep Decision free of signatures, paths, and flags.
A corollary of an already-promoted decision is not its own entry: add it as a bullet in that
parent entry's Consequences line instead.
-->

## Interview

<!-- Q&A from the clarifying interview. One Q/A pair per exchange. -->

**Q:** <question asked>
**A:** <user answer>

## Design Decisions

<!-- Key design choices made in this plan. One entry per significant decision. -->

### [1] <Short decision title>

- **Decision:** What was chosen.
- **Alternatives:** What else was considered and why rejected. May read `none`.
- **Rationale:** Why this choice. For `yes`, name the /speq-adr-rules criterion and the search result.
- **Consequences:** Effects, trade-offs, or corollary decisions folded in here. Omit this line entirely when there are none. <!-- optional -->
- **Supersedes:** <slug of the ADR this replaces> <!-- optional; only when this decision replaces an existing ADR -->
- **Promotes to ADR:** yes

### [2] <Short decision title>

- **Decision:** What was chosen.
- **Alternatives:** What else was considered and why rejected. May read `none`.
- **Rationale:** Why this choice.
- **Consequences:** Effects, trade-offs, or corollary decisions folded in here. Omit this line entirely when there are none. <!-- optional -->
- **Promotes to ADR:** no

## Review Findings

<!-- Significant review findings that changed direction: plan-review findings (prefix title "[plan-review]"), populated by speq-plan/speq-plan-pr after plan-reviewer resolves a blocker, and code-review findings, populated by speq-implement after code review. -->

### [1] <Finding title>

- **Finding:** What the reviewer identified.
- **Direction change:** How the implementation was adjusted.
- **Promotes to ADR:** yes / no
