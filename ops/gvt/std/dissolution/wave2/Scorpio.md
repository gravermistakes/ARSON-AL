# Dissolution Manifest: Scorpio

**Date:** 2026-06-26
**Repo:** /home/user/Scorpio
**Dissolution Wave:** 2

## Units Table

| Unit Path | Action | Category | Destination |
|-----------|------|----------|-------------|
| src/lib/gemini.ts (SENTINEL prompt) | Analyze code; identify attack surfaces; sequence exploits (Agent-SAST logic) | probes | probes/scorpio/agent-sast/gemini-prompts.ts |
| src/lib/gemini.ts (AUDITOR prompt) | Track findings; generate threat-intel prose; behavioral risk profiles (Agent report generation) | proofs | proofs/scorpio/agent-report-generation/gemini-prompts.ts |
| src/types.ts (Vulnerability + AuditLog + ExploitScenario schemas) | Finding schema (structured vulnerability model with severity, vector, remediation, exploit chaining) | proofs | proofs/scorpio/agent-report-generation/finding-schema.ts |

## Bulk/Deferred Items

None. Prompts and schemas are text files (~1 KB each).

## Drop List

- src/components/, src/App.tsx, src/main.tsx (React UI; consumer product wrapper)
- src/lib/utils.ts (React/UI utilities)
- src/index.css (styling)
- vite.config.ts, tsconfig.json, package.json, package-lock.json (build system)
- index.html, public/ (static assets)
- .env.example (environment template)
- .git/, .gitignore, README.md, LICENSE, metadata.json (repository metadata)

## Rationale

**Scorpio** is a **consumer web app wrapper** around **two Agent-pentest capabilities**:

1. **SENTINEL prompt** → Agent-powered SAST (Static Application Security Testing). Given code, output structured findings (vulnerability ID, severity, vector, description, remediation, status). This is a **probe** (analyze code → find issues).

2. **AUDITOR prompt** → Agent-powered report generation. Given findings, output narrative threat-intel prose with behavioral risk profiles. This is a **proof** (findings → report/dossier).

3. **Vulnerability schema** (ID, severity, vector, remediation, status, timestamp) + **AuditLog** (audit trail with agent/timestamp/type) + **ExploitScenario** (steps, probability) → **proofs/** (structured finding model for report generation).

**Dissolved by action:**
- SENTINEL (Analyze code) → **probes/scorpio/agent-sast/** (Agent-powered code analysis)
- AUDITOR + Schemas (Generate findings report) → **proofs/scorpio/agent-report-generation/** (Agent-powered dossier generation)

**Dropped:** All React/UI/build infrastructure. The arsenal preserves only the **adversarial prompts** and **schema definitions**. These are reusable across any Agent-powered security analysis system, independent of the React frontend or Vite build.

**Note on reuse:** The SENTINEL and AUDITOR prompts define Agent-based security analysis. Practitioners can instantiate these with any Agent (Gemini, Claude, OpenAI, open-source), any code analysis trigger (pre-commit hook, CI/CD pipeline, batch analysis), and any finding sink (database, report generator, SIEM). The UI is product packaging; the prompts are arsenal primitives.

---

**Status:** COMPLETE. 3 functional units extracted. React UI + build system dropped entirely. PLACEMENT.md created for reference.

## Retention update (nothing dropped)
The React UI / build scaffold, previously listed as a drop, is **retained as
reference** at `probes/scorpio/ui-reference/` (src + index.html + manifests;
`node_modules`/`dist` excluded). It is the frontend of the Agent-SAST tool — kept
for reference per "nothing dropped".

## Duplicates & homomorphs
Homomorph: sentinel->auditor is a gated verification pipeline (see ruvn, loki quality-gates).
Full dedup index: ../DUPLICATES.md
