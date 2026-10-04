[speq-skill](../README.md) / [Docs](./index.md) / FAQ

---

# FAQ

## How do I add a feature?

- Run `/speq:plan add <feature>`, for example `/speq:plan add password reset by email`, and answer the interview
- Read the plan summary. Answer again to change anything you disagree with
- Run `/speq:implement <plan-name>`
- Run `/speq:record <plan-name>`

## How do I change or remove existing behavior?

- Run `/speq:plan change <behavior>` or `/speq:plan remove <behavior>`, for example `/speq:plan change token expiry to 15 minutes`
- The agent searches the specs for the affected scenarios and proposes the deltas
- Then run `/speq:implement` and `/speq:record`, as for a new feature

## How do I fix a bug?

- Run `/speq:plan bug fix for <symptom>`, for example `/speq:plan bug fix for login failing when the email has uppercase letters`
- The agent checks whether a spec already promises the right behavior
- If a spec does, the plan restores the code
- If no spec covers the case, or the spec is wrong, the plan adds or corrects the scenario
- Then run `/speq:implement` and `/speq:record`

## How do I start a greenfield project?

- Run `/speq:mission` and describe the project. With no code, the interview builds `specs/mission.md` and `specs/architecture.md` from your answers alone
- Run `/speq:plan add <first feature>`
- Start small: the mission sets the boundaries and each plan adds one slice

## How do I start with an existing codebase (brownfield)?

- Run `/speq:mission` once. The agent reads the code and drafts `specs/architecture.md`
- Review the draft file and answer its questions about conflicts
- Do not write specs for the whole codebase. Run `/speq:plan` for the change you need next
- If the change touches behavior with no spec yet, the plan writes that feature's spec, including what the code does today
- The library grows where you work

## What if I want to revise a plan?

- Run `/speq:plan <plan-name>` again to revise a plan
- Run `/speq:implement` to address the changes and update the verification report
- Run `/speq:record` if the plan is complete

## Is the architecture documented?

- Yes. Read `specs/architecture.md`
- `/speq:mission` creates it, and `/speq:record` keeps it current by merging architecture deltas from plans
- Change the architecture through `/speq:plan`, not by editing the file

## Where do I find decisions?

- Run `speq decision-log show` to print every accepted ADR. The files live in `specs/_decision/`
- A plan in progress keeps its design choices in `specs/_plans/<plan-name>/decision-log.md`
- At the end of `/speq:plan` you see any proposed ADR and can drop it
- See [Decision Log](./decision-log.md)

## How do I find the right spec?

- Run `speq search query "<terms>"`, then `speq feature get <domain>/<feature>`
- After you change a spec, run `speq feature validate`

## How do I check that the spec library is healthy?

- Run `/speq:audit`
- It reports problems with structure, ADRs, the mission, and unrecorded plans
- It asks before each fix

## How do I upgrade to a newer version?

- Run `/speq:audit`
