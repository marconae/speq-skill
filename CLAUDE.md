# speq-skill: Local Development Rules

## Rules

- **Mental model**: this repo builds speq-skill while it uses speq-skill. `.claude/skills/` is both dev tooling and the source `scripts/plugin/build.sh` compiles into the plugin.
- **CLI**: use the local build only — `./target/debug/speq <cmd>` (`./target/release/speq <cmd>` after a release build). Do not use the global `speq` or `cargo run --`.
- **Skill names**: use `/speq-*` in this repo. The installed plugin uses `/speq:*`. `build.sh` renames them. Do not rename by hand.
- **Version**: `Cargo.toml` is the only source of the version number. Read it through `scripts/lib/version.sh` (`get_version`). Do not hardcode the version anywhere else.
- **Tests**: put integration test specs in files under `tests/fixtures/`. Do not write specs as inline strings.
- **Commits**: use Conventional Commits — `<type>[scope]: <description>`. Types: `feat`, `fix`, `perf`, `refactor`, `test`, `docs`, `spec`, `chore`. Mark a breaking change with `!` after the type, or a `BREAKING CHANGE:` footer.
- **Markdown**: keep one line per paragraph and per list item in prose. Do not wrap prose to a fixed width. Fenced code, tables, diagrams, YAML frontmatter, and `**Field:**` lines keep their own line breaks.
- **Mission**:
    - @specs/mission.md applies and describes the `speq` CLI only.
    - Skill purpose and intent live in the skill files and `docs/`.

## Commands

- Build the plugin: `./scripts/plugin/build.sh`
- Build a release artifact: `./scripts/release/build.sh <vX.Y.Z>`
- Test a release artifact: `./scripts/release/test.sh <vX.Y.Z>`
