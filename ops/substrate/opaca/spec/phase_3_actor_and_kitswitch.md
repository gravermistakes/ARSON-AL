# Phase 3 — Pseudocode: Actor lifecycle, kit-switch engine, supervision, seeding

How an actor lives, switches kits, survives crashes, and traverses
deterministically. Traces to FR2, FR3, FR6, FR7, FR8, NFR1–5, NFR8.

## 1. Actor state

An actor is a Riot process holding its kit, its seed, and a handle to the shared
world. No shared mutable state — the world is reached by message, not pointer
(NFR1).

```ocaml
type actor_state = {
  id        : string;
  kit       : kit;                 (* current loadout *)
  seed      : Splitmix.t;          (* §4 *)
  world     : world_handle;        (* message-passing handle, FR10 *)
  manifest  : target_manifest;     (* scope, Phase 2 §5 *)
  history   : finding_id list;     (* what this actor produced *)
}
```

## 2. Actor loop (FR2, FR3, NFR4)

The actor runs its kit's tools, emits findings, then asks the switch engine what
kit to wear next. It never dies to change kits — it rebinds `kit` and loops.

```
loop actor_state:
  tools = order_tools(actor_state.kit, actor_state.seed)        # §4 seeding
  for tool in tools:
     element = select_target(actor_state, actor_state.seed)     # §4 seeding
     tref    = authorize(actor_state.manifest, element)         # Phase 2 §4 gate
     result  = invoke(tool, tref)                               # async; never blocks peers (NFR4)
     case result:
        Ok findings -> for f in findings:
                          world_put(actor_state.world, f)       # persist + publish (FR10, FR8)
                          notify_subscribers(f)                 # chain-actor & peers (FR8)
                          next = switch(actor_state.kit, f, actor_state) # §3
                          case next:
                             Switch_to k -> return loop { actor_state with kit = registry_get k }
                             Spawn_child k -> spawn_child(actor_state, k); continue
                             Stay         -> continue
        Error Out_of_scope -> log_drop(element); penalize(actor_state)   # NFR10
        Error e            -> raise e   # let it crash -> supervisor (§5, NFR5)
  return loop actor_state   # kit exhausted, no switch -> re-run (recon loop idle)
```

## 3. Kit-switch engine (FR3 — loops emerge here)

Pure function: `(kit, finding, state) -> action`. First matching rule wins; ties
broken by seed (NFR8). No loop is hardcoded — the recon/scan/validate/report
topology is the *observed consequence* of these rules across actors.

```ocaml
val switch : kit -> finding -> actor_state -> action
```

```
switch kit finding state:
  pat = classify(finding)                 # finding -> finding_pattern (Phase 2 §3)
  matches = [ a | On(p,a) <- kit.switch_rules, p = pat ]
  case matches:
     []      -> Stay
     [a]     -> a
     a :: _  -> seed_pick(state.seed, matches)   # deterministic tiebreak
```

`classify` maps a concrete finding to its pattern, e.g.
`Vuln _ -> Vulnerability`, `Exploit{works=true} -> Exploit_works`,
`Surface{kind=Source_file} -> Source_code`.

The reference rule set (documented, testable — T-FR3) that yields the four loops:

```
new_surface    -> recon kit      (recon loop)
vulnerability  -> exploit kit     (scan -> pick)
exploit_works  -> validation kit  (validation loop)
validation_confirmed -> report kit
validation_failed    -> scan kit  (wrong hypothesis, prune)
report_submitted     -> recon kit (next target)
```

## 4. Seeding (FR7, NFR8 — SplitMix64)

A non-cryptographic splittable PRNG (project spec: SplitMix64). Seeded from the
root engagement seed; **forked** when an actor spawns a child so lineages stay
reproducible.

```ocaml
module Splitmix : sig
  type t
  val of_seed : int64 -> t
  val next    : t -> int64 * t        (* pure: returns value + advanced state *)
  val fork    : t -> t * t            (* split for a child actor *)
end
```

Seed influences exactly four decisions (OPACA.md): kit-switch tiebreak (§3),
tool ordering (`order_tools`), exploration depth (how many cycles before a
forced switch), target-element selection (`select_target` when many candidates).

Every actor logs `(actor_id, root_seed, fork_path)` so a run is replayable:
same seed + same kits + same rules ⇒ identical traversal (T-FR7).

## 5. Supervision tree (FR6, NFR5)

Riot supervisors. Restart strategy: `one_for_one` at actor level (one actor
crashing doesn't restart peers); restarted actor resumes from its **last
persisted kit + world state** (FR10), not from scratch.

```
Opaca (root supervisor)
├── KitRegistry            (loads/validates kits; resolves tool paths — T-FR1)
├── TargetSupervisor(t1)   (one_for_one)
│   ├── Actor(recon)
│   ├── Actor(scan)
│   ├── Actor(validate)
│   └── ChainActor         (background subscriber, §6, FR8)
└── TargetSupervisor(t2) ...
```

Restart pseudocode:

```
on actor_crash(actor_id, reason):
   log(actor_id, reason)                       # crash = kit-reliability signal -> score
   st = world_load_actor(actor_id)             # last snapshot (FR10)
   restart Actor with st.kit, st.seed, st.history
```

## 6. Chain actor (FR8)

A background actor subscribing to all findings. Holds low-severity findings;
when a braid reaches a critical threshold, switches to the chain-builder kit and
emits a `Chain`.

```
chain_actor:
  on finding f where severity(f) in {P3,P4}:
     pool.add(f)
     if braidable(pool):                       # graph search over relations
        chain = build_chain(pool)              # chain-builder kit
        world_put(world, Chain chain)          # bonus score (Phase 4 §4)
        pool.remove(chain.parts)
```

## 7. TDD anchors

- **T3.1 (T-FR3)** Feed a scripted finding sequence; assert the observed kit
  sequence equals the expected recon→scan→validate→report path.
- **T3.2 (T-FR7)** Same root seed ⇒ byte-identical traversal log across two runs;
  two different seeds ⇒ divergent logs over the same target/kits/rules.
- **T3.3 (T-FR6)** Kill an actor mid-kit; supervisor restarts it with the same
  kit and recovered history.
- **T3.4 (FR8)** Given N P3/P4 findings that braid, the chain actor emits exactly
  one `Chain` and removes its parts from the pool.
- **T3.5 (NFR4)** A tool invocation that blocks for T seconds does not delay a
  second actor's invocation (assert concurrency via timestamps).

<!-- generated: spec; sign-off epoch appended at commit -->
