# ruv-fann-neural-bridge: WASM Inference Engine for Scoring

## What This Is

A specialized **inference-only** neural runtime compiled to WebAssembly. Takes trained ruv-FANN models and deploys them for real-time decision scoring at the edge (browser, edge compute, embedded systems).

**Core responsibility:** Deploy learned scoring networks (kit-selection, triage classification, threat severity) with sub-millisecond latency.

## Source Path
- Repo: `/home/user/ruv-fann-neural-bridge`
- Entry: `lib.rs` (bridge API, WASM exports)
- Optimizer: `neural/optimizer.rs` (learned weight tuning)
- SIMD config: `simd-config.rs` (CPU acceleration)
- Build: `build-wasm-optimized*.sh` (Wasm compilation + optimization)

## Substrate Role

This is the **deployable neural scoring layer** in Opaca:
1. Load a trained neural network (weights + config)
2. Run inference on a scored action or finding
3. Return decision (severity, exploit probability, next action)
4. Sub-ms latency (no cold-start, no API round-trip)

Used in:
- **Triage**: Classify findings (false positive vs true positive)
- **Scoring**: Rate exploitability, impact, urgency
- **Kit-selection**: Learned routing (which probe/pick/proof for this target?)

## Bulk Deferred

Full repo (WASM builds, benchmarks) copied on follow-up. For now:
- `lib.rs` — WASM FFI contract + inference API
- `Cargo.toml` — Dependencies
- `simd-config.rs` — SIMD acceleration (CPU-optimized path)
- `neural/optimizer.rs` — Weight optimization post-training

## Copied Files

- `lib.rs` — Core WASM bridge (load model, run inference)
- `Cargo.toml` — Workspace dependencies
- `simd-config.rs` — CPU feature detection + SIMD instructions
- `README-neural-bridge.md` — Feature description

## Next Steps (Follow-Up)

```bash
# Copy full build artifacts and WASM output
bulk: clone /home/user/ruv-fann-neural-bridge -> /home/user/ARSON-AL/ops/substrate/neural-bridge/
# Full build: npm run build:all (produces wasm-loader.js + optimized .wasm modules)
```
