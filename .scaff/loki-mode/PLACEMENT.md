# loki-mode: Autonomous Engagement Loop & Triage Methodology

## What This Is

The **reference autonomous system architecture** for Opaca. loki-mode demonstrates:

1. **RARV Cycle** (Reason → Act → Reflect → Verify) — The engagement loop that powers Opaca
2. **Phase Machine** (reconnaissance, scan, validation, chain, report) — Taxonomy of arsenal stages
3. **Council Pattern** (multi-perspective voting) — Ensures no single perspective dominates
4. **Quality Gates** (10-gate system) — Triage discipline before findings ship
5. **Memory System** (episodic/semantic/procedural) — Agent-state layer for learning

## Source Path
- Repo: `/home/user/loki-mode`
- Autonomy loop: `autonomy/run.sh` + `autonomy/loki` (RARV cycle, phase machine)
- Council voting: `autonomy/completion-council.sh` (multi-agent consensus)
- Quality gates: `skills/quality-gates.md` + review system (10 gates, anti-sycophancy)
- Memory system: `memory/` (episodic/semantic + consolidation pipeline)
- Healing mode: `references/legacy-healing-patterns.md` (friction-as-semantics for old systems)
- Dashboard: `dashboard/` (runtime telemetry + manual overrides)

## Substrate Role

loki-mode is the **engagement loop reference** for Opaca. Key translations:

### RARV Cycle → Opaca Loop
```
REASON     → Analyze target, construct threat model (int/ components)
ACT        → Fire probes (probes/), fuzz (picks/), validate (proofs/)
REFLECT    → Analyze findings, update threat model, detect chains
VERIFY     → Run council vote, quality gates, anti-sycophancy checks
```

### Phases → Opaca Stages
```
reconnaissance → int/ target modeling
scan           → probes/ enumeration + fingerprinting
validation     → proofs/ triage + PoC confirmation
chain          → background loop braiding low findings into critical chains
report         → proofs/ report generation + platform submission
```

### Quality Gates → Triage Discipline
```
Gate 1: Static analysis (CodeQL, ESLint) — Code quality baseline
Gate 2: 3-reviewer parallel blind review — Unbiased technical eval
Gate 3: Anti-sycophancy check — Devil's advocate on unanimous approval
Gate 4: Severity-based blocking — Critical/High/Medium blocks ship
Gate 5: Test coverage gates — >80% unit, 100% pass before ship
Gate 6: Backward compatibility — Healing mode: preserve friction, don't break old systems
Gate 7-10: Custom domain gates (crypto, cloud, etc.)
```

**Strongest reusable piece:** `skills/quality-gates.md` + 3-reviewer methodology. This directly maps to Opaca's **finding triage** — every finding must survive 3-reviewer consensus + anti-sycophancy before shipping to a bug bounty platform.

### Memory System → Agent State
```
Episodic   → Specific engagement traces (e.g., "SQLi in login endpoint on target 10.0.0.5")
Semantic   → Generalized patterns (e.g., "common-config WAF bypass patterns")
Procedural → Learned skills (e.g., "optimal timing for timing attacks on AWS ELBs")
```

Uses episodic-to-semantic consolidation: raw findings consolidate into reusable knowledge.

## OPACK-Overlap Caveat

loki-mode is a **competing full autonomous system** — it has its own swarm orchestrator, its own memory system, its own phase machine. **Do NOT verbatim import loki-mode into Opaca.** Instead:

1. Adopt the **RARV loop structure** (reference pattern)
2. Reimplement **quality-gates** in Opaca (domain-specific: finding triage, not code review)
3. Reuse **healing-mode patterns** (friction-as-semantics for legacy probes/picks)
4. Study the **memory consolidation pipeline** (episodic → semantic is a strong pattern)
5. Skip the full runtime (dashboard, CLI, loki-mode-specific scoring) — Opaca recombines on its own substrate.

## Copied Files

- `SKILL.md` — Full loki-mode specification (for reference)
- `README-loki-mode.md` — Feature overview
- `autonomy/run.sh.excerpt` — First 100 lines of RARV loop (structure reference)
- `quality-gates.md` → `/ops/standards/loki-mode/` (reusable triage pattern)
- `memory/` → `/ops/substrate/loki-mode-memory/` (episodic-semantic consolidation code)

## Next Steps (Follow-Up)

When implementing Opaca's autonomous loop:

```
1. Adopt RARV cycle structure from loki-mode/autonomy/run.sh
2. Implement quality-gates per findings (not code, per CWE + CVSS)
3. Wire memory consolidation (episodic → semantic pipeline)
4. Custom 6-animal team coordination (Bear, Cat, Owl, Puppy, Rabbit, Wolf)
   Inspired by loki's 3-reviewer + council patterns.
```
