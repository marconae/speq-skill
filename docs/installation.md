[speq-skill](../README.md) / [Docs](./index.md) / Installation

---

# Installation

## Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/scripts/install.sh | bash
```

---

## Prerequisites

- **macOS or Linux** (Windows via WSL)
- **Claude Code CLI** installed and configured
- **Rust toolchain** (installed if missing or via [rustup](https://rustup.rs/))

---

## What Gets Installed

| Component | Location |
|-----------|----------|
| `speq` CLI | `~/.local/bin/speq` |
| Plugin files | `~/.speq-skill/` |
| Claude plugins | `~/.claude/plugins/` (symlinked) |

The installer automatically:
- Downloads the latest release from GitHub
- Symlinks the `speq` CLI to your PATH
- Installs the speq-skill plugin for Claude Code
- Installs required MCP servers (Serena, Context7)

---

## Installation from Source

```bash
# Clone the repository
git clone https://github.com/marconae/speq-skill && cd speq-skill

# Build and install CLI + plugin
./scripts/local-install.sh
```

Requires Rust toolchain (install via [rustup](https://rustup.rs/)).

---

## Verify Installation

```bash
# Check CLI is available
speq --version

# Check plugin is installed
ls ~/.claude/plugins/speq-skill

# Test in Claude Code
claude
/speq:mission  # Should show the mission creator workflow
```

---

## Update

Re-run the install script to get the latest version:

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/scripts/install.sh | bash
```

---

## Uninstall

```bash
# If installed from source
./scripts/uninstall.sh

# If installed via quick install
rm -rf ~/.speq-skill
rm ~/.local/bin/speq
rm ~/.claude/plugins/speq-skill
```

---

## Troubleshooting

### `speq: command not found`

Add `~/.local/bin` to your PATH:

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.local/bin:$PATH"

# Reload shell
source ~/.bashrc  # or ~/.zshrc
```

### Rust build errors

Ensure Rust toolchain is installed and up to date:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update
```

### Plugin not found in Claude Code

1. Verify plugin is installed:
   ```bash
   ls ~/.claude/plugins/speq-skill
   ```

2. Restart Claude Code:
   ```bash
   claude
   ```

3. Check plugin loads:
   ```
   /speq:mission
   ```

### MCP server connection errors

The plugin depends on Serena and Context7 MCP servers. If you see connection errors:

1. Check servers are installed:
   ```bash
   ls ~/.claude/plugins/speq-skill/mcp/
   ```

2. Verify server configuration in `~/.claude/plugins/speq-skill/.mcp.json`

3. Restart Claude Code to reconnect

---

## Dependencies

| Dependency | Purpose | License |
|------------|---------|---------|
| [Serena](https://github.com/oraios/serena) | Semantic code navigation | MIT |
| [Context7](https://github.com/upstash/context7) | Library documentation | MIT |
| [snowflake-arctic-embed-xs](https://huggingface.co/Snowflake/snowflake-arctic-embed-xs) | Embeddings model (~23MB) | Apache 2.0 |

The embeddings model downloads automatically on first `speq search` command.
