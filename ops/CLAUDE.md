# OPS

## What's here
The engine, fully dissolved by function — no repo wrappers. Buckets:
- `Opaca/` — the runtimes that hunt: `engine/` contains `actors/` and `swarm/` (ruv-swarm + daa-swarm + loki
  swarm orchestration).
- `gaming/` — `score` and `rep`
- `mem/` — the neural engines (ruv-fann core, neural-bridge, neuro-divergent, synaptic neural-mesh, kimi-fann) + memory stores (loki memory).
- `gvt/` — governance: `hyg/` (AST10, loki quality-gates, DeTTECT coverage), `std/` (ponytail, methodology, dissolution).

Product/build/docs scaffolding is parked at root `.scaff/`.

## Build
- Ponytail: `cp -R gvt/std/ponytail/skills/* ~/.claude/skills/`.
- Opaca / OPACK substrate: [!!!please pay attention!!!]

## Feeds
- Loop: Perpetual — gvt/ enforced always; actors/ run the loops; gaming/ scores; mem/ remembers + learns.
- Consumes: agent instructions. Emits: the engine + the governance.

## Issues
- wave manifests under `gvt/std/dissolution/` cite pre-dissolution paths; sort by function (see `gvt/std/dissolution/REORG.md`), not the stale path.
