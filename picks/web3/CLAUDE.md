<!-- generated: 1782639068 -->
# PICKS/WEB3
## What's here
Ways in — smart contract fuzzers and exploitation tools for Web3 environments.
- `evm-fuzzers.md` — key EVM property-based and sequence fuzzers (Echidna, Foundry Fuzz, ChainFuzz, Harvey, sFuzz) used to identify deep-state vulnerability violations.

## Build
No compilation required. These are tool references and execution playbooks utilized by automated or manual agents.

## Test
Verify fuzzers by writing custom assertion tests against target Solidity contracts inside a Foundry workspace:
```bash
forge test
```

## Feeds
- **Loop:** Scan/Chain — a scanner hit indicating complex state rules triggers the use of fuzzers to find invariant-breaking exploit sequences.
- **Consumes:** exposed contract ABIs and transaction patterns from probes/web3.
- **Emits:** crash states and transaction sequences demonstrating exploitability -> proofs/web3 for report generation.

## Issues
- Currently includes only tool metadata and playbooks. Need to integrate actual fuzzing harness templates.
