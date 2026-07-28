[speq-skill](../README.md) / [Docs](./index.md) / Spec Library

---

# Spec Library

The permanent spec format: BDD/Gherkin scenarios that express requirements in RFC 2119 keywords, organized under `specs/`.

---

## Spec structure

Specs live in a two-level hierarchy under `specs/`:

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

Every `spec.md` has five required parts:

1. **`# Feature:`** — the feature name and a one-sentence description
2. **Description** — free-text context below the feature heading
3. **`## Background`** — bullet list of facts that apply to all scenarios
4. **`## Scenarios`** — container for one or more scenario blocks
5. **`### Scenario:`** — individual scenario with GIVEN/WHEN/THEN steps

A complete minimal spec:

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

## BDD and Gherkin

Specs use a Markdown adaptation of the Gherkin [Given-When-Then](https://cucumber.io/docs/gherkin/reference/) pattern:

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

*AND* inherits the type of the step before it. In the example above, the first AND is another GIVEN. The last AND is another THEN.

## RFC 2119 keywords

Specs use [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) keywords to express requirement levels. Keywords MUST be UPPERCASE.

| Keyword | Meaning |
|---------|---------|
| **MUST** / **SHALL** | Mandatory. The implementation must satisfy this. |
| **MUST NOT** / **SHALL NOT** | Prohibited. The implementation must not do this. |
| **SHOULD** / **SHOULD NOT** | Recommended (or discouraged). Follow it unless there is a good reason not to. |
| **MAY** | Optional. The implementation can include or omit this. |

Rules for keyword usage:

- *THEN* steps must contain at least one RFC 2119 keyword. These steps define what the system must do.
- *GIVEN* and *WHEN* steps can omit keywords. These steps describe context and actions, not requirements.
- Keywords must appear in UPPERCASE so that the validator recognizes them.

## Structure and AI coding agents

- GIVEN/WHEN/THEN explicitly establishes state, action, and expected outcome, instead of leaving intent to prose that the agent must interpret.
- SHALL and SHALL NOT mark what is mandatory and what is prohibited. This includes negative requirements that an agent otherwise adds by default, for example retries or fallback logic.
- SHOULD and MAY mark what is recommended or optional, so that the agent does not over-implement.
- Edge cases, for example empty input, null values, or timeouts, need their own scenario. An agent will not infer behavior for cases that the spec omits.

## Fine-grained context via the speq CLI

The `speq` CLI retrieves specs at three levels of granularity:

- **Domain** — `speq domain list` shows all spec domains
- **Feature** — `speq feature get <domain>/<feature>` retrieves a single feature spec
- **Scenario** — `speq search query "..."` returns matching scenarios, not whole files

Agents operate within a context window. Loading the full library wastes tokens.

## Validation

The `speq` CLI enforces all the structural rules that this page describes:

```
speq feature validate
```

The validator checks:

- Required sections are present (`# Feature:`, `## Background`, `## Scenarios`, `### Scenario:`)
- THEN steps contain at least one RFC 2119 keyword
- Keywords are UPPERCASE
- Step formatting follows the `* *KEYWORD* <text>` pattern

See [CLI Reference](./cli-reference.md) for full command documentation.
