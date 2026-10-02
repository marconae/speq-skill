[speq-skill](../README.md) / [Docs](./index.md) / Installation

---

# Installation

## Quick install

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

> [!NOTE]
> When a pre-built `speq` binary is available for your platform (Linux x86_64/ARM64, macOS Apple Silicon), the installer downloads it. If no binary is available, the installer builds from source with the Rust toolchain. If Rust is missing, the installer offers to install [rustup](https://rustup.rs/) for you.

Open Claude Code or Codex. Start with the matching trigger: `/speq:mission` in Claude Code, or `$` in Codex.

## Prerequisites

- macOS or Linux (Windows via WSL)
- Claude Code CLI or Codex CLI/App, installed and configured
- Rust toolchain, needed only if your platform has no pre-built binary. If Rust is missing, the installer installs it for you. You can also get Rust from [rustup](https://rustup.rs/).
- Optional: [Serena](https://github.com/oraios/serena), installed globally. The installer offers to install it. See [MCP Servers](./mcp-servers.md). `uv` is needed only if the `serena` command is not installed yet and you let the installer install it. Get it from [astral.sh/uv](https://astral.sh/uv/).

## What the installer installs

| Component | Location |
|-----------|----------|
| `speq` CLI | `~/.local/bin/speq` |
| Plugin files | `~/.speq-skill/` |
| Claude marketplace payload | `~/.speq-skill/` |
| Codex plugin payload | `~/.speq-skill/codex/plugins/speq-skill/` |
| Codex marketplace manifest | `~/.speq-skill/codex/.agents/plugins/marketplace.json` |
| Codex marketplace registration | `~/.codex/config.toml` (`speq-skill-local`) |
| Codex skills | `$CODEX_HOME/skills/speq-*` or `~/.codex/skills/speq-*` |
| Embeddings model | `~/.cache/speq/models/` (or `$SPEQ_CACHE_DIR/models/`) |

The installer also:
- Copies the pre-built `speq` binary to your PATH. On platforms without one, for example Intel Mac, it downloads the release source instead and builds `speq` with the Rust toolchain.
- Installs the speq-skill plugin for Claude Code and Codex
- When Codex is installed, registers the local Codex marketplace via `codex plugin marketplace add`
- Asks before it installs Serena for Claude Code or Codex, when it is not registered yet. It installs the Serena CLI with `uv tool install`, then registers Serena as a user-scope MCP server. It skips Serena when it is already registered or when a Serena plugin is enabled in Claude Code. It reads your answer from the terminal, so the prompts also work with `curl ... | bash`. Without a terminal it prints the commands instead
- Installs Codex skills into `$CODEX_HOME/skills` so Codex can load the `$`-triggered `speq:*` skills
- Downloads the [snowflake-arctic-embed-xs](https://huggingface.co/Snowflake/snowflake-arctic-embed-xs) embeddings model into `~/.cache/speq/models/`

## Install from source

If your platform has no pre-built binary, or you want to build manually, follow these steps:

```bash
# Clone the repository
git clone https://github.com/marconae/speq-skill && cd speq-skill

# Build and install CLI + plugin
./scripts/local-install.sh
```

> [!NOTE]
> This build requires the Rust toolchain. Install it via [rustup](https://rustup.rs/).

## Verify installation

```bash
# Check CLI is available
speq --version

# Check Claude marketplace payload
ls ~/.speq-skill/plugins/speq-skill/.claude-plugin/plugin.json

# Check Codex plugin payload
ls ~/.speq-skill/codex/plugins/speq-skill/.codex-plugin/plugin.json

# Check Codex marketplace registration
grep -n "speq-skill-local" ~/.codex/config.toml

# Check Codex skills
ls ~/.codex/skills/speq-mission

# Test in Claude Code
claude
/plugin # should show speq:* skills

# Test in Codex
codex
# In Codex, type $ and select the speq:mission skill
```

## Update

Re-run the install script to get the latest version:

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/uninstall.sh | bash
```

If you installed from source, run locally instead:

```bash
./uninstall.sh
```

## Troubleshooting

### `speq: command not found`

Add `~/.local/bin` to your PATH.

### Serena tools are missing

The plugin does not configure Serena. Install it globally. The skills work without it. `/speq:audit` reports when it is missing. See [MCP Servers](./mcp-servers.md) for the install commands.

### Rust build errors

If your platform has no pre-built binary, or you ran `./scripts/local-install.sh` directly, update your Rust toolchain:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update
```

### Plugin not found in Claude Code

1. Verify the plugin is installed:
   ```bash
   ls ~/.speq-skill/plugins/speq-skill/.claude-plugin/plugin.json
   ```

2. Restart Claude Code:
   ```bash
   claude
   ```

3. Verify the plugin loads:
   ```
   /speq:mission
   ```

### Plugin not found in Codex

1. Verify the Codex plugin payload exists:
   ```bash
   ls ~/.speq-skill/codex/plugins/speq-skill/.codex-plugin/plugin.json
   ```

2. Verify the Codex marketplace is registered:
   ```bash
   grep -n "speq-skill-local" ~/.codex/config.toml
   ```

3. If missing, register it manually:
   ```bash
   codex plugin marketplace add ~/.speq-skill/codex
   ```

4. Verify the Codex skill copies exist:
   ```bash
   ls ~/.codex/skills/speq-mission/SKILL.md
   ```

5. Restart Codex. Then invoke:
   ```
   # In Codex, type $ and select the speq:mission skill
   ```

## Dependencies

| Dependency | Purpose |
|------------|---------|
| [Serena](https://github.com/oraios/serena) | Semantic code navigation |
| [snowflake-arctic-embed-xs](https://huggingface.co/Snowflake/snowflake-arctic-embed-xs) | Embeddings model (~86MB) |

> [!NOTE]
> The installer downloads the embeddings model into `~/.cache/speq/models/` during installation.
