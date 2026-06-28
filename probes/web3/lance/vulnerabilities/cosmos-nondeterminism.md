# Cosmos Non-Determinism

## Hunt Targets

- Usage of Go's native map iteration (`for k, v := range myMap`) inside consensus-critical state machine paths (keepers, handlers, BeginBlocker/EndBlocker).
- Use of floating-point arithmetic (using `float32` or `float64`) or standard float-handling libraries within state computations.
- Usage of system clock reads (`time.Now()`) instead of block time (`ctx.BlockTime()`).
- Concurrent goroutines executing state mutations or reading unstable state during state machine execution.

## Exploit Checks

- Identify the specific transaction or ABCI call path containing the non-deterministic code.
- Run multi-node simulation (or integration test framework) sending identical txs where the state divergence triggers a validator consensus discrepancy.
- Prove that execution results in a chain-wide halt due to consensus mismatch.

## Reject Conditions

- Code is executed only in CLI clients, query handlers, or client-side tooling.
- Map keys are explicitly sorted before iteration.
- Floats are only used for logging or off-chain metrics, not state modifications.

## Evidence Required

- Non-deterministic code location and target execution path.
- State-changing write operation that relies on the non-deterministic output.
