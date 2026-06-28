# OPS

## What's here
The engine, fully dissolved by function — no repo wrappers. Buckets:
- `actors/` — the runtimes that hunt: `engine/` (opack ECS + OPACA spec),
  `loop/` (loki RARV engagement loop), `swarm/` (ruv-swarm + daa-swarm + loki
  swarm orchestration).
- `gaming/` — `score/` (scoring logic) + `reward/` (advanced_evolution,
  loki token-economics, synaptic market, guild-hall tiers).
- `mem/` — the neural engines (ruv-fann core, neural-bridge, neuro-divergent,
  synaptic neural-mesh, kimi-fann) + memory stores (loki memory).
- `gvt/` — governance: `consensus/` (QuDAG), `hyg/` (AST10, loki quality-gates,
  DeTTECT coverage), `std/` (ponytail, methodology, dissolution).

Product/build/docs scaffolding is parked at root `.scaff/`.

## Build
- Ponytail: `cp -R gvt/std/ponytail/skills/* ~/.claude/skills/`.
- Opaca / OPACK substrate: see `actors/engine/`.

## Feeds
- Loop: Perpetual — gvt/ enforced always; actors/ run the loops; gaming/ scores;
  mem/ remembers + learns.
- Consumes: nothing (top of the stack). Emits: the engine + the governance.

## Issues
- wave manifests under `gvt/std/dissolution/` cite pre-dissolution paths; sort by
  function (see `gvt/std/dissolution/REORG.md`), not the stale path.
