<!-- generated: spec; sign-off epoch appended at commit -->
# Opaca — skeleton spec

One doc. The skeleton: what Opaca is, how the outside connects, how kits are
formatted and used, how an actor works, and how it fits together. Architecture
is in [`../OPACA.md`](../OPACA.md); engine requirements in
[`../SWARM-ECS-SPEC.md`](../SWARM-ECS-SPEC.md). OCaml shown is shape, not syntax.

## 1. What Opaca is

The compiled load-bearer. It carries the tools, the loops, and the state so the
human and the LLM don't. One primitive holds it up: the **actor** — a Riot
process running a **kit**. The actor switches kits in place on a finding instead
of dying. Loops are not coded; they **emerge** from kit-switch rules. Many
**seeded** actors run one target at once for broad coverage; Riot supervises so
one crash doesn't sink the engagement.

The dividing line: compiled OCaml does everything deterministic (scope gating,
evidence checks, severity scoring, dedup, state, kit-switching). The LLM is an
**oracle** consulted only for judgment (triage real/FP, attack-path, report
prose, a tool failure the rules don't cover, kit-selection ambiguity).

## 2. The actor — stochastic semi-deterministic gamebot

An actor is game-enemy-AI grade, not chatbot grade. Its loop:

```
loop:
  perceive   -> read world state + its mailbox (findings from self/others)
  decide     -> evaluate kit-switch rules against (finding, state)
                ties / ordering / depth broken by the actor's seeded PRNG
  act        -> run the current kit's tools (gated by scope), emit findings
  maybe ask  -> only on rule ambiguity, call the LLM oracle, cache the verdict
```

**Semi-deterministic.** Rules are fixed and compiled; the **SplitMix64 seed**
supplies all randomness (kit-switch tiebreak, tool order within a kit, how long
to stay in a kit, which target element next). So:

- same seed + same target + same kits + same rules -> identical traversal (replay)
- different seeds -> different traversal, broader coverage than any single path

Seeds are logged and forkable (a child actor forks the parent's PRNG). This is
the whole "intelligence": cheap, fast, reproducible, swarmable. No per-step LLM.

**Check:** run one actor twice on a fixed finding-feed with the same seed; assert
identical kit sequence + findings. Re-run with a new seed; assert a different
path over the same surface.

## 3. Kits — declarative, plug-and-play

A kit is a bundle of tools for a purpose plus the rules for when to leave it.
Declared as data the registry loads at startup (drop a kit in, no recompile for
now; we accept recompile later as it hardens). Shape:

```
kit "web2-recon" {
  tools = [
    binary "noir"        { path="probes/noir/noir"; args=["--ai-context","-o","sarif"]; emits=surface }
    script "scope-parser"{ path="int/web2/bug-reaper/analyze_scope.py";               emits=manifest }
  ]
  switch_rules = [
    on vulnerability -> "web2-exploit"
    on new_surface   -> "web2-recon"     # loop
    on source_code   -> "sast-probe"
  ]
}
```

- **tools**: compiled binary or subprocess script; each declares what Finding
  type it emits. Adapter parses tool output (SARIF/JSON/lines) into findings.
- **switch_rules**: `on <finding-kind> -> <kit-name>`. The registry validates
  every referenced tool path and kit name at load (fail fast).
- Kits map to the dissolved arsenal dirs (probes/picks/proofs/int) by verb.

**Check:** a kit naming a missing tool path or unknown switch-target is rejected
at load with the offending name.

## 4. Connection surface

The outside drives Opaca through, in priority order:

1. **MCP server (primary)** — kits exposed as tools; the LLM calls `switch_kit`,
   `get_findings`, `submit_report`, `spawn_actor`. This is the Claude Code path.
2. **Plugin / stdio bridge** — JSON-lines in/out for direct piping.
3. **HTTP REST** — `POST /actor/{id}/switch_kit`, `GET /findings`, … for
   dashboards / non-Claude callers.

All three are thin shells over the same engine verbs; none of them *is* the
engine. Riot provides the socket/TCP IO and `--json` structured output.

## 5. How it fits together

```
Opaca (root supervisor)
├── KitRegistry            loads kit data, resolves tool paths, serves kits by name
├── Store (SQLite)         world + findings + actor state + seed log + scores
├── TargetSupervisor(t)    per target
│   ├── Actor (seed a)     gamebot in some kit
│   ├── Actor (seed b)     …concurrent, different seed
│   └── ChainActor         background: subscribes to all findings, braids P3s -> P1
└── Interfaces             MCP / stdio / HTTP shells over the engine verbs
```

- **Findings** are typed (surface | vuln | exploit | chain | report), carry
  CWE/CVSS/evidence and relations (`DerivedFrom`, `ChainsWith`, `EvidenceFor`),
  and live in the Store as the world. Illegal transitions (report before a
  confirmed validation) are unrepresentable.
- **Scope invariant**: no tool emits toward a target element absent from the
  authorized manifest — enforced in compiled code before any adapter fires;
  out-of-scope -> drop + log + score penalty, never send.
- **Scoring**: deterministic, replayable, per engagement thread (P1=100/P2=50/
  P3=20/P4=5, chain bonus, FP-avoided bonus, crash penalty, time-to-finding).
  Score feeds kit selection — learned preference, not hardcoded (the `ops/mem`
  + `ops/actors/ruv-fann` learning layer).
- **Persistence**: SQLite, one embedded DB, no external server. Snapshot =
  reopen the DB; replay = re-run a logged seed.

## 6. What's decided vs open

Decided: actor=gamebot (seeded semi-deterministic); kits=declarative plug-and-play;
store=SQLite; MCP primary. Open (defer): exact kit-file syntax, the SQLite schema,
which OCaml effect/scheduler details Riot settles on. Build order follows §5
top-down: Store + KitRegistry, then one Actor + one kit over stdio, then MCP,
then concurrency + ChainActor.
