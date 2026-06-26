<!-- generated: 1782457509 -->
# OPS

## What's here
The perpetual loop — the engine, its memory, the standards it enforces, the
hygiene it keeps. Four buckets:
- `actors/` — the engagement engine + agent/swarm runtimes: opack/Opaca (ECS
  substrate + SWARM-ECS-SPEC + OPACA.md), ruv-fann (+ruv-swarm), synaptic-mesh,
  neural-bridge, loki-mode (RARV autonomy), advanced_evolution (kit-config
  evolution), guild-hall (gamified engagement scoring). Runs the loops.
- `mem/` — memory + learned state: loki-mode episodic/semantic memory; the
  advanced_evolution learning loop (cross-ref into actors/).
- `std/` — standards of practice: the ponytail ladder, research + dissolution
  methodology, the wave manifests. How we write and sort.
- `hyg/` — hygiene / compliance / coverage: AST10 agentic-skills compliance,
  loki-mode quality gates, DeTTECT detection-coverage (evasion-surface) tooling.

## Build
- Ponytail: `cp -R std/ponytail/skills/* ~/.claude/skills/`.
- Opaca/OPACK substrate: see `actors/opack` + `actors/OPACA.md`.

## Test
Per-tool; see each subtree's own README / CLAUDE.md.

## Feeds
- Loop: Perpetual — std/ enforced always; actors/ run the loops; mem/ persists
  state + score; hyg/ keeps coverage + compliance.
- Consumes: nothing (top of the stack). Emits: the engine + the standards.

## Issues
- wave-2 manifests under `std/dissolution/` carry pre-reorg paths; see
  `std/dissolution/REORG.md` for the old->new map.
