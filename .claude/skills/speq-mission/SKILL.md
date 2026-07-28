---
name: speq-mission
description: "Create or update specs/mission.md through a Socratic interview; detects brownfield vs greenfield. Use when the user asks to bootstrap or initialize a speq project, write or revise the project mission, or when /speq-audit reports mission drift and seeds this skill with its findings."
---

# Mission Creator

You are creating a project mission file (`specs/mission.md`) through an interactive interview.

**Golden Rule:** NEVER assume. ALL content MUST come from user answers or code exploration.

## Required Skills

Invoke before starting:
- `/speq-ext-research`: tech stack research
- `/speq-cli`: spec structure
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text

Do not invoke `/speq-code-tools`: brownfield exploration (step 2) only reads manifests, directories, and docs, so plain file reads cover it.

## Workflow

### 0. Load Project Hook

Check for `.speq/mission-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/mission-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue without mention.

### 1. Project Detection

Determine project type:

```
Cargo.toml, package.json, go.mod, etc. exists?
├─ Yes → Brownfield (existing code)
└─ No  → Greenfield (new project)

specs/ or similar directory exists?
├─ Yes → Has existing specs (read them)
└─ No  → No specs yet
```

### 2. Brownfield Exploration

For existing projects, gather context BEFORE interviewing:

1. **Tech stack**: read the manifest(s): `Cargo.toml`, `package.json`, `go.mod`, `pyproject.toml`/`requirements.txt`, `pom.xml`/`build.gradle`. The list is not exhaustive; a project can use other languages and several technologies at once.
2. **Commands**: look for existing scripts: `package.json` scripts, Makefile targets, `Cargo.toml` aliases, `pyproject.toml` scripts.
3. **Structure**: list top-level directories; find the main source directories (`src`, `lib`, `app`).
4. **Docs**: read `README.md`, `docs/`, and any existing `specs/`.

### 3. Research Phase

For technologies discovered or mentioned:

- **Context7 MCP**: query library documentation for correct API usage
- **WebSearch**: research best practices, alternatives, common patterns

Use research to inform interview questions and validate user choices.

### 4. Clarifying Interview

Conduct a Socratic interview via `AskUserQuestion` for EVERY section below. Never fill in content without asking. Each question reveals assumptions, surfaces contradictions, or narrows scope. Brownfield: present what step 2 discovered and ask the user to confirm or correct it ("I found [X]. Is this accurate? What would you add or change?") instead of asking cold.

#### 4.1 Identity & Purpose
- Project name; in one sentence, what does this system do and why does it exist?
- What problem does this solve, and who experiences it?
- Why do existing solutions fall short?
- Brownfield: "The README says [Y]. Is this still the current purpose?"

#### 4.2 Target Users
- Who are the primary users? What are they trying to achieve? What is their typical workflow?
- Brownfield: "Based on the code, it seems targeted at [X]. Is this correct?"

#### 4.3 Core Capabilities
Apply **User Story Mapping** (Patton): identify activities, then decompose into capabilities.
- What are the 3-5 core capabilities this system provides? (What it does, not how.)
- Brownfield: "I found these main modules: [X, Y, Z]. What capabilities do they represent?"

#### 4.4 Out of Scope
- What does this project explicitly NOT do?
- What features might users expect but won't be supported?

#### 4.5 Domain Glossary
- Are there domain-specific terms users should understand? Any terms used differently than their common meaning?
- Brownfield: "I noticed these terms in the code: [X, Y]. What do they mean in this context?"

#### 4.6 Tech Stack
- Language/runtime, framework, database, testing framework. Greenfield: ask each. Brownfield: confirm the discovered stack ("I found: Rust with tokio, clap for CLI, no database. Correct?").
- Use Context7 to research mentioned technologies.

#### 4.7 Commands
- Build, test, lint/format, and coverage commands. Greenfield: ask each. Brownfield: confirm discovered commands and ask for any missing ones ("No coverage command found. What should it be?").

#### 4.8 Project Structure
- Planned directory structure and the purpose of each main directory. Brownfield: present the discovered structure and ask for clarification on purpose.

#### 4.9 Architecture
- High-level architecture pattern (layered, hexagonal, event-driven, etc.)? Key components and their responsibilities? How does data flow through the system?

#### 4.10 Constraints
- Technical (browser-only, offline-first)? Business (GDPR, multi-tenant)? Performance (response time, memory limits)?

#### 4.11 External Dependencies
- What external services/APIs does this depend on? What happens if each dependency is unavailable?

### 5. Generate Mission

After collecting ALL information:

1. Create `specs/` directory if needed
2. Generate `specs/mission.md` using `references/mission-template.md` as structure
3. Fill with ACTUAL collected information (no placeholders)
4. Present to user for review

### 6. Review & Iterate

Present the generated mission.md and ask: "Does this accurately capture your project? Anything to add, change, or remove?" Iterate until the user approves.

## Interview Guidelines

### Question Batching

Group questions into MECE partitions (max 3-4 per `AskUserQuestion` call). Each group covers one dimension without overlap:

| Phase | Questions to Group |
|-------|-------------------|
| Identity | Name, summary, problem |
| Users | Personas, goals, workflows |
| Capabilities | Core features, out of scope |
| Technical | Stack, commands, structure |
| Constraints | Technical, business, performance |

### Adaptive Depth

| Project Complexity | Interview Depth |
|-------------------|-----------------|
| Simple CLI tool | Minimal (skip architecture, external deps) |
| Web application | Standard (all sections) |
| Distributed system | Deep (detailed architecture, failure modes) |

### Never Assume

| Wrong | Right |
|-------|-------|
| "I'll use Jest for testing" | "What testing framework do you want?" |
| "Architecture is MVC" | "What architecture pattern fits best?" |
| "Coverage target is 80%" | "What coverage target do you want?" |
