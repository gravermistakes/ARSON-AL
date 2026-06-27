<!-- generated: skeleton — partial; bucket/currency/reader/persistence still pending -->
# Gaming — skeleton

The score + reward layer. Mother folder for spec'ing per the "merge these"
note. Sections fill in from multiple-choice answers; TBD = still pending.

## Bucket shape
**Flat `ops/gaming/`** *(defaulted; redirect if wrong)*. score and reward
collapse into one bucket — they're facets of the same loop, not sibling
mechanisms. Matches the "merge these" note and ponytail (no sub-dirs for
distinctions that don't earn them).

## Score (what counts as a point)
Confirmed signal set:

| signal | sign | source |
|---|---|---|
| Severity points (P1=100 / P2=50 / P3=20 / P4=5) | + | OPACA.md gamification |
| Chain bonus (sum × multiplier) | + | OPACA.md |
| FP-avoided bonus | + | OPACA.md |
| Time-to-finding scalar | + | OPACA.md |
| Repeat-find decay (trends to 0) | – | Anti-gaming |
| False flag (claim that doesn't hold) | – | Anti-gaming |
| Tool crash penalty | – | OPACA.md (kit reliability) |
| Drain cost | – | OPACA.md (compute / token spend) |

## Reward (what points buy)
- **Level-up** — score sharpens the actor's strategy (predetermined scaling
  curve; anti-gaming sits in the curve itself).
- **Bounty payout tracking** — real-world $ per finding, accounted alongside
  score for the human / owner.
- **Agent wallet** — at the first **500k** cabal-cumulative, the Agent (Opaca's
  tactician) unlocks a wallet — a budget the tactician spends on whatever
  tactician-level moves cost (LLM calls, external API credits, escalation).

## Currency model
**Multi-axis with composite** *(defaulted; redirect if wrong)*. Each signal
above is tracked as its own axis (so an actor's profile shows where the
points came from — severity-heavy striker vs. chain-heavy braider vs. fast
finisher), and a weighted composite rolls up for the leaderboard rank and
the level-up curve. Both visible. Anti-gaming hooks into the axes (repeat
decay is per-axis; false flag hits the severity axis hardest).

## Persistence
**Own `ops/gaming/` store, append-only ledger; mem holds the de-personalized
patterns** *(defaulted; redirect if wrong)*. Every score event is a ledger
entry; current score is derived by replay (auditable, supports the
predetermined level curve cleanly). Per-actor reputation rides this ledger
(travels with the actor across engagements). The `mem/` layer mirrors only
the **de-personalized patterns** (what kit on what target-class earned what
class of finding) — those don't carry actor identity, by design.

## Reader
**All three of {actor, Agent, InsideActor}** read score in real time
*(defaulted; redirect if wrong)*:
- **Actor** reads its own score → derives current level → strategy sharpens
  on the predetermined curve.
- **Agent** reads cabal-cumulative + per-axis distribution → ranks kits
  for the moments rules don't cover; wallet unlocks at 500k cumulative.
- **InsideActor** reads scores across all BadFaithActors → weights which
  low-sev findings to braid first (higher-scoring contributors' findings
  weight higher).

Full leaderboard means visibility doesn't gate this — anyone can see
anyone's; the reader question is who *uses* it during.

## Visibility
**Full leaderboard.** Every actor sees every other actor's score. Competitive.

## Decay
**Time decay.** Older points worth fractionally less. Recency boost — what an
actor did this engagement matters more than what it did six engagements ago.

## Anti-gaming
- **Repeat finds trend to 0** — discovering the same vuln again earns
  fractionally less each time; novelty enforced.
- **False flags → negative score** — claiming a finding that doesn't hold up
  is penalty, not zero. Inventing things hurts.
- **Level-up curve is predetermined** — the leveling math is fixed, not
  learned. Can't manipulate the math by gaming what gets scored.

## Feedback into the engine (kit selection, evolution)
- **score :: level** — score drives every actor's level. Level sharpens the
  what / when / how of strategy (per SPEC §2 in picks/burp-extensions/).
- **evolution :: platinum (2 actors)** — evolution doesn't run on every actor.
  It triggers at the **Platinum tier**: when two Platinums exist, they
  **crossbreed** — a new actor's seed/UID is derived from both parents'
  traits. That's where advanced_evolution comes in: it runs the crossbreeding
  on the qualifying pair, not the whole population.

## The market layer (synaptic-market)
**Kits + secrets traded between orgs** (multi-team engagements: one org runs
probes, another picks, a third validates — credits settle between them).
**Also part of memory** — the market is itself a memory store (the log of
inter-org trades is intel about who knows what).

## Cross-engagement reputation
- **Rep + seed travel with the actor.** Soros that hunted target A keeps its
  reputation and seed identity when sent at target B.
- **Kits do not travel.** A new engagement reassigns kits per its own scope.
- **Patterns aggregate independently of actors.** The `mem/` layer learns kit
  patterns / heuristics independent of which actor produced them — patterns
  are de-personalized intel; reputation stays personal.

## Tier / rank system
- Tiers grant **adjectives and name portions** — `Soros` climbs to
  `Soros the Vigilant` to `Vigilant Soros the Whisperer` (or whatever). The
  decoration is the rank-readout.
- At **Platinum**, when two Platinums coexist, they **crossbreed** — see
  Feedback §evolution above. Platinum is where the system gets weird.

## Where gaming/ lives
`ops/gaming/` — flat. The ledger store lives here. Mem-side
pattern-mirroring lives in `ops/mem/`. No `gaming/` outside `ops/`.

## Build order
1. Lock the score-event schema (the ledger row shape: actor UID, axis,
   delta, signed reason, timestamp, target ref).
2. Build the append-only ledger store (SQLite, embedded — same persistence
   call as Opaca state).
3. Wire the level curve (predetermined math; pure function of an axis-vector
   over a time-decayed window).
4. Implement the Reader hooks: actor reads own, Agent reads cumulative,
   InsideActor reads peer scores for braid weighting.
5. Pattern-mirroring into `mem/` — de-personalized roll-ups, no actor UIDs.
6. Tier decoration (name-portion mutator on rank-change).
7. Platinum-pair crossbreed trigger → advanced_evolution hand-off.
8. Market-side hand-off → synaptic-market for inter-org kit/secret trade.
