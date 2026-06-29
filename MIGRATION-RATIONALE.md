# Migration Rationale: Why Opaca Despite Actor/Entity Equivalence

**Date:** 2026-06-29  
**Status:** Approved  
**Decision:** Proceed with OPACK → Opaca migration

---

## The Equivalence Observation

**Critical Insight:** If you replace "actor" with "entity" in Opaca's design, it's structurally identical to OPACK:

| OPACK (C++/Flecs) | Opaca (OCaml/Riot) | Abstraction |
|-------------------|-------------------|-------------|
| Entity | Actor | Thing with state |
| Component | Mailbox/State | Data attached to thing |
| Actuator | Kit | Capability to do stuff |
| Action | Tool execution | The stuff being done |
| System | Message handler | Logic that processes things |
| Hierarchy | Supervision tree | Parent-child relationships |

**Both implement:** Perceive → Decide → Act loop with composable behaviors and fault tolerance.

---

## Why Migrate Anyway?

The migration is **not** about the abstraction model (actor vs entity). It's about the **runtime properties** and **ecosystem fit** for the security research domain.

### 1. Runtime Properties (The Real Differentiators)

#### 1.1 Concurrency Model
- **OPACK:** Single-threaded ECS queries with manual parallelism
  - Flecs has multicore support, but requires explicit system scheduling
  - Shared mutable state (ECS world) requires careful synchronization
  - Parallelism is opt-in and complex to reason about

- **Opaca:** Multicore by default with isolated actors
  - OCaml 5 domains give true parallelism without shared memory
  - Each actor has its own heap (per-actor GC, no stop-the-world)
  - Message passing enforces isolation (no accidental data races)
  - **Benefit:** Safer concurrency, easier to scale to 100+ concurrent probes

#### 1.2 Fault Isolation
- **OPACK:** Crash in one system can corrupt the ECS world
  - Flecs hierarchy helps, but shared state is a single point of failure
  - Debugging requires understanding the entire world state

- **Opaca:** Crash in one actor is isolated by Riot supervision
  - Supervisor restarts crashed actor with clean state
  - Other actors continue unaffected
  - **Benefit:** "Let it crash" philosophy = more robust long-running engagements

#### 1.3 Hot Code Reload (Potential)
- **OPACK:** Requires full recompilation and restart
  - C++ templates bake behavior at compile time
  - Dynamic loading (dlopen) is possible but fragile

- **Opaca:** OCaml supports code reloading (though not trivial)
  - Riot actors can be upgraded without stopping the daemon
  - Kit definitions are data, not compiled code
  - **Benefit:** Update tactics mid-engagement without losing state

#### 1.4 Effects-Based I/O
- **OPACK:** Blocking I/O or manual async (callbacks/futures)
  - Flecs doesn't provide async primitives
  - Tool execution blocks the ECS world

- **Opaca:** Eio provides structured concurrency with effects
  - Direct-style async (looks synchronous, runs concurrent)
  - Cancellation and timeouts are first-class
  - **Benefit:** Cleaner code for I/O-heavy security tools

### 2. Ecosystem Fit

#### 2.1 Security Domain
- **OCaml:** Strong in formal verification, cryptography, networking
  - Used by Jane Street (finance), Docker (containers), Tezos (blockchain)
  - Libraries: mirage (unikernels), ocaml-tls, angstrom (parsers)
  - **Fit:** Security research needs parsers, crypto, network protocols

- **C++:** General-purpose, but security libs are scattered
  - Flecs is great for games, not security
  - **Mismatch:** We're forcing a game engine into offensive security

#### 2.2 Type System
- **OPACK:** C++ templates provide compile-time safety
  - But: Template errors are cryptic, compilation is slow
  - Type safety is opt-in (easy to bypass with casts)

- **Opaca:** OCaml algebraic types + pattern matching
  - Exhaustiveness checking (compiler ensures all cases handled)
  - No null pointers, no segfaults (unless FFI)
  - **Benefit:** Fewer runtime errors, faster iteration

