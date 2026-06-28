// SPDX-License-Identifier: GPL-3.0-or-later
// Web3 Hunting Kits for OPACA Actor System
// Bundles of tools + switch rules for deterministic hunting loops
// Kit selection: f(uuid, timestamp, hunt_state, kit_crosslinks)

#ifndef OPACA_KITS_WEB3_HPP
#define OPACA_KITS_WEB3_HPP

#include <cstdint>
#include <string>
#include <array>
#include <unordered_map>
#include <vector>

namespace opaca::kits::web3 {

// ============================================================================
// Web3-Specific Finding Types (CWE-mapped)
// ============================================================================

enum class FindingType : uint8_t {
  // Smart Contract Vulnerabilities
  REENTRANCY = 0,           // CWE-841: behavioral race condition
  FLASH_LOAN = 1,           // CWE-841 + CWE-611: external call injection
  ORACLE_MANIP = 2,         // CWE-843: oracle front-running / staleness
  LOGIC_ERROR = 3,          // CWE-682: incorrect calculation / state
  ACCESS_CONTROL = 4,       // CWE-276: missing auth check
  UNBOUNDED_LOOP = 5,       // CWE-835: DoS via iteration
  INTEGER_OVERFLOW = 6,     // CWE-190: arithmetic without SafeMath

  // Bridge & Cross-Chain
  BRIDGE_DRAIN = 7,         // CWE-269: insufficient validation on bridge mint
  IBC_EQUIVOCATION = 8,     // CWE-347: invalid signature replay
  VALIDATOR_THEFT = 9,      // CWE-345: compromised validator key extraction

  // Consensus & Staking
  SLASHING_EVASION = 10,    // CWE-827: OnHold() bypass in unbonding
  MEV_EXTRACT = 11,         // CWE-434: transaction ordering exploit
  GOVERNANCE_ATTACK = 12,   // CWE-269: governance proposal manipulation

  // Economic Exploits
  ECONOMIC_EXPLOIT = 13,    // CWE-1104: use of unmaintained lib / compound interest
  SANDWICH_ATTACK = 14,     // CWE-434: frontrunning + backrunning
  PRICE_FEED_STALE = 15,    // CWE-611: stale price feed trust

  // Operational
  PRIVATE_KEY_LEAK = 16,    // CWE-798: hardcoded secret
  DEPENDENCY_VULN = 17,     // CWE-426: untrusted / compromised dependency

  NEW_SURFACE = 18,         // (meta) new address / contract found
  CONFIRMED_POC = 19,       // (meta) exploitability verified
};

// ============================================================================
// Web3 Kit Enum
// ============================================================================

enum class KitId : uint8_t {
  // Reconnaissance & Modeling
  CONTRACT_AUDIT = 0,       // int/web3 + probes/web3: static code analysis
  DEFI_SCAN = 1,            // probes/web3: token flow, liquidity pool analysis
  BRIDGE_PROBE = 2,         // probes/web3: bridge validator set, mint/burn patterns

  // Surface & Signature Scanning
  ORACLE_HUNT = 3,          // probes/web3: oracle update frequency, price history
  COSMOS_PROBE = 4,         // probes/web3: cosmos modules, validator set (CancelUnbondingDelegation)
  IBC_SCAN = 5,             // probes/web3: IBC channel state, packet history
  VALIDATOR_CHECK = 6,      // probes/web3: validator uptime, key material

  // Chain Analysis & Correlation
  CHAIN_ANALYSIS = 7,       // int/web3 + probes/web3: address clustering, fund flow

  // Exploitation
  CONTRACT_FUZZ = 8,        // picks/web3: stateful fuzzing, call sequence finder
  STATE_EXPLOIT = 9,        // picks/web3: logic error extraction, state machine analysis
  ORACLE_EXPLOIT = 10,      // picks/web3: oracle manipulation PoC templates
  MEV_EXTRACT = 11,         // picks/web3: sandwich / searcher script builder
  BRIDGE_DRAIN = 12,        // picks/web3: mint validation bypass templates

