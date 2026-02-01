[speq-skill](../README.md) / [Docs](./index.md) / MCP Servers

---

# MCP Servers

speq-skill integrates with MCP (Model Context Protocol) servers for enhanced code comprehension and research capabilities.

---

## Overview

| Server | Purpose |
|--------|---------|
| [Serena](https://github.com/oraios/serena) | Semantic code navigation and editing |
| [Context7](https://github.com/upstash/context7) | Library documentation lookup |

Both servers are installed automatically by the speq-skill installer as a convenience. They are standard open-source MCP servers installed from their respective repositories — speq-skill does not bundle or modify them. Their behavior, limitations, and licensing are governed by their own documentation.

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
> The Context7 MCP server is open source (MIT licensed), but it connects to a cloud service with a free tier of 1,000 API calls/month. See [Context7 plans](https://context7.com/plans) for details.

**WebSearch** — Researches design patterns, architecture decisions, and industry best practices.

### Combined Workflow

During implementation, the skills work together:

1. **Explore codebase** (Serena) — Understand existing structure
2. **Research APIs** (Context7) — Get correct library usage
3. **Research patterns** (WebSearch) — Inform design decisions
4. **Edit code** (Serena) — Make precise, semantic changes

---

## Configuration

MCP servers are configured in the plugin's `.mcp.json` file:

```
~/.claude/plugins/speq-skill/.mcp.json
```

The installer sets up both Serena and Context7 automatically. See the respective project documentation for advanced configuration:

- [Serena documentation](https://github.com/oraios/serena)
- [Context7 documentation](https://github.com/upstash/context7)
