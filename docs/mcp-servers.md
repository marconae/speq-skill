[speq-skill](../README.md) / [Docs](./index.md) / MCP Servers

---

# MCP Servers

speq-skill works best with [Serena](https://github.com/oraios/serena), an MCP (Model Context Protocol) server for code navigation. You install it once, globally. speq-skill does not bundle or configure it. The skills find its tools at run time.

[Context7](https://github.com/upstash/context7) is optional. If it is installed, the skills use it for library documentation. If not, they skip it. speq-skill does not install or manage Context7.

| Server | What your agent gets | Used by |
|--------|----------------------|---------|
| Serena | Symbol-level code navigation and editing | `/speq:code-tools` |
| Context7 (optional) | Current library documentation | `/speq:ext-research` |

## Why Serena

- It finds symbols, references, and implementations, and edits one symbol at a time
- The agent reads less code and changes only what the plan names

## Set up Serena

The installer asks before it installs Serena. It skips Serena when it is already registered. To do it yourself:

**Claude Code**

```bash
uv tool install -p 3.13 serena-agent
claude mcp add --scope user serena -- serena start-mcp-server --context claude-code --project-from-cwd
```

This needs no plugin marketplace. Do not start Claude Code from your home directory, because `--project-from-cwd` can then scan all of it.

**Codex**

```bash
uv tool install -p 3.13 serena-agent
codex mcp add serena -- serena start-mcp-server --project-from-cwd --context=codex
```

See the [Serena documentation](https://github.com/oraios/serena) for advanced configuration.

## What if Serena is missing

- The skills carry on with the generic read, search, and edit tools
- Nothing warns you during a run
- `/speq:audit` shows one warning

## How the skills use them

```
Explore codebase  → Serena     structure, symbols, references
Research APIs     → Context7   method signatures, usage examples (when installed)
Research patterns → WebSearch  design guidance, best practices
Edit code         → Serena     precise, symbol-level changes
```

Without Context7, library API questions go to WebSearch.

## Upgrading from an earlier version

Earlier versions of speq-skill configured Serena and Context7 inside the plugin and registered them in Codex. The installer leaves existing Serena and Context7 registrations as they are, and offers to install Serena if it is missing.
