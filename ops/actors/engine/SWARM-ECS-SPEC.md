<!-- generated: spec; sign-off epoch appended at commit -->
# SPEC — concurrent/parallel/async actor-level ECS for pentest & bug-hunting

The swarm engine (OPACK's successor / refactor target). This is the
language-agnostic requirements spec; `LANGUAGE-CANDIDATES.md` scores languages
against the `R#` requirements defined here.

## 0. One-line

A multicore swarm of cheap, isolated, hot-swappable **actor-cells** over an **ECS**
world, doing mostly async I/O (network, subprocess, Agent), that **gamifies** found
bugs into a deterministic score — driven by tools and reached up to by outside AI
peers, with the human on the outer ring.

## 1. Domain shape (what actually runs)

- **Workload is I/O-bound, not CPU-bound.** The engine *coordinates*: fan out
  HTTP/DNS/socket probes, shell out to native scanners, wait on OAST callbacks,
  call Agent APIs. Heavy CPU (parsing, fuzzing, SAST) is done by external cells
  (Go/Crystal/C tools) the engine supervises — not in the engine itself.
- **Fan-out is huge and bursty.** One scope explodes into thousands of
  surfaces → candidates → findings. Each wants to be its own unit of work.
- **Everything is unreliable.** Targets hang, tools segfault, networks flap,
  rate-limits bite. Failure is the normal case, not the exception.
- **State is a graph that branches/merges/prunes.** Findings relate
  (`DerivedFrom`, `ChainsWith`, `EvidenceFor`); the chain-loop searches it.
- **Scale is single/paired operator on a laptop.** Not a datacenter. Ops
  simplicity > horizontal scale. Distribution is a nice-to-have, not a need.

## 2. Hard requirements (R#)

| R | Requirement | Why |
|---|---|---|
| **R1** | **Actor-level isolation** — each cell owns its state + mailbox; async message passing; no shared mutable state / no data races by construction | a crashing/compromised cell must not corrupt the world; offensive code must be sandboxable |
| **R2** | **Massive lightweight concurrency** — 10⁴–10⁶ cheap-to-spawn cells (one per surface/candidate/finding) | fan-out is per-entity, not per-thread |
| **R3** | **True multicore parallelism** — cells run in parallel across cores, preemptively scheduled | scan/recon loops must saturate the box |
| **R4** | **Async, non-blocking I/O** — a blocked socket/subprocess/Agent call never stalls other cells | the engine is mostly waiting on I/O |
| **R5** | **Supervision / fault tolerance** — "let it crash"; restart strategies; a dead cell is contained and recoverable | tools and targets fail constantly |
| **R6** | **Hot-swappable cells** — load/unload/upgrade a cell (and its code) at runtime without restarting the world | the operator adds kits/tools mid-hunt; the world (sandbox) outlives any actor |
| **R7** | **ECS world model** — entities + components + queried systems; the world is the durable sandbox, cells are the systems | matches the dissolution model; decouples data from behavior |
| **R8** | **Deterministic, snapshot-able state + scoring** — branch/merge/prune the finding graph; a replayable scoring engine (severity·impact·novelty → points, economy/tokens) | "gamify thru OPACK; score it"; reproducible triage |
| **R9** | **First-class interop** — drive existing cells: Go (vigolium/drogonsec/bofhound), Crystal (noir), C/C++ (flecs), JS (h1dr4), Python (lance/reaper); reach Claude peers via HTTP/MCP | the dissolved arsenal already exists; don't rewrite it |
| **R10** | **Scope/authorization as an enforced invariant** — no traffic leaves without an authorized scope; payloads sandboxed | legal + safety; offensive tooling must not self-pwn or stray |

## 3. Soft requirements (weighted, not gating)

- **S1 Static type / capability safety** — catch protocol/state-machine errors at
  compile time; capability-secure data sharing is a bonus.
- **S2 AOT/native performance** — single-binary, low-overhead, laptop-friendly.
- **S3 Ecosystem** — HTTP/DNS/TLS/parsing/crypto libs; security tooling; package mgmt.
- **S4 Maturity & ops** — stable releases, observability, hireable, won't churn under us.
- **S5 Ergonomics / iteration speed** — fast to write a new cell; ponytail-lazy.
- **S6 Distribution** — optional multi-node, for when "paired" becomes "a few boxes".
- **S7 License** — GPL preferred, MIT/BSD ok (project rule).

## 4. The three layers (a language need not win all of them)

1. **Swarm engine** — the actor/ECS runtime: spawn, schedule, supervise, hot-swap,
   hold the world, score. **R1–R8 dominate here.** This is the refactor decision.
2. **Native cells** — perf/FFI-heavy probes & scanners (SAST, fuzzers, packet work).
   **S2, R9 dominate.** Can be a *different* language called over ports/FFI/MCP.
3. **Reasoning peers** — the two outside AI cells (Claude). Not in this language at
   all; reached over HTTP/MCP. The engine only needs an `Agent`/`NeedsJudgment`
   component + a drain that calls out.

## 5. Explicit non-goals (v1)

- Not a distributed cluster; not multi-tenant; not a web service.
- The engine does not do CPU-heavy scanning itself — it supervises cells that do.
- Not a from-scratch ECS if an existing one (flecs) can be driven via FFI.

## 6. Acceptance test for any candidate

> Spawn 50k cells; have 5% of them crash on purpose; supervise-restart them;
> hot-load a new cell type while running; fan out 1k async HTTP probes across all
> cores; write findings as ECS entities; score + snapshot the world; reload the
> snapshot. If a language makes that **easy and safe**, it's a lead candidate.

— spec by Temper
