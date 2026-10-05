[speq-skill](../README.md) / [Docs](./index.md) / Installation

---

# Installation

## Quick install

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

Open Claude Code or Codex. Start with `/speq:mission` in Claude Code, or type `$` in Codex.

## Prerequisites

- macOS or Linux
- Claude Code CLI or Codex CLI/App, installed and configured
- Optional: [Serena](https://github.com/oraios/serena), installed globally. See [MCP Servers](./mcp-servers.md). `uv` is needed only if the `serena` command is missing and you let the installer install it. Get it from [astral.sh/uv](https://astral.sh/uv/).

## What the installer does

| Component | Location |
|-----------|----------|
| `speq` CLI | `~/.local/bin/speq` |
| Plugin files (Claude marketplace payload) | `~/.speq-skill/` |
| Codex plugin payload | `~/.speq-skill/codex/plugins/speq-skill/` |
| Codex marketplace registration | `~/.codex/config.toml` (`speq-skill-local`) |
| Codex skills | `$CODEX_HOME/skills/speq-*` or `~/.codex/skills/speq-*` |
| Embeddings model ([snowflake-arctic-embed-xs](https://huggingface.co/Snowflake/snowflake-arctic-embed-xs), ~86MB) | `~/Library/Caches/speq/models/` on macOS, `~/.cache/speq/models/` on Linux (or `$SPEQ_CACHE_DIR/models/`) |

- It downloads a pre-built `speq` binary (Linux x86_64/ARM64, macOS Apple Silicon).
- It downloads the embedding model (`snowflake-arctic-embed-xs`) into the platform cache directory (`~/Library/Caches/speq/models/` on macOS, `~/.cache/speq/models/` on Linux) for semantic search
- It installs the plugin for Claude Code and Codex, and registers the local Codex marketplace when Codex is installed.
- It asks before it installs Serena, when Serena is not registered yet.

## Install from source

```bash
git clone https://github.com/marconae/speq-skill && cd speq-skill
./scripts/local-install.sh
```

This needs the Rust toolchain. Install it via [rustup](https://rustup.rs/).

## Verify installation

```bash
speq --version
ls ~/.speq-skill/plugins/speq-skill/.claude-plugin/plugin.json
ls ~/.speq-skill/codex/plugins/speq-skill/.codex-plugin/plugin.json
grep -n "speq-skill-local" ~/.codex/config.toml
ls ~/.codex/skills/speq-mission
```

In Claude Code, `/plugin` shows the `speq:*` skills. In Codex, type `$` and select `speq:mission`.

## Update

Re-run the install script and run `/speq:audit` to update the spec library.

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/uninstall.sh | bash
```

If you installed from source, run `./uninstall.sh` instead.
