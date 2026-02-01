<div align="center">

<img src="assets/speq-skill-logo.svg" alt="Speq Skill Logo" width="200">

# speq-skill

**A light-weight and straightforward system for spec-driven development with Claude Code**

[![spec|driven](https://img.shields.io/badge/spec-driven-blue)](specs/)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/marconae/speq-skill/actions/workflows/ci.yml/badge.svg)](https://github.com/marconae/speq-skill/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[Getting Started](#getting-started) • [Why](#why-i-built-it) • [How It Works](#how-does-it-work) • [Documentation](./docs/) • [Installation](./docs/installation.md)
</div>

---

## Getting Started

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

> [!NOTE]
> The installer builds `speq` from source using the Rust toolchain (installed automatically if missing). There is no binary distribution. See [Installation](./docs/installation.md) for details.

Then run `claude` and type `/speq:mission` to start.

---

## Why I Built It

I want to leverage Claude Code as an effective tool to write software.

There are other spec-driven development tools out there; OpenSpec, BMAD, SpecKit... 

But I was missing the following:

1. A system that is not primped on one language or framework (e.g., Python or TypeScript)
2. A straightforward repeatable workflow (`plan → implement → record`)
3. A **permanent** and growing spec-library
4. A system that keeps the specs **small** to avoid context cluttering
5. A system that keeps **asking me instead of making assumptions**

So I built `speq-skill`. It combines Skills and Agents with a simple CLI called `speq` that adds a semantical search layer to the spec library. The search empowers the coding agent to find the right feature scenarios during planning, but also during the implementation. This avoids reading unnecessary specs into the context window.

## Who should use it?

Vibe Coding does not scale. `speq-skill` adds the missing workflow and guardrails.

If you want to describe what you want and have a coding agent build the code for you, then you should give `speq-skill` a try!

 It adds a lightweight and straightforward system for spec-driven development that engineers the context for the coding agent.

---

## How Does it Work?

```
/speq:plan     →    /speq:implement    →    /speq:record
     │                   │                       │
     ▼                   ▼                       ▼
Creates plan.md     Implementation          Merges deltas into
with deltas         + verification          permanent specs library
```

1. **Plan** — Describe what you want. The agent searches existing specs, asks clarifying questions, and creates a plan with spec deltas.
2. **Implement** — The agent implements features using TDD. Each scenario gets a failing test first, then minimal code to pass.
3. **Record** — Review newly built feature and merge planned deltas into permanent specs. The plan archives to `_recorded/`.

Specs live in `specs/<domain>/<feature>/spec.md`. Plans stage in `specs/_plans/<plan-name>/`. The separation keeps your spec library clean while work is in progress.

---

## Documentation

| Guide | Description |
|-------|-------------|
| [Installation](./docs/installation.md) | Setup CLI and plugin |
| [Workflow](./docs/workflow.md) | Plan → Implement → Record cycle |
| [CLI Reference](./docs/cli-reference.md) | All CLI commands |
| [MCP Servers](./docs/mcp-servers.md) | Serena and Context7 |

---

## Important

`speq-skill` is a plugin for Claude Code and other compatible AI coding agents. This tool provides workflow structure and spec management only—**the AI / coding agent (such as Claude Code) generates all code, specs, or other artifacts**.

## Dependencies

This plugin uses [Serena](https://github.com/oraios/serena) and [Context7](https://github.com/upstash/context7) MCP servers (both MIT licensed). The installer sets them up as a convenience — they are standard open-source servers installed from their respective repositories. Their behavior, limitations, and conditions are governed by their own documentation. Context7's MCP server connects to a cloud service with a free tier (1,000 calls/month) — see [Context7 plans](https://context7.com/plans).

The `speq` CLI downloads the [snowflake-arctic-embed-xs](https://huggingface.co/Snowflake/snowflake-arctic-embed-xs) embeddings model (~23MB) on first run for semantic search (Apache 2.0 licensed).

## License

Free and open-source under [MIT](LICENSE).

---

<div align="center">

Build with Rust 🦀 and made with ❤️ by [marconae](https://deliberate.codes).

</div>