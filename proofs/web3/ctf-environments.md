# Web3 Capture the Flag and PoC Environments

## DeFiVulnLabs (`SunWeb3Sec/DeFiVulnLabs`)

- **Focus**: The highest-value, hands-on, runnable smart contract vulnerability testbed. Contains ~60 individual smart contract vulnerability files written in Foundry (`forge`) format.
- **Vulnerabilities Covered**: Reentrancy, oracle price manipulation, signature replay, flash-loan exploits, uninitialized proxies, access control failure, integer overflow, etc.
- **Usage**: Each lab provides a vulnerable contract and a corresponding exploit testing harness. Essential as raw templates to copy/adapt when writing working PoCs for gates G3 (Exploit) and G4 (Economic) in the Lance validation loop.

---

## Damn Vulnerable DeFi (`damnvulnerabledefi.xyz`)

- **Focus**: The definitive playground for DeFi-specific exploits. Teaches flash loan mechanics, on-chain lending protocols, oracles, governance mechanics, and DEX interactions.
- **Environment**: Managed via Hardhat or Foundry workspaces. Shows how a chain of transactions can drain a protocol.

---

## The Ethernaut (`ethernaut.openzeppelin.com`)

- **Focus**: Web3 smart contract wargame built by OpenZeppelin. Teaches fundamental EVM security concepts, contract architecture details, memory layouts, and programming traps.
- **Challenges**: Fallback, coin flip, telephone, delegation, force, vault, king, reentrancy, gatekeeper, etc.

---

## Paradigm CTF (`paradigm-operations/paradigm-ctf-2021` / `blocksec-ctfs`)

- **Focus**: Extremely advanced multi-protocol DeFi and custom execution layer challenge suites. Covers MEV, sandbox escape, compiler flaws, and complex cross-contract economic manipulations.
