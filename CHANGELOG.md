# Changelog

## 0.13.2

- Fix `build.sh` baking the `speq:` plugin namespace into each skill's own `name:` frontmatter, doubling up with the namespace Claude Code already applies at load time and producing `speq:speq:<skill>` everywhere; skill `name:` is now bare, matching how other plugins (e.g. `ponytail`) declare it

## 0.13.1

- Parallelize `index_specs`/`search_specs` with rayon, using order-preserving collects so ranking stays byte-identical to the serial implementation; 38% faster index build on a 1,000-scenario fixture
- Sharpen skill and agent frontmatter descriptions to name concrete triggers and callers instead of terse one-liners, for more reliable auto-invocation; DRY up orchestrator/sub-agent cross-references and collapse `speq-mission`'s duplicated question blocks

## 0.13.0

- Add repo-local `.speq/<name>-hook.md` project hooks: entry-point skills (`speq-mission`, `speq-plan`, `speq-plan-pr`, `speq-implement`, `speq-implement-pr`, `speq-record`, `speq-audit`) load them as an authoritative first workflow step, able to override any part of the default workflow, including guardrails
- `speq:audit` lists active `.speq/*-hook.md` files as an informational check
- See [docs/hooks.md](./docs/hooks.md)

## 0.12.0

- Add `plan-reviewer` (`opus`/`xhigh`): adversarially reviews a plan's intent fidelity, feasibility, requirement quality, task breakdown, and prose before handoff to `/speq:implement`
- Wire it into `/speq:plan`/`/speq:plan-pr`: BLOCKER findings loop `planner-agent` back to revise (logged as `[plan-review]` `decision-log.md` entries), capped at 2 rounds; unresolved blockers escalate via `AskUserQuestion` (interactive) or `OPEN QUESTIONS:` (headless)
- ADVISORY findings surface in the plan-ready report; never persisted or looped on

## 0.11.0

- Add `speq:audit`: read-only health check running spec-library validators and filesystem checks in one pass, then offers to fix each finding with confirmation
- Checks: `<domain>/<feature>` spec structure, `speq feature validate`, decision-log format and validity, `mission.md` ↔ spec-library sync, unrecorded plans in `_plans/`, `_recorded` gitignore hygiene, `_decision`/`_plans` tracked, recorded-folder naming, library thresholds (>10 scenarios, >8 features), and git hygiene
- Add `audit-agent` (`opus`/`high`): verifies `mission.md` against the spec library and returns inconsistencies, seeding a `/speq:mission` handoff on reconcile
- Remediation is user-gated: trivial fixes apply inline, structural fixes run in a spawned worker, mission drift hands off to `/speq:mission`; the audit itself never edits `mission.md` or authors specs

## 0.10.0

- Replace the single append-only `specs/decision-log.md` with one committed ADR fragment per plan: `specs/_decision/NNN-<plan-name>.md`
- Give each ADR a stable, kebab-case `**ID:**` slug; `Supersedes:`/`Status: Superseded by <slug>` reference slugs instead of `ADR-NNN`
- Add `speq decision-log show`: assembles `specs/_decision/*.md` fragments into one `# Architecture Decision Records` view on stdout, ordered by `NNN-` prefix; writes no merged file
- `speq decision-log validate` now validates the `specs/_decision/` directory: per-fragment structure, required fields, status vocabulary, cross-file slug uniqueness, and reference resolution; an absent/empty directory passes
- Archive completed plans to `specs/_recorded/NNN-<plan-name>/`, a record-time sequence number, instead of a `YYYY-MM-DD-<plan-name>` date prefix
- Drop the `**Date:**` field from ADRs and the plan-level decision log
- Remove the "Prose style" pointer line from the plan/feature/mission/verification/decision-log templates
- `recorder-agent` writes only the new fragment for the plan it records, never editing another; `planner-agent` now records the superseded decision's title
- Migrate the prior `specs/decision-log.md` (ADR-001..005) into `specs/_decision/001-refactor-search-pure-rust-inference.md` and `specs/_decision/002-refactor-embeddings-tract-onnx.md`, then delete the old file

