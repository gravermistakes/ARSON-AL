# Guild Hall Bulk Placement Notes

## Deferred/Dropped Items

The following guild-hall components are consumer product / web-app UI and are **dropped** (not placed):

| Item | Path | Reason | Status |
|------|------|--------|--------|
| **Next.js App** | `src/`, `next.config.js`, `tsconfig.json` | Consumer web application (profile, quest dashboard, leaderboard UI) | drop |
| **Supabase schema** | `supabase/migrations/`, `supabase/functions/` | Remote database migrations and edge functions; tied to specific deployment | bulk: follow-up if architectural interest |
| **Build artifacts** | `package.json`, `package-lock.json`, `tailwind.config.ts`, `vite.config.ts` | Build and styling tooling | drop |
| **Static/public** | `public/`, `.next/` | Build output and static assets | drop |
| **Tests** | `src/tests/` | Product-level smoke tests; not reusable arsenal patterns | drop |
| **GitHub workflows** | `.github/` | CI/CD pipeline (product-specific) | drop |

## Functional Units Extracted

Only the **gamification model and orchestration methodology** were extracted:
- `docs/SECRETS-SCAN-REPORT.md` → `probes/guild-hall/secret-detection/` (secret-pattern detection rules)
- `netlify.toml` → `ops/gamification/guild-hall/security-headers-baseline.toml` (defensive baseline reference)
- `CLAUDE.md` (Campaign Mode NPC orchestration section) → `ops/gamification/guild-hall/orchestration-methodology.md` (engagement/quest/reward model)

## Note

Guild-hall is a full-stack SaaS product. The arsenal captures only:
1. The **gamification substrate** (quest/points/badge/tier/leaderboard scoring model — reframe as engagement/severity/chain/coverage)
2. The **campaign-mode orchestration** (how NPCs manage multi-step engagements)
3. The **secret-detection patterns** (regex rules for scanning)
4. The **security headers baseline** (defensive reference)

The consumer web-app (Next.js, Supabase RLS, React dashboard) is product code and is not part of the arsenal.
