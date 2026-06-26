# Duplicates & homomorphs — dedup manifest

Two kinds of redundancy across the dissolved arsenal:

- **Duplicates (literal)** — *identical content* (same bytes), regardless of
  filename/path. Located by content hash, not by name. Safe to prune to one copy.
- **Homomorphs (essentially)** — *different* components that do the same job
  (functionally equivalent, not byte-identical). Can't be hashed — a judgment
  call. Recombination targets: pick one canonical, route the rest onto it.

## Duplicates (literal) — method + findings

Located by content hash (stdlib, no extra dep):

```sh
find ops picks int probes proofs -type f -not -path '*/.git/*' \
  | xargs sha256sum | sort | uniq -w64 -D     # groups identical-content files
```

Headline: **23,906 files hashed → 2,919 distinct contents are duplicated →
6,841 file instances are a copy of something else** (~29% of files are redundant
bytes). Duplicate instances by component (where the redundant copies sit):

| component | dup file instances | why |
|-----------|-------------------:|-----|
| `ops/actors/synaptic-mesh` | 4,493 | re-vendors QuDAG (≥3× internally), ruv-swarm, and claude-flow that already exist elsewhere; plus `src/rs/QuDAG` vs `src/rs/daa/.../qudag` vs `standalone-crates/synaptic-qudag-core` |
| `ops/actors/ruv-fann` | 1,599 | nested copies of ruv-swarm under `npm/`, `tests/test-*`, `crates/*/.claude` |
| `probes/vigolium` | 394 | wave-1 — vendored test fixtures / grammars |
| `probes/noir` | 203 | wave-1 — vendored tree-sitter grammar duplicates |
| `picks/fuzz-skill` | 64 | wave-1 — multiple sanitizer builds of the same harness |
| `ops/actors/loki-mode` | 32 | repeated `.claude` command packs |
| (long tail) | — | identical `LICENSE`/`.gitignore`/empty `__init__`/`package.json` across vendored trees — trivial, ignore |

The signal is **synaptic-mesh + ruv-fann = ~6,100 of the 6,841 dup instances**:
the wave-2 substrate monorepos ship the same crates (QuDAG, ruv-swarm,
claude-flow, neuro-divergent) more than once. Pruning to one canonical copy each
removes most of the redundant ~365M footprint.

> Note: file-level hashing surfaces a large trivial tail (LICENSE, configs, empty
> files identical across repos). The meaningful dups are whole re-vendored
> *components* — confirmed both by the hash hotspots above and by the recon pass
> (QuDAG was flagged a triple-copy during dissolution).

## Homomorphs (essentially the same)

Functionally equivalent, different bytes — not hash-detectable. Recombination
targets for the Opaca build:

| capability | equivalent implementations | canonical candidate |
|------------|----------------------------|---------------------|
| **agent/swarm orchestration** | `ruv-swarm` (ruv-fann) · `daa-swarm` (synaptic-mesh) · `loki-mode/swarm` · Opaca actors | Opaca actors on OPACK |
| **engagement loop** | loki RARV (Reason-Act-Reflect-Verify) · Opaca kit-switch loop · ruv-swarm orchestration | Opaca kit-switch rules |
| **neural inference** | ruv-fann core · neural-bridge (WASM) · synaptic-neural-wasm / kimi-fann | one scoring engine in `mem/` |
| **scoring / economics** | guild-hall gamification · Opaca severity score (P1=100…) · loki token-economics / ToolOrchestra · synaptic `claude_market` reputation | Opaca gamified score |
| **gated / adversarial verification** | ruvn (scout→grade→synth→fact-check→cite) · loki quality-gates (3-reviewer + anti-sycophancy) · Scorpio sentinel→auditor · lance 7-gate / bug-reaper 4-phase (wave-1) | one proofs/ validation gate |
| **memory system** | loki episodic/semantic/procedural · ruv-swarm-persistence · agentdb / reasoningbank patterns | `mem/` unified store |
| **ATT&CK technique mapping** | DeTTECT `group_mapping` + actor corpus (`hyg/`) · `int/mitre-attack` (wave-1) | one ATT&CK index in int/ |
| **distributed consensus** | QuDAG QR-Avalanche (synaptic) · loki BFT/raft swarm | QuDAG (only if multi-node) |

## Implication

Literal-dup rows prune to one copy (most of synaptic-mesh's and ruv-fann's
redundant bytes). Homomorph rows collapse to one canonical on the OPACK/Opaca
substrate. This file is the dedup index; the per-repo wave-2 manifests
cross-reference it.
