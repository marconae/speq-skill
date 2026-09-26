# Architecture

<!--
STRUCTURAL TEMPLATE - DO NOT COPY-PASTE
Generate actual content from code exploration and interview answers.
Every placeholder MUST be replaced with real information.
This template describes the permanent file `specs/architecture.md`. It records what the system is. An ADR records why.
-->

## Rules

1. The first line is exactly `# Architecture`.
2. The file has six canonical `##` sections, once each, in this order: Overview, Components, Data Flow, Interfaces, Constraints, External Dependencies. Custom `##` sections MAY follow them.
3. A section body holds only `- ` bullets at one level, fenced blocks for ASCII diagrams and charts, and blank lines.
4. Do not use `###` headings, tables, or prose paragraphs.
5. An empty canonical section holds `- None`.
6. Write each Components bullet as `- <name> (<path>): <responsibility> | owns: <data/state> | depends on: <names>`.
7. Write one line per bullet. Do not wrap.
8. Plans change this file through an architecture delta, never by editing it directly. See `architecture-delta-template.md`.

## Skeleton

````markdown
# Architecture

## Overview

```
<ASCII diagram of the main components and the direction of data flow>
```

- <one-line statement of the architecture pattern>

## Components

- <name> (<path>): <responsibility> | owns: <data/state> | depends on: <names>
- <name> (<path>): <responsibility> | owns: <data/state> | depends on: <names>

## Data Flow

- <source> -> <step> -> <sink>: <what moves and in which form>

## Interfaces

- <name>: <who calls it, the contract, the format>

## Constraints

- <technical or performance constraint the design must respect>

## External Dependencies

- <service or API>: <why it is needed> | failure impact: <what breaks if it is unavailable>
````

## Example: Small CLI Tool

````markdown
# Architecture

## Overview

```
┌─────┐     ┌──────────┐     ┌────────┐
│ cli │────▶│ commands │────▶│ output │
└─────┘     └────┬─────┘     └────────┘
                 │
            ┌────▼────┐
            │  store  │
            └─────────┘
```

- Modular CLI. Each module handles one capability.

## Components

- cli (src/cli.rs): parses arguments and dispatches commands | owns: argument definitions | depends on: commands
- commands (src/commands/): runs one handler per subcommand | owns: none | depends on: store, output
- store (src/store.rs): reads and writes the local data file | owns: data file on disk | depends on: none
- output (src/output.rs): formats results for the terminal | owns: none | depends on: none

## Data Flow

- arguments -> cli -> commands -> store: a parsed command reads or writes records
- store -> commands -> output -> stdout: records are formatted as text or JSON

## Interfaces

- CLI: `tool <command> [args]`, exit code 0 on success, non-zero on error
- Data file: JSON at `~/.local/share/tool/data.json`

## Constraints

- Single binary with no runtime services
- Data file format stays backward compatible

## External Dependencies

- None
````