## 0.9.0

- Add `speq:writing-guardrails`: prose guardrails (BLUF, Strunk & White, INCOSE GtWR, ISO 29148, RFC 2119) for speq artifacts and pipeline-composed GitHub PRs/issues/comments
- Load it into every prose-authoring component: planner-agent, recorder-agent (ADR step), speq-implement, speq-mission, speq-plan-pr, speq-implement-pr
- Add a "Prose style" pointer to the plan/feature/mission/verification/decision-log templates

## 0.8.2

- Rename `git-pr-agent` → `git-agent`, generalized into a git/GitHub operations worker (branches, commit, push, PRs, issues, draft-ready); runs one caller-specified operation per invocation, authors no content
- Move plan-pipeline semantics (open-questions.md, blocked banner, feat/<plan-name> naming, spec(plan) messages, comment text) out of the agent into `speq-plan-pr`/`speq-implement-pr`
- `speq-plan-pr` now always leaves the PR as a draft (including the resume-after-blocked path); `speq-implement-pr` remains the only skill that marks it ready

## 0.8.1

- Headless PR pipeline now titles PRs with a conventional-commit feature title `<type>(<scope>): <slug>` derived from the plan-name, instead of `spec(plan): <plan-name>`
- `speq-plan-pr` opens the PR as a draft; `speq-implement-pr` marks it ready once the implementation is pushed

## 0.8.0

- Add a 6th `code-reviewer` category, "YAGNI / Over-Engineering": flags unneeded dependencies, speculative abstractions, dead flexibility, reinvented standard-library logic, and shrinkable code; findings delegate to implementer agents, `[expert]`-tagged when cross-file or subtly correctness-sensitive
- Give all 6 `code-reviewer` categories consistent per-finding `[TAG]` markers
- Add a `Dependency Rule` and `YAGNI Checks` to `speq-code-guardrails`
- Keep `code-reviewer`/`speq-code-guardrails` wording technology-agnostic

## 0.7.0

- Add `speq-plan-pr` and `speq-implement-pr`: headless, non-interactive counterparts to `speq-plan`/`speq-implement` that plan/implement against a `feat/<plan-name>` branch and open or update a PR
- Add `git-pr-agent`, the only sub-agent permitted to write git history or touch a remote
- `planner-agent` gains a headless escalation mode: assume-and-document conventional decisions, escalate only irreducible ones via an `OPEN QUESTIONS:` sentinel

## 0.6.0

- Add a multi-platform release pipeline: CI cross-compiles Linux x86_64/ARM64, macOS, and Windows; the installer tries a pre-built binary download first, falling back to a source build
- Tighten `deny.toml`/`about.toml` license allow-lists to the 5 licenses present in the dependency tree, and explicitly ban the `openssl`/`native-tls`/`boring` crate family
- Fix stale `THIRD_PARTY_LICENSES` notices left over from the `tract-onnx` migration and update README/installation docs to describe pre-built-binary-first installation

## 0.5.1

- Replace `candle-core`/`candle-nn`/`candle-transformers` with `tract-onnx` for embedding inference, loading the upstream `model.onnx` graph directly; removes the vendored `gemm-common` patch
- Simplify provisioned model files to `model.onnx` + `tokenizer.json` (drops `config.json`)
- Fix installer: `main` was silently skipped when piped via `curl | bash`

## 0.5.0

- Replace `fastembed`/ONNX Runtime with a pure-Rust `candle` inference stack
- Add `src/embedding.rs`: `Embedder` struct backed by `candle-transformers` BERT (CPU-only); loads model files from installer-provisioned cache; fails fast with an actionable error when files are missing
- Vendor `gemm-common` with an OOB bounds-check fix for CPUs exposing L4 cache
- Add embedding model provisioning to `install.sh`: downloads `model.safetensors`, `tokenizer.json`, and `config.json` from HuggingFace on every install (always refreshes on update)
- Remove Intel Mac `brew install onnxruntime` workaround from `install.sh` and `scripts/release/build.sh`

