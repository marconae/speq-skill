<div align="center">

<img src="assets/speq-skill-logo.svg" alt="Speq Skill Logo" width="200">

# speq-skill

**A light-weight and straightforward system for spec-driven development with Claude Code**

[![spec|driven](https://img.shields.io/badge/spec-driven-blue)](specs/)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/marconae/speq-skill/actions/workflows/ci.yml/badge.svg)](https://github.com/marconae/speq-skill/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[Getting Started](#getting-started) • [Why](#why-i-built-it) • [How It Works](#how-does-it-work) • [Reference](#reference) • [Installation](#installation)
</div>

---

## Getting Started

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

Then run `claude` and type `/speq:mission` to start.

---

## Why I Built It

I want to leverage Claude Code as an effective tool to write software.

There are other spec-driven development tools out there; OpenSpec, BMAD, SpecKit... But I was missing the following:
1. A straightforward repeatable workflow (`plan → implement → record`)
2. A system that is not primped on one language or framework (e.g., Python or TypeScript)
3. A **permanent** and growing spec-library
4. A system that keeps the specs **small** to avoid context cluttering
5. A system that keeps **asking me instead of making assumptions**

So I built `speq-skill`. It combines Skills and Agents with a simple CLI called `speq` that adds a semantical search layer to the spec library. The search empowers the coding agent to find the right feature scenarios during planning, but also during the implementation. This avoids reading unnecessary specs into the context window.

## Who should use it?

If you want to describe what you want and have a coding agent build the code for you. If you can answer this question with "yes," then you should give `speq-skill` a try!

Vibe Coding does not scale. `speq-skill` fixes this. It adds a lightweight and straightforward system for spec-driven development that engineers the context for the coding agent.

---

## How Does it Work?

```
/speq:plan     →    /speq:implement    →    /speq:record
     │                   │                       │
     ▼                   ▼                       ▼
Creates plan.md     Implementation          Merges deltas into
with deltas         + verification          permanent specs library
```

1. **Plan** — Describe what you want. The agent searches existing specs, asks clarifying questions, and creates a plan with spec deltas.
2. **Implement** — The agent implements features using TDD. Each scenario gets a failing test first, then minimal code to pass.
3. **Record** — Review newly built feature and merge planned deltas into permanent specs. The plan archives to `_recorded/`.

Specs live in `specs/<domain>/<feature>/spec.md`. Plans stage in `specs/_plans/<plan-name>/`. The separation keeps your spec library clean while work is in progress.

---

## Reference

### Skills

**Workflow Skills** — The core spec-driven development flow:

| Skill | Trigger | Purpose |
|-------|---------|---------|
| `/speq:mission` | Project init, specs setup | Interview user, explore codebase, generate `specs/mission.md` |
| `/speq:plan` | Plan mode, feature planning | Search specs, interview user, create plan with deltas |
| `/speq:implement` | Implement plan | TDD cycle with mandatory evidence, generate verification report |
| `/speq:record` | Record approved plan | Merge deltas to permanent specs, validate, archive plan |

**Utility Skills** — Reusable guidance invoked by workflow skills:

| Skill | Purpose |
|-------|---------|
| `/speq:code-tools` | Semantic code navigation via Serena MCP |
| `/speq:ext-research` | External docs via Context7 and WebSearch |
| `/speq:code-guardrails` | TDD workflow and code quality guardrails |
| `/speq:git-discipline` | Git read-only rules |
| `/speq:speq-cli` | speq CLI usage patterns |

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
# Feature: User Login

The system SHALL provide a secure login mechanism for registered users.

## Background

* The system has a registered user with email "user@example.com"
* The user is not currently authenticated

## Scenarios

### Scenario: Successful login

* *GIVEN* valid credentials are provided
* *WHEN* the user submits the login form
* *THEN* the system SHALL authenticate the user
* *AND* the system SHALL redirect to the dashboard

### Scenario: Invalid password

* *GIVEN* an incorrect password is provided
* *WHEN* the user submits the login form
* *THEN* the system MUST reject the authentication
* *AND* the system MUST display an error message
```

**Keywords:** `MUST`, `MUST NOT`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `MAY`

### Structure of your Spec Library

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

## Installation

```bash
curl -fsSL https://raw.githubusercontent.com/marconae/speq-skill/main/install.sh | bash
```

The installer automatically:
- Downloads the marketplace to `~/.speq-skill/`
- Symlinks `speq` CLI to `~/.local/bin/speq`
- Installs the speq-skill plugin and dependencies (Serena, Context7)

**Update:** Re-run the install script to get the latest version.

**From Source (CLI only):**
```bash
cargo install --git https://github.com/marconae/speq-skill speq
```

---

## Important

`speq-skill` is a plugin for Claude Code and compatible AI coding assistants. This tool provides workflow structure and spec management only—**the AI / coding agent (such as Claude Code) generates all code, specs, or other artifacts**.

## Dependencies

This plugin uses [Serena](https://github.com/oraios/serena) and [Context7](https://github.com/upstash/context7) MCP servers (both MIT licensed).

The `speq` CLI downloads the [all-MiniLM-L6-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2) embeddings model (~23MB) on first run for semantic search (Apache 2.0 licensed).

## License

Free and open-source under MIT. See [LICENSE](LICENSE) for details.
