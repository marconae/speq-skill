# Local Development Rules

You are **building speq-skill while using it**.

## Plugin Development Duality

`.claude/skills/` contains `speq-*` prefixed skills that serve two roles:
1. **Development usage** — Used directly when working on this repo
2. **Plugin source** — Transformed by `scripts/build-plugin.sh` into distributable `plugin/`

The build script:
- Copies skills from `.claude/skills/speq-*` to `plugin/skills/*` (drops prefix)
- Merges `.claude/rules/` into skill reference files
- Generates `plugin.json` manifest

Invoke skills locally with `/speq-planner`, `/speq-implementer`, etc.
Built plugin uses `/speq:planner`, `/speq:implementer`, etc.

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