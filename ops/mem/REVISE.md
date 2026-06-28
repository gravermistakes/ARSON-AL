# Mem — revision draft

Bones only. The substrate the records LIVE IN (per gaming SPEC: "no separate
memory system — the records ARE the memory"). Revise in place; `___` = fill me.

## What mem/ holds
- actor records (uuid, score/lvl/pos, rep/rank)
- kit crosslinks (relationship weights between kits)
- patterns (findings + kit sequences, depersonalized post-hunt)
- ___

## Inference engines (already on disk)
- `ruv-fann-core/` — FANN feedforward
- `kimi-fann-core/` — MoE (router + experts)
- `synaptic-neural-mesh/` — QuDAG-backed mesh inference
- `synaptic-neural-wasm/` — ndarray engine
- `neural-bridge/` — JS/WASM shim (ROLE.md)
- `cuda-wasm/` — GPU→WASM transpiler
- `loki-memory/` — episodic/semantic/procedural primitives
- `loki-mode-memory/` — older skeleton (subset of loki-memory)
- `synaptic-memory/` — synaptic mesh memory primitives
- ___

## What gets read during a hunt
- kit-crosslink weights (for actor kit-selection: `f(uuid, timestamp, hunt_state, kit_crosslinks)`)
- ___ (level pre-shape lookup?)
- ___

## What gets read between hunts (post-mortem)
- depersonalization step (findings + kit sequences → patterns, actor UUIDs stripped)
- score/rep fold-up into actor records
- crosslink weight update
- pattern aggregation
- ___

## How records are stored
- actor record: keyed by UUID
- kit crosslinks: ___
- patterns: ___
- backend: ___ (SQLite / flat files / agentdb / ?)

## Depersonalization
- when: post-hunt
- what's stripped: actor UUIDs
- what's kept: kit sequence, finding class, target class, outcome
- ___

## Read/write paths
| who | reads | writes | when |
|---|---|---|---|
| Actor (kit-select) | crosslinks, own UUID-bias | — | mid-hunt |
| Score engine | hunt log | actor record (score axis) | post-hunt |
| Rep engine | hunt outcome | actor record (rep axis) | post-hunt |
| Pattern depersonalizer | hunt log | patterns | post-hunt |
| Agent (actor pick) | actor records | — | mid-hunt selection |
| ___ | ___ | ___ | ___ |

## Which engine does what
- kit-selection inference: ___
- pattern matching / retrieval: ___
- pattern training (post-hunt batch): ___
- chain-rate scoring (rep): ___
- ___

## Persistence backend
- single store or per-bucket? ___
- on-disk format: ___
- crash recovery: ___

## Cross-engagement
- patterns persist forever (or decay?): ___
- crosslinks persist with weight (no path ever blocked, per gaming SPEC)
- actor records persist by UUID
- ___

## Where mem/ lives
- `ops/mem/` (current)
- ___

## Build order
1. ___
2. ___
3. ___
4. ___

## Open
- engine consolidation: ruv-fann-core vs kimi vs synaptic-neural-mesh — which does inference for which task? ___
- loki-memory vs loki-mode-memory: keep both or collapse? ___
- ___
