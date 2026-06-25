<!-- generated: research; sign-off epoch appended at commit -->
# Candidate languages for the swarm-engine refactor

> **DECISION: OCaml 5 + Riot** (effects/domains runtime, Erlang-style actors + supervision).

Scored against `SWARM-ECS-SPEC.md`. The decision that matters is **Layer 1 (the
swarm engine)** — R1–R8. Layers 2/3 (native cells, AI peers) can be polyglot, so
they don't gate the choice. Verdicts below are for the *engine* unless noted.

## How each lands on the engine requirements

Legend: ●=strong, ◐=partial/maturing, ○=weak/absent.

| Lang / runtime | R1 isolate | R2 cheap spawn | R3 parallel | R4 async I/O | R5 supervise | R6 hot-swap | R7 ECS fit | R9 interop(C/subproc) | S1 types | S4 maturity |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| **Elixir (BEAM)** | ● | ● | ● | ● | ● | ● | ◐ | ◐ | ○(dyn) | ● |
| **Gleam (BEAM)** | ● | ● | ● | ● | ● | ◐ | ◐ | ◐ | ● | ◐ |
| **Erlang (BEAM)** | ● | ● | ● | ● | ● | ● | ◐ | ◐ | ○(dyn) | ● |
| **Rust (tokio+ractor/kameo)** | ● | ● | ● | ● | ◐ | ○ | ●(flecs-rs/bevy) | ● | ● | ● |
| **Pony** | ● | ● | ● | ◐ | ◐ | ○ | ◐ | ● | ● | ○(pre-1.0) |
| **OCaml 5 (eio + Riot)** | ◐ | ● | ● | ● | ◐ | ○ | ◐ | ● | ● | ◐ |
| **Crystal** | ○(CSP) | ● | ◐(preview) | ● | ○ | ○ | ◐ | ● | ◐ | ◐ |
| **Idris2** | ◐ | ◐ | ○ | ◐ | ○ | ○ | ○ | ◐ | ●●(dep/linear) | ○(research) |
| **Smalltalk/Pharo** | ◐ | ● | ◐ | ◐ | ◐ | ●(live image) | ◐ | ◐ | ○ | ◐ |

### BEAM family — Erlang / Elixir / Gleam  ·  **lead candidate**
The BEAM was *built* for R1–R6. Millions of cheap, isolated processes; a
preemptive **SMP scheduler** that gives real multicore parallelism; **per-process
GC** (no stop-the-world); **OTP supervision trees** = "let it crash" fault
tolerance; **hot code reloading** = R6 hot-swap as a native runtime feature, not a
hack; built-in distribution (S6 for free). This is the closest thing on Earth to
the spec, and the workload (I/O-bound coordination + supervise external scanners)
is exactly what BEAM is best at.
- **Elixir** — the mature, ergonomic choice; huge ecosystem (Phoenix, Broadway,
  Nx); best docs/hireability. Dynamically typed (S1 weak).
- **Gleam** — static Hindley-Milner types + **type-safe OTP actors** (`gleam_otp`
  v1.x), compiles to Erlang, inherits the whole BEAM/Elixir library world; reached
  first stable in 2024 with very high developer admiration. Buys S1 without leaving
  BEAM. Younger ecosystem; hot-code-reload story is less paved than Elixir's.
- Engine weakness: not a *native* ECS — model the world as a process registry +
  **ETS/Mnesia** component tables (in-memory, transactional, snapshot-able → R7/R8
  fit well in practice). Heavy CPU goes to NIFs/ports (R9), which is fine because
  the spec says scanning lives in external cells anyway.
- **Pick Gleam if we want types; Elixir if we want maturity/speed-to-v1.**

