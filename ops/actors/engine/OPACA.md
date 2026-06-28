# Opaca

Opaca is a compiled daemon-frame for concurrent actor processes that run security engagement loops via kits and procedurally stochastic workflows.
Built on OCaml 5 and the Riot actor framework. Replaces the C++ OPACK
library entirely.

It does three things as one unified system:

1. **Runs actors** — deterministic rudimentary 'AI', with biases, tendencies, perception, state based reasoning, and actuation
2. **Orchestrates loops** — the exploit chaining topology and mentality
3. **Coordinates swarms** — multiple actors working the same target concurrently

## Core concept: Kits

A **kit** is a bundle of tools for a purpose. Examples:

- A web2-recon kit: scope parser + subdomain enumerator + endpoint extractor
- A web3-validation kit: lance 7-gate + triage simulator + scoring engine
- A sast-probe kit: drogonsec + noir + secret detector
- A chain-builder kit: PoC templates + chain templates + impact record
> note: **the above is way off but premise is there**

Kits map directly to the dissolved ARSON-AL component directories. A kit's tools
come from probes/, picks/, proofs/, int/ — wherever they landed by action.
A kit doesn't care what repo a tool came from. It cares what the tools
do together.

## Core concept: Actors and loops

An **actor** is one engagement thread — a Riot process running a kit.

When the actor gets a hit (a finding, a new surface, a failed hypothesis),
it makes a choice as to if it **switches kits**. It doesn't die and spawn a new actor. It changes its loadout and keeps going. At end of engagement its processes are added to the mesh, and its seed uid and **success score** are stored.


Example:
- Actor starts with a recon kit (int/ tools) → finds endpoints
- Switches to a scanning kit (probes/ tools) → finds a vuln
- Switches to an exploitation kit (picks/ tools) → builds a PoC
- Switches to a validation kit (proofs/ tools) → confirms exploitability
- Switches to a reporting kit (proofs/ tools) → writes the finding

A **loop** is the pattern of kit-switches that an actor cycles through.
The recon loop is: int/ kit → probes/ kit → findings refine the model →
back to int/ kit. The actor stays alive through the whole cycle.

Multiple actors run concurrently on the same target. One is in the recon
loop. Another is mid-scan. A third is validating a finding from an hour
ago. They communicate via Riot message passing — a finding from actor A
can trigger a kit-switch in actor B.

The **chain loop** is a background actor that listens to all other actors'
findings. When it sees enough low-severity results to braid into a
critical chain, it switches to the chain-builder kit and assembles the proof.

## How loops emerge

Loops are not hardcoded. They emerge from kit-switch rules:

```
if finding.new_surface    → switch to recon kit
if finding.vulnerability  → switch to exploit kit
if exploit.works          → switch to validation kit
if validation.confirmed   → switch to report kit
if validation.failed      → switch back to scan kit (wrong hypothesis)
if report.submitted       → switch to recon kit (next target)
```

The topology of concurrent loops arises from these rules running
across multiple actors. You don't configure a "recon loop" — you
configure kit-switch rules, and recon-loop behavior emerges.

## Supervision

Riot provides Erlang-style supervision. The daemon runs a supervision
tree:

```
Opaca (root supervisor)
├── TargetSupervisor (per-surface)
│   ├── Actor (recon loop, currently in int/ kit)
│   ├── Actor (scan loop, currently in probes/ kit)
│   ├── Actor (validation, currently in proofs/ kit)
│   └── ChainActor (background, listening)
├── TargetSupervisor (another surface)
│   └── ...
└── KitRegistry (knows what kits exist and their domain)
```

If an actor crashes (tool segfaults, network timeout, OOM), the supervisor restarts it with its last known kit and state. The engagement doesn't die because one probe failed. 

## Actor seeding

Multiple actors work the same engagement simultaneously. Each gets a different seed. The seed influences a variety of difficult to "solve" "traits". 

