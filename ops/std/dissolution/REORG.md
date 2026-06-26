# ops/ reorg — four buckets (actors / mem / std / hyg)

ops/ was dissolved into four buckets. Old -> new top-level map:

| old path | new path |
|----------|----------|
| ops/substrate/{opack,ruv-fann,synaptic-mesh,neural-bridge,OPACA.md,SWARM-ECS-SPEC.md,LANGUAGE-CANDIDATES.md} | ops/actors/* |
| ops/orchestration/loki-mode | ops/actors/loki-mode |
| ops/orchestration/{ruv-fann,synaptic-mesh} (pointers) | ops/actors/orchestration/ |
| ops/gamification/guild-hall | ops/actors/guild-hall |
| ops/optimizer/advanced_evolution | ops/actors/advanced_evolution (canonical) + ops/mem/ pointer |
| ops/substrate/loki-mode-memory | ops/mem/loki-mode-memory |
| ops/standards/ponytail | ops/std/ponytail |
| ops/research-methodology.md | ops/std/research-methodology.md |
| ops/dissolution | ops/std/dissolution |
| ops/standards/ast10 | ops/hyg/ast10 |
| ops/standards/loki-mode | ops/hyg/loki-mode |
| ops/DeTTECT | ops/hyg/DeTTECT |
| ops/c2-infrastructure | picks/c2-infrastructure (C2 = ways in) |

Bucket meaning: **actors** = engine + runtimes; **mem** = memory/state/scoring;
**std** = standards of practice (rules + methodology); **hyg** = hygiene
(compliance / quality gates / detection coverage). Per user: "gamification is
substrate is actors; optimizer is memory and actors." The wave-2 per-repo
manifests still cite pre-reorg paths — this table is the index.
