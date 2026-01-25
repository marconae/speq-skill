# Local Development Rules

You are **building speq-skill while using it**.

## Plugin Development Duality

`.claude/skills/` contains skills that serve two roles:
1. **Development usage** — Used directly when working on this repo
2. **Plugin source** — Transformed by `scripts/build-plugin.sh` into distributable `plugin/`

### Directory Structure

```
.claude/skills/
├── util/                        # Utility skills (reusable guidance)
│   ├── code-tools/SKILL.md
│   ├── ext-research/SKILL.md
│   ├── implementer/SKILL.md
│   ├── git-discipline/SKILL.md
│   └── speq-cli/SKILL.md
├── speq-plan/                   # Workflow skill
├── speq-implement/              # Workflow skill
├── speq-record/                 # Workflow skill
└── speq-mission/                # Workflow skill
```

### Build Script

The build script (`scripts/build-plugin.sh`):
- Copies utility skills from `.claude/skills/util/*` to `plugin/skills/*`
- Copies workflow skills from `.claude/skills/speq-*` to `plugin/skills/*` (drops `speq-` prefix)
- Updates skill name prefixes from local (`speq-*`) to plugin (`speq:*`)
- Updates skill cross-references for plugin context
- Generates `plugin.json` manifest

### Invocation Patterns

| Context | Workflow Skills | Utility Skills |
|---------|-----------------|----------------|
| Local (dev) | `/speq-plan`, `/speq-implement`, `/speq-record`, `/speq-mission` | `/code-tools`, `/ext-research`, `/implementer`, `/git-discipline`, `/speq-cli` |
| Plugin | `/speq:plan`, `/speq:implement`, `/speq:record`, `/speq:mission` | `/speq:code-tools`, `/speq:ext-research`, `/speq:implementer`, `/speq:git-discipline`, `/speq:speq-cli` |

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

## Mission Reference for speq CLI

See `specs/mission.md` for purpose, tech stack, commands, and architecture of the speq CLI.
