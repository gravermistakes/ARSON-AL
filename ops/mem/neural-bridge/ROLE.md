# Role: JS/WASM shim (not a separate engine)

`neural-bridge` has no neural-net of its own — its deps are only
`wasm-bindgen`/`js-sys`/`web-sys`. It is the JS↔WASM marshalling shim for
`ops/mem/ruv-fann-core`'s wasm backend. Kept for the JS-facing interface;
the FANN engine lives in `ruv-fann-core`.
