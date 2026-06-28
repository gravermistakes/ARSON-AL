# proofs — revision draft

Bones. Revise in place; `___` = fill me.

## Action
PoC templates, triage validation, report generation
exploit chains → ops/mem/chains (chain loop state lives in mem)
nothing exits without a PoC

## What's here (species)
- `web3/lance/` — lance 7-gate spine (G3-G6) + report templates + scoring engine
- `web2/bug-reaper/` — 4-phase spine + chain builder + platform triage
- `scorpio/agent-report-generation/` — auditor prompt + finding schema
- `ruvn/dossier-generation/` — citer (findings → markdown dossier)
- `MANIFEST.yml`, `SYSADLOG.md` — dissolution log
- ___

## Loop role
validation — gate candidates for exploitability + economics; only Medium+ that pass triage become reports
chain — InsideActor braids low-sev findings into reports (chain state in ops/mem/chains)

## Emits
- platform-ready reports (Immunefi/Bugcrowd/HackenProof/HackerOne/Intigriti/YWH)
- chained P1/P2 findings
- ___

## Consumes
- candidate findings from probes/
- scope from int/
- ___

## Open
- lance/bug-reaper spines reference int/probes/picks paths (cross-dir refs in MANIFEST)
- recombination onto Opaca reunifies the gates
- ___
