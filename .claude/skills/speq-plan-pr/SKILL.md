---
name: speq-plan-pr
description: "Headless, non-interactive version of /speq-plan for CI or agent-driven runs. Plans a feature without a live interview, commits the result to a feat/plan-name branch, and opens a draft PR. If a decision genuinely needs a human, it persists the partial plan and open questions and asks in a PR comment instead of blocking. Arg: plan name, feature intent text, PR number, or branch name."
model: sonnet
---

# Spec Planner, headless (Orchestrator)

You are a thin orchestrator with no live user to interview. Your goal:
- Turn a feature intent, or an existing plan branch/PR, into a validated plan authored entirely by `planner-agent` in headless mode.
- Land that plan on a `feat/<plan-name>` branch and a draft PR, running every git/`gh` action yourself per `/speq-git-operations`.
- Hand any irreducible decision to a human as a PR comment and stop there.

Rules:
- Delegate all planning judgment to `planner-agent`. Run every git/GitHub action yourself, per `/speq-git-operations`. Your own work: resolve the input, write the plan's status files, brief `planner-agent`/`plan-reviewer`, interpret their returns, and execute the git/gh operations.
- Run the steps in order: resolve target → fetch async answers (resume only) → discovery → delegate planning → branch on the result → report.
- Keep one `feat/<plan-name>` branch and one PR per plan. `create-pr` (per `/speq-git-operations`) reuses an existing PR.

## Required Skills (for the orchestrator)

Invoke before starting:
- `/speq-cli`: spec discovery and search
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text
- `/speq-git-operations`: the git/gh operation-to-command mapping, safety rules, and return formats — you run every operation directly

`planner-agent` and `plan-reviewer` invoke their own required skills.

## Workflow

### 0. Load Project Hook (orchestrator)

Check for `.speq/plan-pr-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/plan-pr-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue without mention.

### 1. Resolve Target

Resolve what to work on and land on the right branch:

```
Run — operation: checkout (per /speq-git-operations)
  target: <the raw argument the caller passed>
```

If `checkout` reports not-found, the argument is free-text feature intent for a new plan. Derive `<plan-name>` per the verb table and create its branch:

```
Run — operation: create-branch (per /speq-git-operations)
  branch: feat/<plan-name>
```

Then read from disk yourself: whether `specs/_plans/<plan-name>/` exists, and whether `specs/_plans/<plan-name>/open-questions.md` exists and is non-empty. Take the PR draft/ready state from the `checkout` return.

| Verb | When |
|------|------|
| `add` | New feature |
| `change` | Modify existing |
| `remove` | Deprecate/delete |
| `refactor` | Restructure, same behavior |
| `fix` | Bug or spec mismatch |

Pattern: `<verb>-<feature-scope>[-<qualifier>]`

#### PR-title derivation (shared with `speq-implement-pr`)

Derive the PR title from `<plan-name>` as a conventional-commit title `<type>(<scope>): <slug>`:

- **type**: map the verb: `add`/`change` → `feat`, `remove` → `chore`, `refactor` → `refactor`, `fix` → `fix`; fallback `chore` for an unparseable name.
- **scope**: the `<feature-scope>` segment (the token after the verb).
- **slug**: the humanized `<plan-name>` (hyphens → spaces).

Example: `add-search-candle` ⇒ `feat(search): add search candle`. With no scope segment, emit `<type>: <slug>`.

### 2. Fetch Answers — resume only

If step 1 found an unresolved `open-questions.md`, pull the human's replies:

```
Run — operation: read-comments (per /speq-git-operations)
  since: <timestamp of your last "flag open questions" commit>
```

Forward the returned Q&A to `planner-agent` in step 4. It stands in for the live interview `speq-plan` would run.

### 3. Discovery (orchestrator)

Gather enough context to brief `planner-agent`, the same lightweight calls `speq-plan` makes:

```bash
speq domain list
speq feature list
speq search query "<relevant terms>"
```

### 4. Delegate to planner-agent

Same shape `speq-plan` uses, with headless framing added:

```
Delegate to planner-agent — Plan <plan-name> (headless)

