# ruv-FANN: Neural Compute Substrate

## What This Is

ruv-FANN is Rust-native neural network library reimplementing the decades-old FANN algorithm with modern performance (SIMD, GPU transpilation, zero-unsafe). It's the neural **compute core** for scoring, kit-selection, and learned decision-making in the Opaca arsenal.

**Three nested components:**
1. **ruv-FANN Core** (src/) — Neural network primitives, layers, activations, backprop
2. **Neuro-Divergent** (neuro-divergent/) — 27+ time-series forecasting models (LSTM, N-BEATS, etc.)
3. **CUDA-WASM** (cuda-wasm/) — GPU compute for training + WASM transpilation for inference

## Source Path
- Repo: `/home/user/ruv-FANN`
- Core: `/src` (lib.rs, network.rs, layer.rs, activation.rs, memory_manager.rs, etc.)
- Forecasting models: `/neuro-divergent` (27+ models)
- GPU/WASM: `/cuda-wasm` + `/opencv-rust` (GPU training, WASM inference)

## Substrate Role

This sits at the **neural layer** of OPACK/Opaca — the compute engine that:
- Scores candidate actions (fast <100ms neural inference)
- Learns kit-selection weights (adaptive kit-config tuning)
- Powers triage decisions (classification networks for severity/type)
- Enables forecasting (time-series prediction of exploit windows, target behavior)

**Key performance targets in Opaca:**
- Inference: <100ms per decision
- Training: 2-4x faster than Python alternatives (27+ models via Neuro-Divergent)
- Deployment: WASM (browser, edge, embedded) + CPU-native (server)

## Bulk Deferred

The full repo (1.8GB+, 70+ crates) should be cloned in follow-up. For now, we copy:
- Core Cargo.toml (manifest)
- lib.rs (entry point + network primitives)
- neuro-divergent/Cargo.toml + README (forecasting models descriptor)
- cuda-wasm/Cargo.toml + README (GPU/WASM transpiler descriptor)

## Copied Files

- `Cargo.toml` — Root workspace manifest (defines ruv-fann + ruv-swarm + neuro-divergent + cuda-wasm)
- `README-ruv-fann.md` — Feature overview
- `lib.rs` — Core neural primitives (Network, Layer, Neuron, activation functions)

## OPACK-Overlap Caveat

ruv-FANN's **branch/merge/prune** loop (in ml-training/ optimizer) overlaps with Opaca's kit-switch branches. We record this as "reference architecture" — when implementing Opaca's evolution loop, examine ruv-FANN's optimizer patterns (darwinian_evolver meets genetic algorithms for hyperparameter tuning).

## Next Steps (Follow-Up)

```bash
# Copy full ruv-FANN source after substrate foundation is set
bulk: clone /home/user/ruv-FANN -> /home/user/ARSON-AL/ops/substrate/ruv-fann/
```
