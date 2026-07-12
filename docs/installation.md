[speq-skill](../README.md) / [Docs](./index.md) / Installation

---

# Installation

## Quick install

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

> [!NOTE]
> The installer downloads a pre-built `speq` binary for your platform when one is available (Linux x86_64/ARM64, macOS Apple Silicon). Otherwise it builds from source with the Rust toolchain, offering to install [rustup](https://rustup.rs/) for you if Rust is missing.

Open Claude Code or Codex and type `/speq:mission` to start.

## Prerequisites

- macOS or Linux (Windows via WSL)
- Claude Code CLI or Codex CLI/App, installed and configured
- Rust toolchain — only needed if no pre-built binary matches your platform; the installer installs it for you if missing, or get it via [rustup](https://rustup.rs/)

## What gets installed

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

The installer also:
- Copies the pre-built `speq` binary to your PATH, or, on platforms without one (e.g. Intel Mac), downloads the release source and builds it with the Rust toolchain
- Installs the speq-skill plugin for Claude Code and Codex
- Registers the local Codex marketplace via `codex plugin marketplace add`, when Codex is installed
- Registers Serena and Context7 via `codex mcp add`, when Codex is installed
- Installs Codex skills into `$CODEX_HOME/skills` so Codex can load `/speq:*`
- Installs plugin MCP configuration for Serena and Context7
- Downloads the `snowflake-arctic-embed-xs` embedding model (~23 MB) into `~/.cache/speq/models/`

## Install from source

To build manually instead of using the pre-built binary — or if your platform has none:

```bash
# Clone the repository
git clone https://github.com/marconae/speq-skill && cd speq-skill

# Build and install CLI + plugin
./scripts/local-install.sh
```

> [!NOTE]
> Requires the Rust toolchain (install via [rustup](https://rustup.rs/)).

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
./scripts/uninstall.sh
```

## Troubleshooting

### `speq: command not found`

Add `~/.local/bin` to your PATH.

### Rust build errors

If the installer fell back to a source build (no pre-built binary for your platform), or you ran `./scripts/local-install.sh` directly, update your Rust toolchain:

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

3. Check the plugin loads:
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

The plugin depends on the Serena and Context7 MCP servers. If you see connection errors:

1. Check the servers are installed:
   ```bash
   ls ~/.speq-skill/plugins/speq-skill/.mcp.json
   ls ~/.speq-skill/codex/plugins/speq-skill/.mcp.json
   ```

2. Verify the server configuration in `~/.speq-skill/plugins/speq-skill/.mcp.json` for Claude, or `~/.speq-skill/codex/plugins/speq-skill/.mcp.json` for Codex.

3. If you use Codex, ensure the marketplace and MCP servers are registered:
   ```bash
   codex plugin marketplace add ~/.speq-skill/codex
   codex mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --project-from-cwd --context=codex
   codex mcp add context7 -- npx -y @upstash/context7-mcp
   ```

4. Restart Claude Code or Codex to reconnect.

## Dependencies

| Dependency | Purpose | License |
|------------|---------|---------|
| [Serena](https://github.com/oraios/serena) | Semantic code navigation | MIT |
| [Context7](https://github.com/upstash/context7) | Library documentation | MIT |
| [snowflake-arctic-embed-xs](https://huggingface.co/Snowflake/snowflake-arctic-embed-xs) | Embeddings model (~23MB) | Apache 2.0 |

> [!NOTE]
> The embeddings model is downloaded during installation into `~/.cache/speq/models/`.
