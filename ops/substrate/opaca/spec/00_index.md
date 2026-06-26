# Opaca — SPARC Specification (Specification + Pseudocode phases)

Spec modules for building **Opaca**, the OCaml 5 + Riot daemon that replaces the
C++ OPACK substrate. Source of truth for *what* Opaca is:
[`../../OPACA.md`](../../OPACA.md). Source of truth for the engine-level
requirements it inherits: [`../../SWARM-ECS-SPEC.md`](../../SWARM-ECS-SPEC.md)
(R1–R10) and the OCaml 5 + Riot decision in
[`../../LANGUAGE-CANDIDATES.md`](../../LANGUAGE-CANDIDATES.md).

These are SPARC **Specification** and **Pseudocode** phase artifacts — no final
code. Illustrative OCaml is shape, not syntax.

| Module | Phase | Covers |
|--------|-------|--------|
| [`phase_1_requirements.md`](phase_1_requirements.md) | Specification | Functional + non-functional requirements, constraints, success criteria, acceptance test |
| [`phase_2_finding_and_kit_types.md`](phase_2_finding_and_kit_types.md) | Pseudocode | Finding type system; kit definition format; tool-adapter + switch-rule types |
| [`phase_3_actor_and_kitswitch.md`](phase_3_actor_and_kitswitch.md) | Pseudocode | Actor lifecycle, kit-switch engine, supervision tree, seeded traversal |
| [`phase_4_interfaces_persistence_scoring.md`](phase_4_interfaces_persistence_scoring.md) | Pseudocode | MCP / stdio / HTTP interfaces, state persistence, gamified scoring + learned kit-selection |

## Substrate this rides on (wave-2 dissolutions)

Opaca is the engine; the wave-2 `ops/` placements are the layers it calls:

- **Neural scoring / learned kit-selection** → `ops/substrate/ruv-fann/`,
  `ops/substrate/neural-bridge/` (sub-100ms inference for severity scoring and
  "which kit next").
- **Gamified engagement scoring model** → `ops/gamification/guild-hall/`
  (quests→engagements, points→severity, leaderboard→seed coverage).
- **Kit-config evolution** → `ops/optimizer/advanced_evolution/` (evolve
  payload/probe configs against a scored evaluator once a finding is exploitable).
- **Distributed multi-node option** → `ops/substrate/synaptic-mesh/` (QuDAG
  branch-vote-merge as a kit-switch-consensus reference; not v1).
- **Loop/quality-gate methodology** → `ops/orchestration/loki-mode/`,
  `ops/standards/loki-mode/` (RARV loop + 10-gate triage discipline as reference,
  not a verbatim port).

## Reading order

1 → 2 → 3 → 4. Phase 1 gates the rest: an item not traceable to a requirement
in Phase 1 does not get built.

<!-- generated: spec; sign-off epoch appended at commit -->
