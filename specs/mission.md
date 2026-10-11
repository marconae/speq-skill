# Mission: speq

> A CLI tool to get, list, and validate feature specifications for spec-driven development with AI coding agents.

## Problem Statement

AI coding agents work more effectively with structured, consistent specifications. Without tooling, feature specs:
- Lack structural consistency (missing sections, malformed scenarios)
- Cannot be verified programmatically

speq provides structural verification and exploration tools for software specifications based on Markdown files.

## Target Users

| Persona | Goal |
|---------|------|
| AI coding agent | Read/write consistent specs during development |
| Developer | Investigate and maintain specs |

## Core Capabilities

1. **Retrieve spec content** — Fetch a full feature spec or a single scenario by path (`speq feature get <domain>/<feature>[/<scenario>]`)
2. **List features and domains** — Display specs in a tree view organized by domain, or list domains alone (`speq domain list`)
3. **Validate spec structure** — Check specs for required sections (Feature, Background, Scenarios), RFC2119 keyword usage, and scenario format
4. **Inspect plans in flight** — List active plans under `_plans/` and validate a plan's delta specs before recording (`speq plan list` / `speq plan validate <plan>`)
5. **Record plan deltas** — Merge approved delta specs from `_plans/` into permanent specs and archive the plan
6. **Track architecture decisions** — Validate and assemble per-plan ADR fragments in `specs/_decision/` into a stable decision record (`speq decision-log validate` / `show`)

## Out of Scope

- Execution engine — Does not run or execute specifications like a test runner
- Code generation — Does not generate implementation code from specs
- Version control — Does not manage spec history beyond archiving recorded plans
- Installation — Does not cover the installer or plugin tooling

Architecture: see specs/architecture.md.

## Domain Glossary

| Term | Definition |
|------|------------|
| Spec | A feature specification file (`spec.md`) written in Gherkin-like Markdown |
| Domain | A grouping of related features (e.g., `cli/`, `validation/`) |
| Feature | A single capability defined with Background and Scenarios sections |
| Scenario | A specific behavior described with GIVEN/WHEN/THEN steps |
| Delta | A proposed change to a spec marked with `<!-- DELTA:NEW -->`, `<!-- DELTA:CHANGED -->`, or `<!-- DELTA:REMOVED -->`, targeting a `### Scenario:`, or realigning an existing feature's `## Background` or `# Feature: <name>` description to match a scenario change |
| Plan | A set of deltas in `_plans/<plan-name>/` awaiting approval |
| Record | The action of moving approved deltas from `_plans/` to permanent specs in `specs/` |
| ADR | An Architecture Decision Record — a single durable decision captured as a slug-identified fragment in `specs/_decision/`. It is not the design section of a plan |

---

## Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Language | Rust (edition 2024) | Performance, reliability, single binary distribution |
| CLI | clap 4.5 | Command-line argument parsing with derive macros |
| Parsing | pulldown-cmark | Markdown parsing for spec analysis |
| Errors | thiserror 2 | Ergonomic error type definitions |
| Testing | assert_cmd, predicates, tempfile | CLI integration testing |

## Commands

```bash
# Build
cargo build --release

# Test
cargo test

# Lint & Format
cargo fmt && cargo clippy

# Gather Code Coverage
cargo llvm-cov
```

## Project Structure

```
speq-skill/
├── src/                  # Rust source
├── specs/                # Feature specifications
│   ├── <domain>/         # Domain grouping
│   │   └── <feature>/    # Feature directory
│   │       └── spec.md   # Specification file
│   ├── _plans/           # Pending plan deltas
│   ├── _recorded/        # Archived recorded plans
│   └── _decision/        # Architecture Decision Record fragments
└── tests/                # Integration tests
```

## Constraints

- **Spec format stability**: Spec structure must remain backward compatible to avoid breaking existing specs
