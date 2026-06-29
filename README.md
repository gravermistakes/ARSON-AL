# ARSON-AL

Autonomous security research engine. Concurrent loops that hunt, validate,
and chain findings into exploitable paths. No pipeline — loops branch, merge,
and prune in parallel.

## How it works

A target enters through `int/`. Recon models the surface. Probes extract what's
exposed. Findings feed back into recon (more surface) and forward into picks
(exploitation). Proofs gate everything — nothing exits without a PoC. Background
chain loop braids low-severity findings into critical chains.

```
int/ models target ──→ probes/ extract surface ──→ picks/ fire
  ↑                        │                          │
  └── findings refine ─────┘          proofs/ gate ←──┘
                                         │
                                   chain loop braids
```

Four (currently, more need to occur) concurrent loops:

- **Recon** — int/ models, probes/ extracts, findings refine the model, repeat.
- **Scan** — probes/ scan exposed surface, hits spawn adjacent probes.
- **Validation** — proofs/ gate for exploitability, failures feed back to recon or scan via chain.
- **Chain** — background. Collects low-severity rejects, braids into critical chains.

Branching: new surface spawns a probe. Source code found spawns SAST. Chain
succeeds = report. Pruning: hypothesis dies, patched, or 3 PoC approaches
exhausted.

## What's where

Everything sorted by what it mechanically does, not where it came from.

```
ops/        the engine
  actors/     deterministic runtimes (opack ECS, swarm BFT, ruv-swarm neural, loki RARV)
  gaming/     scoring + rep (evolution, guild tiers)
  mem/        neural engines (ruv-fann, neural-bridge, synaptic mesh) + memory stores + chain state
  gvt/        governance (quality gates, DeTTECT, ponytail methodology)
int/        target modeling, threat mapping, ATT&CK technique mappings
probes/     enumeration, scanning, fingerprinting, secret detection, SAST, SCA
picks/      fuzzing, cracking, extraction, WAF evasion, deserialization, smuggling
paths/      C2 frameworks, backdoors, traversals, persistent access
proofs/     PoC templates, CVSS scoring, platform report generators
```

## Actors

Three deterministic runtimes in `ops/actors/`. No LLM per step. Each actor's
seed is its UUID — SHAKE256(timestamp) to septal (base 7) truncated to 13 digits. Kit
selection is a pure function of UUID, timestamp, hunt state, and kit crosslinks.
Agent is the tactician, consulted only on uncertainty escalation.

**opack** (C++) — ECS substrate on flecs. Perceive-decide-act loop. Emergent
recon-scan-exploit-validate-report chains from kit scoring, not scripted
sequences. convert to ocaml ASAP

**swarm** (Python) — BFT consensus. HMAC authentication, replay protection,
reputation tracking, PBFT-lite. 43 verified test cases.

**ruv-swarm** (Rust) — Neural swarm orchestration. 12 workspace crates with
ruv-fann ML integration for adaptive decision-making. needs linking to actors

Outer orchestrator: **Loki RARV** — Reason-Act-Reflect-Verify. The RARV loop
wraps the actors but seldom replaces their inner deterministic loops.
