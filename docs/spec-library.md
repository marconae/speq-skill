[speq-skill](../README.md) / [Docs](./index.md) / Spec Library

---

# Spec Library

Specs capture the intent expressed in features and scenarios. This guide covers the spec format, the BDD/Gherkin pattern, RFC 2119 keywords, and why this structure matters when AI coding agents are the primary consumers of the specs.

---

## Spec Structure

Specs live in a two-level hierarchy under the `specs/` directory:

```
specs/
├── <domain>/
│   └── <feature>/
│       └── spec.md
├── auth/
│   ├── login/
│   │   └── spec.md
│   └── token-refresh/
│       └── spec.md
└── billing/
    └── invoice-generation/
        └── spec.md
```

Every `spec.md` has four required parts:

1. **`# Feature:`** — The feature name and a one-sentence description
2. **Description** — Free-text context below the feature heading
3. **`## Background`** — Bullet list of facts that apply to all scenarios
4. **`## Scenarios`** — Container for one or more scenario blocks
5. **`### Scenario:`** — Individual scenario with GIVEN/WHEN/THEN steps

Here is a complete minimal spec:

```markdown
# Feature: Password Strength

The system SHALL enforce minimum password strength requirements during account creation.

## Background

* Passwords are evaluated at registration time
* Strength rules apply to all user types
* The system uses zxcvbn for strength scoring

## Scenarios

### Scenario: Strong password accepted

* *GIVEN* a user is on the registration form
* *WHEN* the user submits a password with a zxcvbn score of 3 or higher
* *THEN* the system SHALL accept the password
* *AND* the system SHALL proceed to account creation

### Scenario: Weak password rejected

* *GIVEN* a user is on the registration form
* *WHEN* the user submits a password with a zxcvbn score below 3
* *THEN* the system SHALL reject the password
* *AND* the system SHALL display a message explaining the weakness
* *AND* the system SHALL NOT lock the user out
```

---

## BDD and Gherkin

Specs use a Markdown adaptation of Gherkin's [Given-When-Then](https://cucumber.io/docs/gherkin/reference/) pattern:

| Keyword | Purpose |
|-------|---------|
| *GIVEN* | Establish preconditions — the state of the world before the action |
| *WHEN* | Describe the action or event that triggers the behavior |
| *THEN* | Assert the expected outcome — what MUST, SHOULD, or MAY happen |
| *AND* | Extend the preceding step type (another GIVEN, WHEN, or THEN) |

Each step is a Markdown bullet with the keyword in italic:

```markdown
* *GIVEN* the user is authenticated
* *AND* the user has admin privileges
* *WHEN* the user deletes a record
* *THEN* the system SHALL remove the record from the database
* *AND* the system SHALL log the deletion event
```

*AND* always inherits the type of the step before it. In the example above, the first AND is another GIVEN; the last AND is another THEN.

---

## RFC 2119 Keywords

Specs use [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) keywords to express requirement levels. Keywords MUST be UPPERCASE.

| Keyword | Meaning |
|---------|---------|
| **MUST** / **SHALL** | Mandatory. The implementation is required to satisfy this. |
| **MUST NOT** / **SHALL NOT** | Prohibited. The implementation is required to *not* do this. |
| **SHOULD** / **SHOULD NOT** | Recommended (or discouraged). Follow unless there is a compelling reason not to. |
| **MAY** | Optional. The implementation can include or omit this at its discretion. |

Rules for keyword usage:

- *THEN* steps must contain at least one RFC 2119 keyword — they define what the system is required to do.
- *GIVEN* and *WHEN* steps may omit keywords — they describe context and actions, not requirements.
- Keywords must appear in UPPERCASE to be recognized by the validator.

---

## Why This Matters for AI Coding Agents

When an AI coding agent reads your specs, structured specs help to understand the intent:

**Vague specs produce vague code.** A sentence like "handle invalid logins appropriately" forces the agent to guess what "appropriately" means. Does it lock the account? Show an error? Log the attempt? The agent will invent an answer based on its training data, not your intent.

**BDD structure is machine-parseable.** The agent knows exactly what state to set up (GIVEN), what action to perform (WHEN), and what outcome to assert (THEN).

**RFC keywords eliminate ambiguity about requirement levels.** SHALL tells the agent this is mandatory. SHOULD tells it this is recommended but not required. MAY tells it this is optional.

**SHALL NOT / MUST NOT prevent "helpful" additions.** Agents tend to add defensive logic: automatic retries, extra validations, fallback behaviors. If your spec says the system SHALL NOT retry after authentication failure, the agent knows to leave that out.

**Edge cases need explicit scenarios.** If you don't specify what happens with empty input, a null value, or a network timeout, the agent might guess.

---

## Fine-Grained Context via the speq CLI

The `speq` CLI is designed for granular spec retrieval, so agents load only what is necessary to understand the task.

- **Domain-level** — `speq domain list` shows all spec domains
- **Feature-level** — `speq feature get <domain>/<feature>` retrieves a single feature spec
- **Scenario-level** — `speq search query "..."` returns matching scenarios, not whole files

This granularity matters because AI coding agents operate within a context window. Loading your entire spec library wastes tokens and dilutes focus. With `speq search`, the agent is able to find the relevant features and scenarios.

---

## Validation

The `speq` CLI enforces all structural rules described on this page:

```
speq feature validate
```

The validator checks:

- Required sections are present (`# Feature:`, `## Background`, `## Scenarios`, `### Scenario:`)
- THEN steps contain at least one RFC 2119 keyword
- Keywords are UPPERCASE
- Step formatting follows the `* *KEYWORD* <text>` pattern

See [CLI Reference](./cli-reference.md) for full command documentation.
