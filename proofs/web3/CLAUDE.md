<!-- generated: 1782639068 -->
# PROOFS/WEB3
## What's here
Exploit validation, vulnerability databases, CTF playgrounds, and audit report archives for Web3 environments.
- `ctf-environments.md` — Capture the Ether, Ethernaut, Damn Vulnerable DeFi, and DeFiVulnLabs (the definitive 60+ runnable Foundry PoC workspace).
- `audit-reports.md` — indexed public audit portfolios and findings engines (Solodit, Code4rena, Consensys Diligence, Sherlock, and 0xNazgul's list).

## Build
No compilation required. These templates and databases are read by validation and triage agents during PoC building and triage simulation.

## Test
Verify links to report archives periodically. Run a local target check on Solodit or Code4rena to ensure API endpoints or index paths are functional.

## Feeds
- **Loop:** Validation Loop — the validation engine uses DeFiVulnLabs and other CTF exploit skeletons as template blueprints to draft a running PoC. Reports are referenced to verify severity classifications during triage.
- **Consumes:** raw exploit paths and balance constraints.
- **Emits:** validated proof-of-concept exploits, CVSS scores, and platform-ready triage reports -> ops/ for final consensus gates.

## Issues
- DeFiVulnLabs is extremely helpful but relies on older Foundry dependencies. Solodit and other database indexes are subject to link rot over time.
