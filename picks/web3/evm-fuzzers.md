# EVM Fuzzers

## Echidna (`crytic/echidna`)

### Features
- Property-based fuzzer for Solidity smart contracts.
- Written in Haskell; fast execution engine.
- Supports assertion-based fuzzing and custom property function definitions (`echidna_` prefix).
- Real-time UI displaying coverage and failed properties.
- Integrates with Slither for automatic property generation and contract structure analysis.

### Usage
```bash
echidna-test contract.sol --contract MyContract --config config.yaml
```

---

## Foundry Fuzz (built-in Forge Fuzz)

### Features
- Native property-based testing within the Foundry framework.
- Uses Solidity for both contract implementation and fuzz testing.
- Extremely fast; runs in-memory via the EVM implementation in Rust.
- Generates random inputs for test functions with arguments.
- Supports invariant testing (testing a property across a series of random transactions and state changes).

### Usage
```bash
forge test --match-test testFuzz_myProperty
```

---

## ChainFuzz (`ChainSecurity/ChainFuzz`)

### Features
- Transaction-sequence fuzzer targeting consensus-layer contracts.
- Smart generator for sequence of calls to identify complex deep-state bugs (e.g. reentrancy or governance privilege escalation).
- Generates inputs that respect contract invariants.

---

## Harvey

### Features
- Greybox fuzzer for smart contracts developed in collaboration with Maria Christakis (MPI-SWS).
- Uses input prediction and path-exploration heuristics to maximize code coverage.
- Employs deep transaction sequence fuzzing.

---

## sFuzz

### Features
- Adaptive, multi-objective fuzzer for smart contracts.
- Combines search-based testing with feedback-driven fuzzing.
- Optimizes for both code coverage and the detection of specific vulnerability patterns.
