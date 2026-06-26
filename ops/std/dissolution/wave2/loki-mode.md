# Dissolution: loki-mode → ops/orchestration + ops/standards + ops/substrate

**Wave:** 2 | **Date:** 2026-06-26 | **Verb-Based Sorting:** Autonomous engagement loop + Quality gates + Memory/state system

## Functional Units Table

| Unit | Verb (What It Does) | Category | Destination |
|------|---------------------|----------|-------------|
| `autonomy/run.sh` | **Execute RARV cycle** — Reason → Act → Reflect → Verify loop | ops/orchestration | `ops/orchestration/loki-mode/autonomy/run.sh` |
| `autonomy/loki` (CLI dispatcher) | **Dispatch commands** — Phase selection, human intervention, approval gates | ops/orchestration | `ops/orchestration/loki-mode/autonomy/loki` |
| `autonomy/completion-council.sh` | **Multi-agent consensus voting** — Council determines if engagement complete | ops/orchestration | `ops/orchestration/loki-mode/autonomy/completion-council.sh` |
| `skills/quality-gates.md` | **Define 10 triage gates** — Static analysis, 3-reviewer blind, anti-sycophancy, severity blocks | ops/standards | `ops/standards/loki-mode/quality-gates.md` |
| `memory/engine.py`, `memory/retrieval.py`, `memory/consolidation.py` | **Episodic/semantic/procedural memory** — Store traces, generalize patterns, consolidate skills | ops/substrate | `ops/substrate/loki-mode-memory/` |
| `memory/storage.py` | **Persist memory to disk** — File-based backend (episodes, patterns, skills) | ops/substrate | `ops/substrate/loki-mode-memory/` |
| `memory/schemas.py` | **Memory data structures** — Typed schemas (Episode, Pattern, Skill) | ops/substrate | `ops/substrate/loki-mode-memory/` |
| `dashboard/server.py` | **Runtime telemetry + manual override** — Real-time progress, pause/stop/input signals | ops/orchestration | `ops/orchestration/loki-mode/dashboard/` |
| `dashboard/static/` (web UI) | **Browser-based control** — Visualize phases, findings, override decisions | ops/orchestration | `ops/orchestration/loki-mode/dashboard/static/` |
| `references/legacy-healing-patterns.md` | **Friction-as-semantics for old systems** — Preserve institutional knowledge, incremental modernization | ops/standards | `ops/standards/loki-mode/legacy-healing.md` |
| `references/openai-patterns.md` | **OpenAI Agents SDK patterns** — Guardrails, tripwires, handoffs | ops/standards | `ops/standards/loki-mode/agent-patterns.md` |
| `references/production-patterns.md` | **2025 what-works patterns** — HN learnings on production AI systems | ops/standards | `ops/standards/loki-mode/production-patterns.md` |
| `providers/` (multi-provider abstraction) | **Support Claude/Codex/Gemini** — Provider-agnostic interface | ops/orchestration | `ops/orchestration/loki-mode/providers/` |
| `skills/`, other methodology docs | **Development methodology** — SPARC phases, TDD, testing patterns | ops/standards | `ops/standards/loki-mode/` |
| `SKILL.md` | **Loki-mode specification** (reference for Opaca design) | ops/orchestration | `ops/orchestration/loki-mode/SKILL.md` |
| Tests, examples | **Validation suite** | proofs/templates | `proofs/templates/loki-mode/` |
| `Dockerfile`, deployment | **Containerization** | ops/substrate | `ops/substrate/loki-mode-deployment/` |

## Drop List

- `.git/`, `.github/` — Version control
- `.loki/` state files (session.json, etc.) — Runtime state, not part of codebase
- `.loki-test-tmp/` — Test scaffolding
- `web-app/` (frontend build) — Defer full build to follow-up
- `vscode-extension/` — IDE plugin (not core to Opaca)
- Large test suites (dashboard E2E tests with Playwright) — Reference only

## Rationale

loki-mode is the **reference autonomous system** and strongest demonstration of how Opaca should orchestrate itself. Three orthogonal layers:

### 1. Autonomy Loop (RARV Cycle) → ops/orchestration

```
REASON        → Analyze target/findings, update threat model
ACT           → Spawn probes (probes/), fuzz payloads (picks/), validate (proofs/)
REFLECT       → Analyze results, detect chains, update repo state
VERIFY        → Council vote (3+ reviewers), anti-sycophancy check
→ Repeat or escape (completion council votes done/continue)
```

This is **Opaca's skeleton**. Opaca will recombine this loop onto its compiled (C++/OPACK) substrate instead of bash, but the structure is identical.

### 2. Quality Gates → ops/standards

**10-gate finding triage discipline** (strongest reusable piece):

