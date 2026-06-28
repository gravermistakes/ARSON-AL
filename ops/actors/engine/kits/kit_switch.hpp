// SPDX-License-Identifier: GPL-3.0-or-later
// Deterministic Kit Selection for OPACA Actors
// Pure function: f(uuid, timestamp, hunt_state, crosslinks) -> KitId
// No LLM, no side effects, C++20 constexpr where possible

#ifndef OPACA_KITS_KIT_SWITCH_HPP
#define OPACA_KITS_KIT_SWITCH_HPP

#include <cstdint>
#include <string>
#include <string_view>
#include <array>
#include <algorithm>
#include <cmath>
#include "web3.hpp"

namespace opaca::kits {

// ============================================================================
// Hunt State — immutable snapshot passed to kit selection
// ============================================================================

struct HuntState {
  // Findings accumulated in this hunt
  uint32_t finding_count = 0;
  uint32_t severity_p1_count = 0;  // Critical (90-100 CVSS)
  uint32_t severity_p2_count = 0;  // High (70-89)
  uint32_t severity_p3_count = 0;  // Medium (50-69)
  uint32_t severity_p4_count = 0;  // Low (0-49)

  // Most recent finding type
  web3::FindingType last_finding_type = web3::FindingType::NEW_SURFACE;
  uint64_t last_finding_timestamp = 0;

  // Kit history in this hunt (UUID bits used to seed exploration)
  web3::KitId current_kit = web3::KitId::CONTRACT_AUDIT;
  uint8_t kit_switches_in_hunt = 0;

  // Engagement phase (0=recon, 1=scan, 2=exploit, 3=validate, 4=report)
  uint8_t engagement_phase = 0;

  // Time since hunt start (seconds)
  uint64_t hunt_elapsed_sec = 0;

  // Actor level (0-8, from REVISE.md tier ladder)
  uint8_t actor_level = 0;

