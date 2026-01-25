# Local Development Rules

You are **building speq-skill while using it**.

## Plugin Development Duality

`.claude/skills/` contains skills that serve two roles:
1. **Development usage** — Used directly when working on this repo
2. **Plugin source** — Transformed by `scripts/build-plugin.sh` into distributable `plugin/`

### Directory Structure

```
.claude/skills/
├── speq-plan/                   # Workflow skill (speq- prefix)
├── speq-implement/              # Workflow skill
├── speq-record/                 # Workflow skill
├── speq-mission/                # Workflow skill
├── code-guardrails/             # Utility skill (no prefix)
├── code-tools/                  # Utility skill
├── ext-research/                # Utility skill
├── git-discipline/              # Utility skill
└── speq-cli/                    # Utility skill
```

### Build Script

The build script (`scripts/plugin/build.sh`):
- Copies utility skills from `.claude/skills/` to `plugin/skills/`
- Copies workflow skills from `.claude/skills/speq-*` to `plugin/skills/*` (drops `speq-` prefix)
- Updates skill name prefixes from local (`speq-*`) to plugin (`speq:*`)
- Updates skill cross-references for plugin context
- Generates `plugin.json` manifest

### Invocation Patterns

| Context | Workflow Skills | Utility Skills |
|---------|-----------------|----------------|
| Local (dev) | `/speq-plan`, `/speq-implement`, `/speq-record`, `/speq-mission` | `/code-guardrails`, `/code-tools`, `/ext-research`, `/git-discipline`, `/speq-cli` |
| Plugin | `/speq:plan`, `/speq:implement`, `/speq:record`, `/speq:mission` | `/speq:code-guardrails`, `/speq:code-tools`, `/speq:ext-research`, `/speq:git-discipline`, `/speq:speq-cli` |

## speq CLI Invocation

**This repo builds `speq` while using it.** Always invoke via local build:

```bash
./target/debug/speq <command>    # After cargo build
./target/release/speq <command>  # After cargo build --release
```

**Never use:**
- `speq` (global)
- `cargo run --` (inconsistent)
- Nested paths like `../target/debug/speq`

## Scripts Directory

```
scripts/
├── release/
│   ├── build.sh    # Build release artifact for current platform
│   └── test.sh     # Test release artifact locally
└── plugin/
    └── build.sh    # Build Claude plugin from .claude/skills/
```

### Release Scripts

```bash
# Build release for current platform
./scripts/release/build.sh v0.2.0

# Test release artifact (builds if needed)
./scripts/release/test.sh v0.2.0
```

### Plugin Scripts

```bash
# Build distributable plugin
./scripts/plugin/build.sh
```

## Mission Reference for speq CLI

See `specs/mission.md` for purpose, tech stack, commands, and architecture of the speq CLI.
