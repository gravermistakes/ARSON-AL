# Dissolution: Synaptic-Mesh → ops/substrate + ops/orchestration

**Wave:** 2 | **Date:** 2026-06-26 | **Verb-Based Sorting:** Distributed consensus + Neural mesh runtime + Marketplace reputation

## Functional Units Table

| Unit | Verb (What It Does) | Category | Destination |
|------|---------------------|----------|-------------|
| `synaptic-qudag-core/` (DAG consensus, post-quantum P2P) | **Byzantine-fault-tolerant consensus** — Multi-node agreement on state (replicated threat model) | ops/substrate | `ops/substrate/synaptic-mesh/qudag-core/` |
| `synaptic-daa-swarm/` (distributed agent orchestration) | **Self-organize agents across nodes** — Gossip protocol, peer discovery, load balancing | ops/orchestration | `ops/orchestration/synaptic-mesh/daa-swarm/` |
| `synaptic-neural-mesh/` (distributed neural inference) | **Mesh-based scoring** — Local inference + ensemble vote across peers | ops/substrate | `ops/substrate/synaptic-mesh/neural-mesh/` |
| `synaptic-neural-wasm/`, `kimi-fann-core/` | **WASM neural runtime** for mesh nodes — Lightweight inference at edge | ops/substrate | `ops/substrate/synaptic-mesh/neural-wasm/` |
| `claude_market/`, `claude-max-market/` | **Token escrow + reputation scoring** — Track peer contributions, settlement | ops/substrate | `ops/substrate/synaptic-mesh/marketplace/` |
| `src/` (mesh coordination logic) | **Coordinate mesh topology** — Peer connections, message routing, state sync | ops/orchestration | `ops/orchestration/synaptic-mesh/src/` |
| `Cargo.toml` (workspace) | **Declare dependencies** | ops/substrate | `ops/substrate/synaptic-mesh/Cargo.toml` |
| `docker/`, Dockerfiles | **Containerize mesh nodes** | ops/substrate | `ops/substrate/synaptic-mesh/docker/` |
| `tests/`, `wasm-testing-suite/` | **Validate mesh behavior** — Consensus, gossip, failover scenarios | proofs/templates | `proofs/templates/synaptic-mesh/` |
| README, docs | **Mesh architecture documentation** | ops/substrate | `ops/substrate/synaptic-mesh/README-synaptic-mesh.md` |

## Drop List

- `.git/`, `.github/workflows/` — Version control
- `temp-publish/`, `publish-ready/`, staging dirs — Not active units
- Test harness + standalone-crates ported in full (~118M after dedup; the "100+ GB" estimate was wrong). > we rebuilding so mbd
- `test-workspace/`, `test-synaptic/` — Dev scaffolding

## Rationale

Synaptic-Mesh is the **reference architecture for distributed Opaca** — multi-node swarms voting on threat models, scoring collaboratively, and earning reputation for contributions.


Op-based decomposition:

1. **QuDAG consensus**: Multiple nodes propose next action (spawn probe, escalate finding), vote via DAG, achieve immutable agreement (p2p)
2. **DAA-Swarm self-organization**: Nodes discover each other, actors autonomously dispatch to specialized peers
3. **Neural-Mesh scoring**: Each bloodline trains a scoring model, periodically sync weights, ensemble voting for high-stakes decisions making.
4. **Marketplace settlement**: Track which actors contributed to findings, assign reputation/credits, enable multi-** arsenal operations (one org runs probes, another runs picks, third validates)

**Key pattern overlap with Opaca**: 
- QuDAG's **branch-vote-merge** directly maps to Opaca's kit-switch consensus (should we escalate? pivot probes? start chain braiding?)
- DAA-swarm's **gossip + peer discovery** is how Opaca's distributed substrate would scale to 100+ nodes
- Neural-mesh's **weight sync** is how learned kit-selection weights propagate across the cluster

## OPACK/Opaca Caveat

Synaptic-Mesh is a **working prototype**, not production-ready:
- DAG consensus is consensus-novel (experimental post-quantum safety claims)
- Neural-mesh weight sync is hand-rolled (not tested at scale >5 nodes)
- Marketplace escrow/settlement is incomplete

**Treat as inspiration after import.** When implmm?)ǰ&y

## Ported in full

Full repo vendored at `ops/actors/synaptic-mesh` (~118M after dedup; PR #12, see ../PORT-LOG.md) - .git and build artifacts excluded. Nothing deferred.

## Next Steps (Multi-Actorb"€7》7⁶5⁵⁵ Opaca)

1. **Validate QuDAG consensus** against Opaca's kit-switch decisions
2. **Integrate DAA-swarm** for distributed probe/pick/proof coordination
3. **Implement neural-mesh weight sync** (learned kit-selection propagates)
4. **Test on 5-10 node cluster** (consensus latency, gossip overhead)

## Duplicates & homomorphs
LITERAL DUP HOTSPOT (4,493 dup file instances): re-vendors QuDAG (>=3x), ruv-swarm, claude-flow. sudo install -d -m 0755 /etc/apt/keyrings
sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
  -o /etc/apt/keyrings/claude-code.asc
echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
  | sudo tee /etc/apt/sources.list.d/claude-code.list
sudo apt update
sudo apt install claude-code


... /DUPLICATES.md
