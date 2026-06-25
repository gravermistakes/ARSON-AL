<!-- generated: 1782416571 -->
# PROBES

## What's here
Enumeration, fingerprinting, endpoint extraction, surface discovery, and the
detection knowledge that says *what to look for*. The engine of the scan loop.
- `web3/lance/` — Web3 detection knowledge (gate G2): `audit-rules.md` (the
  class-based detection ruleset), `vulnerabilities/` (13 exploit-class playbooks
  — reentrancy, oracle manipulation, flash-loan, bridge replay, signature
  replay, AMM/vault/accounting invariants, upgradeability, Move capability/race),
  and `chains/` (EVM, Sui Move, cross-chain bridge, L2-specific risks).
- `web2/bug-reaper/` — Web2 recon (`recon.md`: subdomain enum, fingerprinting,
  JS mining, endpoint discovery), white-box SAST (`source-code-audit.md`), and
  the detection ruleset (`audit-rules.md`).
- `drogonsec/` — **executable** Go scanner: SAST (20+ langs), SCA (CVE deps),
  secret leaks (50+ patterns), IaC misconfig; SARIF/CWE/CVSS, OWASP 2025. The
  first real probe binary. `make build` then `./bin/drogonsec scan <path>`.

## Build
- Knowledge (web2/web3): nothing compiles — playbooks read by the scanning agent.
- `drogonsec/`: `cd drogonsec && make build` (go 1.26; outputs `bin/drogonsec`).

## Test
- Knowledge: read a playbook against a known-vulnerable pattern.
- `drogonsec/`: `cd drogonsec && go test ./...`; smoke `./bin/drogonsec scan .`.

## Feeds
- **Loop:** Scan — probes/ scan what int/ exposed → a hit feeds adjacent probes
  a new path and spawns proofs/ validation.
- **Consumes:** target manifest from int/.
- **Emits:** candidate findings (SARIF/code-path context) → proofs/ to gate.

## Issues
- Lance/bug-reaper detection knowledge split from their skills (spines in proofs/).
- drogonsec gives probes/ its first teeth (SAST/SCA/secrets). Endpoint-extraction
  engines (noir Crystal, vigolium Go) still to dissolve here.
