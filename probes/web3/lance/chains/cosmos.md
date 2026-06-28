# Cosmos Chain Guide

## Intake

Capture:
- `go.mod` Cosmos SDK and CometBFT / Tendermint versions.
- Custom `x/` module list defined in `app.go`.
- CosmWasm version and contract targets if present.
- IBC channel configuration and enabled protocols.
- Active validator set count and governance parameters.

## Cosmos Focus Areas

- **ABCI Lifecycle Hooks**: BeginBlocker, EndBlocker, and DeliverTx state transition paths.
- **Non-determinism**: Ensure no floating-point operations (`float64`), un-ordered map iterations, or `time.Now()` / system clock reads are used in consensus-critical code.
- **Panic Boundaries**: Unhandled panics within ABCI handlers that can cause chain-wide DoS (chain halt).
- **KV Store Key Serialization**: Validate safe serializations to avoid prefix collisions (e.g. pool ID string collisions allowing unauthorized state manipulation).
- **AnteHandler Chains**: Order and completeness of AnteHandlers for signature verification, gas fee enforcement, and tx filtering.
- **IBC Trust Assumptions**: Trust assumptions of channels, IBC client validation, and relay logic constraints.

## Evidence Sources

- `go.mod` dependency specifications.
- `app.go` application manager definition and module routing.
- Individual keeper implementation files.
- AnteHandler registration chain in `app.go`.
- Protobuf (`.proto`) message designs.

## Common Traps

- Flagging map iteration in CLI commands (which is safe as it's not consensus-critical).
- Flagging generic Go panics outside the state machine path.
- Recommending float usage fixes without considering determinism constraints.