### Rust (tokio + ractor/kameo, + hecs/bevy_ecs, or drive flecs-rs)  ·  **strong alternative / native-cell language**
Native AOT performance, memory safety without GC, the best FFI/subprocess story of
any candidate, and a deep security/networking ecosystem. `ractor` gives Erlang-like
supervision trees; `kameo` gives fault-tolerant async actors on tokio; `bevy_ecs`/
`hecs` give a real in-process ECS; **flecs-rs lets Rust drive the existing
OPACK/flecs core** — so a "refactor" could keep the C ECS and rewrite only the
orchestration in Rust. Costs: **no native hot code reload** (R6 needs `libloading`/
plugin gymnastics), the actor model is a library not the language (R1 is by
discipline + types, not enforced like Pony/BEAM), and it's the slowest to write.
**Best where R9/S2 dominate — the native cells — and a viable whole-engine choice
if hot-swap can be relaxed to "restart a subtree".**

### Pony  ·  **purest on paper, highest risk — spike, don't bet**
The only language whose *type system* enforces the spec: actor-model native,
**reference capabilities** (`iso/trn/ref/val/box/tag`) give compile-time data-race
freedom *and* a capability-security model tailor-made for offensive code, **per-actor
concurrent GC**, AOT, real parallelism. But it's **pre-1.0 with breaking changes**,
a tiny ecosystem (you'd write HTTP/TLS/DNS/security libs yourself), **no hot code
reload**, and a small community. For a single/paired tool that must ship, that's
too much yak-shaving. **Worth a research spike to steal ideas (the cap model), not
the refactor target.**

### OCaml 5 (eio + Riot)  ·  **typed-functional native alternative to BEAM**
OCaml 5 brought **domains** (parallelism) + **effect handlers** (direct-style
concurrency); **eio** is the effects-based async I/O stack; **Riot** is an
in-development Erlang-style actor lib (processes, supervision, message passing) on
top. Strong static types, fast native code, excellent C FFI, real production use
(Jane Street, Docker on eio, ICFP'25). The actor/supervision layer (Riot) is young
and the effects ecosystem is still settling (Picos is unifying schedulers). **The
pick if you want BEAM-style actors with AOT + a strong type system and are willing
to ride a maturing stack.**

### Crystal  ·  **keep for individual cells, not the engine**
Already in the arsenal (noir is Crystal); Ruby-grade ergonomics, AOT, clean C FFI.
But **parallelism is still preview** after ~6 years (`-Dpreview_mt`,
`-Dexecution_context`; the 2025 "execution contexts" work renamed to
Concurrent/Parallel and is improving but not default), the model is **CSP (fibers +
channels), not supervised actors** (R1/R5 weak), and there's no hot-swap. Great for
writing a fast standalone probe/cell that the engine supervises — consistent with
noir — **not the swarm runtime.**

### Idris2  ·  **a verification layer, not the runtime**
Dependent + **linear/quantitative types (QTT)** and **session types** can *prove*
the dangerous protocols correct: the scope-authorization invariant (R10), the
finding state-machine (no "report before validated"), the scoring rules (R8). It has
channels/message-passing primitives (Chez backend) but is **research-grade** — no
ecosystem, no perf story, not a system-builder. **Use it to specify/verify the
safety-critical state machines, then implement them in the engine language.** High
leverage, low surface area.

### Smalltalk / Pharo  ·  **the live operator console, not the engine**
The **live image** is native hot-swap + total runtime introspection — a fantastic
operator cockpit to watch/steer a running hunt. But weak performance, weak security/
networking ecosystem, single-image fragility. **Consider for the human's outer-ring
console; not the swarm.**

### Honorable mentions (named, not shortlisted)
- **Go** — goroutines/CSP, superb subprocess/FFI, and *the arsenal's scanners are
  already Go*; but no actor isolation/supervision and no hot-swap → great cell
  language, wrong engine. **Akka/Pekko on Scala/JVM** — mature actors+supervision,
  but JVM weight and ops cost overshoot single/paired scale. **Zig** — has a flecs
  binding (zflecs) and could drive OPACK with manual control, but concurrency is
  hand-rolled. **Nim** — flexible, async, C FFI, but actor/supervision is library-thin.

## Candidate shortlist for the refactor (ranked)

Engine-only. Which languages are viable to rewrite the swarm engine in, best fit first:

1. **Gleam (BEAM)** — type-safe actors + OTP supervision + hot code reload + millions of processes. Best fit to R1–R6 with static types.
2. **Elixir (BEAM)** — same runtime, more mature, faster to ship; dynamically typed.
3. **Rust (ractor/kameo + an ECS, or driving flecs-rs)** — native perf and the only path that keeps the existing C/flecs core; weak on hot-swap.
4. **OCaml 5 (eio + Riot)** — typed, AOT, BEAM-style actors on a maturing stack.
5. **Pony** — purest fit on paper (cap-secure actors), but pre-1.0 / thin ecosystem → spike, not bet.

Evaluated and **not** viable as the engine: **Crystal** (parallelism still preview, CSP not supervised actors), **Idris2** (research-grade, no runtime story), **Smalltalk/Pharo** (weak perf/ecosystem). Erlang is viable but Gleam/Elixir dominate it for this use.

## Sources
- Pony — [ponylang/ponyc](https://github.com/ponylang/ponyc), [Reference Capabilities (tutorial)](https://tutorial.ponylang.io/reference-capabilities/reference-capabilities.html), [Deny Capabilities for Safe, Fast Actors (paper)](https://www.ponylang.io/media/papers/fast-cheap.pdf)
- Crystal — [Execution Contexts RFC-0002](https://github.com/crystal-lang/rfcs/blob/main/text/0002-execution-contexts.md), [Parallel Programming in Crystal 2025](https://dev.to/kojix2/a-practical-guide-to-parallel-programming-in-crystal-2025-1lbg), [Crystal 1.17 release](https://crystal-lang.org/2025/07/16/1.17.0-released/)
- Gleam — [gleam-lang/otp](https://github.com/gleam-lang/otp), [gleam_otp v1.2 docs](https://hexdocs.pm/gleam_otp/index.html), [Gleam & the BEAM scheduler under load](https://blog.veydh.com/2025/2025-03-gleam-beam-concurrency-benchmark/)
- OCaml 5 — [eio](https://github.com/ocaml-multicore/eio), [awesome-multicore-ocaml (Riot, Picos, domainslib)](https://github.com/ocaml-multicore/awesome-multicore-ocaml), [Eio 1.0 release (Tarides)](https://tarides.com/blog/2024-03-20-eio-1-0-release-introducing-a-new-effects-based-i-o-library-for-ocaml/), [Jane St/Docker on OCaml 5 @ ICFP'25](https://anil.recoil.org/notes/icfp25-ocaml5-js-docker)
- Rust actors — [Comparing Rust actor libraries (Actix/Coerce/Kameo/Ractor/Xtra)](https://tqwewe.com/blog/comparing-rust-actor-libraries/), [kameo](https://github.com/tqwewe/kameo), [ractor](https://github.com/slawlor/ractor)
- Elixir/BEAM — [Hot Code Reloading in Elixir (AppSignal)](https://blog.appsignal.com/2021/07/27/a-guide-to-hot-code-reloading-in-elixir.html), [Erlang hot code updates (Underjord)](https://underjord.io/how-i-use-erlang-hot-code-updates.html)
- flecs bindings — [SanderMertens/flecs](https://github.com/sandermertens/flecs), [Flecs-Rust](https://github.com/Indra-db/Flecs-Rust), [Flecs.NET](https://github.com/BeanCheeseBurrito/Flecs.NET)
- Idris2 — [Idris 2: QTT in Practice (paper)](https://arxiv.org/abs/2104.00480), [System.Concurrency.Channels](https://www.idris-lang.org/docs/current/base_doc/docs/System.Concurrency.Channels.html)

— research by Temper
