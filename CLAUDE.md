# Local Development Rules

You are building `speq-skill` while using it: the skills in `.claude/skills/` are simultaneously this repo's dev tooling and the source that `scripts/plugin/build.sh` compiles into the Claude and Codex plugin.

## Rules

- **CLI**: always invoke the local build — `./target/debug/speq <cmd>` (or `./target/release/speq <cmd>` after a release build). Never the global `speq` or `cargo run --`.
- **Skill names are context-specific**: `/speq-*` in this repo, `/speq:*` in the installed plugin. `build.sh` performs the `speq-*` → `speq:*` rename, drops the `speq-` folder prefix, and stamps version + author from `Cargo.toml`.
- **Version**: `Cargo.toml` is the single source of truth. `scripts/lib/version.sh` (`get_version`) feeds `build.sh` and the docker tests — never hard-code the version anywhere else.
- **Tests**: integration tests SHALL use fixtures under `tests/fixtures/`, not inline spec strings.
- **Git**: `git-agent` is the only agent permitted to write git history or touch a remote; every other agent is read-only.
- **Commits** follow Conventional Commits — `<type>[scope]: <description>` (+ optional body/footer). Types: `feat` (MINOR), `fix` (PATCH), `perf`, `refactor`, `test`, `docs`, `spec`, `chore`. Breaking change = `!` after type/scope or a `BREAKING CHANGE:` footer (MAJOR).
- **Expert tasks**: `planner-agent` marks reasoning-heavy `tasks.md` lines `[expert]`; `speq-implement` routes those to `implementer-expert-agent`, the rest to `implementer-agent`. Tag sparingly.
- **Model routing** is hardcoded in each skill/agent frontmatter and stamped by `build.sh` — see `docs/model-routing.md`.
- **Mission scope**: `specs/mission.md` is the mission for the `speq` CLI only — not for the skills. Skill purpose and intent live in the skill files and `docs/`.

## Commands

- Build the plugin: `./scripts/plugin/build.sh`
- Release artifact build / test: `./scripts/release/build.sh <vX.Y.Z>` · `./scripts/release/test.sh <vX.Y.Z>`
