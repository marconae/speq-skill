[speq-skill](../README.md) / Docs

---

<div align="center">

<img src="../assets/speq-skill-logo.svg" alt="Speq Skill Logo" width="150">

# speq-skill Documentation

</div>

---

## Quick Links

| Guide | Description |
|-------|-------------|
| [Installation](./installation.md) | Setup CLI and plugin |
| [Workflow](./workflow.md) | Plan → Implement → Record cycle |
| [CLI Reference](./cli-reference.md) | All CLI commands |
| [MCP Servers](./mcp-servers.md) | Serena and Context7 |

---

## Components

### CLI (`speq`)

The `speq` CLI provides spec discovery and semantic search:

- **Domain commands** — List and explore spec domains
- **Feature commands** — Get, list, and validate feature specs
- **Search commands** — Semantic search across scenarios
- **Record command** — Merge plan deltas into permanent specs

See [CLI Reference](./cli-reference.md) for full command documentation.

### Workflow Skills

The core spec-driven development flow:

| Skill | Purpose |
|-------|---------|
| `/speq:mission` | Interview user, explore codebase, generate `specs/mission.md` |
| `/speq:plan` | Search specs, interview user, create plan with deltas |
| `/speq:implement` | Test-Driven Development (TDD) cycle with mandatory evidence, generate verification report |
| `/speq:record` | Merge deltas to permanent specs, validate, archive plan |

See [Workflow](./workflow.md) for detailed documentation.

### Utility Skills

Reusable guidance invoked by workflow skills:

| Skill | Purpose |
|-------|---------|
| `/speq:code-tools` | Semantic code navigation via Serena Model Context Protocol (MCP) |
| `/speq:ext-research` | External docs via Context7 and WebSearch |
| `/speq:code-guardrails` | TDD cycle and code quality guardrails |
| `/speq:git-discipline` | Git read-only rules |
| `/speq:cli` | speq CLI usage patterns |

### MCP Servers

External tools integrated via MCP:

| Server | Purpose |
|--------|---------|
| [Serena](https://github.com/oraios/serena) | Semantic code navigation and editing |
| [Context7](https://github.com/upstash/context7) | Library documentation lookup |

See [MCP Servers](./mcp-servers.md) for configuration and usage.
