# ops/ reorg — four buckets, by what each thing does

ops/ dissolved into four buckets. Sorted by function, not by where it sits.

## actors/ — the engine and the runtimes that hunt

| component | what it does |
|-----------|--------------|
| opack | C++ ECS engine: branch / merge / prune / state |
| OPACA.md, SWARM-ECS-SPEC.md, LANGUAGE-CANDIDATES.md | the engine spec + the OCaml/Riot decision |
| ruv-fann (+ ruv-swarm) | neural compute + the swarm-orchestration runtime |
| synaptic-mesh | distributed multi-node swarm + consensus (reference) |
| neural-bridge | WASM neural inference — the deployable scoring engine |
| loki-mode | autonomous engagement-loop runtime (RARV) |
| advanced_evolution | evolutionary kit-config optimizer; runs as an actor |
| guild-hall | gamified engagement-scoring model |

## mem/ — what the engine remembers and scores

| component | what it does |
|-----------|--------------|
| loki-mode-memory | episodic / semantic / procedural memory store |
| advanced_evolution (pointer) | the learning loop — configs evolved from scored history |

## std/ — the standards we work by

| component | what it does |
|-----------|--------------|
| ponytail | the lazy-coding ladder every tool is written under |
| research-methodology.md | graded-evidence research method |
| dissolution/ | the dissolution method + wave manifests + dedup index |

## hyg/ — what keeps it clean, compliant, covered

| component | what it does |
|-----------|--------------|
| ast10 | OWASP agentic-skills compliance framework |
| loki quality-gates | review + anti-sycophancy quality gates |
| DeTTECT | ATT&CK detection-coverage → evasion-surface tooling |

## left ops/ entirely

| component | what it does |
|-----------|--------------|
| c2-infrastructure → picks/ | C2 server provisioning — a way in, not engine meta |

Old per-repo manifests under `std/dissolution/` still cite pre-reorg paths; sort
by the function above, not the stale path.
