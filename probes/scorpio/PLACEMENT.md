# Scorpio Bulk Placement Notes

## Deferred/Dropped Items

The following Scorpio components are UI-scaffold or build-system artifacts and are **dropped** (not placed):

| Item | Path | Reason | Status |
|------|------|--------|--------|
| **React UI** | `src/components/`, `src/App.tsx`, `src/main.tsx` | Consumer web-app frontend for prompt/finding visualization | drop |
| **Build config** | `vite.config.ts`, `tsconfig.json`, `package.json`, `package-lock.json` | Build tooling and dependencies for UI | drop |
| **Static files** | `index.html`, `public/` | HTML entry point and static assets | drop |
| **.env files** | `.env.example` | Environment template (no secrets extracted) | drop |

## Functional Units Extracted

Only the **LLM-pentest logic** was extracted:
- `src/lib/gemini.ts` → `probes/scorpio/llm-sast/gemini-prompts.ts` (SENTINEL + AUDITOR prompts)
- `src/types.ts` → `proofs/scorpio/llm-report-generation/finding-schema.ts` (Vulnerability/AuditLog/ExploitScenario schemas)

These prompts and schemas are reusable across any LLM-based code security analysis pipeline, independent of the React UI.

## Note

The UI is a consumer product wrapper. The arsenal captures only the adversarial reasoning logic (prompts) and data schema (findings model).
