<!-- generated: 1782415219 -->
# OPS

## What's here
Standards, substrate, and orchestration — the perpetual loop. Rules every other
directory is built under.
- `standards/ponytail/` — the lazy-senior-dev coding standard (YAGNI → reuse →
  stdlib → native → installed dep → one line → minimum) + review/audit/debt
  enforcement modes, lifecycle hooks, and multi-agent adapters.
- `substrate/opack/` — C++ ECS multi-agent library (flecs-based). The compiled
  substrate tool "cells" recombine onto; C++ owns branching/merging/pruning/
  state, the LLM rides on top for triage and prose.
Future peers: AST10 compliance, CI/CD.

## Build
- Ponytail: config + skills, nothing compiles. Install into an agent:
  `cp -R standards/ponytail/skills/* ~/.claude/skills/` (or load the plugin via
  `standards/ponytail/integration/claude-plugin/`).
- OPACK: `cmake -S substrate/opack -B substrate/opack/build && cmake --build
  substrate/opack/build` (CPM pulls flecs). `-DOPACK_BUILD_EXAMPLES=ON` for demos.

## Test
- Ponytail: `node standards/ponytail/integration/pi-extension/test/extension.test.js`;
  skills are prose contracts — verify by reading `skills/ponytail/SKILL.md`.
- OPACK: `ctest --test-dir substrate/opack/build` (BUILD_TESTING on by default).

## Feeds
- **Loop:** Perpetual — always enforced, never exits.
- **Consumes:** nothing (top of the stack).
- **Emits:** the ladder. Every line written in probes/ picks/ proofs/ int/
  passes it. `ponytail-review` / `ponytail-audit` gate diffs and repos for slop.

## Issues
- Adapters in `integration/adapters/` are per-agent copies of one rule; upstream
  kept them in sync with a script that was dropped as packaging cruft. Edit the
  skill, re-derive adapters by hand if needed.
- OPACK is upstream-flagged WIP ("project not ready yet"); treat the substrate
  API as unstable. benchmarks/experiments/docs were dropped — don't enable those
  CMake options.
- AST10 compliance and CI/CD not yet dissolved.