## 0.4.3

Security patch — no functional changes:
- `openssl` 0.10.76 → 0.10.79: fixes 7 advisories (buffer overflows in `digest_final`, AES key-wrap, and PEM callback; undefined behavior in `X509Ref::ocsp_responders`; PSK/cookie trampoline memory leak)
- `quinn-proto` 0.11.13 → 0.11.14: fixes unauthenticated remote DoS via panic in QUIC transport parameter parsing
- `rand` 0.9.2 → 0.9.4: fixes unsound aliased mutable reference when using a custom logger with `rand::rng()`

Transitive dependencies pulled in by `fastembed` → `hf-hub` / `tokenizers` / `rav1e`.

## 0.4.2

- Fix double MCP server registration: remove `mcpServers` wrapper from `mcp.json`, `mcp-codex.json`, `plugin.json`, and `codex-plugin.json`
- Add regression tests (`mcp_json_uses_flat_format`, `mcp_codex_json_uses_flat_format`) asserting the flat structure for both Claude and Codex configs

## 0.4.1

- Fix Serena MCP server startup: replace `"--project", "${PWD}"` with `"--project-from-cwd"` in `scripts/plugin/mcp.json`
- Add regression tests in `tests/mcp_config.rs` asserting both `mcp.json` and `mcp-codex.json` use `--project-from-cwd` and contain no static `${PWD}` path

## 0.4.0

- Add Codex plugin generation alongside the existing Claude Code marketplace payload
- Register the Codex marketplace through `codex plugin marketplace add ~/.speq-skill/codex`, keeping `~/.speq-skill` as the single install root
- Install generated Codex skills into `$CODEX_HOME/skills`
- Keep installed skills invocable as `/speq:*` on both Claude Code and Codex
- Add Codex plugin MCP declarations for Serena and Context7
- Hardcode Codex model routing for the initial platform support release; dynamic routing config is deferred

## 0.3.1

- New `speq decision-log validate` command, validating `specs/decision-log.md` against ADR/Nygard format (sequential numbering, required fields, Status values)
- `speq plan validate` now validates optional `decision-log.md` in plan directories; absence is not an error
- New `src/validate/decision_log.rs` module with `validate_plan_log` and `validate_permanent_log`
- `planner-agent` generates `decision-log.md` capturing design decisions; `recorder-agent` promotes curated entries to permanent ADR log

## 0.3.0

- Split `speq-plan` and `speq-record` into thin orchestrators; heavy work now runs in dedicated sub-agents (`planner-agent`, `recorder-agent`)
- Add `implementer-expert-agent` sub-agent for reasoning-heavy tasks tagged `[expert]` in `tasks.md`; `speq-implement` partitions tasks by tag and routes accordingly
- Pin `model` and `effort` per sub-agent in frontmatter (opus/xhigh for planning, expert implementation, and review; sonnet/high for standard implementation; sonnet/medium for recording)
- Document model routing strategy in CLAUDE.md

## 0.2.9

- Reject mismatched and unclosed delta markers during record parsing
- Fall back to writable local cache when system cache is not writable
- Add `SPEQ_CACHE_DIR` environment variable to override cache location

## 0.2.8

- Support building on Intel Mac (x86_64-apple-darwin) via platform-conditional `ort-load-dynamic`
- Add OpenSSL prerequisite check to installer
- Add semantic anchors to skills and documentation
- Remove broken Anthropic Cookbook link from documentation
- Update LICENSE copyright to speq-skill contributors

## 0.2.7

- Add semantic anchors to skills and documentation

## 0.2.5

- Fix word boundary matching for RFC 2119 keywords
- Add curl-pipeable uninstaller

## 0.2.4

- Add `plan list` command
- Migrate MCP config to plugin
- Fix installer exit when Rust toolchain is missing

## 0.2.2

- Initial release
