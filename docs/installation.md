[speq-skill](../README.md) / [Docs](./index.md) / Installation

---

# Installation

## Quick install

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

Open Claude Code or Codex. Start with `/speq:mission` in Claude Code, or type `$` in Codex.

## Prerequisites

- macOS or Linux (Windows via WSL)
- Claude Code CLI or Codex CLI/App, installed and configured
- Rust toolchain, needed only if your platform has no pre-built binary. The installer offers to install [rustup](https://rustup.rs/) for you.
- Optional: [Serena](https://github.com/oraios/serena), installed globally. See [MCP Servers](./mcp-servers.md). `uv` is needed only if the `serena` command is missing and you let the installer install it. Get it from [astral.sh/uv](https://astral.sh/uv/).

## What the installer does

| Component | Location |
|-----------|----------|
| `speq` CLI | `~/.local/bin/speq` |
| Plugin files (Claude marketplace payload) | `~/.speq-skill/` |
| Codex plugin payload | `~/.speq-skill/codex/plugins/speq-skill/` |
| Codex marketplace registration | `~/.codex/config.toml` (`speq-skill-local`) |
| Codex skills | `$CODEX_HOME/skills/speq-*` or `~/.codex/skills/speq-*` |
| Embeddings model ([snowflake-arctic-embed-xs](https://huggingface.co/Snowflake/snowflake-arctic-embed-xs), ~86MB) | `~/.cache/speq/models/` (or `$SPEQ_CACHE_DIR/models/`) |

- It downloads a pre-built `speq` binary (Linux x86_64/ARM64, macOS Apple Silicon). Without one, it builds `speq` from source with the Rust toolchain.
- It installs the plugin for Claude Code and Codex, and registers the local Codex marketplace when Codex is installed.
- It asks before it installs Serena, when Serena is not registered yet. It installs the Serena CLI with `uv tool install`, then registers Serena as a user-scope MCP server. It skips Serena when it is already registered or a Serena plugin is enabled in Claude Code.
- It reads your answer from the terminal, so the prompts also work with `curl ... | bash`. Without a terminal it prints the commands instead.

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

Re-run the install script.

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/uninstall.sh | bash
```

If you installed from source, run `./uninstall.sh` instead.

## Troubleshooting

- **`speq: command not found`**: add `~/.local/bin` to your PATH.
- **Serena tools are missing**: the plugin does not configure Serena. The skills work without it, and `/speq:audit` reports when it is missing. See [MCP Servers](./mcp-servers.md) for the install commands.
- **Rust build errors**: run `rustup update`, then install again.
- **Plugin not found in Claude Code**: check that `~/.speq-skill/plugins/speq-skill/.claude-plugin/plugin.json` exists, then restart Claude Code and run `/speq:mission`.
- **Plugin not found in Codex**: check that `~/.speq-skill/codex/plugins/speq-skill/.codex-plugin/plugin.json` and `~/.codex/skills/speq-mission/SKILL.md` exist. If the marketplace is not in `~/.codex/config.toml`, run `codex plugin marketplace add ~/.speq-skill/codex`. Restart Codex.
