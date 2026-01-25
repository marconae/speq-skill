<div align="center">

<img src="assets/speq-skill-logo.svg" alt="Speq Skill Logo" width="200">

# speq-skill

**A light-weight and straightforward system for spec-driven development with Claude Code**

[![spec|driven](https://img.shields.io/badge/spec-driven-blue)](specs/)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/marconae/speq-skill/actions/workflows/ci.yml/badge.svg)](https://github.com/marconae/speq-skill/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[Why](#why-i-built-it) • [Getting Started](#getting-started) • [How It Works](#how-does-it-work) • [Reference](#reference)
</div>

---

## Why I Built It

I want to leverage Claude Code as an effective tool to write software.

There are other spec-driven development tools out there; OpenSpec, BMAD, SpecKit... But I was missing the following:
1. A straightforward repeatable workflow (`plan → implement → record`)
2. A system that is not primped on one language or framework (e.g. Python or TypeScript only)
3. A permanent and growing spec-library
4. A system that keeps the specs **small** to avoid context cluttering
5. A system that keeps asking me instead of making assumptions

So I built `speq-skill`. It combines Claude Skills with a simple CLI called `speq` that adds a semantical search layer to the spec library. The search empowers the coding agent to find the right feature scenarios during planning, but also during the implementation. This avoids reading unnecessary specs into the context window. 

## Who should use it?

If you want to describe what you want and have a coding agent build the code for you. If you can answer this question with "yes", then you should give `speq-skill` a try!

Vibe Coding does not scale. `speq-skill` fixes this. It adds a lightweight and straightforward system for spec-driven development that engineers the context for the coding agent.

## Getting Started

```bash
# Install CLI
cargo install --git https://github.com/marconae/speq-skill speq

# Install plugin
claude plugin install speq@https://github.com/marconae/speq-skill

# Run Claude Code
claude

# Start with a mission
You: /speq:mission-creator
```

---

## How Does it Work?

```
/spec-planner →    /spec-implementer  →  /spec-recorder
     │                  │                   │
     ▼                  ▼                   ▼
Creates plan.md    TDD implementation    Merges deltas
with deltas        + verification        to permanent specs
```

1. **Plan** — Describe what you want. The agent searches existing specs, asks clarifying questions, and creates a plan with spec deltas.
2. **Implement** — The agent implements features using TDD. Each scenario gets a failing test first, then minimal code to pass.
3. **Record** — Review newly built feature and merge planned deltas into permanent specs. The plan archives to `_recorded/`.

Specs live in `specs/<domain>/<feature>/spec.md`. Plans stage in `specs/_plans/<plan-name>/`. The separation keeps your spec library clean while work is in progress.

---

## Reference

### Skills

Four Claude Code skills form the workflow:

| Skill | Trigger | Purpose |
|-------|---------|---------|
| `/mission-creator` | Project init, specs setup | Interview user, explore codebase, generate `specs/mission.md` |
| `/spec-planner` | Plan mode, feature planning | Search specs, interview user, create plan with deltas |
| `/spec-implementer` | Implement plan | TDD cycle with mandatory evidence, generate verification report |
| `/spec-recorder` | Record approved plan | Merge deltas to permanent specs, validate, archive plan |

### CLI

`speq` provides spec discovery and semantic search:

```bash
# Structure
speq domain list                              # List all domains
speq feature list                             # Tree view of all features
speq feature list <domain>                    # Features in one domain

# Content
speq feature get cli/validate                 # Full feature spec
speq feature get "cli/validate/Error case"   # Single scenario

# Search
speq search query "error handling"            # Semantic search across all scenarios
speq search query "validation" --limit 5      # Limit results

# Validation
speq feature validate                         # Validate all specs
speq feature validate cli                     # Validate domain
speq feature validate cli/validate            # Validate single feature

# Recording
speq record <plan-name>                       # Merge plan deltas to permanent specs
```

### Spec Format

Specs use Gherkin-like Markdown with RFC 2119 keywords:

```markdown
# Feature Name

Description of the feature.

## Background

Common context for all scenarios.

## Scenarios

### Happy path

  Given some precondition
  When the user performs an action
  Then the system SHALL respond with expected result

### Error case

  Given an invalid state
  When the user attempts the action
  Then the system MUST return an error
```

**Keywords:** `MUST`, `MUST NOT`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `MAY`

### Directory Structure

```
specs/
├── mission.md                    # Project purpose, tech stack, commands
├── <domain>/
│   └── <feature>/
│       └── spec.md               # Permanent specs
├── _plans/
│   └── <plan-name>/
│       ├── plan.md               # Implementation plan
│       ├── verification-report.md
│       └── <domain>/<feature>/spec.md  # Delta specs
└── _recorded/
    └── <plan-name>/              # Archived completed plans
```

---

## Important

`speq-skill` is a plugin for Claude Code and compatible AI coding assistants. This tool provides workflow structure and spec management only—**the AI agent generates all code and specifications**. You are responsible for reviewing and approving all AI-generated output before use.

## Dependencies

This plugin uses [Serena](https://github.com/oraios/serena) and [Context7](https://github.com/upstash/context7) MCP servers (both MIT licensed).

## License

Free and open-source under MIT. See [LICENSE](LICENSE) for details.
