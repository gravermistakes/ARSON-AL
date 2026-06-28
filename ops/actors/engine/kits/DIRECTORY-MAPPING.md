# Web3 Kit Directory Mapping

Maps OPACA kits to dissolved ARSON-AL component directories (int/, probes/, picks/, proofs/, paths/).
Every tool in a kit comes from one of these directories; kit naming reflects action, not origin.

## Mapping Table

| Kit | Lance Gate | Primary Source | Secondary | Purpose |
|-----|-----------|-----------------|-----------|---------|
| **CONTRACT_AUDIT** | 1 (Scope) | `int/web3/` + `probes/web3/noir` | `int/web3/contract-decompiler` | Static analysis, bytecode parsing |
| **DEFI_SCAN** | 2 (Surface) | `probes/web3/` | `int/web3/` | Token flow, AMM pool state, oracle state |
| **BRIDGE_PROBE** | 2 (Surface) | `probes/web3/` | `proofs/web3/` | Validator enumeration, mint/burn indexing |
| **ORACLE_HUNT** | 2 (Surface) | `probes/web3/` | (none) | Price update frequency, deviation analysis |
| **COSMOS_PROBE** | 2 (Surface) | `probes/web3/cosmos/` | `int/web3/` | Module enumeration, staking state (OnHold bypass detection) |
| **IBC_SCAN** | 2 (Surface) | `probes/web3/ibc/` | (none) | Packet tracing, channel status, ICS validation |
| **VALIDATOR_CHECK** | 2 (Surface) | `probes/web3/` | (none) | Key extraction, consensus monitoring, slashing conditions |
| **CHAIN_ANALYSIS** | 3 (Detail) | `int/web3/` | `probes/web3/` | Address clustering, fund flow, entity labeling |
| **CONTRACT_FUZZ** | 5 (Exploit) | `picks/web3/` | `probes/web3/` | Echidna, Halmos, call sequence discovery |
| **STATE_EXPLOIT** | 5 (Exploit) | `picks/web3/` | (none) | State graph builder, transition analyzer, invariant violator |
| **ORACLE_EXPLOIT** | 5 (Exploit) | `picks/web3/` | (none) | Oracle manipulation templates, flashloan library |
| **MEV_EXTRACT** | 5 (Exploit) | `picks/web3/` | (none) | Sandwich builder, searcher scripts, block builder emulator |
| **BRIDGE_DRAIN** | 5 (Exploit) | `picks/web3/` | (none) | Mint validation fuzzer, signature replay, bridge path analysis |
| **POC_BUILDER** | 6 (Validate) | `proofs/web3/` + `picks/web3/` | (none) | Hardhat, Foundry, Web3.py harnesses + template instantiation |
| **LANCE_VALIDATE** | 6 (Validate) | `proofs/web3/` | (none) | 7-gate evaluator, severity scorer, scope validator |
| **ECONOMIC_CALC** | 6 (Validate) | `proofs/web3/` | (none) | Gas cost, MEV profit, bridge drain $-value calculation |
| **IMMUNEFI_REPORT** | 7 (Report) | `proofs/web3/` | (none) | Severity mapper for Immunefi bounty submission |
| **BUGCROWD_REPORT** | 7 (Report) | `proofs/web3/` | (none) | Severity mapper for Bugcrowd (multi-chain hunters) |
| **GOVERNANCE_CHAIN** | 4 (Chain) | `proofs/web3/` | (none) | Multi-bug chain builder, exploit chain templates |
| **VALIDATOR_BACKDOOR** | 0 (Persistence) | `paths/web3/` | (none) | Validator phishing, key extraction (rare, high-level actors) |
| **BRIDGE_SIDECHAIN** | 0 (Persistence) | `paths/web3/` | (none) | Bridge C2, token smuggling (rare, high-level actors) |

## Directory Inventory

### `int/web3/` — Target Modeling & Threat Diagrams
- contract-decompiler
- abi-parser
- address-clustering
- fund-flow-tracer
- entity-labeler
- cosmos-module-enum (Cosmos ecosystem mapping)

### `probes/web3/` — Surface Discovery & Scanning
**Smart Contracts:**
- noir (SAST, general)
- drogonsec (Solidity-specific SAST)
- token-flow-tracer
- amm-state-dump (Uniswap / Balancer)
- lending-oracle (AAVE / Compound oracle snapshot)

**Bridges:**
- bridge-validator-enum
- mint-burn-indexer
- message-queue-scan

**Oracles:**
- oracle-update-monitor
- price-history-correlate
- chainlink-deviation-analyzer

**Cosmos/IBC:**
- cosmos-module-enum
- staking-state-snapshot
- ibc-channel-scanner
- ibc-packet-tracer
- channel-status-mapper
- ics-validator-check
- validator-key-extractor
- consensus-state-monitor
- slashing-condition-analyzer

