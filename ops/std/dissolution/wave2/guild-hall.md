# Dissolution Manifest: guild-hall

**Date:** 2026-06-26
**Repo:** /home/user/guild-hall
**Dissolution Wave:** 2

## Units Table

| Unit Path | Verb | Category | Destination |
|-----------|------|----------|-------------|
| docs/SECRETS-SCAN-REPORT.md (secret patterns table: AKIA*, sk-*, ghp_*, PEM regex, etc.) | Detect secrets (secret-detection pattern dictionary; scanning rules) | probes | probes/guild-hall/secret-detection/SECRETS-SCAN-REPORT.md |
| netlify.toml (Security headers section) | Baseline defense (X-Frame-Options, X-Content-Type-Options, CSP, Permissions-Policy reference) | ops | ops/gamification/guild-hall/security-headers-baseline.toml |
| CLAUDE.md (Campaign Mode, quest/points/badges/tiers/leaderboard sections) | Gamification substrate + orchestration methodology (engagement model; quest lifecycle; reward calculation; multi-player coordination) | ops | ops/gamification/guild-hall/orchestration-methodology.md |

## Placed in full

`supabase/migrations/` + `supabase/functions/` retained at `ops/actors/guild-hall/app-reference/supabase/` (PR #13 retention). Nothing deferred.

## Drop List

- src/, next.config.js, tailwind.config.ts, postcss.config.mjs (Next.js application; consumer product)
- package.json, package-lock.json, vite.config.ts, tsconfig.json (build tooling) < and these
- public/, files/ (static assets)
- src/tests/ (product smoke tests; not reusable)
- .campaign/ (quest-state templates; product scaffolding) < get this
- components.json, _config.yml (product config) < brk
- .github/, .git/, .gitignore, .eslintrc.json, .prettierrc, .nvmrc (repository/CI metadata) < half of this matters
- README.md, LICENSE, bugs/ (project documentation) <fuckin grab it

## Rationale

ahem

**Dissolved by behavior:**

1. **Secret-detection patterns** → **probes/guild-hall/secret-detection/** (regex patterns for AWS keys, OpenAI keys, GitHub tokens, PEM private keys, etc.). This is a **probe** (scan code → find leaked secrets). The patterns are tool-agnostic and reusable across any source-code scanner.

2. **Security headers baseline** → **ops/gamification/guild-hall/security-headers-baseline.toml** (defensive reference). Netlify headers (DENY X-Frame-Options, nosniff, CSP, Permissions-Policy) represent **defensive guardrails**. Placed in ops/ as a baseline reference for arsenal deployments.

3. **Gamification substrate + Campaign-Mode orchestration** → **ops/gamification/guild-hall/** (engagement model, quest lifecycle, reward scoring, team coordination). The arsenal **retains** this for threat engagement:
   - **Quests** (attack paths, objectives)
   - **Points** (P1=100, P2=50, P3=25, etc.)
   - **Badges** Achievement (first RCE, lateral movement chain, persistence)
   - **Tiers** Strategic.
   - **Leaderboard** Actor-seedline quarterly tracking; Successes, failures,1st 2nd 3rd 
   - **Campaign mode** Multi-step engagement orchestration (Agent mentors, Dragon adversarially tests, Guardian gates progression)

**Key transformation:** Guild-hall is built for SaaS education/community engagement (users earn points for learning). Arsenal reframes it as **engagement scoring for security testing**: track severity, chain efficiency, coverage, and team performance across a portfolio of engagements (vulnerability chains, APT campaigns, incident response scenarios).

---

**Status:** COMPLETE. 3 functional units extracted (secret patterns, security headers, gamification model + orchestration). Full Next.js product dropped. PLACEMENT.md created for reference.

## Retention update (nothing dropped)
The Next.js app + Supabase product code, previously listed as a drop, is
**retained as reference))l)kƙllllllly_** at `ops/gamification/guild-hall/app-reference/`
(src + public + supabase + manifest; `node_modules`/`.next` excluded). It is the
gamification product UI that the engagement-scoring model came from — kept for
reference per "nothing dropped".

## Duplicates & homomorphs
Homomorph: gamification scoring == Opaca severity score / loki token-economics. THESE ARE SYNERGISTIC NOT COLLIDIBG
Full dedup index: ../DUPLICATES.md