Seeds are logged.
- Debugging (why did actor 3 chain but actor 1 didn't?)
- Regression (re-run seeds after a kit change, verify same findings)
- Coverage proof (show N distinct paths were explored)

Seed quality: something esoterically derived.

## Agent connection

Opaca connects to the Agent layer through multiple interfaces, in
priority order:

1. **MCP server** — Opaca exposes its kits as MCP tools. The Agent
   can call `switch_kit`, `get_findings`, `submit_report`, `spawn_actor`.
   This is the primary interface for Claude Code integration.

2. **Stdin/stdout** — For direct Claude Code piping. Opaca reads
   structured commands, emits structured results. JSON lines.

3. **HTTP** — REST API for non-Claude Agents or dashboards. POST to
   `/actor/{id}/switch_kit`, GET `/findings`, etc.

The Agent does NOT run every step. The actors run their kits
autonomously — compiled OCaml executing the deterministic parts
(scope gating, evidence checks, severity scoring, dedup, state
management). The Agent gets called only when:

- **Triage judgment** — is this finding real or a false positive? (pushes to human)
- **Attack path generation** — given these surfaces, what's the play?
- **Report prose** — write the narrative for a human reviewer
- **Problem solving** — a tool failed in a way the rules don't cover
- **Kit selection ambiguity** — two kits could apply, which one?

Everything else is compiled code running on Riot's multicore scheduler.

## What Riot provides (don't rebuild)

- Actor spawn, supervision, crash recovery
- Multicore scheduling (actors across all cores automatically)
- Message passing between actors (findings, kit-switch signals)
- Algebraic effects for control flow (branching, cancellation)
- TCP/socket IO for MCP server and HTTP
- Telemetry for observability
- Built-in logging with structured output
- Timer wheels for timeouts and polling
- `--json` output for agent-readable structured data

## What needs building

- **Kit definition format** — how to declare a kit (what tools, what
  inputs, what outputs, what kit-switch rules)
- **Kit registry** — actor that loads kit definitions, resolves tool
  paths, validates dependencies
- **Kit-switch engine** — the rules that decide when an actor changes
  kits, based on findings and state
- **Finding type system** — OCaml types for findings (surface, vuln,
  exploit, chain, report) that carry CWE/CVSS/evidence
- **Tool adapters** — how Opaca invokes external tools (noir, vigolium,
  drogonsec are compiled binaries; scripts are subprocesses)
- **State persistence** — actor state survives daemon restart (SQLite
  or flat files — not an external database)
- **MCP server** — expose kits as MCP tools
- **Stdio bridge** — for Claude Code integration
- **Target manifest format** — what int/ emits and all actors consume

## What a kit definition looks like

```ocaml
(* Not final syntax — illustrating the shape *)

let web2_recon_kit = Kit.define
  ~name:"web2-recon"
  ~tools:[
    Tool.binary ~name:"noir" ~path:"probes/noir/noir"
      ~args:["--ai-context"; "-o"; "sarif"]
      ~output:Finding.surface;
    Tool.script ~name:"scope-parser" ~path:"int/web2/bug-reaper/analyze_scope.py"
      ~output:Finding.manifest;
  ]
  ~switch_rules:[
    On Finding.vulnerability, Switch_to "web2-exploit";
    On Finding.new_surface,   Switch_to "web2-recon";  (* loop *)
    On Finding.source_code,   Switch_to "sast-probe";
  ]
```

## How ARSON-AL tools become kits

The dissolved components in ops/int/probes/picks/proofs/ are the
raw material. Kits are assembled from them:

| Kit | Draws from | Purpose |
|-----|-----------|---------|
| web2-recon | int/web2, probes/noir, probes/web2 | Map web2 target surface |
| web3-recon | int/web3, probes/web3 | Map web3 target surface |
| web2-exploit | picks/web2, picks/ssti-research | Get in (web2) |
| web3-exploit | picks/web3 (future) | Get in (web3) |
| sast-probe | probes/drogonsec, probes/noir | Source-level scanning |
| fuzz | picks/fuzz-skill | Memory-bug fuzzing |
| web2-validate | proofs/web2 | Confirm + report (web2) |
| web3-validate | proofs/web3 | Confirm + report (web3) |
| chain-builder | proofs/web2, proofs/web3 | Braid P3s into P1s |
| osint-recon | int/osint-tools, int/h1dr4 | Person/infra research |
| redteam | int/h3retik, picks/h3retik, probes/h3retik | Full red-team ops |

Kits are composable. A new tool dissolved into picks/ automatically
becomes available to any kit that draws from picks/.

## Gamification

Actors earn scores for their engagement threads:

- Finding confirmed → points by severity (P1=100, P2=50, P3=20, P4=5)
- Chain completed → bonus (sum of parts × multiplier)
- False positive avoided → small bonus
- Tool crash → penalty (kit reliability signal)
- Time-to-finding → efficiency score

Scores feed back into kit selection — if the sast-probe kit has a
higher hit rate than the fuzz kit on this target type, prefer it.
Not hardcoded preferences. Learned from the engagement history.

## Name

Opaca. From alpaca — a domesticated load-bearing camelid.
It carries the tools, the loops, the state, so you don't have to.
Built on OCaml 5 + Riot. The engine that ARSON-AL rides on.
