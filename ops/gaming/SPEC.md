<!-- generated: skeleton — partial; bucket/currency/reader/persistence still pending -->
# Gaming — skeleton

The score + reward layer. Mother folder for spec'ing per the "merge these"
note. Sections fill in from multiple-choice answers; TBD = still pending.

## Bucket shape
TBD

## Score (what counts as a point)
TBD — currency model still pending. Confirmed signals: severity points (P1=100,
P2=50, P3=20, P4=5 — from OPACA.md), chain bonus, FP-avoided bonus, crash
penalty, time-to-finding scalar, drain cost, false-flag penalty (see Anti-gaming).

## Reward (what points buy)
- **Level-up** — score sharpens the actor's strategy (predetermined scaling
  curve; anti-gaming sits in the curve itself).
- **Bounty payout tracking** — real-world $ per finding, accounted alongside
  score for the human / owner.
- **Agent wallet** — at the first **500k** cabal-cumulative, the Agent (Opaca's
  tactician) unlocks a wallet — a budget the tactician spends on whatever
  tactician-level moves cost (LLM calls, external API credits, escalation).

## Currency model
TBD

## Persistence
TBD

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
TBD (probably stays at ops/gaming/, but persistence answer ties this).

## Build order
TBD — locks once Bucket / Currency / Persistence are picked.