## Plan Name
<plan-name>

## Interview Mode
headless

## User Intent
<feature intent text, or "see resume Q&A below">

## Clarifying Interview Results
<the free-text feature intent (new plan), or the Q&A text step 2 fetched (resume) — this stands in for a live interview>

## Existing Context
<the exact `speq domain list` / `speq feature list` / `speq search query "..."` calls you ran, each followed by its output — name the query, not just the result>

## External Research
none — agent to research as needed

## Project Hook
<if active: note ".speq/plan-pr-hook.md — read it and apply it" — otherwise omit this section>

## Your Task
Produce spec deltas, plan.md, and the architecture delta (when the plan changes the architecture) per your normal workflow. You are in headless mode: follow your "Headless / Non-Interactive Mode" section — assume and document conventional decisions, escalate only irreducible ones via the OPEN QUESTIONS: sentinel. Tag deep-reasoning tasks with [expert].

Return the list of files created and the validation result, or an OPEN QUESTIONS: block if you had to stop.
```

### 5. Adversarial Plan Review

Skip this step if step 4 returned `OPEN QUESTIONS:`; step 6 handles that. Otherwise spawn `plan-reviewer` to challenge the plan. Maximum 2 rounds total, same shape `speq-plan` uses:

```
Delegate to plan-reviewer — Review <plan-name> (round 1)

## Plan Name
<plan-name>

## User Intent
<the feature intent text / resume Q&A used in step 4>

## Clarifying Interview Results
<same text passed to planner-agent in step 4 — headless mode has no live interview>

## Plan Artifacts
plan.md, decision-log.md, every specs/_plans/<plan-name>/**/spec.md delta, and specs/_plans/<plan-name>/architecture.md when present

## Project Hook
<if active: note ".speq/plan-pr-hook.md — read it and apply it" — otherwise omit this section>
```

It writes its findings to `specs/_plans/<plan-name>/review/round-1.md` and returns only `PLAN REVIEW round 1: BLOCKERS: <n>, ADVISORY: <n>, INTENT: <n>, HUMAN: <n> — <path>`. `INTENT` counts the BLOCKERs on the Intent Fidelity axis alone; `HUMAN` counts every BLOCKER tagged `Escalation: HUMAN` (Intent Fidelity included) per `/speq-plan-review`'s Escalation Class section — the rest are `MECHANICAL`.

**If `INTENT > 0`:** the plan solves a different problem than the one asked. Read the Intent-Fidelity BLOCKER text from the round file, fold it into step 6's `OPEN QUESTIONS:` branch, and stop.

**Plan Size classification** (compute before respawning `plan-reviewer` for round 2): the plan is `small` when all three hold: the plan-name's verb (per the verb table) is `fix`; `decision-log.md`'s `## Design Decisions` section is empty; the plan directory has no `architecture.md`. Otherwise `full`.

**If `INTENT == 0` and BLOCKER findings exist:** respawn `planner-agent` with the path to `review/round-1.md`. Instruct it to read the BLOCKER findings, execute each `Fix:` line, log each resolved blocker as a `[plan-review]`-prefixed `## Review Findings` entry in `decision-log.md`, re-validate, and return its per-finding `Resolved:`/`Could not resolve:` report per `/speq-planning`'s Revision Mode.

**Round 2 runs only if round 1's `HUMAN` count was greater than 0.** A round 1 with `HUMAN: 0` means every BLOCKER was `MECHANICAL` — the reviewer already judged none of them needed adversarial re-checking, only a fix. Skip straight to the ship decision below; do not respawn `plan-reviewer`.

- **`HUMAN > 0`:** respawn `plan-reviewer` for round 2 with the path to `review/round-1.md` plus the computed `Plan Size: small | full` field, to confirm resolution (or, on `small`, confirm and stop there). Do not run a third adversarial review round.
  - **BLOCKERs remaining after round 2, split by `Escalation`:** a `MECHANICAL` remainder gets one more direct pass — respawn `planner-agent` with the path to `review/round-2.md` and the still-open `MECHANICAL` findings; same Revision Mode format, no further `plan-reviewer` round (correctness here is checkable, not adjudicated). A `HUMAN` remainder folds into step 6's `OPEN QUESTIONS:` branch as the questions list.

