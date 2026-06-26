# Dissolution Manifest: Scorpio

**Date:** 2026-06-26
**Repo:** /home/user/Scorpio
**Dissolution Wave:** 2

## Units Table

| Unit Path | Verb | Category | Destination |
|-----------|------|----------|-------------|
| src/lib/gemini.ts (SENTINEL prompt) | Analyze code; identify attack surfaces; sequence exploits (LLM-SAST logic) | probes | probes/scorpio/llm-sast/gemini-prompts.ts |
| src/lib/gemini.ts (AUDITOR prompt) | Track findings; generate threat-intel prose; behavioral risk profiles (LLM report generation) | proofs | proofs/scorpio/llm-report-generation/gemini-prompts.ts |
| src/types.ts (Vulnerability + AuditLog + ExploitScenario schemas) | Finding schema (structured vulnerability model with severity, vector, remediation, exploit chaining) | proofs | proofs/scorpio/llm-report-generation/finding-schema.ts |

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

**Scorpio** is a **consumer web app wrapper** around **two LLM-pentest capabilities**:

1. **SENTINEL prompt** → LLM-powered SAST (Static Application Security Testing). Given code, output structured findings (vulnerability ID, severity, vector, description, remediation, status). This is a **probe** (analyze code → find issues).

2. **AUDITOR prompt** → LLM-powered report generation. Given findings, output narrative threat-intel prose with behavioral risk profiles. This is a **proof** (findings → report/dossier).

3. **Vulnerability schema** (ID, severity, vector, remediation, status, timestamp) + **AuditLog** (audit trail with agent/timestamp/type) + **ExploitScenario** (steps, probability) → **proofs/** (structured finding model for report generation).

**Dissolved by verb:**
- SENTINEL (Analyze code) → **probes/scorpio/llm-sast/** (LLM-powered code analysis)
- AUDITOR + Schemas (Generate findings report) → **proofs/scorpio/llm-report-generation/** (LLM-powered dossier generation)

**Dropped:** All React/UI/build infrastructure. The arsenal preserves only the **adversarial prompts** and **schema definitions**. These are reusable across any LLM-powered security analysis system, independent of the React frontend or Vite build.

**Note on reuse:** The SENTINEL and AUDITOR prompts define LLM-based security analysis. Practitioners can instantiate these with any LLM (Gemini, Claude, OpenAI, open-source), any code analysis trigger (pre-commit hook, CI/CD pipeline, batch analysis), and any finding sink (database, report generator, SIEM). The UI is product packaging; the prompts are arsenal primitives.

---

**Status:** COMPLETE. 3 functional units extracted. React UI + build system dropped entirely. PLACEMENT.md created for reference.

## Retention update (nothing dropped)
The React UI / build scaffold, previously listed as a drop, is **retained as
reference** at `probes/scorpio/ui-reference/` (src + index.html + manifests;
`node_modules`/`dist` excluded). It is the frontend of the LLM-SAST tool — kept
for reference per "nothing dropped".
