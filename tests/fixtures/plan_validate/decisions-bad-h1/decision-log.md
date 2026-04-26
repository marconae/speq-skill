# Decision Log: wrong-name

Date: 2026-04-27

## Design Decisions

- **Decision:** Use a line-oriented state machine to parse decision logs.
- **Alternatives:** Full Markdown AST via pulldown-cmark.
- **Rationale:** Line scanning is simpler for the flat structure of decision logs.
- **Promotes to ADR:** no
