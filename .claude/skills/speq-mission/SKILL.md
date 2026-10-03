---
name: speq-mission
description: "Create or update specs/mission.md and the first specs/architecture.md through a Socratic interview; detects brownfield vs greenfield. Use when the user asks to bootstrap or initialize a speq project, write or revise the project mission, or when /speq-audit reports mission drift and seeds this skill with its findings."
---

# Mission Creator

You are creating a project mission file (`specs/mission.md`) and the first architecture file (`specs/architecture.md`) through an interactive interview.

**Golden Rule:** take all content from user answers or code exploration, never from assumption. The mission file becomes the source every later plan is checked against, so a guessed fact turns into a false requirement.

## Required Skills

Invoke before starting:
- `/speq-ext-research`: tech stack research
- `/speq-cli`: spec structure
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text

Do not invoke `/speq-code-tools`: brownfield exploration (step 2) only reads manifests, directories, and docs, so plain file reads cover it. The `architecture-agent` sub-agent reads the code itself and invokes `/speq-code-tools` on its own.

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

specs/architecture.md exists?
├─ Yes → Update mode: skip topics 4.9 to 4.11 (see below)
└─ No  → mission.md still has ## Architecture, ## External Dependencies, or technical constraints?
         ├─ Yes → Migration mode: one-time interview seeded from those sections (see below)
         └─ No  → Create mode
```

**Update mode.** `specs/architecture.md` exists. Skip the interview topics for Architecture, technical and performance constraints, and External Dependencies. Tell the user: "Change the architecture through /speq-plan." Interview the remaining topics as usual.

Brownfield and not Update mode: step 2 spawns `architecture-agent`. Greenfield and Update mode: no spawn.

**Migration mode.** `specs/architecture.md` is missing and `specs/mission.md` still holds the old sections. Run topics 4.9 to 4.11 once, seeded with the `architecture-agent` draft, which already reconciles the old sections with the code ("The code shows [X]. mission.md said [Y]. Which is current?"). Without a draft (no source files), seed with the old content ("mission.md lists [X] as the architecture. Is this still accurate?"). Place the approved file at `specs/architecture.md`. Remove the old Architecture and External Dependencies sections and the technical and performance constraint lines from `specs/mission.md`. Add the line `Architecture: see specs/architecture.md.` after Out of Scope.

### 2. Brownfield Exploration

For existing projects, gather context BEFORE interviewing:

1. **Tech stack**: read the manifest(s): `Cargo.toml`, `package.json`, `go.mod`, `pyproject.toml`/`requirements.txt`, `pom.xml`/`build.gradle`. The list is not exhaustive; a project can use other languages and several technologies at once.
2. **Commands**: look for existing scripts: `package.json` scripts, Makefile targets, `Cargo.toml` aliases, `pyproject.toml` scripts.
3. **Structure**: list top-level directories; find the main source directories (`src`, `lib`, `app`).
4. **Docs**: read `README.md`, `docs/`, and any existing `specs/`.
5. **Architecture draft**: applies in brownfield Create and Migration mode when source files exist beyond manifests. Skip it in Update mode and in greenfield. Choose a draft path outside the repo (for example under `mktemp -d`). Delegate to `architecture-agent`:

   ```
   Delegate to architecture-agent: Draft specs/architecture.md from the code

   ## Context
   Repo root: <path>
   Mission path: specs/mission.md
   Template: /speq-plan references/architecture-template.md
   Draft path: <draft path>
   Project Hook: <content of .speq/mission-hook.md, or none>

   ## Your Task
   Write the draft to the draft path and the evidence to <draft path>.evidence.md. Return only the summary in your output format.
   ```

   Keep the returned summary (draft path, counts, `Conflicts`, `Questions`, `Not explored`) for step 4. Do NOT read the draft or the evidence file. The user reads the draft.

### 3. Research Phase

For technologies discovered or mentioned:

- **Context7 MCP** (when installed): query library documentation for correct API usage
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
- Use Context7, when installed, to research mentioned technologies.

#### 4.7 Commands
- Build, test, lint/format, and coverage commands. Greenfield: ask each. Brownfield: confirm discovered commands and ask for any missing ones ("No coverage command found. What should it be?").

#### 4.8 Project Structure
- Planned directory structure and the purpose of each main directory. Brownfield: present the discovered structure and ask for clarification on purpose.

#### 4.9 Architecture
Answers go to `specs/architecture.md`.
- With a draft: tell the user the draft path and ask them to review it there. Ask one `AskUserQuestion` per `Conflicts` item and replace the cold question with each `Questions` item. Tell the user what `Not explored` lists and ask about it. Corrections: the user edits the draft file, or you resume the same agent with the correction. Never rewrite the draft yourself.
- Without a draft: high-level architecture pattern (layered, hexagonal, event-driven, etc.)? Key components and their responsibilities? How does data flow through the system?

#### 4.10 Constraints
- Technical (browser-only, offline-first) and performance (response time, memory limits): answers go to the Constraints section of `specs/architecture.md`.
- Business (GDPR, multi-tenant): answers stay in `specs/mission.md`.

#### 4.11 External Dependencies
Answers go to `specs/architecture.md`. With a draft, topics 4.10 (technical and performance) and 4.11 are covered by the same review and the same `Conflicts` and `Questions` items. Ask only what the summary leaves open.
- What external services/APIs does this depend on? What happens if each dependency is unavailable?

### 5. Generate Mission

After collecting ALL information:

1. Create `specs/` directory if needed
2. Generate `specs/mission.md` using `references/mission-template.md` as structure
3. Generate `specs/architecture.md` using `references/architecture-template.md` from `/speq-plan` as structure, unless it already exists (update mode) or an `architecture-agent` draft exists (the draft is the file, see step 6)
4. Fill the generated files with ACTUAL collected information (no placeholders)
5. Present the generated files to user for review. For a draft, present its path only

### 6. Review & Iterate

Present the generated mission.md and architecture.md and ask: "Do these accurately capture your project? Anything to add, change, or remove?" Iterate until the user approves.

With an `architecture-agent` draft, the user approves the draft file. Then move it into place with `mv <draft path> specs/architecture.md`, so the content never enters your context. The agent already reported `structure ✓`. If the user edited the draft, check the moved file against the structural rules of `/speq-audit` check 14 with grep (first line, six `##` sections in order, no `###`, no table rows), not a full read. Delete the evidence file.

## Interview Guidelines

### Question Batching

Group questions into MECE partitions (max 3-4 per `AskUserQuestion` call). Each group covers one dimension without overlap:

| Phase | Questions to Group |
|-------|-------------------|
| Identity | Name, summary, problem |
| Users | Personas, goals, workflows |
| Capabilities | Core features, out of scope |
| Technical | Stack, commands, structure |
| Constraints | Technical, business, performance (technical and performance go to architecture.md) |

### Adaptive Depth

| Project Complexity | Interview Depth |
|-------------------|-----------------|
| Simple CLI tool | Minimal (architecture.md with Overview and Components only, other sections `- None`) |
| Web application | Standard (all sections) |
| Distributed system | Deep (detailed architecture, failure modes) |

### Never Assume

| Wrong | Right |
|-------|-------|
| "I'll use Jest for testing" | "What testing framework do you want?" |
| "Architecture is MVC" | "What architecture pattern fits best?" |
| "Coverage target is 80%" | "What coverage target do you want?" |
