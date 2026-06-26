# advanced_evolution: LLM-Driven Kit Evolution Engine

## What This Is

A **Darwinian optimizer** that evolves ARSON-AL's kit configurations (payload strings, SAST rulesets, probe patterns) via generational mutation and LLM-guided selection.

**Core loop:** Problem → Mutate Configs → Evaluate → Score → Select Best → Repeat

This is the **learn-by-doing** layer: given a failed finding or low-confidence result, automatically explore variants and score which config works better.

## Source Path
- Repo: `/home/user/advanced_evolution`
- Evolver: `darwinian_evolver/` (core GA engine)
  - `evolver.py` — Evolution loop (mutation, crossover, selection)
  - `population.py` — Candidate kit configs (genotypes)
  - `problem.py` — Problem model (scoring target)
  - `git_based_problem.py` — Git-sourced test cases
  - `learning_log.py` — History + replay for learning
  - `storage.py` — Persist evolving populations
- Examples: `example_problems/` (test harnesses for demo problems)
- Experiments: `imbue_experiments/` (research logs)

## Substrate Role

In the **Opaca engine**, this becomes the **kit evolution loop**:

1. **Finding arrives**: "SQLi in login endpoint, probe found 12 variants, but only 1 exploitable"
2. **Evolution triggered**: Evolve payload strings (fuzz mutations)
3. **Scoring**: Each variant gets CVSS/exploitability score from neural scorer
4. **Selection**: Best-scoring variants encoded back into kit config (probes/picks/proofs)
5. **Replay**: Log the successful mutation so future similar targets use the evolved config

**Examples of evolved kits:**
- WAF-bypass payloads (learned obfuscation patterns)
- SAST rulesets (precision tuning to reduce false positives)
- Probe timing strategies (optimal sleep/retry intervals for target classes)
- Injection vectors (evolve charset, encoding, payload structure)

## Implementation Notes

**Code style caveat:** Uses Python (violates ARSON-AL's "no Python when practical" rule). Rationale: LLM-guided mutation + generation is simpler in Python; the evolved configs (JSON/TOML) are language-agnostic and exported to Rust/Go probes.

**Integration**: The evolver outputs **config snapshots** (TOML files in ops/optimizer/configs/) which are imported by probes, picks, and proofs at startup.

## Bulk Deferred

Full repo (extensive example harness, visualization dashboards) copied on follow-up. For now:
- `pyproject.toml` — Dependencies
- `README.md` — Feature overview

Core Python modules (evolver.py, population.py, problem.py) are lightweight and can be copied selectively once Opaca substrate foundation is ready.

## Copied Files

- `README-adv-evo.md` — Feature overview
- `pyproject.toml` — Python dependencies + build config

## Next Steps (Follow-Up)

```bash
# Copy Python evolver when integrating learning loop
bulk: clone /home/user/advanced_evolution/darwinian_evolver -> /home/user/ARSON-AL/ops/optimizer/advanced_evolution/
# Integration: Call via subprocess or embedded Python for evolving kit configs
```
