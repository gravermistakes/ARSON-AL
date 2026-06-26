# Dissolution Manifest: ruvn

**Date:** 2026-06-26
**Repo:** /home/user/ruvn
**Dissolution Wave:** 2

## Units Table

| Unit Path | Verb | Category | Destination |
|-----------|------|----------|-------------|
| src/agents/scout.ts | Decompose research question into subqueries (research orchestration) | int | int/ruvn/research-pipeline/scout.ts |
| src/agents/web-searcher.ts | Gather raw intelligence via WebSearch MCP (research execution) | int | int/ruvn/research-pipeline/web-searcher.ts |
| src/agents/source-grader.ts | Grade evidence A/B/C/D by authority/freshness/relevance (adversarial verification) | int | int/ruvn/research-pipeline/source-grader.ts |
| src/agents/synthesizer.ts | Synthesize findings from A/B sources only (distilled threat intelligence) | int | int/ruvn/research-pipeline/synthesizer.ts |
| src/agents/fact-checker.ts | Adversarially verify claims in synthesis (quality gate) | int | int/ruvn/research-pipeline/fact-checker.ts |
| src/agents/citer.ts | Generate dossier markdown with citations; every claim cites a graded source | proofs | proofs/ruvn/dossier-generation/citer.ts |
| CLAUDE.md (Evidence grading rubric + adversarial verify discipline) | Methodology (A/B/C/D grading rubric; verification workflow) | ops | ops/research-methodology.md |

## Drop List

- bin/, __tests__/, scripts/ (build/test infrastructure)
- package.json, package-lock.json, tsconfig.json (build tooling)
- .claude/, .codex/, .openclaw/, .vscode/ (IDE/harness config)
- .github/, .git/, .gitignore (repository metadata)
- AGENTS.md, README.md, LICENSE, SYSTEM.md, install.md (project documentation)
- capability-table.json, cli-config.yaml, rvm.manifest.toml, trust.json (CLI metadata)
- optional-mcps/ (optional integration templates)

## Rationale

**ruvn** is a **research evidence pipeline**: decompose question → gather sources → grade evidence → synthesize → verify → cite. Dissolution sorted by verb + output type:

**Research pipeline (int/):**
- scout, web-searcher, source-grader, synthesizer, fact-checker all remain in **int/ruvn/research-pipeline/** because they collectively form an **intelligence-gathering and verification system**. Each agent processes the previous agent's output through a **quality gate** (grading → filtering → verification), forcing information to flow through adversarial checks. This discipline is a core arsenal capability: **gather → grade → verify → synthesize**.

**Report generation (proofs/):**
- citer → **proofs/ruvn/dossier-generation/** (transforms synthesized findings into cited dossier markdown). Report generation is **proof** of intelligence, not the intelligence itself.

**Methodology (ops/):**
- Evidence grading rubric (A/B/C/D framework) + adversarial verification discipline → **ops/research-methodology.md**. This is a **repeatable methodology**, not tied to the gamma-entrainment (40Hz neuro) use case. The rubric and pipeline can be applied to any threat research domain.

**Key principle:** The gamma-entrainment domain framing (compare neuro protocols, dosing, 40Hz) is the **example use case**. The arsenal preserves the **functional units** (scout/grader/synthesizer/fact-checker/citer agents) and the **methodology** (evidence grading + adversarial verification) as reusable patterns, independent of neuroscience. Future use cases: malware analysis, APT profiling, supply-chain research — all follow the same pipeline.

---

**Status:** COMPLETE. 7 functional units placed. No bulk items. All logic and methodology preserved; example domain context kept for reference.

## Duplicates & homomorphs
Homomorph of loki quality-gates / Scorpio sentinel-auditor / lance+bug-reaper gates (gated adversarial verification).
Full dedup index: ../DUPLICATES.md
