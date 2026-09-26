# Architecture Delta Template

<!--
STRUCTURAL TEMPLATE - DO NOT COPY-PASTE
Generate actual content for the sections your plan changes.
Copy the current section from `specs/architecture.md`, then edit it.
-->

Captures architecture changes in a plan before `/speq-record` applies them to `specs/architecture.md`. The file is `specs/_plans/<plan>/architecture.md`. It is optional.

## Rules

1. The first line is `# Architecture Delta: <plan-name>`.
2. The second non-empty line is the BASE comment: `<!-- BASE: <output of git hash-object specs/architecture.md> -->`. Run the command when you write the delta.
3. Wrap each changed section in one DELTA block. Put the `## <Section>` heading inside the block.
4. Nothing may appear outside blocks except the H1, the BASE comment, and blank lines.
5. Body lines follow the line rules of `architecture-template.md`: `- ` bullets, fenced blocks, blank lines. No `###`, no tables, no prose paragraphs.

## Anchors

The anchor of a block is its first non-empty line. It MUST be `## <Section>`. It MUST be the only `##` line in the block. No two blocks share an anchor.

## Markers

```
<!-- DELTA:NEW -->     <!-- /DELTA:NEW -->
<!-- DELTA:CHANGED --> <!-- /DELTA:CHANGED -->
<!-- DELTA:REMOVED --> <!-- /DELTA:REMOVED -->
```

| Kind | Effect | Allowed on |
|------|--------|------------|
| CHANGED | Replaces the heading and body up to the next `##` | Any section, canonical or custom |
| NEW | Appends a section at the end of the file | Non-canonical sections only. The anchor MUST NOT exist yet |
| REMOVED | Deletes the section. The block holds only the heading | Non-canonical sections only. The anchor MUST exist |

The canonical sections are Overview, Components, Data Flow, Interfaces, Constraints, and External Dependencies. They always exist, so NEW and REMOVED do not apply to them. Use CHANGED to edit them.

## Deletion Semantics

A CHANGED block replaces the whole section body. Any line the block omits is deleted from `specs/architecture.md`. Copy the current section, then edit it. Never write only the lines that changed.

## Missing Target

If `specs/architecture.md` does not exist, the delta is invalid. Run `/speq-mission` first. State `Architecture: no change: specs/architecture.md absent` in the decision log.

## Example: CHANGED

```markdown
# Architecture Delta: add-export-command
<!-- BASE: 3b18e512dba79e4c8300dd08aeb37f8e728b8dad -->

<!-- DELTA:CHANGED -->
## Components

- cli (src/cli.rs): parses arguments and dispatches commands | owns: argument definitions | depends on: commands
- commands (src/commands/): runs one handler per subcommand | owns: none | depends on: store, export, output
- export (src/export.rs): writes records to CSV and JSON files | owns: none | depends on: store
- store (src/store.rs): reads and writes the local data file | owns: data file on disk | depends on: none
- output (src/output.rs): formats results for the terminal | owns: none | depends on: none
<!-- /DELTA:CHANGED -->
```

## Example: NEW

Adds a custom section that does not exist yet.

```markdown
# Architecture Delta: add-deployment-notes
<!-- BASE: 3b18e512dba79e4c8300dd08aeb37f8e728b8dad -->

<!-- DELTA:NEW -->
## Deployment

- Release: single static binary per platform
- Install: unpacked into `~/.local/bin`
<!-- /DELTA:NEW -->
```

## Example: REMOVED

The block holds only the heading. The section MUST be a custom section that exists.

```markdown
# Architecture Delta: drop-deployment-notes
<!-- BASE: 3b18e512dba79e4c8300dd08aeb37f8e728b8dad -->

<!-- DELTA:REMOVED -->
## Deployment
<!-- /DELTA:REMOVED -->
```

## Anti-Pattern: Prose Outside a Block

```markdown
# Architecture Delta: add-export-command
<!-- BASE: 3b18e512dba79e4c8300dd08aeb37f8e728b8dad -->

This plan adds an export module.

<!-- DELTA:CHANGED -->
## Components
- export (src/export.rs): writes records to files | owns: none | depends on: store
<!-- /DELTA:CHANGED -->
```

Wrong. The prose line sits outside a block, so the delta is invalid. Move the explanation to `plan.md`. This block is also wrong because it lists only the new bullet. It deletes every other component. Copy the full section, then edit it.

## Anti-Pattern: NEW or REMOVED on a Canonical Section

```markdown
<!-- DELTA:NEW -->
## Interfaces
- CLI: `tool export <file>`
<!-- /DELTA:NEW -->
```

Wrong. Interfaces is a canonical section and always exists. Use `DELTA:CHANGED` with the full section body.
