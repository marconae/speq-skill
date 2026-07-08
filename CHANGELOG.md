# Changelog

## 0.8.1

- Headless PR pipeline now titles PRs with a conventional-commit feature title `<type>(<scope>): <slug>` derived from the plan-name, instead of `spec(plan): <plan-name>`
- `speq-plan-pr` opens the PR as a draft; `speq-implement-pr` marks it ready once the implementation is pushed

## 0.8.0

- Add a 6th `code-reviewer` category, "YAGNI / Over-Engineering": flags unneeded dependencies, speculative abstractions (single-implementation interfaces/generics/config values), dead flexibility (unused feature flags/extension points), reinvented standard-library logic, and shrinkable code — every finding is delegated to the implementer agents like any other finding, with `[expert]` tagging for removals that have cross-file or subtle-correctness implications
- Give all 6 `code-reviewer` categories consistent per-finding `[TAG]` markers (previously 5 categories shared one blanket tag each)
- Add a `Dependency Rule` and `YAGNI Checks` to `speq-code-guardrails` so unneeded dependencies and speculative abstractions are avoided at implementation time, not just caught by review afterward
- Keep `code-reviewer`/`speq-code-guardrails` wording technology-agnostic — the reviewer runs against arbitrary target-project languages, not just this repo's own Rust codebase

## 0.7.0

- Add `speq-plan-pr` and `speq-implement-pr`: headless, non-interactive counterparts to `speq-plan`/`speq-implement` that plan/implement against a `feat/<plan-name>` branch and open or update a PR
- Add `git-pr-agent`, the only sub-agent permitted to write git history or touch a remote
- `planner-agent` gains a headless escalation mode: assume-and-document conventional decisions, escalate only irreducible ones via an `OPEN QUESTIONS:` sentinel

## 0.6.0

- Add a multi-platform release pipeline: CI cross-compiles Linux x86_64/ARM64, macOS, and Windows; the installer tries a pre-built binary download first, falling back to a source build
- Tighten `deny.toml`/`about.toml` license allow-lists to the 5 licenses actually present in the dependency tree, and explicitly ban the `openssl`/`native-tls`/`boring` crate family
- Fix stale `THIRD_PARTY_LICENSES` notices left over from the `tract-onnx` migration and update README/installation docs to describe pre-built-binary-first installation

## 0.5.1

- Replace `candle-core`/`candle-nn`/`candle-transformers` with `tract-onnx` for embedding inference, loading the upstream `model.onnx` graph directly — removes the vendored `gemm-common` patch and resolves the L4-cache panic structurally rather than patching around it
- Simplify provisioned model files to `model.onnx` + `tokenizer.json` (drops `config.json`)
- Fix installer: `main` was silently skipped when piped via `curl | bash` (`BASH_SOURCE[0]` is empty in that mode)

## 0.5.0

- Replace `fastembed`/ONNX Runtime with a pure-Rust `candle` inference stack — eliminates the Intel Mac crash caused by missing ONNX Runtime prebuilt binaries
- Add `src/embedding.rs`: `Embedder` struct backed by `candle-transformers` BERT (CPU-only); loads model files from installer-provisioned cache; fails fast with an actionable error when files are missing
- Vendor `gemm-common` with an OOB bounds-check fix for CPUs that expose L4 cache (prevents panic in `gemm-common 0.19.0`)
- Add embedding model provisioning to `install.sh`: downloads `model.safetensors`, `tokenizer.json`, and `config.json` from HuggingFace on every install (always refreshes on update)
- Remove Intel Mac `brew install onnxruntime` workaround from `install.sh` and `scripts/release/build.sh`

## 0.4.3

Security patch — no functional changes:
- `openssl` 0.10.76 → 0.10.79: fixes 7 advisories (buffer overflows in `digest_final`, AES key-wrap, and PEM callback; undefined behavior in `X509Ref::ocsp_responders`; PSK/cookie trampoline memory leak)
- `quinn-proto` 0.11.13 → 0.11.14: fixes unauthenticated remote DoS via panic in QUIC transport parameter parsing
- `rand` 0.9.2 → 0.9.4: fixes unsound aliased mutable reference when using a custom logger with `rand::rng()`

All are transitive dependencies pulled in by `fastembed` → `hf-hub` / `tokenizers` / `rav1e`. Direct dependencies and public API are unchanged.

## 0.4.2

- Fix double MCP server registration: remove `mcpServers` wrapper from `mcp.json`, `mcp-codex.json`, `plugin.json`, and `codex-plugin.json` — platforms expect a flat server map, not a nested object
- Add regression tests (`mcp_json_uses_flat_format`, `mcp_codex_json_uses_flat_format`) asserting the flat structure for both Claude and Codex configs

## 0.4.1

- Fix Serena MCP server startup: replace `"--project", "${PWD}"` with `"--project-from-cwd"` in `scripts/plugin/mcp.json` (bash variable was never expanded inside a JSON file)
- Add regression tests in `tests/mcp_config.rs` asserting both `mcp.json` and `mcp-codex.json` use `--project-from-cwd` and contain no static `${PWD}` path

## 0.4.0

- Add Codex plugin generation alongside the existing Claude Code marketplace payload
- Register the Codex marketplace through `codex plugin marketplace add ~/.speq-skill/codex` while keeping `~/.speq-skill` as the single install root
- Install generated Codex skills into `$CODEX_HOME/skills` so Codex can discover `/speq:*`
- Keep installed skills invocable as `/speq:*` on both Claude Code and Codex
- Add Codex plugin MCP declarations for Serena and Context7
- Hardcode Codex model routing for the initial platform support release; dynamic routing config is deferred

## 0.3.1

- New `speq decision-log validate` command — validates `specs/decision-log.md` against ADR/Nygard format (sequential numbering, required fields, Status values)
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

- Fix word boundary matching for RFC 2119 keywords (prevents false positives on substrings like "note"/"not")
- Add curl-pipeable uninstaller

## 0.2.4

- Add `plan list` command
- Migrate MCP config to plugin
- Fix installer exit when Rust toolchain is missing

## 0.2.2

- Initial release
