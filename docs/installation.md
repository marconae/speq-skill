[speq-skill](../README.md) / [Docs](./index.md) / Installation

---

# Installation

## Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

> [!NOTE]
> The installer downloads a pre-built `speq` binary for your platform when one is available (Linux x86_64/ARM64, macOS Apple Silicon), falling back to building from source using the Rust toolchain. If you don't have Rust installed and a source build is needed, the installer will offer to install it for you via [rustup](https://rustup.rs/).

Then open Claude Code or Codex and type `/speq:mission` to start.

---

## Prerequisites

- **macOS or Linux** (Windows via Windows Subsystem for Linux (WSL))
- **Claude Code CLI** or **Codex CLI/App** installed and configured
- **Rust toolchain** — only needed if no pre-built binary matches your platform (installed automatically if missing, or via [rustup](https://rustup.rs/))

---

## What Gets Installed

| Component | Location |
|-----------|----------|
| `speq` CLI | `~/.local/bin/speq` |
| Plugin files | `~/.speq-skill/` |
| Claude marketplace payload | `~/.speq-skill/` |
| Codex plugin payload | `~/.speq-skill/codex/plugins/speq-skill/` |
| Codex marketplace manifest | `~/.speq-skill/codex/.agents/plugins/marketplace.json` |
| Codex marketplace registration | `~/.codex/config.toml` (`speq-skill-local`) |
| Codex MCP server registrations | `~/.codex/config.toml` (`serena`, `context7`) |
| Codex skills | `$CODEX_HOME/skills/speq-*` or `~/.codex/skills/speq-*` |
| Embeddings model | `~/.cache/speq/models/` (or `$SPEQ_CACHE_DIR/models/`) |

The installer automatically:
- Downloads a pre-built `speq` binary for your platform (Linux x86_64/ARM64, macOS Apple Silicon) and copies it to your PATH; other platforms (e.g. Intel Mac) fall back to downloading the release source and building `speq` with the Rust toolchain
- Installs the speq-skill plugin for Claude Code and Codex
- Registers the local Codex marketplace with `codex plugin marketplace add` when Codex is installed
- Registers Serena and Context7 with `codex mcp add` when Codex is installed
- Installs Codex skills into `$CODEX_HOME/skills` so Codex can load `/speq:*`
- Installs plugin MCP configuration for Serena and Context7
- Downloads the `snowflake-arctic-embed-xs` embedding model (~23 MB) into `~/.cache/speq/models/`

---

## Installation from Source

To build from source manually instead of using the pre-built binary (or if your platform has no pre-built binary):

```bash
# Clone the repository
git clone https://github.com/marconae/speq-skill && cd speq-skill

# Build and install CLI + plugin
./scripts/local-install.sh
```

> [!NOTE]
> Requires Rust toolchain (install via [rustup](https://rustup.rs/)).

---

## Verify Installation

```bash
# Check CLI is available
speq --version

# Check Claude marketplace payload
ls ~/.speq-skill/plugins/speq-skill/.claude-plugin/plugin.json

# Check Codex plugin payload
ls ~/.speq-skill/codex/plugins/speq-skill/.codex-plugin/plugin.json

# Check Codex marketplace registration
grep -n "speq-skill-local" ~/.codex/config.toml

# Check Codex MCP server registrations
codex mcp list

# Check Codex skills
ls ~/.codex/skills/speq-mission

# Test in Claude Code
claude
/plugin # should show speq:* skills

# Test in Codex
codex
/speq:mission
```

---

## Update

Re-run the install script to get the latest version:

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

---

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/uninstall.sh | bash
```

If installed from source, you can also run locally:

```bash
./scripts/uninstall.sh
```

---

## Troubleshooting

### `speq: command not found`

Add `~/.local/bin` to your PATH.

### Rust build errors

Only relevant if the installer fell back to a source build (no pre-built binary for your platform) or you ran `./scripts/local-install.sh` directly. Ensure Rust toolchain is installed and up to date:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update
```

### Plugin not found in Claude Code

1. Verify plugin is installed:
   ```bash
   ls ~/.speq-skill/plugins/speq-skill/.claude-plugin/plugin.json
   ```

2. Restart Claude Code:
   ```bash
   claude
   ```

3. Check plugin loads:
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

4. Verify the Codex MCP servers are registered:
   ```bash
   codex mcp list
   ```

5. If missing, register them manually:
   ```bash
   codex mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --project-from-cwd --context=codex
   codex mcp add context7 -- npx -y @upstash/context7-mcp
   ```

6. Verify the Codex skill copies exist:
   ```bash
   ls ~/.codex/skills/speq-mission/SKILL.md
   ```

7. Restart Codex and invoke:
   ```
   /speq:mission
   ```

### MCP server connection errors

The plugin depends on Serena and Context7 MCP servers. If you see connection errors:

1. Check servers are installed:
   ```bash
   ls ~/.speq-skill/plugins/speq-skill/.mcp.json
   ls ~/.speq-skill/codex/plugins/speq-skill/.mcp.json
   ```

2. Verify server configuration in `~/.speq-skill/plugins/speq-skill/.mcp.json` for Claude or `~/.speq-skill/codex/plugins/speq-skill/.mcp.json` for Codex

3. Ensure the Codex marketplace and MCP servers are registered if you use Codex:
   ```bash
   codex plugin marketplace add ~/.speq-skill/codex
   codex mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --project-from-cwd --context=codex
   codex mcp add context7 -- npx -y @upstash/context7-mcp
   ```

4. Restart Claude Code or Codex to reconnect

---

## Dependencies

| Dependency | Purpose | License |
|------------|---------|---------|
| [Serena](https://github.com/oraios/serena) | Semantic code navigation | MIT |
| [Context7](https://github.com/upstash/context7) | Library documentation | MIT |
| [snowflake-arctic-embed-xs](https://huggingface.co/Snowflake/snowflake-arctic-embed-xs) | Embeddings model (~23MB) | Apache 2.0 |

> [!NOTE]
> The embeddings model is downloaded during installation into `~/.cache/speq/models/`.
