# On-Chain Forensics and Reverse Engineering

## Blockchain Explorers

- **Etherscan** (`etherscan.io`): EVM canonical block explorer. Used for contract read/write, transaction verification, event monitoring.
- **Bscscan** (`bscscan.com`), **Polygonscan** (`polygonscan.com`), **Blockchair** (`blockchair.com`), **Blockchain.com**: Multi-chain explorers for parsing ledgers.

---

## Contract Decompilers

- **Dedaub** (`dedaub.com`): High-quality EVM decompiler and static analysis platform. Essential for reverse engineering bytecode of unverified contracts.
- **Panoramix** (`palkeo/panoramix`): Local Python-based decompiler for EVM bytecode.
- **ABI Decompiler** (`Decurity/abi-decompiler`): Recovers Solidity-like function definitions and ABI signatures from bytecode.

---

## Transaction Visualization and Tracing

- **MistTrack** (`misttrack.io`): Forensics and address tracking utility. Excellent for money-flow tracing and asset recovery investigations.
- **Phalcon BlockSec** (`phalcon.blocksec.com`): In-depth transaction simulation and visualization tool. Displays exact reentrancy calls, balance changes, and internal EVM state logs.
- **ethtx.info**: Decodes EVM transaction traces to show execution details and internal calls.
- **eigenphi.io**: Traces and visualizes front-running, sandwich, and arbitrage MEV transactions.
- **samczsun tx viewer** (`tx.eth.samczsun.com`): Technical execution trace viewer for deep analysis of exploit sequences.
- **Tenderly Debugger** (`tenderly.co`): Allows step-by-step transaction debugging, local state overrides, and simulation of target execution.

---

## Rug pull and Token Trust Checkers

- **Rug Pull Finder**, **bscheck**, **rugscreen**, **QuillCheck**, **poocoin rugcheck**, **Token Sniffer**, **Rugdoc Honeypot**: Used for quick assessment of contract owner centralization, blacklists, minting functions, and liquidity lock status during target modeling.
