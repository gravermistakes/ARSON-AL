<!-- generated: 1782419851 -->
# Crucible — wiring tools/skills as ECS cells to replace agents

Design proposals for the OPACK Migration (root `CLAUDE.md`). Goal: let the
compiled flecs ECS own orchestration, state, branching/merging/pruning — so we
stop spending an Agent **agent** on every coordination step. Agents are expensive,
drift, and serialize work; ECS systems are deterministic, parallel, and free.

**The thesis:** most of what we'd use agents for is *dispatch and state*, not
*reasoning*. Move dispatch/state into the ECS. Reserve the Agent for the few steps
that genuinely need judgment. One reasoning service replaces a swarm.

## Vocabulary (how the arsenal maps onto flecs)

- **Entity** — a unit of work/data: a `Target`, a `Surface` (endpoint), a
  `Candidate` finding, an `Evidence` blob, a `Report`.
- **Component** — data on an entity: `Scope{authorized}`, `Endpoint{method,path}`,
  `Class{ssti}`, `Severity{cvss}`, `Economic{capital,profit}`.
- **Tag** — a zero-size state marker: `Theoretical`, `Reproduced`, `Accepted`,
  `Rejected`, `NeedsAgent`.
- **System** — a query + a function. **This is where a dissolved tool lives.**
- **Relationship** — a typed edge: `(DerivedFrom, parent)`, `(EvidenceFor, finding)`.

---

## Proposal 1 — Tool-as-System (capability cells, no dispatcher agent)

Each dissolved tool registers as a system whose **query is its trigger** and
whose **writes are its output**. No agent decides "should I run noir?" — the
scheduler runs it exactly when matching entities exist.

```
// noir: produce an endpoint inventory from any un-mapped source tree
system Noir   query: SourceTree, !EndpointInventory   -> spawn Endpoint entities + add EndpointInventory
system Drogon query: SourceTree, !SastDone            -> spawn Finding(Class), add SastDone
system Vigolium query: LiveTarget, !Scanned           -> spawn Finding(Class, Evidence), add Scanned
```

**Replaces:** the "module-dispatch / which-scanner-do-I-pick" agent. The
`probes/` registry becomes a set of systems; selection is query-matching.

## Proposal 2 — The four loops are tag state machines (no orchestrator agent)

lance's 7 gates and bug-reaper's 4 phases become **tag transition systems** on a
`Finding`. Branching/merging/pruning is pure ECS — C++ owns it, per the root doc.

```
Finding: Theoretical -> Reproduced -> EconValidated -> TriageAccepted -> Reportable
                                   \-> Rejected (pruned)

system G3_Exploit  query: Finding, Theoretical            -> +Reproduced | +Rejected
system G4_Economic query: Finding, Reproduced, MarketClass-> +EconValidated | +Rejected
system G5_FalsePos query: Finding, EconValidated          -> +Accepted | +Rejected
system G6_Triage   query: Finding, Accepted, Severity>=Med -> spawn Report
```

The **chain loop** is just a query: `Finding, Rejected|Low` joined over
`(ChainsWith, ...)` relationships, searching for a combination whose summed
score crosses critical. No agent re-reads a transcript — it's a graph query.

**Replaces:** every per-phase "validation agent" / "triage coordinator". One
system per gate, running on all findings in parallel.

## Proposal 3 — Agent as a scarce component (the one place agents survive)

Don't give every cell an Agent. A system that hits genuine ambiguity attaches
`NeedsAgent{kind, prompt_ctx}` and stops. A **single reasoning drain system**
batches every `NeedsAgent` entity per tick, makes one batched call, writes the
verdict back as components, removes the tag.

```
system anyGate ... if undecidable -> +NeedsAgent{kind: "triage", ctx: finding_id}
system ReasoningDrain query: NeedsAgent   -> batch N, one Agent call, write verdicts, -NeedsAgent
```

Only four `kind`s ever need the model (per the root doc): **triage**,
**attack-path synthesis**, **report prose**, **novel problem-solving**. Everything
else is deterministic.

**Replaces:** N task-specific agents with **1** batched reasoning service. This
is the biggest agent-count cut — and it caps token spend by construction
(`NeedsAgent` queue length is observable and throttleable).

## Proposal 4 — Skills/playbooks as data, not actors (knowledge cells)

The detection knowledge (lance `vulnerabilities/`, bug-reaper picks,
blockchain-appsec, ATT&CK) becomes **rule entities**, not prompts to separate
agents. One detection system queries the rule table; adding a vuln class is
adding a row, not spawning an agent.

```
DetectionRule{class: reentrancy} -(AppliesTo)-> Chain{evm}
system Detect query: Surface, Chain  +  for each DetectionRule(AppliesTo, chain) -> Candidate
```

Skills with real procedure (the SKILL.md spines) stay as **prompt templates the
ReasoningDrain selects by `kind`** — the spine's "reference map" becomes the set
of rule-entities the system already has in the world, so the stale-path problem
from this session's one bug (`SYSADLOG` 1782416720) disappears at migration.

**Replaces:** "one agent per vuln class / per skill" with one system + many
rule rows.

## Proposal 5 — Provenance & scoring as systems (gamify thru OPACK)

Cross-loop links (the MANIFEST `cross_refs`) become relationships
(`DerivedFrom`, `EvidenceFor`, `ChainsWith`). Then:

- **Scoring system**: reads `Severity`/`Impact`/`Economic`, assigns points —
  "Score it" from the root doc, deterministic.
- **Pruning system**: `query: Finding, Patched | (PoCAttempts>=3)` -> delete.
- **shodansnipe recon cells**: its agents become systems carrying `RateLimit`
  / `Budget` components; the scheduler enforces concurrency via components, not
  via an agent negotiating with itself.

**Replaces:** the "scorekeeper" and "recon-coordinator" agents; rate/budget
control moves from prompt discipline to typed components.

---

## What this buys

| Before (agents) | After (cells) |
|---|---|
| 1 dispatcher + 1 agent per module | systems matched by query (0 dispatch agents) |
| 1 agent per phase/gate, serialized | 1 system per gate, parallel over all findings |
| every agent holds an Agent | 1 batched `ReasoningDrain`, throttleable |
| 1 agent per vuln class/skill | 1 detection system + N rule rows |
| orchestration in transcripts | orchestration in tags + relationships (C++) |

Net: agents collapse to **~1 reasoning service** plus deterministic systems.
Token cost becomes the length of the `NeedsAgent` queue, which is measurable and
bounded — instead of N agents each free-running.

## Smallest first step

Build one vertical slice in `opack`: `Target -> Surface -> Candidate -> Finding`
with Proposals 1–3 only (tool-as-system, tag gates, one ReasoningDrain). Wire
exactly one probe (drogonsec), one gate (G3 exploit), one Agent kind (triage).
Prove the loop runs with a single model call per ambiguous finding, then widen.

— Crucible, 1782419851