1. Static analysis (CodeQL, ESLint) — Code quality baseline
2. 3-reviewer parallel blind review — Unbiased evaluation
3. Anti-sycophancy check (devil's advocate) — Ensure dissent
4. Severity-based blocking — Critical/High/Medium blocks ship
5. Test coverage gate (>80% unit, 100% pass) — Quality threshold
6. Backward compatibility (healing mode) — Don't break old systems
7-10. Custom domain gates (crypto, cloud, etc.)

For **Opaca**, translate these to **finding gates**:
- Gate 1: SAST static analysis (CodeQL + custom rules)
- Gate 2: 3-reviewer consensus (blind manual triage)
- Gate 3: Anti-sycophancy (if all reviewers agree exploitable, challenge once)
- Gate 4: CVSS + CWE-based blocking (critical bugs block ship)
- Gate 5: PoC required (must demonstrate working exploit)
- Gate 6: Healing (preserve friction in legacy systems, incremental upgrades only)
- Gate 7-10: Platform-specific (Immunefi vs Bugcrowd gate differences)

### 3. Memory System → ops/substrate

**Episodic → Semantic consolidation**:
- **Episodic**: Specific engagement traces ("SQLi on 10.0.0.5, payload X worked, took 120ms")
- **Semantic**: Generalized patterns ("SQLi on Apache+PHP, WAF-bypass charset ∈ {αβγ}, success rate 73%")
- **Procedural**: Learned skills ("optimal timing for timing attacks on AWS ELBs: 50ms poll interval")

Opaca will use this for:
- **Replay**: Similar target matches episodic pattern → use learned kit config
- **Transfer**: "I learned SQLi on Target A, applying to Target B"
- **Forecasting**: "Next high-confidence target matches semantic pattern #12 (prior success rate 81%)"

## OPACK/Opaca Caveat

loki-mode is a **complete competing system** — it has full CLI, dashboard, multi-provider support, multi-agent swarm. **Do NOT verbatim port loki-mode into Opaca.** Instead:

1. **Adopt RARV structure**: ops/orchestration/loki-mode/autonomy/run.sh is the pattern
2. **Implement quality-gates for findings**: Adapt loki's 10-gate system to triage discipline
3. **Use episodic-semantic pipeline**: Memory consolidation code is reusable (ops/substrate/loki-mode-memory/)
4. **Skip loki's runtime**: Dashboard, CLI, multi-provider abstraction are loki-specific

Opaca will recombine these patterns onto its own substrate (OPACK/C++ engine + neural layer).

## Bulk Deferred

Full loki-mode repo (370K+ lines, extensive Docker/CI, Playwright test suite) marked for follow-up:
```bash
bulk: clone /home/user/loki-mode -> /home/user/ARSON-AL/ops/orchestration/loki-mode/
# Use selectively: autonomy/ is essential, dashboard/ is optional, web-app/ is reference only
```

Currently copied: SKILL.md + autonomy core (run.sh excerpt) + quality-gates.md + memory code (structure captured).

## Next Steps (Implementing Opaca)

### Phase 1: Adopt RARV Loop
```
1. Copy autonomy/run.sh structure into Opaca's phase-machine
2. Implement Reason → Act → Reflect → Verify cycle
3. Wire council voting (3-reviewer consensus before shipping)
```

### Phase 2: Implement Quality Gates
```
1. Define finding gates (SAST → blind review → anti-sycophancy → CVSS block → PoC required)
2. Build finding triage CLI (approve/conditional/reject per gate)
3. Integrate 3-reviewer routing (assign reviewer teams)
```

### Phase 3: Wire Memory System
```
1. Implement episodic storage (raw findings + contexts)
2. Add semantic consolidation (generalize patterns)
3. Enable replay (similar target → learned kit config)
```

### Phase 4: Recombine onto Opaca Substrate
```
1. Replace bash RARV with Opaca's C++ phase-machine
2. Wire Opaca's neural-bridge for scoring
3. Implement Opaca's swarm (reference ruv-swarm patterns)
```

## Files Copied (Wave 2)

- `SKILL.md` → ops/orchestration/loki-mode/
- `autonomy/run.sh.excerpt` (first 100 lines) → ops/orchestration/loki-mode/autonomy/
- `skills/quality-gates.md` → ops/standards/loki-mode/
- `memory/*.py` → ops/substrate/loki-mode-memory/
- `README.md` → ops/orchestration/loki-mode/README-loki-mode.md

## Duplicates & homomorphs
Homomorphs: RARV loop (engagement loop), swarm (orchestration), token-economics (scoring), quality-gates (adversarial verification), episodic/semantic memory.
Full dedup index: ../DUPLICATES.md
