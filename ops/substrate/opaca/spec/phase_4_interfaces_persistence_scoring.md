# Phase 4 — Pseudocode: Interfaces, persistence, scoring

The outer ring: how the LLM drives Opaca, how state survives restart, and how
engagements are scored. Traces to FR9, FR10, FR11, C2, C7, NFR8.

## 1. MCP server (FR9 — primary interface, C7)

Opaca exposes kits as MCP tools over Riot's socket IO. The LLM calls these; the
compiled engine runs everything between calls. The LLM is invoked *by* Opaca
only for the judgment steps (C7), not for every step.

```
tool switch_kit   { actor_id, kit_name }      -> { ok, current_kit }
tool get_findings { target?, since?, severity? } -> finding list (JSON)
tool submit_report{ finding_id, platform }    -> { submitted, report_id }
tool spawn_actor  { target, kit_name, seed? } -> { actor_id, seed }
```

The five judgment call-outs Opaca makes *to* the LLM (the inverse direction):

| Trigger | LLM is asked |
|---------|--------------|
| `Vuln` needs triage | real or false positive? |
| surfaces mapped, no rule fires | what's the attack path? |
| `Report` body empty | write the narrative |
| `Error e` no rule covers | how to proceed? |
| switch ties beyond seed scope | which kit? |

Everything else (scope gate, dedup, severity scoring, state, kit-switch on clear
rules) is compiled OCaml.

## 2. Stdio bridge (FR9 — direct Claude Code piping)

JSON lines in, JSON lines out. Same command/result vocabulary as MCP, no socket.

```
stdin  : {"cmd":"spawn_actor","target":"...","kit":"web2-recon"}\n
stdout : {"event":"finding","kind":"surface","id":"...","locus":"..."}\n
         {"event":"actor_switched","actor_id":"...","to":"web2-exploit"}\n
```

## 3. HTTP REST (FR9 — non-Claude LLMs / dashboards)

```
POST /actor/{id}/switch_kit   {kit_name}
GET  /findings?target=&severity=
POST /actor                   {target, kit, seed?}
GET  /actor/{id}/score
```

Priority order is MCP > stdio > HTTP (OPACA.md): HTTP exists for dashboards and
non-Claude peers, not as the primary path.

## 4. State persistence (FR10, C2 — SQLite or flat, no external DB)

The world is the durable sandbox (NFR7). Persist on every `world_put` so a
restart recovers actors and findings (T-FR10).

```ocaml
module World : sig
  val put         : world_handle -> finding -> unit       (* upsert by id + relations *)
  val get         : world_handle -> finding_id -> finding option
  val snapshot    : world_handle -> path -> unit          (* full dump *)
  val load        : path -> world_handle
  val save_actor  : world_handle -> actor_state -> unit   (* kit, seed, history *)
  val load_actor  : world_handle -> string -> actor_state
end
```

- Default backend: **SQLite** (single file, transactional, snapshot via
  `VACUUM INTO`). Flat-file backend behind the same signature for the
  no-dependency case. Selection is config, not code (C2).
- Findings table keyed by `id`; relations table for the graph (Phase 2 §2);
  actors table for restart (Phase 3 §5).

## 5. Scoring (FR11, NFR8 — deterministic, replayable)

Pure function over the finding stream; same stream ⇒ same score (T-FR11). The
score is a kit-reliability + efficiency signal that feeds kit selection.

```ocaml
val score_event : finding -> outcome -> int
```

```
points:
  Vuln/Exploit confirmed   -> by severity: P1=100 P2=50 P3=20 P4=5
  Chain completed          -> (sum of parts) * multiplier
  false_positive_avoided   -> +small
  tool_crash               -> -penalty            (kit-reliability signal)
  time_to_finding          -> efficiency bonus (decays with wall-clock)
```

### Learned kit-selection (feeds from the wave-2 substrate)

Scores are not hardcoded preferences — they accrue into per-(target-type, kit)
hit-rates that bias future `select`/tiebreak decisions. The learning compute is
the dissolved neural substrate, not new engine code:

- `ops/substrate/ruv-fann/`, `ops/substrate/neural-bridge/` — sub-100ms
  inference scoring "which kit next" from accumulated hit-rates.
- `ops/gamification/guild-hall/` — the engagement-scoring model
  (points/leaderboard) reframed onto findings/seeds.
- `ops/optimizer/advanced_evolution/` — evolve a kit's tool configs against the
  score once a finding is confirmed exploitable.

Opaca calls these over the same boundaries as any tool (FR5/FR9); the engine
stays compiled OCaml. Wiring them is post-v1 — v1 ships static severity points
with the learned layer behind a flag.

## 6. TDD anchors

- **T4.1 (T-FR10)** `snapshot` → kill daemon → `load` → world + every actor
  state identical (findings, relations, kit, seed, history).
- **T4.2 (T-FR11)** Replaying the same finding stream yields the identical total
  score and per-kit hit-rates.
- **T4.3 (FR9)** Each MCP tool round-trips a valid JSON request→response;
  malformed input is rejected, not crashed.
- **T4.4 (C7)** A clear-rule kit-switch makes zero LLM calls; only the five
  judgment triggers (§1) invoke the LLM (assert via a recording LLM mock).
- **T4.5 (C2)** The SQLite and flat-file backends pass the same World test suite
  behind the identical signature.

## 7. Build order (implementation sequencing for the Refinement phase)

1. Phase 2 types + scope gate + smart constructors (no I/O) — testable pure core.
2. World persistence (Phase 4 §4) against the type core.
3. Kit registry + one tool adapter (a binary, e.g. noir) — T-FR1.
4. Single actor loop + kit-switch engine + seeding (Phase 3) — T-FR3/T-FR7.
5. Supervision + restart (Phase 3 §5) — T-FR6.
6. Chain actor (Phase 3 §6).
7. MCP server, then stdio, then HTTP (Phase 4 §1–3).
8. Static scoring (Phase 4 §5); learned layer behind a flag last.

Each step ships with its TDD anchors green before the next starts.

<!-- generated: spec; sign-off epoch appended at commit -->
