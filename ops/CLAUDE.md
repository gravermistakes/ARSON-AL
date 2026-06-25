<!-- generated: 1782415219 -->
# OPS

## What's here
Standards, substrate, and orchestration — the perpetual loop. Rules every other
directory is built under. Currently: `standards/ponytail/` — the lazy-senior-dev
coding standard (YAGNI → reuse → stdlib → native → installed dep → one line →
minimum) plus its review/audit/debt enforcement modes, lifecycle hooks, and
multi-agent adapters. Future peers: AST10 compliance, OPACK ECS substrate, CI/CD.

## Build
Ponytail is config + skills, nothing compiles. Install into an agent:
`cp -R standards/ponytail/skills/* ~/.claude/skills/` (or load the plugin via
`standards/ponytail/integration/claude-plugin/`).

## Test
`node standards/ponytail/integration/pi-extension/test/extension.test.js`
Skills are prose contracts — verify by reading `skills/ponytail/SKILL.md`.

## Feeds
- **Loop:** Perpetual — always enforced, never exits.
- **Consumes:** nothing (top of the stack).
- **Emits:** the ladder. Every line written in probes/ picks/ proofs/ int/
  passes it. `ponytail-review` / `ponytail-audit` gate diffs and repos for slop.

## Issues
- Adapters in `integration/adapters/` are per-agent copies of one rule; upstream
  kept them in sync with a script that was dropped as packaging cruft. Edit the
  skill, re-derive adapters by hand if needed.
- AST10, OPACK not yet dissolved — `standards/` and substrate are stubs.
