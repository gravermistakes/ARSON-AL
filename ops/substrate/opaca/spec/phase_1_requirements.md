# Phase 1 — Specification: Requirements

What Opaca must do, must not do, and how we know it's done. Every later module
traces back to an ID here. Derived from `OPACA.md` and `SWARM-ECS-SPEC.md`.

## 1. Goal

A compiled daemon that runs security engagement loops: simulates actors,
orchestrates the recon/scan/validation/chain topology, and coordinates swarms on
one target — as one supervised OCaml 5 + Riot process tree. Replaces OPACK.

## 2. Functional requirements

| ID | Requirement |
|----|-------------|
| **FR1** | **Kit registry** — load kit definitions, resolve tool paths, validate tool dependencies, expose kits by name. |
| **FR2** | **Actor lifecycle** — spawn an actor as a Riot process running one kit; an actor switches kits in place on a finding/state signal; it does not die-and-respawn to change loadout. |
| **FR3** | **Kit-switch engine** — evaluate switch rules against a finding + actor state and decide the next kit (or stay). Loops emerge from rules; loops are not hardcoded. |
| **FR4** | **Finding type system** — typed findings (surface, vuln, exploit, chain, report) carrying CWE / CVSS / evidence; relations (`DerivedFrom`, `ChainsWith`, `EvidenceFor`). |
| **FR5** | **Tool adapters** — invoke external tools: compiled binaries (noir, vigolium, drogonsec) and subprocess scripts; parse their output into findings. |
| **FR6** | **Supervision** — a tree (Opaca → TargetSupervisor → Actors + ChainActor + KitRegistry); a crashed actor restarts with its last kit + state. |
| **FR7** | **Seeded traversal** — each actor gets a SplitMix64 seed influencing tiebreaks, tool ordering, exploration depth, target-element selection; seeds are logged and replayable; forkable on child spawn. |
| **FR8** | **Chain actor** — a background actor subscribes to all findings; braids enough low-severity findings into a critical chain via the chain-builder kit. |
| **FR9** | **LLM interfaces** — MCP server (primary: `switch_kit`, `get_findings`, `submit_report`, `spawn_actor`), stdio JSON-lines bridge, HTTP REST — in that priority order. |
| **FR10** | **State persistence** — actor + world state survives daemon restart via SQLite or flat files (no external database). |
| **FR11** | **Scoring** — deterministic, replayable gamified score per engagement thread (severity points, chain bonus, FP-avoided bonus, crash penalty, time-to-finding); score feeds kit selection. |
| **FR12** | **Target manifest** — a single manifest format that `int/` tools emit and all actors consume. |

## 3. Non-functional requirements (engine, from SWARM-ECS-SPEC R1–R10)

| ID | Requirement | Opaca/Riot satisfier |
|----|-------------|----------------------|
| **NFR1 (R1)** | Actor isolation — own state + mailbox, async message passing, no shared mutable state | Riot processes |
| **NFR2 (R2)** | 10⁴–10⁶ cheap cells (one per surface/candidate/finding) | Riot lightweight processes |
| **NFR3 (R3)** | True multicore parallelism | OCaml 5 domains + Riot scheduler |
| **NFR4 (R4)** | Async non-blocking I/O; one blocked call never stalls others | effects-based scheduler |
| **NFR5 (R5)** | Supervision / let-it-crash / restart strategies | Riot supervisors (FR6) |
| **NFR6 (R6)** | Hot-swap kits at runtime without restarting the world | kit-switch is data, not code reload (FR3) — see note |
| **NFR7 (R7)** | World model: durable sandbox, actors are systems over it | world state store (FR10) |
| **NFR8 (R8)** | Deterministic, snapshot-able state + replayable scoring | seed log (FR7) + scoring (FR11) + persistence (FR10) |
| **NFR9 (R9)** | First-class interop with Go/Crystal/C/JS/Python tools + Claude over MCP/HTTP | tool adapters (FR5) + LLM interfaces (FR9) |
| **NFR10 (R10)** | Scope/authorization as an enforced invariant — no traffic leaves an unauthorized scope | scope gate, see §5 |

> **NFR6 note:** Opaca delivers R6 at the *kit* layer (an actor changes loadout
> live by switching kits the registry already holds), not via OCaml hot *code*
> reload. New tool **binaries** hot-add by dropping them where the registry
> resolves paths; new engine **code** still needs a daemon restart (state
> survives via FR10). Record this as an accepted limitation, not a gap.

## 4. Constraints

- **C1** Engine language: OCaml 5 + Riot (decided — `LANGUAGE-CANDIDATES.md`).
- **C2** No external database; SQLite or flat files only (FR10).
- **C3** No Python in the engine (project rule; Python tools still drive via FR5
  as subprocesses).
- **C4** License GPL preferred, MIT/BSD ok.
- **C5** Unix epoch timestamps everywhere.
- **C6** Findings carry severity, CWE, CVSS, exploit path, PoC, remediation;
  SARIF out for CI, CycloneDX for SBOM.
- **C7** The LLM runs only the judgment steps (triage, attack-path, report prose,
  unhandled tool failure, kit-selection ambiguity). Everything else is compiled
  OCaml. This is a hard boundary, not a preference.

## 5. The scope invariant (NFR10 — safety-critical)

No actor may emit traffic toward a target element absent from the authorized
target manifest. This is enforced in compiled code, before any tool adapter
fires, and cannot be bypassed by a kit, a rule, or the LLM. Modeled as a guard
every `Tool.invoke` passes through (Phase 2 §4, Phase 3 §2). Verified by
NFR10's TDD anchor below. Out-of-scope attempt → drop + log + score penalty,
never send.

## 6. Success criteria / acceptance test

Adapted from `SWARM-ECS-SPEC.md` §6:

> Spawn 50k actors; force 5% to crash; supervise-restart them with last kit +
> state; hot-add a new kit while running; fan out 1k async tool invocations
> across all cores; write findings as typed world entities; score + snapshot the
> world; reload the snapshot and replay one actor's seed to the same path.

Opaca is "done enough for v1" when that runs **easy and safe** on a laptop.

## 7. TDD anchors (gate the build; each becomes a test in later phases)

- **T-FR1** Registry rejects a kit whose tool path/dependency is missing.
- **T-FR3** Given finding X + state S, the switch engine returns a deterministic
  next-kit; the recon/scan/validate/report loop *emerges* from the documented
  rule set (assert observed kit sequence == expected for a scripted finding feed).
- **T-FR4** Finding round-trips through serialization without loss; an illegal
  transition (e.g. `report` before a `validation.confirmed`) is unrepresentable
  or rejected.
- **T-FR6** Kill an actor mid-kit; supervisor restarts it with the same kit and
  recovered state.
- **T-FR7 / NFR8** Two runs of the same seed produce the identical traversal;
  different seeds produce different traversals over the same target/kits/rules.
- **T-NFR10** A kit configured to hit an out-of-scope element never emits; the
  attempt is dropped, logged, and penalized.
- **T-FR10** Snapshot → kill daemon → reload → world + actor state identical.
- **T-FR11** Same finding stream scores identically across runs (replayable).

<!-- generated: spec; sign-off epoch appended at commit -->
