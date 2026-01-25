---
name: recorder
description: |
  Apply approved plan deltas to permanent feature specs.
  Use after a plan is approved to merge deltas from specs/_plans/<plan-name>/ into specs/.
  Invoke explicitly with /spec-recorder <plan-name> to record approved changes.
  Handles NEW, CHANGED, and REMOVED scenarios from delta specs.
---

# Spec Recorder

Apply approved plan deltas to permanent feature specifications.

## Quick Start

After a plan is approved, run:

```bash
speq record <plan-name>
```

This command:
1. Finds all delta specs in `specs/_plans/<plan-name>/`
2. Merges each delta into its target spec at `specs/<domain>/<feature>/spec.md`
3. Strips all DELTA markers from the merged output
4. Validates the merged specs
5. Archives the plan to `specs/_recorded/<plan-name>/`

## Spec Hierarchy

```
specs/
├── <domain>/
│   └── <feature>/
│       └── spec.md          # Permanent specs
├── _plans/
│   └── <plan-name>/
│       └── <domain>/<feature>/spec.md  # Delta specs
└── _recorded/
    └── <plan-name>/         # Archived plans
```

## Manual Workflow (if needed)

### 1. Load Plan

```
Read: specs/_plans/<plan-name>/plan.md
List: specs/_plans/<plan-name>/**/spec.md
```

### 2. For Each Feature Delta

```
Read delta: specs/_plans/<plan-name>/<domain>/<feature>/spec.md
Check: specs/<domain>/<feature>/spec.md exists?
├─ No  → New feature: copy delta (strip markers)
└─ Yes → Merge changes into existing spec
```

### 3. Apply Delta Operations

| Marker | Action |
|--------|--------|
| `DELTA:NEW` | Add scenario to spec |
| `DELTA:CHANGED` | Replace existing scenario |
| `DELTA:REMOVED` | Delete scenario from spec |

### 4. Validate

```bash
speq feature validate <domain>/<feature>
```

### 5. Archive Plan

Move `specs/_plans/<plan-name>/` to `specs/_recorded/<plan-name>/`

## Merge Rules

### New Feature (no existing spec)

1. Copy entire delta spec to `specs/<domain>/<feature>/spec.md`
2. Remove all `<!-- DELTA:* -->` markers
3. Validate result

### Existing Feature

1. Parse delta for marked scenarios
2. For each `DELTA:NEW`: append scenario to Scenarios section
3. For each `DELTA:CHANGED`: replace scenario with same name
4. For each `DELTA:REMOVED`: delete scenario with same name
5. Remove markers from merged content
6. Validate result

## Example

**Delta** (`specs/_plans/auth-upgrade/auth/login/spec.md`):
```markdown
## Scenarios

<!-- DELTA:NEW -->
### Scenario: Two-factor auth

* *GIVEN* a user with 2FA enabled
* *WHEN* the user logs in
* *THEN* the system SHALL prompt for 2FA code
<!-- /DELTA:NEW -->

<!-- DELTA:REMOVED -->
### Scenario: Guest login
<!-- /DELTA:REMOVED -->
```

**Command**:
```bash
speq record auth-upgrade
```

**Result**:
- `specs/auth/login/spec.md` updated:
  - "Two-factor auth" scenario added
  - "Guest login" scenario removed
  - No DELTA markers in permanent spec
- Plan archived to `specs/_recorded/auth-upgrade/`
