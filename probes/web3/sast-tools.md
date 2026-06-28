# EVM SAST and Analysis Tools

## Static Analysis Tools (SAST)

### Slither (`crytic/slither`)
- **Action**: Detects common Solidity vulnerabilities, computes code metrics, and generates contract visual graphs.
- **Vulnerabilities detected**: Reentrancy, uninitialized state variables, shadowing, locked ether, missing events.

### Mythril (`ConsenSys/mythril`)
- **Action**: Security analysis tool for EVM bytecode using symbolic execution, SMT solving, and taint analysis.
- **Vulnerabilities detected**: Integer overflows, reentrancy, default visibility, transaction order dependency.

### Manticore (`trailofbits/manticore`)
- **Action**: Symbolic execution tool for smart contracts and binaries. Can systematically explore execution paths and generate exploit inputs.

### MythX
- **Action**: Security analysis service for Solidity smart contracts, integrating static analysis, symbolic execution, and fuzzing into a unified cloud SaaS pipeline.

### Securify2
- **Action**: Static analyzer for Solidity smart contracts. Uses pattern matching and semantic reachability rules.

---

## Code Visualization and Metrics

### Surya (`ConsenSys/surya`)
- **Action**: Generates visual representations of smart contract structure, including call graphs, inheritance graphs, and control flow graphs.
- **Usage**: Used to map call trees and discover external call interfaces during audit target modeling.

### Solgraph
- **Action**: Visualizes smart contract control flow by generating a DOT graph displaying function execution flows and security levels.

### Solidity Visual Developer
- **Action**: VS Code extension providing inline metrics, call-graph overlays, inheritance views, and signature checks.
