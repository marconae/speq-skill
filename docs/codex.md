[speq-skill](../README.md) / [Docs](./index.md) / Using with Codex

---

# Using speq-skill with Codex

This guide explains how to use `speq-skill` in Codex.

## Prerequisites

- macOS or Linux
- Codex installed and configured
- Rust toolchain (installer can install it if missing)

## Install

Install the `speq` CLI:

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

Install Codex skill files:

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/scripts/install-codex-skills.sh | bash
```

Restart Codex to pick up new skills.

## Verify

```bash
speq --version
ls ~/.codex/skills/speq-*
```

## Workflow in Codex

Codex does not use Claude slash commands. Invoke skills in natural language by name:

```text
Use speq-mission to create specs/mission.md for this project.
Use speq-plan to create a plan for <feature>.
Use speq-implement to execute <plan-name>.
Use speq-record to merge and archive <plan-name>.
```

The underlying workflow remains the same:

`mission -> plan -> implement -> record`

## Notes on Agent-Specific Commands

Some skill text references Claude-specific commands (for example `/clear`). In Codex, treat these as intent:

- `/clear` means "start with a fresh context window"
- Claude slash command names map to skill names in your prompt

