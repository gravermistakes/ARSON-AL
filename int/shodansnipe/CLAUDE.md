<!-- generated -->
# int/shodansnipe

## What's here
ShodanSnipe's recon-side cells, kept as the int/ slice. Per root CLAUDE.md's
**OPACK Migration**: shodansnipe's agents and modules become Opaca cells —
this dir holds the recon-flavored ones; tools/ moved to `probes/shodansnipe`,
methodology to `ops/gvt/std/shodansnipe-skills`, engine/UI parked at
`.scaff/shodansnipe`.

- `agents/` — 7 CrewAI agents that plan/execute recon, scope, OSINT, vuln,
  threat-intel, nmap-recon, and report. Each becomes a **BadFaithActor** kit
  loadout once recombined onto Opaca.
- `core/` — recon substrate kept here:
  - `scope.py` — scope parsing / boundary
  - `query_advisor.py` — picks Shodan queries from scope
  - `threat_feeds.py` — threat-intel feeds

## Build
Python 3.12 + CrewAI (legacy stack). Build/run scripts are at `.scaff/shodansnipe/launchers`.

## Feeds
- Loop: Recon — scope → query advisor → Shodan/OSINT tools (in
  `probes/shodansnipe`) → findings refine the dossier.
- Consumes: target scope.
- Emits: enriched recon dossier into int/'s loop; vuln/threat findings into
  the chain.

## Issues
- This is a competing engine in miniature (CrewAI + MCP). Per OPACK Migration,
  the *agents* are interesting (they become Opaca cells); the engine plumbing
  at `.scaff/shodansnipe` is reference, not the target architecture.