### `picks/web3/` — Fuzzing, Exploitation, Extraction
**Contract Fuzzing:**
- echidna (stateful fuzzing)
- halmos (symbolic execution)
- call-sequence-finder

**State Exploitation:**
- state-graph-builder
- state-transition-analyzer
- invariant-violator

**Oracle Manipulation:**
- oracle-manipulation-templates
- flashloan-library
- price-update-frontrunner

**MEV Extraction:**
- sandwich-builder
- searcher-script-templates
- block-builder-emulator

**Bridge Exploitation:**
- mint-validation-fuzzer
- signature-replay-generator
- bridge-path-analyzer

### `proofs/web3/` — PoC Templates, Validation, Impact Modeling
**PoC Infrastructure:**
- hardhat-template
- foundry-test-harness
- web3py-interaction-library
- poc-template-instantiator

**Validation & Scoring:**
- lance-7-gate-evaluator (implements all 7 gates)
- severity-scorer (CVSS → Immunefi/Bugcrowd mapping)
- scope-validator
- gas-cost-calculator
- mev-profit-modeler
- bridge-drain-value-calculator

**Report Generation:**
- immunefi-report-generator
- severity-mapper-immunefi
- bugcrowd-report-generator
- severity-mapper-bugcrowd
- chain-builder-governance (braids low-severity into P1s)
- exploit-chain-templates

### `paths/web3/` — Persistence & C2 (Post-Breach, Rare in Bounty)
- validator-phishing (targeted social engineering)
- key-material-extractor
- bridge-c2-framework (token smuggling channel)
- token-smuggling-script

## Lance 7-Gate Flow

Actors progress through gates as they move between kits:

```
Gate 1: Scoping
  ├─ CONTRACT_AUDIT (int/web3 + probes/web3/noir)
  └─ Goal: Understand target, identify surface

Gate 2: Surface Discovery
  ├─ DEFI_SCAN, BRIDGE_PROBE, ORACLE_HUNT, COSMOS_PROBE, IBC_SCAN, VALIDATOR_CHECK
  └─ Goal: Map all exposed services, state

Gate 3: Detail Gathering
  ├─ CHAIN_ANALYSIS (int/web3 fund flow)
  └─ Goal: Correlate addresses, build dossier

Gate 4: Impact Quantification
  ├─ GOVERNANCE_CHAIN (background loop, low-severity braiding)
  └─ Goal: Estimate $-value of findings

Gate 5: Exploit Development
  ├─ CONTRACT_FUZZ, STATE_EXPLOIT, ORACLE_EXPLOIT, MEV_EXTRACT, BRIDGE_DRAIN
  └─ Goal: Build PoC, prove exploitability

Gate 6: Validation
  ├─ POC_BUILDER, LANCE_VALIDATE, ECONOMIC_CALC (proofs/web3)
  └─ Goal: Confirm PoC works, calculate impact

Gate 7: Reporting
  ├─ IMMUNEFI_REPORT, BUGCROWD_REPORT (proofs/web3 generators)
  └─ Goal: Submit to platform with required severity/narrative

## Crosslink Examples (Kit Chaining)

Kit selection is driven by findings + crosslinks. Common chains:

**Recon → Scan:**
```
CONTRACT_AUDIT (gate 1)
  → finding: new_surface (bridge contracts)
  → BRIDGE_PROBE (gate 2, xlink weight 0.75)
  → finding: validator_set_mutable
  → VALIDATOR_CHECK (gate 2, xlink weight 0.7)
```

**Scan → Exploit:**
```
ORACLE_HUNT (gate 2)
  → finding: oracle_manip (stale price detected)
  → ORACLE_EXPLOIT (gate 5, xlink weight 0.8)
  → finding: confirmed_poc (exploit works)
  → POC_BUILDER (gate 6)
```

**Parallel Loops:**
```
Actor A: CONTRACT_FUZZ → findings on reentrancy → STATE_EXPLOIT
Actor B: COSMOS_PROBE → findings on staking → VALIDATOR_CHECK
         (both running concurrently, communicate via Riot message passing)
```

## Dissolution Principle

- **Action determines kit, not origin.** A tool from `picks/` goes into EXPLOIT kit, not SOURCE kit, even if it was originally in a SAST repo.
- **Kits are boundaries.** All tools in a kit share a phase of engagement (scoping, scanning, exploiting, validating, reporting).
- **No tool is duplicated.** Each tool lives in exactly one kit (and one directory).

## Anti-Censorship / Anti-Gaming

- **Crosslinks are soft weights.** No kit is ever fully blocked; low-level actors can attempt high-level kits (with penalty).
- **Finding-driven switches are hard.** If actor finds `SLASHING_EVASION`, it can bypass crosslink weights and jump to MEV_EXTRACT.
- **UUID seeding is ungrindable.** Actor cannot engineer its UUID to prefer high-scoring kits (SHAKE256 hash of timestamp).
- **Rep/score are independent.** Kit selection never cares about score; level-gating only.
