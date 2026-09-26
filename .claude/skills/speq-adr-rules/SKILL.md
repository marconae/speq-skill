---
name: speq-adr-rules
description: The rules for when a decision becomes an ADR. The default is no, only durable decisions qualify, search first, one decision per ADR, and a human accepts an ADR when they record the plan. Triggered by planner-agent, plan-reviewer, recorder-agent, and adr-audit-agent.
---

# ADR Rules

An ADR is a rare record of a durable decision. Most plans produce none. This skill is the single owner of the rules. Other skills and agents cite it and do not restate it.

## 1. Default No

- The expected number of `Promotes to ADR: yes` entries per plan is zero.
- Set `yes` only when a criterion in rule 2 applies and you can name it.
- Two or more `yes` entries in one plan get re-judged. Re-read each one against rule 2 and rule 3 before you keep it.
- A plan with many Design Decisions entries or many resolved review findings does not owe an ADR for each. Most decisions are local and stay `no`.

## 2. Criteria for Yes

A decision qualifies only when at least one of these holds:

- It affects multiple components or teams.
- It sets a long-lived constraint that binds future work.
- It is a major technology, API, persistence, security, or deployment choice.
- It rejects a plausible alternative, and the reason will matter to a later reader.

The entry's Rationale names the criterion that applies and states the result of the search in rule 5.

## 3. Never an ADR

- Naming, file placement, and folder layout.
- A one-off bug fix.
- A scope trim or a task ordering choice.
- Implementation detail: function signatures, file paths, flags, data layout inside one module.
- A workaround specific to one plan's own mechanics, such as a spec-merge gap or a review-loop correction.
- A corollary of another decision. Record it as a bullet in the parent entry's Consequences line.
- A statement that already lives in a feature spec scenario.

## 4. Where It Goes Instead

| Content | Destination |
|---------|-------------|
| Conventions that bind contributors or agents | `CLAUDE.md`, `AGENTS.md`, a `.speq/*-hook.md` file, or `specs/mission.md` Constraints |
| WHAT the system is: components and boundaries | `specs/mission.md` Architecture |
| HOW this plan builds it | `plan.md` `## Design` |
| Execution steps | `plan.md` `## Implementation Tasks` |

To change an existing ADR, write a new ADR with `Supersedes: <slug>`. Do not edit the old fragment.

## 5. Search First

Before you set `yes`, run `speq decision-log show` and read the result. If an ADR already covers the decision, do not promote a duplicate. Either set `no`, or supersede that ADR when the decision has changed. State the search result in Rationale.

## 6. One Decision, No Detail

- One ADR records one decision.
- Decision states the choice in plain words. It holds no function signatures, file paths, or flags.
- An ADR names a library but never pins its version. A version number lives in the manifest or lockfile. The Decision states the policy instead, for example "use the registry release, not a git tag".
- If the Decision needs a list of paths, signatures, or version numbers to make sense, the content is implementation detail. Move it to `plan.md` `## Design`.

## 7. Shape

An ADR has four short sections: Context, Decision, Options Considered, Consequences.

- Options Considered appears only when a real alternative was rejected.
- Consequences appears only when the entry carries a Consequences line.
- Each section is a few sentences. A section that needs a page is more than one decision or holds implementation detail.

## 8. Status Lifecycle

- An ADR is proposed at plan time. A `Promotes to ADR: yes` entry that passes rules 1 to 5 is the proposal. The plan skill prints it as an ADR candidate.
- Recording the plan is the acceptance. `recorder-agent` writes `Status: Accepted` when `/speq-record` runs.
- To reject a proposal, change the entry to `Promotes to ADR: no` and run the plan skill again before you record. In a headless run, comment on the draft PR and run `/speq-plan-pr` again with the PR number.
- `Deprecated` and `Superseded by <slug>` follow the existing rules.

## 9. Vocabulary

| Term | Meaning |
|------|---------|
| ADR | A permanent record in `specs/_decision/` of one durable decision |
| Design section | The `## Design` part of `plan.md`. It describes how one plan builds its change. It is not an ADR |
| Decision-log entry | A note in a plan's `decision-log.md`. Only an entry marked `Promotes to ADR: yes` can become an ADR |
| ADR candidate | A decision-log entry marked `Promotes to ADR: yes`. It is the proposal, shown when the plan is ready |
| Accepted | An ADR written by `/speq-record`. Recording the plan is the acceptance |
