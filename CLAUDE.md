# Development Rules

## Rules

- **Mental model**: this repo builds speq-skill while it uses speq-skill. `.claude/skills/` is both dev tooling and the source `scripts/plugin/build.sh` compiles into the plugin.
- **CLI**: use the local build only — `./target/debug/speq <cmd>` (`./target/release/speq <cmd>` after a release build). Do not use the global `speq` or `cargo run --`.
- **Skill names**: use `/speq-*` in this repo. The installed plugin uses `/speq:*`. `build.sh` renames them. Do not rename by hand.
- **Version**: `Cargo.toml` is the only source of the version number. Read it through `scripts/lib/version.sh` (`get_version`). Do not hardcode the version anywhere else.
- **Tests**: put integration test specs in files under `tests/fixtures/`. Do not write specs as inline strings.
- **Models**: do not add embedding model or tokenizer files (`model.onnx`, `tokenizer.json`, other weights) to this repo. Users get them through `install.sh`, and tests get them through `ensure_model_cached()` in `tests/common/mod.rs`. The only exception is the empty placeholder in `tests/fixtures/model-stub/`.
- **Commits**: use Conventional Commits — `<type>[scope]: <description>`. Types: `feat`, `fix`, `perf`, `refactor`, `test`, `docs`, `spec`, `chore`. Mark a breaking change with `!` after the type, or a `BREAKING CHANGE:` footer.
- **Changelog**: `CHANGELOG.md` lists user-facing features and behavior changes only. Do not add `Fix:` entries, bug-fix notes, internal agent or skill names, tags, refactors, dependency bumps, or other technical detail. Describe what a user sees or does differently.
- @specs/mission.md applies and describes the `speq` CLI only.
- @specs/architecture.md describes the speq CLI architecture only.
- Skill purpose and intent live in the skill files and `docs/`.

## Commands

- Build the plugin: `./scripts/plugin/build.sh`
- Build a release artifact: `./scripts/release/build.sh <vX.Y.Z>`
- Test a release artifact: `./scripts/release/test.sh <vX.Y.Z>`
