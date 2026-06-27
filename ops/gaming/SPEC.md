<!-- generated: skeleton — partial; bucket/currency/reader/persistence still pending -->
# Gaming — skeleton

The score + reward layer. Mother folder for spec'ing per the "merge these"
note. Sections fill in from multiple-choice answers; TBD = still pending.

## Bucket shape
**`ops/gaming/score/` + `ops/gaming/rep/`.** Two sub-buckets:
- **`score/`** — the live per-engagement tally (points earned and lost in flight).
- **`rep/`** — the persistent cross-engagement reputation (rides with the
  actor's UID across targets).

"Reward" as a separate top-level bucket dissolves — what score and rep
produce (level-up, bounty payout, Agent wallet) is downstream output, not a
sibling concept.

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
**Two factors, multi-axis derived.** Two base factors underlie everything:
`score` and `rep`. Every other axis (severity contribution, chain
contribution, time-to-finding, novelty, drain, false-flag penalty, repeat
decay…) is a **derivation** off those two — not a parallel coordinate.
Reading a derived axis tells you *how* this actor's score/rep came to be;
score and rep themselves are the only canonical numbers.

## Persistence
**Per actor UID.** Score and rep are both keyed by the actor's UID — the
durable identity. The UID carries its history forward through every
engagement; nothing else stores a score. No engagement-level ledger; no mem
sidecar; the actor IS the record.

## Reader
**Post-mortem only.** No actor — and no other CabalActor — reads score
during an engagement. Score and rep update *between* engagements, not
within. While a hunt is live, the score is closed-book.

What happens mid-engagement instead:
- **The Agent chooses actors, not kits.** When the Agent's role kicks in,
  it picks *which actor* (which UID, level, rep) to deploy on the next
  move. Kit selection is not the Agent's job.
- **Actors choose their own kits**, by preference function + stochastic
  mix of `(timestamp, score, engagement state)`. The UID-derived
  disposition gives baseline kit preferences; the stochastic component
  varies by now-time, score-so-far, and what's happening in the
  engagement. Semi-random — biased by who the actor is, not arbitrary.

Loop: **Agent says "Soros works the next move" → Soros consults its kit
preferences modulated by (now, my score, what's happening) → Soros picks a
kit and goes.** Score only re-enters the picture after the hunt closes.

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
`ops/gaming/score/` and `ops/gaming/rep/`. Per-actor-UID stores live in
each. No external sidecar in `mem/`. No `gaming/` outside `ops/`.

## Build order
1. Lock the **actor-UID record** schema (score and rep keyed by UID; the
   derived axes computed on read, not stored).
2. Build the per-UID store under `score/` (in-engagement live) and `rep/`
   (cross-engagement persistent).
3. Wire the **post-mortem update**: at engagement close, fold score deltas
   into rep, apply time-decay on rep, retire engagement-local score.
4. Wire the **predetermined level curve** as a pure function over an
   actor's rep + score history.
5. Implement the **actor kit-preference function**: UID-derived baseline
   biased by `(timestamp, score, engagement state)`; semi-random sampler.
6. Implement the **Agent actor-selection** path (Agent picks UIDs, not
   kits) — the hand-off from rules-can't-decide to Agent.
7. Tier decoration (name-portion mutator on rep-tier change).
8. Platinum-pair crossbreed trigger → advanced_evolution hand-off.
9. Market-side hand-off → synaptic-market for inter-org kit/secret trade.
10. Agent wallet unlock at 500k cabal-cumulative.
