---
name: speq-recorder
description: |
  Apply approved plan deltas to permanent feature specs.
  Use after a plan is approved to merge deltas from specs/_plans/<plan-name>/ into specs/.
  Invoke explicitly with /speq-recorder <plan-name> to record approved changes.
  Handles NEW, CHANGED, and REMOVED scenarios from delta specs.
---

# Spec Recorder

Merge plan deltas into permanent specs and archive completed plans.

Get plan name from user prompt or ask if none specified.

## Workflow

### Phase 1: Verify Implementation

```
Check: specs/_plans/<plan-name>/verification-report.md exists?
├─ Yes → Proceed
└─ No  → STOP: "Run /speq-implementer <plan-name> first."
```

### Phase 2: Load Plan

```bash
speq feature list                 # Current spec library
```

```
Read: specs/_plans/<plan-name>/plan.md
List: specs/_plans/<plan-name>/**/spec.md
```

### Phase 3: Apply Deltas

For each delta spec in `specs/_plans/<plan-name>/<domain>/<feature>/spec.md`:

```
specs/<domain>/<feature>/spec.md exists?
├─ No  → Copy delta (strip markers)
└─ Yes → Merge using markers below
```

| Marker | Action |
|--------|--------|
| `DELTA:NEW` | Append scenario |
| `DELTA:CHANGED` | Replace scenario with same name |
| `DELTA:REMOVED` | Delete scenario with same name |

After each merge:
1. Strip all `<!-- DELTA:* -->` markers
2. Validate: `speq feature validate <domain>/<feature>`

### Phase 4: Optimize Library

Check thresholds after merging:

| Metric | Threshold | Action |
|--------|-----------|--------|
| Scenarios per spec | >10 | Ask user about split |
| Domain features | >8 | Ask about sub-domains |

**Never assume** — use `AskUserQuestion` for organization decisions.

### Phase 5: Finalize

1. Validate: `speq feature validate`
2. Archive: `mv specs/_plans/<plan-name> specs/_recorded/<plan-name>`

```
✓ Verification report confirmed
✓ All deltas merged
✓ Spec library validated
✓ Plan archived
```

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Record without verification report | Implementation not proven |
| Assume split/domain decisions | User must confirm |
| Skip validation | Broken specs may result |
| Leave DELTA markers | Pollutes permanent specs |
