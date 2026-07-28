[speq-skill](../README.md) / [Docs](./index.md) / MCP Servers

---

# MCP Servers

speq-skill integrates with two MCP (Model Context Protocol) servers. These servers improve code comprehension and research.

---

## Overview

| Server | Purpose |
|--------|---------|
| [Serena](https://github.com/oraios/serena) | Semantic code navigation and editing |
| [Context7](https://github.com/upstash/context7) | Library documentation lookup |

The generated plugin declares both servers in its MCP configuration. The plugin launches both servers from their upstream packages. See the documentation of each server for its behavior, limits, and license terms.

---

## How speq-skill uses them

### Code comprehension (Serena)

The `/speq:code-tools` skill uses Serena for semantic code operations:

- **Explore** — Navigates the codebase structure at the symbol level: classes, functions, and methods
- **Understand** — Finds where symbols are defined and referenced
- **Edit** — Makes precise changes to specific symbols, with no changes to the surrounding code
- **Verify** — Confirms that changes do not break existing references

### External research (Context7 + WebSearch)

The `/speq:ext-research` skill combines Context7 and WebSearch:

```
Need library API details?
├─ Yes → Context7 (method signatures, usage examples)
└─ No  → Need design guidance?
         ├─ Yes → WebSearch (patterns, best practices)
         └─ No  → Proceed with existing knowledge
```

**Context7** — Queries library documentation for correct, up-to-date API usage.

> [!NOTE]
> The Context7 MCP server is open source (MIT licensed) but connects to a cloud service. See [Context7](https://context7.com) for details.

**WebSearch** — Researches design patterns, architecture decisions, and industry best practices.

### Combined workflow

During implementation, the skills work together:

1. **Explore codebase** (Serena) — Examines the existing structure
2. **Research APIs** (Context7) — Gets the correct library usage
3. **Research patterns** (WebSearch) — Informs design decisions
4. **Edit code** (Serena) — Makes precise, semantic changes

---

## Configuration

Each generated plugin configures its MCP servers in a `.mcp.json` file:

```
~/.speq-skill/plugins/speq-skill/.mcp.json
~/.speq-skill/codex/plugins/speq-skill/.mcp.json
```

The Claude plugin starts Serena with the Claude Code context. The Codex plugin starts Serena with the Codex context and the `--project-from-cwd` flag. This setup follows the [Codex client guidance](https://oraios.github.io/serena/02-usage/030_clients.html#codex-cli-and-app) from Serena.

When the Codex CLI is available, the installer takes three steps. It registers the local Codex marketplace with `codex plugin marketplace add`. It keeps the MCP declarations in the generated plugin payload. Then it registers the Codex MCP servers with these commands:

```bash
codex mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --project-from-cwd --context=codex
codex mcp add context7 -- npx -y @upstash/context7-mcp
```

See the documentation of each project for advanced configuration:

- [Serena documentation](https://github.com/oraios/serena)
- [Context7 documentation](https://github.com/upstash/context7)
