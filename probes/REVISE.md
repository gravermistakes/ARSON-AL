# probes — revision draft

Bones. Revise in place; `___` = fill me.

## Action
maps, enum, scrapers, crawlers

## What's here (species)
- `noir/` — endpoint/attack-surface extractor (Crystal)
- `vigolium/` — DAST scanner (Go, 266 modules, in-process agent engine)
- `drogonsec/` — SAST/SCA/secrets/IaC scanner (Go)
- `web2/bug-reaper/` — Web2 recon methodology + audit rules
- `web3/lance/` — Web3 detection knowledge (G2)
- `web3/blockchain-appsec/` — OWASP web3 standard
- `DeTTECT/` — detection-coverage + visibility (reframed: evasion-surface map)
- `scorpio/` — LLM-SAST (sentinel prompt + schema)
- `awesome-bugbounty-tools/` — 239 probes-class species (with INTEGRATION-TABLE)
- `shodansnipe/` — enum/scrape modules
- `guild-hall/` — secret-detection pattern dictionary
- ___

## Loop role
scan — what int/ exposed gets enumerated/fingerprinted; hits feed picks/

## Emits
- candidate findings (SARIF, CWE/CVSS context, code-path)
- surface inventory (endpoints, subdomains, params, secrets)
- ___

## Consumes
- target manifest from int/
- ___

## Open
- vigolium + drogonsec + noir = three real binaries; integration as kits?
- ___
