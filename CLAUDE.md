# Local Development Rules

You are **building speq-skill while using it**.

## Plugin Development Duality

`.claude/skills/` contains skills that serve two roles:
1. **Development usage** — Used directly when working on this repo
2. **Shared plugin source** — Transformed by `scripts/plugin/build.sh` into Claude and Codex plugin artifacts

### Directory Structure

```
.claude/skills/
├── speq-plan/                   # Workflow skill (orchestrator)
├── speq-implement/              # Workflow skill (orchestrator)
├── speq-record/                 # Workflow skill (orchestrator)
├── speq-mission/                # Workflow skill
├── speq-code-guardrails/        # Utility skill
├── speq-code-tools/             # Utility skill
├── speq-ext-research/           # Utility skill
├── speq-git-discipline/         # Utility skill
└── speq-cli/                    # Utility skill

.claude/agents/
├── planner-agent.md             # heavy planning
├── implementer-agent.md         # standard implementation
├── implementer-expert-agent.md  # hard, reasoning-heavy tasks
├── code-reviewer.md             # adversarial review
└── recorder-agent.md            # deterministic spec merge
```

### Build Script

The build script (`scripts/plugin/build.sh`):
- Copies all skills from `.claude/skills/speq-*` into Claude and Codex plugin payloads (drops `speq-` prefix for folders)
- Transforms frontmatter names: `name: speq-*` → `name: speq:*`
- Transforms references: `/speq-*` → `/speq:*`
- Translates Claude-only workflow syntax out of Codex generated files
- Stamps version and author from `Cargo.toml` into `plugin.json`
- Builds Claude marketplace structure and Codex plugin/marketplace structure in `dist/marketplace/`

### Invocation Patterns

| Context | Workflow Skills | Utility Skills |
|---------|-----------------|----------------|
| Local (dev) | `/speq-plan`, `/speq-implement`, `/speq-record`, `/speq-mission` | `/speq-code-tools`, `/speq-ext-research`, `/speq-code-guardrails`, `/speq-git-discipline`, `/speq-cli` |
| Installed plugin (Claude/Codex) | `/speq:plan`, `/speq:implement`, `/speq:record`, `/speq:mission` | `/speq:code-tools`, `/speq:ext-research`, `/speq:code-guardrails`, `/speq:git-discipline`, `/speq:cli` |

## Model Routing Strategy

Model routing is hardcoded in generated artifacts for `0.4.0`. Dynamic model-routing configuration is deferred to a later release.

Claude defaults:
- `speq-plan`, `speq-implement`, `speq-record`: `model: sonnet`
- `speq-mission` and utility skills: inherit caller model
- heavy agents (`planner-agent`, `implementer-expert-agent`, `code-reviewer`): `model: opus`, `effort: xhigh`
- `implementer-agent`: `model: sonnet`, `effort: high`
- `recorder-agent`: `model: sonnet`, `effort: medium`

Codex defaults:
- `speq:plan`, `speq:implement`, `speq:record`: `model: gpt-5.4`, `effort: medium`
- `speq:mission` and utility skills: inherit caller model
- heavy agents (`planner-agent`, `implementer-expert-agent`, `code-reviewer`): `model: gpt-5.5`, `effort: xhigh`
- `implementer-agent`: `model: gpt-5.4`, `effort: high`
- `recorder-agent`: `model: gpt-5.4`, `effort: medium`

### Principle

> Orchestration is cheap. Reasoning is expensive. Put the expensive tier only where defects compound.

Workflow skills (`speq-plan`, `speq-implement`, `speq-record`) are thin orchestrators. They read tasks.md, dispatch sub-agents, and confirm results — all tool-call heavy, reasoning light. The sub-agents they spawn do the actual work.

### Sub-agent routing table

| Sub-agent | Tier | Spawned by | Rationale |
|-----------|------|------------|-----------|
| `planner-agent` | heavy reasoning | `speq-plan` | Architectural tradeoffs, ADR authoring, MECE decomposition. Defects here compound through every downstream task. |
| `implementer-agent` | standard | `speq-implement` | Default for coding tasks. |
| `implementer-expert-agent` | heavy reasoning | `speq-implement` | Only for tasks tagged `[expert]` in tasks.md. Concurrency, cross-file refactors, non-obvious correctness. |
| `code-reviewer` | heavy reasoning | `speq-implement` | Adversarial review requires holding two large artifacts in mind and surfacing non-obvious defects. |
| `recorder-agent` | mechanical | `speq-record` | Apply delta markers, validate, archive. No reasoning premium. |

Actual model and effort values are stamped into generated platform artifacts by `scripts/plugin/build.sh`.

### Expert-task tagging

`planner-agent` tags tasks that require deep reasoning with `[expert]` at the end of the task line in `tasks.md`:

```markdown
- [ ] 2.1 Add CLI flag parsing
- [ ] 2.2 Implement lock-free queue for concurrent spec writes [expert]
```

`speq-implement` partitions tasks by tag before spawning: untagged → `implementer-agent`, tagged → `implementer-expert-agent`. If the plan is under-tagged, the orchestrator may add `[expert]` when materializing tasks.md — but sparingly. Over-tagging wastes tokens; under-tagging risks defects.

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
    └── build.sh    # Build Claude and Codex plugin artifacts from .claude/skills/
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

## Testing Rules

### Integration Tests

Integration tests SHALL use test fixtures instead of inline strings.

```
tests/
└── fixtures/           # Test fixture files
    ├── valid-plan/
    │   ├── plan.md
    │   └── domain/feature/spec.md
    └── invalid-spec/
        └── spec.md
```

**Do:**
```rust
let fixture_path = Path::new("tests/fixtures/valid-plan");
let result = validate_plan(fixture_path, "valid-plan");
```

**Don't:**
```rust
let content = r#"# Feature: Test
## Background
* context
## Scenarios
### Scenario: Test
* *GIVEN* setup
"#;
fs::write(tmp.path().join("spec.md"), content).unwrap();
```

**Rationale:** Fixtures are easier to maintain, can be validated by the tool itself, and provide realistic test data.

## Mission Reference for speq CLI

See `specs/mission.md` for purpose, tech stack, commands, and architecture of the speq CLI.