  // Hunt scope: 0=web2, 1=web3, 2=cosmos/ibc, 3=multichain
  uint8_t scope_type = 1;
};

// ============================================================================
// Crosslink Weights Snapshot (for this hunt)
// ============================================================================

struct CrosslinkSnapshot {
  const std::unordered_map<web3::CrosslinkKey, double, web3::CrosslinkKeyHash>* weights;
  double default_weight = 0.3;
};

// ============================================================================
// UUID-Based Seeding (deterministic but unpredictable)
// Extract bits from the 13-digit septal UUID to influence kit selection
// ============================================================================

// Extract a specific bit range from UUID (seeded traversal bias)
constexpr uint8_t uuid_bit_range(std::string_view uuid_str, uint8_t start_bit, uint8_t bits) {
  // Convert septal digits to a numerical base (simplified: treat as base-10)
  uint64_t uuid_num = 0;
  for (char c : uuid_str) {
    if (c >= '0' && c <= '6') {  // Septal: 0-6
      uuid_num = uuid_num * 7 + (c - '0');
    }
  }
  // Extract bit range
  uint8_t mask = (1u << bits) - 1;
  return static_cast<uint8_t>((uuid_num >> start_bit) & mask);
}

// Interpret UUID bits as a kit preference offset (0-20, biasing kit selection)
constexpr web3::KitId uuid_preferred_kit(std::string_view uuid_str, uint8_t actor_level) {
  uint8_t offset = uuid_bit_range(uuid_str, 0, 5);  // 5 bits -> 0-31
  // Constrain to available kits (0-20)
  offset = offset % 21;

  // Level gating: only return kits this actor can use
  if (web3::kit_min_level(static_cast<web3::KitId>(offset)) <= actor_level) {
    return static_cast<web3::KitId>(offset);
  }
  // Fallback: return first available kit for this level
  for (uint8_t i = 0; i <= 20; ++i) {
    auto kit = static_cast<web3::KitId>(i);
    if (web3::kit_min_level(kit) <= actor_level) {
      return kit;
    }
  }
  return web3::KitId::CONTRACT_AUDIT;  // Last resort
}

// ============================================================================
// Core Kit Selection Algorithm
// ============================================================================

// Score a candidate kit: (1) current fit (2) crosslink boost (3) UUID bias
inline double score_kit(
  web3::KitId candidate,
  const HuntState& state,
  const CrosslinkSnapshot& xlinks,
  std::string_view uuid_str,
  uint64_t timestamp
) {
  double score = 1.0;

  // 1. PHASE ALIGNMENT — what phase are we in?
  // Phase 0 (recon): prefer audit, scan, probe kits
  // Phase 1 (scan): prefer oracle/bridge/cosmos probes
  // Phase 2 (exploit): prefer fuzz, oracle-exploit, mev-extract, bridge-drain
  // Phase 3 (validate): prefer poc-builder, lance-validate
  // Phase 4 (report): prefer immunefi/bugcrowd/governance report generators

  switch (state.engagement_phase) {
    case 0: // Recon
      if (candidate == web3::KitId::CONTRACT_AUDIT ||
          candidate == web3::KitId::DEFI_SCAN ||
          candidate == web3::KitId::CHAIN_ANALYSIS) {
        score *= 2.0;
      }
      break;

    case 1: // Scan
      if (candidate == web3::KitId::ORACLE_HUNT ||
          candidate == web3::KitId::BRIDGE_PROBE ||
          candidate == web3::KitId::COSMOS_PROBE ||
          candidate == web3::KitId::IBC_SCAN ||
          candidate == web3::KitId::VALIDATOR_CHECK) {
        score *= 2.0;
      }
      break;

    case 2: // Exploit
      if (candidate == web3::KitId::CONTRACT_FUZZ ||
          candidate == web3::KitId::STATE_EXPLOIT ||
          candidate == web3::KitId::ORACLE_EXPLOIT ||
          candidate == web3::KitId::MEV_EXTRACT ||
          candidate == web3::KitId::BRIDGE_DRAIN) {
        score *= 2.0;
      }
      break;

    case 3: // Validate
      if (candidate == web3::KitId::POC_BUILDER ||
          candidate == web3::KitId::LANCE_VALIDATE ||
          candidate == web3::KitId::ECONOMIC_CALC) {
        score *= 2.0;
      }
      break;

    case 4: // Report
      if (candidate == web3::KitId::IMMUNEFI_REPORT ||
          candidate == web3::KitId::BUGCROWD_REPORT ||
          candidate == web3::KitId::GOVERNANCE_CHAIN) {
        score *= 2.5;
      }
      break;
  }

  // 2. CROSSLINK BOOST — was the current kit productive?
  // If we're in the middle of a hunt, look at crosslink weight from current -> candidate
  if (state.current_kit != candidate && xlinks.weights) {
    web3::CrosslinkKey link = { state.current_kit, candidate };
    auto it = xlinks.weights->find(link);
    double xlink_weight = (it != xlinks.weights->end()) ? it->second : xlinks.default_weight;
    score *= (1.0 + xlink_weight * 1.5);  // Boost by crosslink strength
  }

  // 3. FINDING-TYPE DRIVEN SWITCH
  // Certain findings mandate kit switches (hard gate)
  bool finding_matches = false;
  switch (state.last_finding_type) {
    case web3::FindingType::ORACLE_MANIP:
    case web3::FindingType::PRICE_FEED_STALE:
      if (candidate == web3::KitId::ORACLE_EXPLOIT) finding_matches = true;
      break;

    case web3::FindingType::FLASH_LOAN:
    case web3::FindingType::REENTRANCY:
      if (candidate == web3::KitId::CONTRACT_FUZZ ||
          candidate == web3::KitId::STATE_EXPLOIT) finding_matches = true;
      break;

    case web3::FindingType::BRIDGE_DRAIN:
      if (candidate == web3::KitId::BRIDGE_DRAIN) finding_matches = true;
      break;

    case web3::FindingType::SLASHING_EVASION:
    case web3::FindingType::MEV_EXTRACT:
      if (candidate == web3::KitId::MEV_EXTRACT) finding_matches = true;
      break;

    case web3::FindingType::GOVERNANCE_ATTACK:
      if (candidate == web3::KitId::GOVERNANCE_CHAIN) finding_matches = true;
      break;

    case web3::FindingType::CONFIRMED_POC:
      if (candidate == web3::KitId::IMMUNEFI_REPORT ||
          candidate == web3::KitId::BUGCROWD_REPORT) finding_matches = true;
      break;

    default:
      break;
  }
  if (finding_matches) score *= 3.0;  // Hard boost for finding-driven switches

  // 4. LEVEL GATING
  // If actor is below min level for kit, hard penalty (but not blocked)
  if (web3::kit_min_level(candidate) > state.actor_level) {
    score *= 0.1;  // 90% penalty; path open, just unlikely
  }

  // 5. UUID BIAS (deterministic exploration)
  // Compare candidate to UUID-preferred kit; if they match, small boost
  auto preferred = uuid_preferred_kit(uuid_str, state.actor_level);
  if (candidate == preferred) {
    score *= 1.2;  // 20% boost for UUID-aligned choice
  }

  // 6. TIME-BASED SATURATION
  // If we've been in one kit for >30 minutes, prefer switching
  // (encourages exploration; timestamp is unix epoch in seconds)
  uint64_t kit_age_sec = (timestamp > state.last_finding_timestamp) ?
    (timestamp - state.last_finding_timestamp) : 0;
  if (kit_age_sec > 1800 && candidate != state.current_kit) {
    score *= 1.3;
  }

  return score;
}

// Main kit selection function (pure, no side effects)
inline web3::KitId select_kit(
  std::string_view uuid,
  uint64_t timestamp,
  const HuntState& state,
  const CrosslinkSnapshot& xlinks
) {
  // Candidate pool: all kits that actor level allows
  std::array<web3::KitId, 21> candidates;
  uint8_t candidate_count = 0;

  for (uint8_t i = 0; i <= 20; ++i) {
    auto kit = static_cast<web3::KitId>(i);
    if (web3::kit_min_level(kit) <= state.actor_level) {
      candidates[candidate_count++] = kit;
    }
  }

  // If no candidates at this level, return current kit
  if (candidate_count == 0) {
    return state.current_kit;
  }

  // Score all candidates
  std::array<double, 21> scores = {};
  for (uint8_t i = 0; i < candidate_count; ++i) {
    scores[i] = score_kit(candidates[i], state, xlinks, uuid, timestamp);
  }

  // Find max-scoring kit (ties broken by kit ID for determinism)
  uint8_t best_idx = 0;
  double best_score = scores[0];
  for (uint8_t i = 1; i < candidate_count; ++i) {
    if (scores[i] > best_score ||
        (scores[i] == best_score && candidates[i] < candidates[best_idx])) {
      best_score = scores[i];
      best_idx = i;
    }
  }

  return candidates[best_idx];
}

} // namespace opaca::kits

#endif // OPACA_KITS_KIT_SWITCH_HPP
