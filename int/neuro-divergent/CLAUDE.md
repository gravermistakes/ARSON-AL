<!-- generated -->
# int/neuro-divergent

## What's here
The forecasting / predictive-intel substrate. Rust neural-forecasting library
(NeuralForecast-API-compatible) built on ruv-FANN. Used by recon to predict
target activity windows, attack-window scoring, campaign trend modeling — any
"what will this series look like next?" intel question.

Five sub-crates, one function (forecasting):
- `neuro-divergent-core/` — types, traits, common forecasting interface
- `neuro-divergent-models/` — 27+ neural forecasting models (LSTM, N-BEATS, TCN,
  Transformers, etc.); the species roster
- `neuro-divergent-data/` — data loading + preprocessing
- `neuro-divergent-training/` — training loops
- `neuro-divergent-registry/` — model registry / lookup
- `src/` — umbrella crate re-exporting the above

## Build
`cargo build` from this dir (workspace).

## Feeds
- Loop: Recon — feed it target time-series (request patterns, deploy cadence,
  CVE-disclosure patterns) → forecast → BadFaithActor decides timing.
- Consumes: target manifests + time-series from probes/scrapers.
- Emits: forecast tensors + confidence into int/'s dossier for the engagement.

## Issues
- Heavy ML dep tree (~5 sub-crates). Compile-once, then call as a library; not
  a daily-rebuild thing.
- benches/docs/examples/plans/tests parked at .scaff/neuro-divergent (rebuild
  needs them — pull back when wiring tests).
