# Cosmos ABCI Panic Chain Halt

## Hunt Targets

- Usage of unhandled `panic()` or standard Go runtime crashes (like nil-pointer dereferences or slice out-of-bounds indices) within `BeginBlocker`, `EndBlocker`, or custom keeper state transition handlers.
- Safe-wrapper omission in third-party library calls inside ABCI hooks.

## Exploit Checks

- Craft a specific payload or transaction that triggers the execution path of the unhandled panic inside `BeginBlocker` or `EndBlocker`.
- Verify the validator process terminates instantly when processing the block.
- Prove that because all validators receive the same block, they all crash, resulting in a permanent chain halt until manual state patch or software rollback.

## Reject Conditions

- Panic is caught by a higher-level recovery handler (`recover()`) and wrapped cleanly.
- The path cannot be triggered by arbitrary users or transaction inputs (e.g. requires superuser/governance keys).

## Evidence Required

- Target panic/crash site and corresponding code path.
- Specific transaction payload or environment state required to trigger the panic.
