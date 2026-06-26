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

## Bulk/Deferred Items

| Item | Reason | Note |
|------|--------|------|
| supabase/migrations/, supabase/functions/ | Database schema + edge functions; tied to specific remote Supabase deployment | bulk: copy on follow-up if architectural interest; otherwise remains product-code |

## Drop List

- src/, next.config.js, tailwind.config.ts, postcss.config.mjs (Next.js application; consumer product)
- package.json, package-lock.json, vite.config.ts, tsconfig.json (build tooling)
- public/, files/ (static assets)
- src/tests/ (product smoke tests; not reusable)
- .campaign/ (quest-state templates; product scaffolding)
- components.json, _config.yml (product config)
- .github/, .git/, .gitignore, .eslintrc.json, .prettierrc, .nvmrc (repository/CI metadata)
- README.md, LICENSE, bugs/ (project documentation)

## Rationale

**guild-hall** is a **full-stack SaaS product** (Next.js + Supabase + React dashboard + campaign-mode orchestration). The arsenal extracts only **reusable engagement/gamification components**, NOT product code.

**Dissolved by verb:**

1. **Secret-detection patterns** → **probes/guild-hall/secret-detection/** (regex patterns for AWS keys, OpenAI keys, GitHub tokens, PEM private keys, etc.). This is a **probe** (scan code → find leaked secrets). The patterns are tool-agnostic and reusable across any source-code scanner.

2. **Security headers baseline** → **ops/gamification/guild-hall/security-headers-baseline.toml** (defensive reference). Netlify headers (DENY X-Frame-Options, nosniff, CSP, Permissions-Policy) represent **defensive guardrails**. Placed in ops/ as a baseline reference for arsenal deployments.

3. **Gamification substrate + Campaign-Mode orchestration** → **ops/gamification/guild-hall/** (engagement model, quest lifecycle, reward scoring, team coordination). The arsenal **reframes** this for threat engagement:
   - **Quests** = Engagements (attack paths, objectives)
   - **Points** = Severity scores (P1=100, P2=50, P3=25, etc.)
   - **Badges** = Achievement milestones (first RCE, lateral movement chain, persistence)
   - **Tiers** = Coverage levels (shallow recon, deep compromise, full dominance)
   - **Leaderboard** = Actor/seed coverage tracking (who owns what, efficiency metrics)
   - **Campaign mode** = Multi-step engagement orchestration (Gandalf mentors, Dragon adversarially tests, Guardian gates progression)

**Key transformation:** Guild-hall is built for SaaS education/community engagement (users earn points for learning). Arsenal reframes it as **engagement scoring for security testing**: track severity, chain efficiency, coverage, and team performance across a portfolio of engagements (vulnerability chains, APT campaigns, incident response scenarios).

**Dropped:** All consumer product code (Next.js app, React components, Supabase RLS/auth, product UI). These are tied to the specific SaaS deployment and are not reusable arsenal primitives. The **engagement model** and **orchestration methodology** are reusable; the **product implementation** is not.

---

**Status:** COMPLETE. 3 functional units extracted (secret patterns, security headers, gamification model + orchestration). Full Next.js product dropped. PLACEMENT.md created for reference.

## Retention update (nothing dropped)
The Next.js app + Supabase product code, previously listed as a drop, is
**retained as reference** at `ops/gamification/guild-hall/app-reference/`
(src + public + supabase + manifest; `node_modules`/`.next` excluded). It is the
gamification product UI that the engagement-scoring model came from — kept for
reference per "nothing dropped".
