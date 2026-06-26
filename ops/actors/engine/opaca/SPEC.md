
# Opaca — skeleton spec

 [`../OPACA.md`](../OPACA.md); engine requirements in
[`../SWARM-ECS-SPEC.md`](../SWARM-ECS-SPEC.md). OCaml shown is shape, not syntax.

## 1. What Opaca is

The compiled load-bearer. It carries the tools, the loops, and the state so the
human and the Agent don't. One primitive holds it up: the **actor** — a Riot
process running a **kit**. The actor switches kits in place on a finding instead
of dying. Loops are only soft-coded; their strength
**emerges** from kit-switch rules. Many
**seeded** actors run one target at once for broad coverage; 
Riot supervises so one crash doesn't sink the engagement.

The dividing line: compiled OCaml does everything deterministic (scope gating,
evidence checks, severity scoring, dedup, state, kit-switching). The Agent is a **tactician** consulted only for judgment (PoC Building,Verification,
Complex Linkage patterns, attack-paths,report prose, a tool
failure the rules don't cover, kit-selection ambiguity).

## 2. The actor — stochastic semi-deterministic gamebot

An actor is game-enemy grade, not Agentic grade. Its loop:


EXAMPLE - NOT LITERAL
```
loop:
  perceive   -> read world state + its mailbox (findings from self/others)
  decide     -> evaluate kit-switch rules against (finding, state)
                ties / ordering / depth broken by the actor's seed
  act        -> run the current kit's tools (FOLLOWING WORKFLOW), emit findings
  maybe ask  -> only on extreme uncertainty, call the agent tactician
```

**Semi-deterministic.** Rules are fixed and compiled; the **SplitMix64 seed** (maybe different seeds tho)
supplies randomness (kit-switch tiebreak, tool order at forks within a kit, how long
to try a kit, which target element next). So:

- same seed + same target + same kits + same 'timestamp' + same rules -> identical traversal (replay)
- different seeds -> different traversal, broader coverage than any single path

Seeds are logged and forkable (a child actor forks the parent's PRNG)《unsure》. This is part of the cognition. No per-step Agent babysitting.


## 3. Kits — declarative, plug-and-play

A kit is a bundle of tools for a purpose plus the rules for when to leave it. (this is the only accuracy)

## 4. Connection surface

The outside drives Opaca through, in priority order:

1. **MCP server (primary)** — kits exposed as tools; the Agent calls `switch_kit`,
   `get_findings`, `submit_report`, `spawn_actor`. This is the Claude Code path.
2. **Plugin / stdio bridge** — JSON-lines in/out for direct piping. (i.e marketplace.json plugin.json)
3. **custom API**.

All three are thin shells over the same engine actions; none of them *is* the
engine. Riot provides the socket/TCP IO and structured output.

## 5. How it fits together

```
Opaca (Spine, interfaces here)
├── TargetSupervisor(t)    per entrance point & tree
│   ├── Actor (seed a)     gamebot, grabs kit t assigned
│   ├── Actor (seed b)     concurrent, different seed
│   └── ChainActor         background: records everything categorically
 
```

- **Findings** are typed (surface | vuln | exploit | chain | report), carry
  CWE/CVSS/evidence, distance from entry point and relations (`DerivedFrom`, `ChainsWith`, `EvidenceFor`),
  and live in the Store as the world. Illegal transitions (report before a confirmed validation) are not findings.
- **Scope invariant**: out-of-scope -> drop + log + score penalty : 5 fails = cull ; stored to neural mesh
- **Scoring**: deterministic, replayable, per engagement thread ( chain bonus, FP-avoided bonus, crash penalty, time-to-finding scalar, drain cost, ).
  Score feeds kit selection — learned preference (the `ops/mem` + `ops/actors/ruv-fann` learning layer).
- **Persistence**:

## 6. What's decided vs open

Decided: actor=gamebot (seeded semi-deterministic); kits=declarative plug-and-play;
Open (defer): exact kit-file syntax, memory base,
which OCaml effect/scheduler details Riot settles on, Build order.
