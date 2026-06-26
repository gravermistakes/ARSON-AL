# Synaptic-Mesh: Distributed Consensus & Neural Runtime for Multi-Node Opaca

## What This Is

A **distributed swarm runtime** enabling multi-node consensus, P2P engagement, and mesh neural computation. Three interlocking layers:

1. **QuDAG Core** (synaptic-qudag-core) — Post-quantum P2P DAG consensus (Byzantine-fault-tolerant agreement protocol)
2. **DAA-Swarm** (synaptic-daa-swarm) — Distributed agent orchestration (self-organizing swarm behavior)
3. **Synaptic-Neural-Mesh** (synaptic-neural-mesh) — Mesh-based neural inference (distributed scoring + collaborative filtering)

Plus a marketplace layer (claude_market, claude-max-market) for token escrow/reputation scoring — reframed as **engagement-credit tally** for multi-node kit arbitration.

## Source Path
- Repo: `/home/user/Synaptic-Mesh`
- QuDAG: `/standalone-crates/synaptic-qudag-core` (P2P consensus, DAG structure)
- DAA-Swarm: `/standalone-crates/synaptic-daa-swarm` (agent coordination, self-organization)
- Neural-Mesh: `/standalone-crates/synaptic-neural-mesh` (distributed inference)
- Marketplace: `/standalone-crates/claude-*-market` (token escrow, reputation)

## Substrate Role

For **multi-node Opaca** deployments (distributed arsenal swarms):

### QuDAG — Distributed Consensus
- Replaces centralized state with Byzantine-fault-tolerant P2P consensus
- Enables **distributed kit-switch decisions** (10+ nodes vote on next action)
- Post-quantum safe (critical for long-lived threat intel archives)

### DAA-Swarm — Self-Organizing Agents
- Agents autonomously find peer nodes (gossip protocol)
- Load-balanced probe/pick/proof execution across cluster
- Learns which peers hold which specialties (reputation tracking)

### Neural-Mesh — Collaborative Scoring
- Each node trains a local scoring network (kit selection, triage)
- Periodically sync weights via DAG consensus
- Inference uses local model (fast) + ensemble vote (accurate)

### Marketplace — Engagement Credits
- Track which nodes contributed to successful findings
- Reputation-based routing (send high-value probes to trusted nodes)
- Credit escrow + settlement for multi-team arsenal operations

## Bulk Deferred

Full Synaptic-Mesh repo (100+ GB test harness, Docker images) is too large. For now, we copy:
- `Cargo.toml` files (manifests for all 4 crates)
- `README.md` (feature overview)

The prototype is incomplete (marked as "experimental" in many crates). Record **caveat:** "Reference architecture, not production implementation. When implementing distributed Opaca, treat as inspiration for DAG consensus + swarm self-organization patterns."

## Copied Files

- `README-synaptic-mesh.md` — Project overview
- `qudag-core/Cargo.toml` + `README.md` — Post-quantum consensus descriptor
- `neural-mesh/Cargo.toml` + `README.md` — Mesh inference descriptor
- `daa-swarm/Cargo.toml` + `README.md` — Swarm orchestration descriptor

## OPACK-Overlap Caveat

Synaptic-Mesh's **DAG consensus + gossip broadcast** directly overlaps with Opaca's multi-node state replication. Similarly, QuDAG's branch-vote-merge flow mirrors Opaca's kit-switch consensus. Treat Synaptic-Mesh as a **working reference**, not a verbatim import. Opaca will recombine these patterns into a simpler, compiled substrate.

## Next Steps (Follow-Up)

```bash
# Clone full Synaptic-Mesh if building multi-node Opaca
bulk: clone /home/user/Synaptic-Mesh -> /home/user/ARSON-AL/ops/substrate/synaptic-mesh/
# Caution: 100+ GB test suites. Select standalone-crates/ only for production.
```
