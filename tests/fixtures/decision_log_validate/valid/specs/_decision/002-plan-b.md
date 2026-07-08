# Decisions: plan-b

## ADR: Faster scanner

**ID:** faster-scanner
**Plan:** plan-b
**Status:** Accepted
**Supersedes:** use-line-scanner

### Context

The line-oriented scanner became a bottleneck once fragment counts grew.

### Decision

Replace the line scanner with a faster single-pass scanner.
