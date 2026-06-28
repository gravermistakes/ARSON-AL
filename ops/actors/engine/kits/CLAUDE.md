# Kits (OPACA Actor Engine)

## What's here
Web3 hunting kits: tool bundles + switch rules for deterministic OPACA actors.
Actors load kits mid-hunt (no respawn) and switch based on findings + state.
Pure C++ headers; no Python, no LLM per step.

## Build
```bash
# Verify compilation (C++20 required)
clang++ -std=c++20 -Wall -Wextra kits/web3.hpp -fsyntax-only
clang++ -std=c++20 -Wall -Wextra kits/kit_switch.hpp -fsyntax-only
```

## Test
Kits are linked into the OPACA actor runtime; validation via:
- Lance 7-gate integration test (all gates fire, severity maps correctly)
- UUID seeding reproducibility (same UUID + state = same kit choice)
- Crosslink weights consistency (xlinks never block paths, only weight)
- Level gating boundary (actor below min_level gets soft penalty, not hard block)

## Feeds
**Consumes:** hunt_state (finding_count, severity, current_kit, engagement_phase, actor_level),
crosslink_weights (persistent across hunts), UUID (septal-encoded, 13 digits).
**Emits:** selected kit_id (fed to kit runtime loader), kit tools list (spawns probes/picks/proofs binaries).
Loop: actor.perceive → kit_switch.select() → actor.load_kit() → actor.act() → findings update state → repeat.

## Issues
- Crosslink weights are soft; no path ever fully blocked (anti-gaming: can't starve competitors)
- Level gating: exponential penalty (0.1x) below min_level, not a hard gate (allows high-risk high-reward)
- UUID bias: only 5 bits used; 2 actors with similar UUIDs may cluster; acceptable for diversity

<!-- generated: 1719604800 -->
