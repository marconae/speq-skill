[speq-skill](../README.md) / [Docs](./index.md) / MCP Servers

---

# MCP Servers

speq-skill integrates with popular MCP (Model Context Protocol) servers for enhanced code comprehension and research capabilities.

---

## Overview

| Server | Purpose |
|--------|---------|
| [Serena](https://github.com/oraios/serena) | Semantic code navigation and editing |
| [Context7](https://github.com/upstash/context7) | Library documentation lookup |

Both servers are declared in the generated plugin MCP configuration as a convenience. They are standard open-source MCP servers launched from their respective upstream packages — speq-skill does not bundle or modify them. Their behavior, limitations, and licensing are governed by their own documentation.

---

## How speq-skill Uses Them

### Code Comprehension (Serena)

The `/speq:code-tools` skill leverages Serena for semantic code operations:

- **Explore** — Navigate codebase structure at the symbol level (classes, functions, methods)
- **Understand** — Find where symbols are defined and referenced
- **Edit** — Make precise changes to specific symbols without touching surrounding code
- **Verify** — Confirm changes haven't broken references

This replaces raw text operations (`grep`, `find`, `sed`) with semantic operations that understand code structure.

### External Research (Context7 + WebSearch)

The `/speq:ext-research` skill combines Context7 and WebSearch:

```
Need library API details?
├─ Yes → Context7 (method signatures, usage examples)
└─ No  → Need design guidance?
         ├─ Yes → WebSearch (patterns, best practices)
         └─ No  → Proceed with existing knowledge
```

**Context7** — Queries library documentation for correct, up-to-date API usage. Prevents hallucinated method names or deprecated patterns.

> [!NOTE]
> The Context7 MCP server is open source (MIT licensed), but it connects to a cloud service. See [Context7](https://context7.com) for details.

**WebSearch** — Researches design patterns, architecture decisions, and industry best practices.

### Combined Workflow

During implementation, the skills work together:

1. **Explore codebase** (Serena) — Understand existing structure
2. **Research APIs** (Context7) — Get correct library usage
3. **Research patterns** (WebSearch) — Inform design decisions
4. **Edit code** (Serena) — Make precise, semantic changes

---

## Configuration

MCP servers are configured in each generated plugin's `.mcp.json` file:

```
~/.speq-skill/plugins/speq-skill/.mcp.json
~/.speq-skill/codex/plugins/speq-skill/.mcp.json
```

The Claude plugin starts Serena with the Claude Code context. The Codex plugin starts Serena with the Codex context and `--project-from-cwd`, matching Serena's [Codex client guidance](https://oraios.github.io/serena/02-usage/030_clients.html#codex-cli-and-app).

The installer registers the local Codex marketplace with `codex plugin marketplace add` when the Codex CLI is available, and keeps the MCP declarations in the generated plugin payload. See the respective project documentation for advanced configuration:

- [Serena documentation](https://github.com/oraios/serena)
- [Context7 documentation](https://github.com/upstash/context7)
