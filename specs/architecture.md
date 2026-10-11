# Architecture

## Overview

```
                    ┌──────────────┐
 argv ─────────────▶│ cli (clap)   │
                    └──────┬───────┘
                           ▼
                    ┌──────────────┐
                    │ main.rs      │ dispatches, prints output
                    └──┬────┬────┬─┘
       ┌───────────────┘    │    └────────┐
       ▼                    ▼             ▼
 ┌──────────┐          ┌────────┐    ┌────────┐
 │ feature  │          │ plan   │    │ record │
 │ tree     │          └───┬────┘    └───┬────┘
 └────┬─────┘              │             │
      │                    ▼             ▼
      │        ┌───────────────────────────────┐
      └───────▶│ validate                      │
               │ parser rules                  │
               │ report decision_log           │
               └───────────────┬───────────────┘
                               ▼
                         specs/ on disk
```

- Modular single-binary CLI. One module per capability. The library crate exposes the modules and `main.rs` is a thin dispatcher.

## Components

- cli (src/cli.rs): defines commands and arguments with clap derive | owns: argument definitions | depends on: none
- main (src/main.rs): parses arguments, calls module handlers, prints results, sets the exit code | owns: output formatting of command results | depends on: cli, feature, plan, record, tree, validate
- validate (src/validate/): parses spec Markdown and applies structural rules and RFC2119 checks | owns: parse results and validation reports | depends on: feature
- validate::decision_log (src/validate/decision_log.rs): validates decision fragments in specs/_decision and plan decision logs, and assembles the permanent log | owns: none | depends on: none
- feature (src/feature.rs): discovers domains and features by walking directories under specs | owns: feature path model | depends on: none
- tree (src/tree.rs): renders the domain and feature tree view | owns: none | depends on: feature
- plan (src/plan.rs): lists active plans and validates a plan's delta specs and decision log | owns: plan validation results | depends on: validate, record
- record (src/record.rs): parses delta markers, merges deltas into permanent specs in memory, writes them, archives the plan to specs/_recorded | owns: recorded plan archive | depends on: none

## Data Flow

- arguments -> cli -> main -> module handler -> stdout: a command runs and prints its result, exit code 1 on failure
- specs/<domain>/<feature>/spec.md -> feature -> validate -> main: discovery feeds parsing and rule checks
- specs/_plans/<plan>/ -> plan -> validate: delta files are checked for marker balance, anchors, and structure
- specs/_plans/<plan>/ -> record -> specs/ and specs/_recorded: deltas are merged in memory, written, then the plan is archived

## Interfaces

- CLI: `speq domain|feature|plan|record|decision-log`, output on stdout, errors exit non-zero
- Spec format: Gherkin-like Markdown in `specs/<domain>/<feature>/spec.md`
- Delta markers: `<!-- DELTA:NEW|CHANGED|REMOVED -->` blocks in plan spec files

## Constraints

- Minimal dependencies to keep maintenance low
- Single binary with no runtime services or databases

## External Dependencies

- None
