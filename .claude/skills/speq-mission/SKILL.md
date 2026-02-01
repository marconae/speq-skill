---
name: speq-mission
description: |
  Interactive workflow for creating project mission files.
  Use when initializing specs or documenting existing projects.
  Invoke with /speq-mission to start mission creation.
  Detects brownfield vs greenfield and adapts interview.
  Triggers: mission creation, project initialization, specs setup, document project.
---

# Mission Creator

You are creating a project mission file (`specs/mission.md`) through an interactive interview.

**Golden Rule:** NEVER assume. ALL content MUST come from user answers or code exploration.

## Required Skills

Invoke before starting:
- `/speq-code-tools` — Codebase exploration
- `/speq-ext-research` — Tech stack research
- `/speq-cli` — Spec structure

## Workflow

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

#### 2.1 Detect Tech Stack

| File | Indicates |
|------|-----------|
| `Cargo.toml` | Rust project → read dependencies |
| `package.json` | Node.js → read dependencies, scripts |
| `go.mod` | Go project → read dependencies |
| `pyproject.toml` / `requirements.txt` | Python → read dependencies |
| `pom.xml` / `build.gradle` | Java/Kotlin |

**Important:** This is an exemplary list. Project may use other languages or frameworks. Project may use several technologies.

#### 2.2 Detect Commands

Look for existing scripts:

```bash
# package.json scripts
# Makefile targets
# Cargo.toml aliases
# pyproject.toml scripts
```

#### 2.3 Explore Structure

```bash
# List top-level directories
ls -la

# Find main source directories
find . -type d -name "src" -o -name "lib" -o -name "app" | head -20
```

#### 2.4 Read Existing Docs

Check for existing documentation:
- `README.md`
- `docs/`
- Existing `specs/` if any

### 3. Research Phase

For technologies discovered or mentioned:

- **Context7 MCP** — Query library documentation for correct API usage
- **WebSearch** — Research best practices, alternatives, common patterns

Use research to inform interview questions and validate user choices.

### 4. Clarifying Interview

Use `AskUserQuestion` for EVERY section. Never fill in content without asking.

#### 4.1 Identity & Purpose

**For greenfield:**
```
Questions:
- What is the project name?
- In one sentence, what does this system do and why does it exist?
- What problem does this solve?
- Who experiences this problem?
- Why do existing solutions fall short?
```

**For brownfield:**
```
Present discovered information, then ask:
- "I found [X]. Is this accurate? What would you add/change?"
- "The README says [Y]. Is this still the current purpose?"
```

#### 4.2 Target Users

```
Questions:
- Who are the primary users of this system?
- What are they trying to achieve?
- What is their typical workflow?
```

For brownfield: "Based on the code, it seems targeted at [X]. Is this correct?"

#### 4.3 Core Capabilities

```
Questions:
- What are the 3-5 core capabilities this system provides?
- (Describe what it does, not how)
```

For brownfield: "I found these main modules: [X, Y, Z]. What capabilities do they represent?"

#### 4.4 Out of Scope

```
Questions:
- What does this project explicitly NOT do?
- What features might users expect but won't be supported?
```

#### 4.5 Domain Glossary

```
Questions:
- Are there domain-specific terms users should understand?
- Any terms used differently than their common meaning?
```

For brownfield: "I noticed these terms in the code: [X, Y]. What do they mean in this context?"

#### 4.6 Tech Stack

**For greenfield:**
```
Questions:
- What language/runtime?
- What framework (if any)?
- What database (if any)?
- What testing framework?
```

Use Context7 to research mentioned technologies.

**For brownfield:**
```
Present discovered stack:
- "I found: Rust with tokio, clap for CLI, no database. Correct?"
- "Testing appears to use cargo test. Any additional test frameworks?"
```

#### 4.7 Commands

**For greenfield:**
```
Questions:
- What is the build command?
- What is the test command?
- What is the lint/format command?
- What is the coverage command?
```

**For brownfield:**
```
Present discovered commands:
- "Found in package.json: npm run build, npm test. Are these correct?"
- "No coverage command found. What should it be?"
```

#### 4.8 Project Structure

**For greenfield:**
```
Questions:
- What is the planned directory structure?
- What is the purpose of each main directory?
```

**For brownfield:**
```
Present discovered structure and ask for clarification on purpose.
```

#### 4.9 Architecture

```
Questions:
- What is the high-level architecture pattern? (layered, hexagonal, event-driven, etc.)
- What are the key components and their responsibilities?
- How does data flow through the system?
```

#### 4.10 Constraints

```
Questions:
- Technical constraints? (browser-only, offline-first, etc.)
- Business constraints? (GDPR, multi-tenant, etc.)
- Performance constraints? (response time, memory limits, etc.)
```

#### 4.11 External Dependencies

```
Questions:
- What external services/APIs does this depend on?
- What happens if each dependency is unavailable?
```

### 5. Generate Mission

After collecting ALL information:

1. Create `specs/` directory if needed
2. Generate `specs/mission.md` using `references/mission-template.md` as structure
3. Fill with ACTUAL collected information (no placeholders)
4. Present to user for review

### 6. Review & Iterate

```
Present the generated mission.md and ask:
- "Does this accurately capture your project?"
- "Anything to add, change, or remove?"
```

Iterate until user approves.

## Interview Guidelines

### Question Batching

Group related questions (max 3-4 per `AskUserQuestion` call) to avoid overwhelming:

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