#### 2.3 Tooling
- **OPACK:** CMake, GDB, Valgrind
  - Build times increase with templates
  - Debugging template errors is painful

- **Opaca:** Dune, OCaml debugger, Merlin (IDE support)
  - Fast incremental compilation
  - Better error messages
  - **Benefit:** Faster development cycle

### 3. Operational Benefits

#### 3.1 Deployment
- **OPACK:** Static binary, but large (Flecs + all tools)
  - Recompilation required for any change
  - Distribution is monolithic

- **Opaca:** Daemon + kit files (data)
  - Update kits without recompiling engine
  - Distribute kits separately (marketplace model)
  - **Benefit:** Faster updates, community contributions

#### 3.2 Observability
- **OPACK:** Manual logging, Flecs explorer (GUI)
  - Hard to trace across systems
  - No structured telemetry

- **Opaca:** Riot provides structured logging + telemetry
  - Trace messages across actors
  - Metrics built-in (actor count, message rate, etc.)
  - **Benefit:** Easier debugging and monitoring

#### 3.3 Testing
- **OPACK:** Unit tests require full ECS world setup
  - Integration tests are slow (compile + run)
  - Hard to mock external tools

- **Opaca:** Actors are isolated, easy to test
  - Mock actors for integration tests
  - Property-based testing (QCheck)
  - **Benefit:** Faster test suite, better coverage

---

## What We're NOT Gaining

**Honesty check:** These are NOT reasons to migrate:

1. **Abstraction model** — Actor vs Entity is just naming
2. **Composability** — Both support it (ECS components vs actor kits)
3. **Hierarchy** — Both have it (Flecs parent/child vs Riot supervision)
4. **Performance** — OCaml is slower than C++ for CPU-bound tasks
   - But: Security research is I/O-bound (network, disk, subprocesses)
   - Acceptable tradeoff: 2x slower is fine if we gain safety + flexibility

---

## The Real Question: Could We Enhance OPACK Instead?

**Alternative:** Keep OPACK, add:
1. Dynamic kit loading (dlopen)
2. Better concurrency (thread pool)
3. Supervision (custom crash handlers)
4. Hot reload (plugin system)

**Why not?**
- **Complexity:** Retrofitting these into C++/Flecs is harder than starting fresh
- **Safety:** C++ makes it easy to shoot yourself in the foot (data races, memory leaks)
- **Ecosystem:** We'd still be fighting the game-engine paradigm
- **Time:** Building a robust plugin system in C++ takes as long as the migration

**Verdict:** Migration is less risky than retrofitting.

---

## Decision Summary

**We migrate because:**
1. **Safer concurrency** — Isolated actors > shared ECS world
2. **Better fault tolerance** — Supervision > manual error handling
3. **Ecosystem fit** — OCaml security libs > C++ game engine
4. **Operational flexibility** — Runtime kits > compile-time templates
5. **Developer experience** — Better types, tools, errors

**We accept:**
1. **Performance cost** — 2x slower is acceptable for I/O-bound work
2. **Learning curve** — Team needs to learn OCaml + Riot
3. **Ecosystem maturity** — Riot is younger than Flecs
4. **Migration risk** — 6-9 months of parallel operation

**The actor/entity equivalence is real, but the runtime properties are not equivalent.**

---

## Appendix: Hybrid Approach (Rejected)

**Considered:** Keep Flecs ECS core, add OCaml orchestration layer on top.

**Why rejected:**
- **Complexity:** Two languages, two runtimes, FFI overhead
- **Debugging:** Harder to trace across language boundary
- **Deployment:** More dependencies, larger binary
- **Benefit unclear:** If we're adding OCaml anyway, why keep C++?

**Verdict:** Full migration is cleaner than hybrid.

---

**Approved by:** Technical Lead  
**Next:** Proceed with Phase 0 (Foundation) per roadmap
