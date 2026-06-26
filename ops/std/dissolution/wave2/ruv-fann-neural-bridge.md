# Dissolution: ruv-fann-neural-bridge → ops/substrate

**Wave:** 2 | **Date:** 2026-06-26 | **Verb-Based Sorting:** WASM neural inference + performance telemetry

## Functional Units Table

| Unit | Verb (What It Does) | Category | Destination |
|------|---------------------|----------|-------------|
| `lib.rs`, `wasm/` | **Export neural inference to WASM** — Load models, run forward pass, return scores | ops/substrate | `ops/substrate/neural-bridge/lib.rs` |
| `wasm-loader.js`, `wasm-performance-benchmark.js` | **Load and benchmark WASM modules** in browser/Node.js | ops/substrate | `ops/substrate/neural-bridge/wasm/` |
| `neural/optimizer.rs` | **Optimize weights post-training** — Quantization, pruning, distillation | ops/substrate | `ops/substrate/neural-bridge/neural/` |
| `simd-config.rs` | **Detect CPU features, enable SIMD paths** — CPU acceleration for inference | ops/substrate | `ops/substrate/neural-bridge/simd-config.rs` |
| `build.rs`, `build-wasm-optimized*.sh` | **Compile Rust→WASM with optimizations** — Size/perf tradeoff | ops/substrate | `ops/substrate/neural-bridge/build/` |
| `performance/monitor.rs` | **Track inference latency, memory** — Telemetry for scoring pipeline | ops/substrate | `ops/substrate/neural-bridge/performance/` |
| `Cargo.toml`, `wasm-build.toml` | **Declare WASM dependencies and config** | ops/substrate | `ops/substrate/neural-bridge/Cargo.toml` |
| `README.md` | **Feature documentation** | ops/substrate | `ops/substrate/neural-bridge/README-neural-bridge.md` |

## Drop List

- `.git/` — Version control
- `.swarm/` — Workspace state
- Intermediate build artifacts (`.wasm` compiled objects should not be committed)

## Rationale

ruv-fann-neural-bridge is the **deployable scoring engine** for Opaca's learned kit-selection and triage classification. It bridges the gap between training (ruv-FANN) and inference (deployed probes/picks/proofs).

Verb-based decomposition:

1. **Inference**: Load trained model → run forward pass → return decision (severity, exploit probability, next kit)
2. **Optimization**: Quantize weights, prune unnecessary neurons, distill ensembles for smaller .wasm binaries
3. **Acceleration**: Detect CPU features (SIMD), route to optimized code paths
4. **Telemetry**: Track latency, memory per decision (ensure <100ms SLA)
5. **Build**: Compile Rust to WASM with size/perf tradeoffs (wasm-opt via binaryen)

**Substrate role**: Sits at the **inference boundary** — every scoring decision (triage: true-positive or false-positive? finding severity: critical or medium?) goes through the neural-bridge WASM module.

## Bulk Deferred

Full repo with all WASM build artifacts and benchmarks marked for follow-up:
```bash
bulk: clone /home/user/ruv-fann-neural-bridge -> /home/user/ARSON-AL/ops/substrate/neural-bridge/
# Includes: Full build output, WASM modules, performance benchmarks
```

Currently copied: Core lib.rs + Cargo.toml + SIMD config + README (structure captured).

## Next Steps

1. **Wire neural-bridge into Opaca's scoring loop** — Every finding triage decision uses WASM inference
2. **Validate <100ms latency** target (critical for real-time engagement)
3. **Integrate model versioning** (which trained model is deployed? A/B test scoring strategies)

## Duplicates & homomorphs
Homomorph of ruv-fann core + synaptic-neural-wasm (neural inference) -> one mem/ scoring engine.
Full dedup index: ../DUPLICATES.md