**Ship decision, whichever path produced the final fix pass** (round 1 alone when `HUMAN: 0`, or the round-2 `MECHANICAL` follow-up above): clean only if `speq plan validate` passed AND every finding came back `Resolved:`. Any `Could not resolve:` line, or a failed re-validate, is not a clean return — fold that finding into `OPEN QUESTIONS:` too, with its `Could not resolve:` reason as the question text. A `MECHANICAL` tag means the reviewer judged it not to need human judgment; it does not guarantee a fix exists on the first retry, and this is not the place to find out by shipping it silently.

**ADVISORY findings:** carry into step 7's terminal report only — never a PR comment or body content, per step 6. Read from the last round file that actually ran — round 1's, if round 2 was skipped entirely (`HUMAN: 0`) or ran confirm-only (`Plan Size: small`); round 2's otherwise. Never block or persist them.

### 6. Branch on the Result

**Clean return** (no `OPEN QUESTIONS:` sentinel, no unresolved BLOCKERs from step 5): ship the plan as a draft.

1. Confirm `speq plan validate <plan-name>` passes.
2. Commit the plan and open the draft PR with one composite call:
   ```
   Run — operation: ship-draft (per /speq-git-operations)
     paths: the plan directory, excluding specs/_plans/<plan-name>/notes/planning.md
     message: spec(plan): <plan-name>
     title: <the derived <type>(<scope>): <slug>>
     body: per references/pr-body-template.md, ending "Draft pending
           implementation. Run /speq:implement-pr <plan-name> to implement
           and mark ready."
   ```
3. If this resumes a previously blocked plan, clear the block yourself: delete `specs/_plans/<plan-name>/open-questions.md` and the `> **Status:** blocked …` banner line from `plan.md`, then:
   ```
   Run — operation: commit (per /speq-git-operations)
     paths: the plan directory, excluding specs/_plans/<plan-name>/notes/planning.md
     message: spec(plan): resolve open questions for <plan-name>

   Run — operation: push (per /speq-git-operations)
   ```
   The PR stays a draft. `speq-implement-pr` is the only skill that marks it ready.

No comment on a clean return. `ADVISORY` findings and Design Decisions entries never need human attention — that is what makes them `ADVISORY` and not `BLOCKER` — so they stay silent in `review/round-<N>.md` and `decision-log.md`, reachable through the PR body's `<details>` pointer for anyone who wants them. A PR comment is for something that needs the approver's eyes; nothing on this path does.

