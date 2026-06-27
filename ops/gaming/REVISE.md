# Gaming — revision draft

Bones only. Your calls in your words; my prose stripped. Revise in place.

## Bucket shape
- `ops/gaming/score/`
- `ops/gaming/rep/`
- *reward dissolves — downstream output, not a sibling bucket*

## Score (what counts)
- severity contribution
- chain contribution
- time-to-finding
- novelty / repeat-find decay (trends to 0)
- false-flag penalty (negative)
- tool-crash penalty
- drain cost
- ___

## Rep (what carries across)
- accumulates from score
- time-decayed
- per actor UID
- ___

## Currency model
- 2 factors: **score**, **rep**
- everything else is derived
- ___

## Reader (when is score read)
- **post-mortem only**
- mid-engagement:
  - Agent chooses actors (not kits)
  - actors choose their own kits
  - kit choice = UID preference + stochastic on `(timestamp, score, engagement state)`
- ___

## Persistence
- per actor UID
- the actor IS the record
- no engagement ledger
- no mem sidecar
- ___

## Visibility
- full leaderboard
- ___

## Decay
- time decay on rep
- ___

## Anti-gaming
- repeat finds → 0
- false flags → negative
- level curve predetermined (cannot game the math)
- ___

## Tier system
- adjectives + name portions per rank
- Platinum: 2 platinums coexist → **crossbreed** → new actor
- ___

## Evolution
- `score :: level`
- `evolution :: Platinum (2 actors)` → advanced_evolution hand-off
- ___

## Market layer (synaptic-market)
- kits + secrets traded between orgs (multi-team)
- also a memory aspect (log of inter-org trades = intel)
- ___

## Reward outputs (what score+rep produce)
- level-up (sharpens strategy; predetermined scaling)
- bounty payout tracking ($ per finding)
- Agent wallet unlocks at 500k cabal-cumulative
- ___

## Cross-engagement
- rep + seed travel with actor
- kits do not travel
- patterns aggregate **independently of actors** (de-personalized into mem)
- ___

## Where it lives
- `ops/gaming/score/`
- `ops/gaming/rep/`
- ___

## Build order
1. ___
2. ___
3. ___
4. ___

## Open
- ___
