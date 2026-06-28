# ARSON-AL

Autonomous security research engine. Sorted by action, not origin.

## Structure

```
ops/        engine — actors, scoring, neural, governance
int/        target modeling, threat mapping, scope parsing
probes/     enumeration, scanning, fingerprinting, secret detection
picks/      fuzzing, cracking, extraction, WAF evasion
paths/      C2, backdoors, traversals
proofs/     PoC, exploit chains, triage, report generation
```

## Actors

Three deterministic runtimes in `ops/actors/`. No LLM per step — Agent is
tactician consulted on uncertainty escalation only.

| Actor | Lang | What |
|-------|------|------|
| opack | C++ | ECS substrate. Perceive-decide-act loop, SHAKE256 UUID seed. |
| swarm | Python | BFT consensus. HMAC auth, reputation, PBFT-lite. |
| ruv-swarm | Rust | Neural swarm. 12 crates, ruv-fann ML integration. |

Outer orchestrator: Loki RARV (`ops/actors/loop/`). Reason-Act-Reflect-Verify.

## Dissolution

Every repo clones to `staging/`, gets cracked open by function.
Each component answers one question: **ops, int, probe, pick, path, or proof?**
Origin repo name is irrelevant. Only the action matters.

## Governance

`ops/gvt/` — consensus (QuDAG), hygiene (AST10, quality gates, DeTTECT),
standards (ponytail, methodology, dissolution manifests).

Ponytail: `cp -R ops/gvt/std/ponytail/skills/* ~/.claude/skills/`

## Build

| What | How |
|------|-----|
| opack | `cd ops/actors/engine/opack && mkdir build && cd build && cmake .. && make` |
| swarm tests | `cd ops/actors/swarm/swarm && python3 -m pytest tests/ -v` |
| ruv-swarm | `cd ops/actors/swarm/ruv-swarm && cargo check -p ruv-swarm-core` |
| loki syntax | `bash -n ops/actors/loop/loki/autonomy/run.sh` |

## License

GPL preferred. MIT/BSD ok.