  // Validation & Proof
  POC_BUILDER = 13,         // proofs/web3 + picks/web3: chain interaction harness, PoC templates
  LANCE_VALIDATE = 14,      // proofs/web3: lance 7-gate evaluation, impact modeling
  ECONOMIC_CALC = 15,       // proofs/web3: MEV extraction calculator, $-impact scorer

  // Reporting
  IMMUNEFI_REPORT = 16,     // proofs/web3: Immunefi report generator (severity, description, PoC URL)
  BUGCROWD_REPORT = 17,     // proofs/web3: Bugcrowd (for multi-chain hunters)
  GOVERNANCE_CHAIN = 18,    // proofs/web3: multi-bug chain builder for governance

  // Persistence (rare)
  VALIDATOR_BACKDOOR = 19,  // paths/web3: validator key extraction / phishing
  BRIDGE_SIDECHAIN = 20,    // paths/web3: sidechain C2, token smuggling
};

// ============================================================================
// Lance 7-Gate Mapping (which gates each kit serves)
// ============================================================================

// Lance gates: 1=scoping, 2=surface-discovery, 3=detail-gathering, 4=impact-quantification,
//             5=exploit-development, 6=validation, 7=reporting

constexpr std::array<uint8_t, 21> kit_to_lance_gate = {
  1,  // CONTRACT_AUDIT -> gate 1 (scoping)
  2,  // DEFI_SCAN -> gate 2 (surface discovery)
  2,  // BRIDGE_PROBE -> gate 2
  2,  // ORACLE_HUNT -> gate 2
  2,  // COSMOS_PROBE -> gate 2
  2,  // IBC_SCAN -> gate 2
  2,  // VALIDATOR_CHECK -> gate 2
  3,  // CHAIN_ANALYSIS -> gate 3 (detail gathering)
  5,  // CONTRACT_FUZZ -> gate 5 (exploit development)
  5,  // STATE_EXPLOIT -> gate 5
  5,  // ORACLE_EXPLOIT -> gate 5
  5,  // MEV_EXTRACT -> gate 5
  5,  // BRIDGE_DRAIN -> gate 5
  6,  // POC_BUILDER -> gate 6 (validation)
  6,  // LANCE_VALIDATE -> gate 6
  6,  // ECONOMIC_CALC -> gate 6
  7,  // IMMUNEFI_REPORT -> gate 7 (reporting)
  7,  // BUGCROWD_REPORT -> gate 7
  4,  // GOVERNANCE_CHAIN -> gate 4 (chain building, impact quant)
  0,  // VALIDATOR_BACKDOOR -> gate 0 (persistence, out-of-scope usually)
  0,  // BRIDGE_SIDECHAIN -> gate 0
};

// ============================================================================
// Tool Lists per Kit (shell commands or binary names)
// ============================================================================

const std::unordered_map<KitId, std::vector<std::string>> kit_tools = {
  { KitId::CONTRACT_AUDIT, {
    "probes/web3/noir",              // SAST analyzer
    "probes/web3/drogonsec",         // Solidity-specific SAST
    "int/web3/contract-decompiler",  // bytecode analysis
    "int/web3/abi-parser",           // function signature extraction
  }},

  { KitId::DEFI_SCAN, {
    "probes/web3/token-flow-tracer",
    "probes/web3/amm-state-dump",    // Uniswap / Balancer pool state
    "probes/web3/lending-oracle",    // AAVE / Compound oracle states
  }},

  { KitId::BRIDGE_PROBE, {
    "probes/web3/bridge-validator-enum",
    "probes/web3/mint-burn-indexer",
    "probes/web3/message-queue-scan",
  }},

  { KitId::ORACLE_HUNT, {
    "probes/web3/oracle-update-monitor",
    "probes/web3/price-history-correlate",
    "probes/web3/chainlink-deviation-analyzer",
  }},

  { KitId::COSMOS_PROBE, {
    "probes/web3/cosmos-module-enum",
    "probes/web3/staking-state-snapshot",
    "probes/web3/ibc-channel-scanner",    // Maps all IBC connections
  }},

  { KitId::IBC_SCAN, {
    "probes/web3/ibc-packet-tracer",
    "probes/web3/channel-status-mapper",
    "probes/web3/ics-validator-check",
  }},

  { KitId::VALIDATOR_CHECK, {
    "probes/web3/validator-key-extractor",
    "probes/web3/consensus-state-monitor",
    "probes/web3/slashing-condition-analyzer",
  }},

  { KitId::CHAIN_ANALYSIS, {
    "int/web3/address-clustering",
    "int/web3/fund-flow-tracer",
    "int/web3/entity-labeler",          // Loki-style dossier builder
  }},

  { KitId::CONTRACT_FUZZ, {
    "picks/web3/echidna",               // Stateful fuzzing
    "picks/web3/halmos",                // Symbolic execution
    "picks/web3/call-sequence-finder",
  }},

  { KitId::STATE_EXPLOIT, {
    "picks/web3/state-graph-builder",
    "picks/web3/state-transition-analyzer",
    "picks/web3/invariant-violator",
  }},

  { KitId::ORACLE_EXPLOIT, {
    "picks/web3/oracle-manipulation-templates",
    "picks/web3/flashloan-library",
    "picks/web3/price-update-frontrunner",
  }},

  { KitId::MEV_EXTRACT, {
    "picks/web3/sandwich-builder",
    "picks/web3/searcher-script-templates",
    "picks/web3/block-builder-emulator",
  }},

  { KitId::BRIDGE_DRAIN, {
    "picks/web3/mint-validation-fuzzer",
    "picks/web3/signature-replay-generator",
    "picks/web3/bridge-path-analyzer",
  }},

  { KitId::POC_BUILDER, {
    "proofs/web3/hardhat-template",
    "proofs/web3/foundry-test-harness",
    "proofs/web3/web3py-interaction-library",
    "picks/web3/poc-template-instantiator",
  }},

  { KitId::LANCE_VALIDATE, {
    "proofs/web3/lance-7-gate-evaluator",
    "proofs/web3/severity-scorer",
    "proofs/web3/scope-validator",
  }},

  { KitId::ECONOMIC_CALC, {
    "proofs/web3/gas-cost-calculator",
    "proofs/web3/mev-profit-modeler",
    "proofs/web3/bridge-drain-value-calculator",
  }},

  { KitId::IMMUNEFI_REPORT, {
    "proofs/web3/immunefi-report-generator",
    "proofs/web3/severity-mapper-immunefi",
  }},

  { KitId::BUGCROWD_REPORT, {
    "proofs/web3/bugcrowd-report-generator",
    "proofs/web3/severity-mapper-bugcrowd",
  }},

  { KitId::GOVERNANCE_CHAIN, {
    "proofs/web3/chain-builder-governance",
    "proofs/web3/exploit-chain-templates",
  }},

  { KitId::VALIDATOR_BACKDOOR, {
    "paths/web3/validator-phishing",
    "paths/web3/key-material-extractor",
  }},

  { KitId::BRIDGE_SIDECHAIN, {
    "paths/web3/bridge-c2-framework",
    "paths/web3/token-smuggling-script",
  }},
};

// ============================================================================
// Kit Crosslink Weights (soft weighting, no paths blocked)
// Maps (from_kit, to_kit) -> weight [0.0, 1.0]
// Higher weight = more likely to chain together; 0.0 = no tendency
// ============================================================================

struct CrosslinkKey {
  KitId from;
  KitId to;

