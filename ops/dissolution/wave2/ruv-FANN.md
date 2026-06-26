# Dissolution: ruv-FANN → ops/substrate + ops/orchestration

**Wave:** 2 | **Date:** 2026-06-26 | **Verb-Based Sorting:** Neural compute + Swarm orchestration

## Functional Units Table

| Unit | Verb (What It Does) | Category | Destination |
|------|---------------------|----------|-------------|
| `src/` (lib.rs, network.rs, layer.rs, activation.rs, connection.rs, neuron.rs, memory_manager.rs, cascade.rs) | **Compute neural networks** — Layers, neurons, backprop, activations | ops/substrate | `ops/substrate/ruv-fann/src/` |
| `src/integration.rs`, `src/mock_types.rs` | **Test/mock infrastructure** for networks | ops/substrate | `ops/substrate/ruv-fann/src/` |
| `neuro-divergent/` (27+ forecasting models: LSTM, TCN, N-BEATS, Transformer) | **Forecast time-series** — Predict future values, exploit windows, target behavior | ops/substrate | `ops/substrate/ruv-fann/neuro-divergent/` |
| `cuda-wasm/` (build.rs, src/, compile GPU→WASM) | **Transpile GPU code to WASM** — Enable neural inference on edge (browser, embedded) | ops/substrate | `ops/substrate/ruv-fann/cuda-wasm/` |
| `opencv-rust/` (OpenCV bindings) | **Computer vision primitives** — Image analysis for recon/target fingerprinting | ops/substrate | `ops/substrate/ruv-fann/opencv-rust/` |
| `ruv-swarm/` (core/, agents/, cli/, transport/, persistence/, ml/, mcp/) | **Orchestrate concurrent agents** — Lifecycle, topologies, state, learned scoring | ops/orchestration | `ops/orchestration/ruv-fann/ruv-swarm/` |
| `ruv-swarm/ml-training/` (evolver, ml-*) | **Optimize swarm hyperparameters** — Learn best topology, weights, kit-selection | ops/optimizer | `ops/optimizer/ruv-fann/` |
| `Cargo.toml` (root + nested) | **Declare dependencies, workspaces** | ops/substrate | `ops/substrate/ruv-fann/Cargo.toml` |
| `examples/`, `benches/` | **Demo + perf baseline** | proofs/templates | `proofs/templates/ruv-fann/` |
| `.github/workflows/`, CI config | **Build + test automation** | ops/standards | `ops/standards/ruv-fann-ci/` |

## Drop List

- `.claude/`, `.claude-flow/` — Claude Code project state (not part of arsenal)
- `.git/` — Version control (preserve in source, not copied)
- `.swarm/`, `.ruv-swarm/`, `.hive-mind/` — Workspace state files
- `archive/`, staging dirs — Not active units
- `.roo/` — Utility metadata

## Rationale

ruv-FANN is the **neural compute substrate** for Opaca — the learned intelligence that scores actions, selects kits, and forecasts threat windows. Dissolved into:

1. **Substrate**: Neural networks themselves (compute primitives, forecasting models, GPU/WASM transpiler, vision library)
2. **Orchestration**: Swarm lifecycle + agent coordination (reference model for Opaca's concurrent loops)
3. **Optimizer**: ML training loop (learning kit-selection weights, swarm hyperparameters)
4. **Proofs**: Examples + benchmarks (performance baselines for neural scoring)

**Key overlap with Opaca:** ruv-swarm's branch/merge/prune + learned scoring is the reference architecture Opaca will recombine. The swarm is **not** Opaca itself, but a working proof-of-concept that demonstrates ephemeral neural intelligence orchestration.

## Bulk Deferred

Full repo (1.8GB+, 70+ crates) marked for follow-up clone:
```bash
bulk: clone /home/user/ruv-FANN -> /home/user/ARSON-AL/ops/substrate/ruv-fann/
# Includes: ml-training/ (evolution loop), benchmarks/, examples/ (full suite)
```

Currently copied: Core Cargo.toml + lib.rs + README (structure captured, light).

## Next Steps

1. **Validate Opaca neural layer** against ruv-FANN patterns (scoring networks, forecasting models, WASM deployment)
2. **Integrate ml-training evolver** (learn optimal kit mutations)
3. **Port ruv-swarm reference** into Opaca's swarm runtime (topologies, consensus)
