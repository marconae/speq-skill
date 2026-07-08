# Decisions: avoid-decision-log-conflicts

## ADR: Per-plan fragment files replace the shared decision log

**ID:** per-plan-decision-fragments
**Plan:** avoid-decision-log-conflicts
**Status:** Accepted

### Context

Parallel plans routinely hit merge conflicts in `specs/decision-log.md`. `recorder-agent` computes the next ADR number by counting existing `## ADR-` headings, appends the block at end-of-file, and the validator enforces gap-free numbering starting at ADR-001. Two records therefore collide three ways — same next number, same insertion point, and a no-gaps rule that forces both into one sequence.

### Decision

Store one committed ADR fragment per plan at `specs/_decision/NNN-<plan-name>.md`; delete `specs/decision-log.md`. Different plans touch different files, so git never conflicts on the text.

### Options Considered

| Option | Verdict |
|--------|---------|
| Per-plan fragment files | ✓ Chosen — only disjoint file paths remove the git conflict structurally |
| Keep one shared file with smarter merge or a write lock | ✗ Rejected — a shared append-only file still forces coordination on a single artifact |

### Consequences

Parallel plans write disjoint files, so git never contends on the text. Reading the full decision history requires assembling fragments rather than reading one file.

## ADR: Stable slug identity replaces sequential ADR numbers

**ID:** slug-identity-not-sequential-numbers
**Plan:** avoid-decision-log-conflicts
**Status:** Accepted

### Context

Sequential `ADR-NNN` numbering requires a single shared counter. Parallel plans promoting ADRs independently cannot agree on the next number without coordination, which reintroduces the conflict that per-plan fragment files were meant to remove.

### Decision

Identify each ADR by an explicit kebab-case `**ID:**` slug, unique across all fragments. `Supersedes:` and `Status: Superseded by <slug>` reference slugs instead of numbers. Delete the sequential-number validator.

### Options Considered

| Option | Verdict |
|--------|---------|
| Kebab-case slug identity | ✓ Chosen — slugs are stable identifiers that survive independent, out-of-order promotion |
| Keep `ADR-NNN` numbering | ✗ Rejected — numbers force renumbering and global coordination across parallel plans |

### Consequences

Cross-references survive independent promotion without renumbering. Slug uniqueness must be validated across the full `specs/_decision` directory.

## ADR: Render the aggregate log on demand

**ID:** render-decision-log-on-demand
**Plan:** avoid-decision-log-conflicts
**Status:** Accepted

### Context

A committed, merged view of all ADRs (such as a regenerated `specs/decision-log.md`) is itself a shared artifact that parallel plans would contend over when writing back to it.

### Decision

Add `speq decision-log show` to assemble and print the full log to stdout on demand. The command never writes a merged file to disk.

### Options Considered

| Option | Verdict |
|--------|---------|
| Render on demand via `speq decision-log show` | ✓ Chosen — a rendered view has no artifact to contend over |
| Regenerate a committed `specs/decision-log.md` after each record | ✗ Rejected — any committed aggregate reintroduces the shared-file conflict |

### Consequences

Readers run a command instead of opening a file. No merged artifact exists to diff or conflict on.

## ADR: Duplicate NNN- prefixes across fragments are acceptable

**ID:** duplicate-fragment-prefixes-acceptable
**Plan:** avoid-decision-log-conflicts
**Status:** Accepted

### Context

Assigning a globally unique `NNN-` prefix at record time requires parallel plans to serialize on a shared counter, which reintroduces coordination even after fragment files remove the text conflict.

### Decision

Assign the `NNN-` prefix by counting existing entries at record time and tolerate duplicate prefixes across parallel plans. No renumber or serialization machinery.

### Options Considered

| Option | Verdict |
|--------|---------|
| Duplicate prefixes allowed, no serialization | ✓ Chosen — slugs already disambiguate identity, so a prefix tie is only cosmetic |
| Serialize numbers at record time to guarantee uniqueness | ✗ Rejected — reintroduces coordination and offers no identity benefit |

### Consequences

Two fragments may share an `NNN-` prefix. `speq decision-log show` orders fragments by `(prefix, filename)` to keep output deterministic despite the tie.