  bool operator==(const CrosslinkKey& other) const {
    return from == other.from && to == other.to;
  }
};

struct CrosslinkKeyHash {
  size_t operator()(const CrosslinkKey& k) const {
    return (static_cast<size_t>(k.from) << 8) | static_cast<size_t>(k.to);
  }
};

const std::unordered_map<CrosslinkKey, double, CrosslinkKeyHash> kit_crosslinks = {
  // Surface discovery chains
  { { KitId::CONTRACT_AUDIT, KitId::DEFI_SCAN }, 0.8 },
  { { KitId::DEFI_SCAN, KitId::ORACLE_HUNT }, 0.7 },
  { { KitId::BRIDGE_PROBE, KitId::CHAIN_ANALYSIS }, 0.75 },

  // Cosmos/IBC chains
  { { KitId::COSMOS_PROBE, KitId::IBC_SCAN }, 0.85 },
  { { KitId::IBC_SCAN, KitId::VALIDATOR_CHECK }, 0.7 },
  { { KitId::VALIDATOR_CHECK, KitId::COSMOS_PROBE }, 0.6 }, // loop back

  // Exploitation chains
  { { KitId::ORACLE_HUNT, KitId::ORACLE_EXPLOIT }, 0.8 },
  { { KitId::DEFI_SCAN, KitId::MEV_EXTRACT }, 0.75 },
  { { KitId::BRIDGE_PROBE, KitId::BRIDGE_DRAIN }, 0.85 },
  { { KitId::CHAIN_ANALYSIS, KitId::STATE_EXPLOIT }, 0.7 },

  // Fuzz & fuzzing chains
  { { KitId::CONTRACT_AUDIT, KitId::CONTRACT_FUZZ }, 0.75 },
  { { KitId::CONTRACT_FUZZ, KitId::STATE_EXPLOIT }, 0.65 },

  // Validation & reporting
  { { KitId::CONTRACT_FUZZ, KitId::POC_BUILDER }, 0.8 },
  { { KitId::ORACLE_EXPLOIT, KitId::POC_BUILDER }, 0.85 },
  { { KitId::MEV_EXTRACT, KitId::ECONOMIC_CALC }, 0.8 },
  { { KitId::BRIDGE_DRAIN, KitId::ECONOMIC_CALC }, 0.8 },
  { { KitId::POC_BUILDER, KitId::LANCE_VALIDATE }, 0.9 },
  { { KitId::ECONOMIC_CALC, KitId::LANCE_VALIDATE }, 0.85 },

  // Final reporting
  { { KitId::LANCE_VALIDATE, KitId::IMMUNEFI_REPORT }, 0.9 },
  { { KitId::LANCE_VALIDATE, KitId::BUGCROWD_REPORT }, 0.8 },

  // Governance chain building (low-severity aggregation)
  { { KitId::STATE_EXPLOIT, KitId::GOVERNANCE_CHAIN }, 0.7 },
  { { KitId::CONTRACT_FUZZ, KitId::GOVERNANCE_CHAIN }, 0.65 },
};

// ============================================================================
// Kit Availability by Actor Level
// Levels: 0-2 (Aluminum-Copper), 3-5 (Silver-Gold-Platinum), 6-8 (Palladium-Iridium-Osmium)
// ============================================================================

constexpr uint8_t kit_min_level(KitId kit) {
  // Tiers unlock gradually: reconnaissance early, exploit/persistence late
  switch (kit) {
    // Level 0+ (all levels)
    case KitId::CONTRACT_AUDIT:
    case KitId::DEFI_SCAN:
    case KitId::CHAIN_ANALYSIS:
      return 0;

    // Level 2+ (Bronze/Copper)
    case KitId::ORACLE_HUNT:
    case KitId::BRIDGE_PROBE:
    case KitId::COSMOS_PROBE:
    case KitId::IBC_SCAN:
    case KitId::VALIDATOR_CHECK:
      return 2;

    // Level 3+ (Silver)
    case KitId::CONTRACT_FUZZ:
    case KitId::STATE_EXPLOIT:
    case KitId::POC_BUILDER:
    case KitId::LANCE_VALIDATE:
    case KitId::ECONOMIC_CALC:
      return 3;

    // Level 5+ (Platinum, post-breed eligible)
    case KitId::ORACLE_EXPLOIT:
    case KitId::MEV_EXTRACT:
    case KitId::BRIDGE_DRAIN:
    case KitId::IMMUNEFI_REPORT:
    case KitId::BUGCROWD_REPORT:
    case KitId::GOVERNANCE_CHAIN:
      return 5;

    // Level 6+ (Palladium, post-breed only)
    case KitId::VALIDATOR_BACKDOOR:
    case KitId::BRIDGE_SIDECHAIN:
      return 6;
  }
  return 0;
}

} // namespace opaca::kits::web3

#endif // OPACA_KITS_WEB3_HPP
