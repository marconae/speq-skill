# Decision Log: with-decisions

Date: 2026-04-27

## Design Decisions

- **Decision:** Use a line-oriented state machine to parse decision logs.
- **Alternatives:** Full Markdown AST via pulldown-cmark.
- **Rationale:** Decision logs have a small, regular surface; line scanning is simpler and more direct.
- **Promotes to ADR:** yes
