# Dissolution: advanced_evolution → ops/optimizer

**Wave:** 2 | **Date:** 2026-06-26 | **Verb-Based Sorting:** Genetic algorithm + LLM-guided mutation + Configuration evolution

## Functional Units Table

| Unit | Verb (What It Does) | Category | Destination |
|------|---------------------|----------|-------------|
| `darwinian_evolver/evolver.py` | **Run genetic algorithm loop** — Mutation, crossover, selection generations | ops/optimizer | `ops/optimizer/advanced_evolution/evolver.py` |
| `darwinian_evolver/population.py` | **Manage candidate kit configurations** — Genotype (config) → phenotype (behavior) | ops/optimizer | `ops/optimizer/advanced_evolution/population.py` |
| `darwinian_evolver/problem.py` | **Score kit variants** — Evaluate fitness (exploitability, WAF bypass rate, false-positive reduction) | ops/optimizer | `ops/optimizer/advanced_evolution/problem.py` |
| `darwinian_evolver/git_based_problem.py` | **Source test cases from Git** — Problem instances (vulnerable repos, targets, findings) | ops/optimizer | `ops/optimizer/advanced_evolution/git_based_problem.py` |
| `darwinian_evolver/learning_log.py` | **Record evolution history** — Mutations tried, winners selected, replay for future targets | ops/optimizer | `ops/optimizer/advanced_evolution/learning_log.py` |
| `darwinian_evolver/storage.py` | **Persist population across runs** — Save evolved configs to disk (JSON/TOML) | ops/optimizer | `ops/optimizer/advanced_evolution/storage.py` |
| `example_problems/` | **Demo evolution targets** — Sample problems for testing evolver | proofs/templates | `proofs/templates/advanced_evolution/` |
| `imbue_experiments/` | **Research logs and results** | proofs/templates | `proofs/templates/advanced_evolution/experiments/` |
| `scripts/`, tests | **Utility scripts** | ops/optimizer | `ops/optimizer/advanced_evolution/scripts/` |
| `pyproject.toml` | **Declare Python dependencies** | ops/optimizer | `ops/optimizer/advanced_evolution/pyproject.toml` |
| README | **Feature documentation** | ops/optimizer | `ops/optimizer/advanced_evolution/README-adv-evo.md` |

## Drop List

- `.git/`, `.github/` — Version control
- `.devcontainer/`, Dockerfile — Dev container (preserve in source, reference in docs)
- Large PNG visualizations (readme_*.png) — Too large; keep README reference only
- `.offload-image-cache` — Build cache

## Rationale

advanced_evolution is the **kit evolution engine** — given a problem (e.g., "SQLi bypass WAF filter X"), automatically evolve payload strings, probe parameters, or SAST rulesets to maximize exploitability while minimizing false positives.

Verb-based decomposition:

1. **Evolve**: Mutate kit config (payload charset, encoding, sleep timings, injection vector) → run against target → score result
2. **Score**: Neural network (or explicit rubric) rates variant fitness (exploitability: 0-100, false-positive rate: 0-50%)
3. **Select**: Keep top N variants, discard worst → next generation
4. **Log**: Record which mutations worked (enables transfer learning to similar targets)
5. **Persist**: Save evolved config as TOML/JSON importable by probes/picks/proofs

**Integration with Opaca**:

```
Finding arrives: "SQLi in login, standard payloads blocked"
  → advanced_evolution triggered
  → Evolve 10 payload variants (charset mutations, obfuscation)
  → Score each via neural-bridge (exploitability + WAF evasion)
  → Select best 3 variants
  → Log mutations → future SQLi probes use learned payloads
  → Export evolved config to probes/picks/proofs Cargo.toml
```

## Implementation Notes

**Python caveat:** Uses Python (violates ARSON-AL's "no Python when practical" rule). Justification:
- Genetic algorithm + LLM-guided mutation is naturally expressive in Python
- Evolved configs (JSON/TOML) are language-agnostic
- Can be called via subprocess from Rust/Go Opaca core
- Alternative: Rewrite in Rust (higher effort, less readable)

**LLM guidance**: The evolver can be enhanced with LLM-driven mutation suggestions:
```python
# Future enhancement
suggestion = llm.suggest_mutation(
    problem_type="SQLi WAF bypass",
    previous_winners=[payload1, payload2],
    generation=15
)
```

## Bulk Deferred

Full repo (research experiments, large test suites, visualization) marked for follow-up:
```bash
bulk: clone /home/user/advanced_evolution -> /home/user/ARSON-AL/ops/optimizer/advanced_evolution/
# Includes: Full experiment logs, example problems, visualization code
```

Currently copied: Core Python modules + pyproject.toml + README (structure captured).

## Next Steps

1. **Integrate into Opaca's learning loop** — When finding confirmed exploitable, trigger evolution
2. **Define fitness rubric** per vulnerability class (SQLi fitness ≠ XSS fitness ≠ crypto fitness)
3. **Implement LLM guidance** (optional enhancement for mutation suggestions)
4. **Export evolved configs** as Opaca's probes/picks/proofs update their TOML configs
5. **Measure transfer**: Do evolved payloads work on similar targets? (e.g., SQLi payloads learned on Target A work on Target B?)
