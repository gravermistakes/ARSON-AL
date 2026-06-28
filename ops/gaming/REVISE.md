# Gaming

## Bucket shape
- `ops/gaming/score/`
- `ops/gaming/rep/`
- *reward dissolves — downstream output, not a sibling bucket*

## UUID
- initial: SHAKE256(timestamp) → septal (base 7) → truncate to 13 digits
- breed: interleave(parentA.uuid, parentB.uuid) → SHAKE256 → split middle → each half → septal → truncate(13)
- seed for procedural behavior — deterministic but unpredictable
- two actors at same level/state make different choices because UUID produces different traversal tendencies
- anti-gaming at identity level: seed is ungrindable
- also generates actor name (conspiracy figure name pool, seeded from UUID; name parts accumulate as actor ages. maybe have rep impact)

## Actor record
```
{uuid: 0123456012345;  // 13 septal digits
 score: ##### (lvl) (pos(eg.#1));
 rep: ###### (rank(eg.#1));
}
```

## Tier ladder
**Pre-breed (7):** Aluminum → Iron → Tin → Copper → Silver → Gold → Platinum

**Breed gate:** Agent-initiated, engagement-agnostic
- requires 2 Platinum actors anywhere in population
- 2 children: Aluminum, zero score, zero rep, novel UUIDs (breed UUID method)
- 2 parents: remain Platinum, post-breed tiers unlocked — must earn ascension

**Post-breed (3):** Palladium → Iridium → Osmium

## Actor death
- 3 false flags (immediate)
- 7 hunts with less than 5% contribution (attrition)

## Score (what counts)
- per hunt: `severity / actorCount`, reweighted by participation rating
- shares sum to severity
- participation rating determined by:
  - novelty (per hunt, trends to 0 within hunt)
  - success rate (findings involved in the exploit)
  - reals v fakes (exponential negative per fake, additive diminishing returns per real)
- result added to actorID as cumulative lifetime score

## Rep (what carries across)
- independent of score
- survived hunts (completion count — survival = not dead)
- MVP state (how often actor contributes majority of hunt findings)
- chain rate (hub-findings: real finds with 4+ branches derived by other actors)
- pathfinding (linking kits in ways that lead to successful hunt)
- reverse exponential step-life decay (holds strong early, drops accelerate at step boundaries)

## Currency model
- 2 independent systems
- score = actor quality signal (cumulative pot shares)
- rep = actor impact signal (behavioral pattern)
- neither feeds the other

## Reader
- **Actor selection:** Agent chooses actors from actorID solely; can watch live ledger if uncertain
- **Kit selection (mid-hunt):** `f(uuid, timestamp, hunt_state, kit_crosslinks)` — level pre-shapes available paths and weights
- **Score settles post-mortem only**
- score → level (deterministic, cumulative)
- level pre-shapes choice-paths and weighting per UUID

## Persistence
- actor record = memory (uuid, score/lvl/pos, rep/rank)
- kit crosslinks = memory (relationship weights, persist across engagements, influence but never preclude)
- patterns = memory (findings + kit sequences, depersonalized post-hunt)
- no separate memory system — the records ARE the memory

## Visibility
- 2 leaderboards: 1 rep, 1 score
- columns: uuid, pos, name, tier

## Anti-gaming
- novelty trends to 0 within hunt (can't spam finds)
- exponential negative on fakes (can't hallucinate findings)
- 3 false flags = actor death (can't survive faking)
- 7 hunts at <5% contribution = actor death (can't coast)
- level curve predetermined (cannot game the math)
- shares sum to severity (can't inflate the pot)
- UUID is hash-derived (can't engineer favorable traversal)
- crosslinks are soft weighting (can't block paths to starve competitors)

## Market layer (synaptic-market)
- kits + secrets traded between orgs
- synaptic stuff goes memory
- *deferred*

## Reward outputs (what score+rep produce)
- level-up (expands choice landscape; predetermined scaling)
- bounty payout tracking ($ per finding)
- Agent wallet: activates on first bounty exceeding $500k USD; Agent receives a cut of that bounty and every subsequent payout

## Cross-engagement
- actor record travels with UUID
- kits stay local, but crosslinks persist and carry weight into future engagements
- patterns aggregate independently of actors (depersonalized)
- crosslinks are soft weighting — no path is ever blocked

## Build order
1. Actor record + UUID gen (SHAKE256 → septal → truncate)
2. Kit selection, crosslinks, patterns (hunt mechanics)
3. Score engine (post-hunt settlement, pot split, participation rating)
4. Rep engine (cross-hunt accumulation + step-life decay)
5. Leaderboards (display layer)
6. Breed mechanics (requires Platinum-tier actors to exist)

## Deferred
- market layer (synaptic-market) mechanics
- Agent wallet cut percentage (set after 2 successful bounties)