**`OPEN QUESTIONS:` returned** (from step 4, or from step 5's round-1 Intent gate, unresolved `HUMAN` round-2 BLOCKERs, or a `MECHANICAL` finding that a fix pass — round 1's own when `HUMAN: 0`, or round 2's follow-up — came back `Could not resolve:` on): persist the partial plan and ask the human async. Author the status files yourself, then delegate only git operations.

Keep each question short: state the decision in 1-2 sentences and point to the round file for full reasoning, never paste the finding's full `Issue`/`Fix` block. The human needs enough to decide, not the reviewer's complete case file.

1. Write `specs/_plans/<plan-name>/open-questions.md`:
   ```markdown
   # Open Questions: <plan-name>

   speq-plan-pr could not complete this plan without human input. What's done so far is committed on this branch. Reply inline on the PR, or resume with `/speq:plan <plan-name>` locally, or re-run `/speq:plan-pr <plan-name>` after commenting.

   - [ ] <question 1 in 1-2 sentences, or a HUMAN-tagged BLOCKER folded in from step 5 — round-1 Intent-Fidelity, or unresolved after round 2 — link: review/round-<N>.md#<anchor>>
   - [ ] <question 2>
   ```
2. Insert `> **Status:** blocked: see open-questions.md` as the first line under `plan.md`'s H1. Skip if already present.
3. Compose the questions checklist as the PR comment body. Nothing else goes in it — `ADVISORY` findings and Design Decisions entries don't need human attention, so they stay out of the comment the same as on the clean path; they're already in `review/round-<N>.md`/`decision-log.md` for anyone who opens them.

Then, with one composite call:
```
Run — operation: flag-blocked (per /speq-git-operations)
  paths: the plan directory, excluding specs/_plans/<plan-name>/notes/planning.md
  message: spec(plan): flag open questions for <plan-name>
  title: <the derived <type>(<scope>): <slug>>
  body: per references/pr-body-template.md's blocked-path rule — a blocked
        plan may have partial Impact info; include it as-is, never
        fabricate the rest
  comment_body: the questions checklist only — nothing else
```

### 7. Report (orchestrator)

Tell the caller whether the plan is ready or blocked, with the PR link either way. Print plan.md's `## Impact` section to the terminal. Print `ADR candidates: none`, or the title of each `Promotes to ADR: yes` entry in `decision-log.md`. Print the Architecture line from the PR body: `Architecture: none`, `Architecture: none (specs/architecture.md absent)`, or the changed sections. Mention any ADVISORY findings, read from `specs/_plans/<plan-name>/review/round-<N>.md`, not from memory, and any Design Decisions entries surfaced — terminal only, per step 6 neither ever reaches the PR. If step 5's round 1 was all-`MECHANICAL` (round 2 skipped) or ran a round-2 `MECHANICAL` follow-up, and it fully resolved, name it in one line ("N mechanical findings fixed, no human input needed") — do not restate what each one was; that detail lives in the round file.

## Spec Hierarchy (reference)

```
specs/
├── <domain>/<feature>/spec.md            # Permanent
├── architecture.md                       # Permanent, changed only by /speq-record
├── _plans/<plan-name>/                   # Active
│   ├── architecture.md                   # Architecture delta, present only when the plan changes the architecture
│   └── open-questions.md                 # Present only while blocked
└── _recorded/<plan-name>/                # Archived
```

## Repos Without architecture.md

If `specs/architecture.md` does not exist, the plan carries no architecture delta. The decision log states `Architecture: no change: specs/architecture.md absent`, the PR body line reads `Architecture: none (specs/architecture.md absent)`, and `/speq-record` rejects any delta. `/speq-audit` reports the gap.

## Work Split (reference)

| Step | Performed by | Why |
|------|--------------|-----|
| Target resolution, discovery, status files, coordination | This skill (pins Sonnet) | Tool-call heavy, reasoning light |
| Spec delta authoring, plan context and decision log, task decomposition, assume-vs-escalate calls | `planner-agent` sub-agent | Reasoning-heavy; defects here compound through implementation |
| Adversarial review, revision loop | `plan-reviewer` sub-agent | Catches intent drift, infeasibility, and ambiguity before implementation |
| Branch, commit, push, PR create/comment | This skill, directly, per `/speq-git-operations` | No separate agent hop — the orchestrator already composed the content and has full git/gh tool access |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Asking the user a live question | Headless — irreducible decisions go to the PR comment |
| Spawning a sub-agent for git/gh work | No agent hop needed — you already have direct tool access and composed the content; a spawn only adds latency |
| Marking the PR ready | `speq-implement-pr` owns `ready-pr`; plans stay draft |
| A third adversarial review round | Bounded to 2 — a `MECHANICAL` remainder gets one direct fix pass instead, a `HUMAN` remainder becomes an open question |
| Pasting a finding's full Issue/Fix text into a PR comment | State the decision in 1-2 sentences, link to `review/round-<N>.md` for the rest |
| Escalating a `MECHANICAL` finding to the human because round 2 didn't close it | Round count is not the escalation test — `Escalation: HUMAN` is; fix mechanical findings directly |
| Running round 2 when round 1's `HUMAN` count is 0 | A round with nothing judgment-worthy left doesn't need a second adversarial pass — fix and ship |
| Posting `ADVISORY` findings or Design Decisions as a PR comment | They never need human attention by definition — silent in `review/round-<N>.md`/`decision-log.md`, reachable via the body's `<details>` pointer |
