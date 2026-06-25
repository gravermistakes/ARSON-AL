<!-- generated: 1782416571 -->
# INT

## What's here
Target modeling, threat mapping, scope parsing, dossier building — the front of
the recon loop. Models a target so probes/ know what surface to hit.
- `web3/lance/` — Web3 scope + intake (gates G0/G1): `parse_web3_scope.py`
  (parse bug-bounty scope docs → manifest), `normalize_targets.py` (normalize
  addresses / repo paths / Sui packages into one manifest shape), and
  `wallet-trust-boundary.md` (admin/keeper/oracle/relayer trust boundaries).
- `web2/bug-reaper/` — Web2 scope analysis: `analyze_scope.py`
  (wildcard/apex/nested-subdomain scope rules).
- `h1dr4/` — agent-native OSINT/investigation toolkit (MCP): case files,
  evidence/source verification, contradiction checks; docs incl SKYNET historical
  bases, case-studies, research. `node index.js` / see `examples/`.
- `bofhound/` — Active Directory recon: parses ldapsearch/BOF logs into
  BloodHound graph data (operator-controlled LDAP, evades collector honeypots).
  `bofhound -i <logs> -o <out>`.
- `mitre-attack/` — curated MITRE ATT&CK resource index; map findings/TTPs
  to ATT&CK techniques.
- `threat-dragon/` — OWASP Threat Dragon: threat-modeling app (diagrams +
  rule engine) with example models under `ThreatDragonModels/`.
- `osint-tools/` — curated OSINT tool collection (1000+) + standalone HTML
  search widgets (court/graves/hashtag) for person/infra research.
- `shodansnipe/` — agentic recon/OSINT toolkit (agents/core/tools/skills).
  **OPACK-migration target**: its agents+modules become ECS cells (see root
  CLAUDE.md "OPACK Migration"); lives as recon here until recombined.

## Build
- Knowledge/scripts (web3/web2): Python, no build.
- `bofhound/`: `poetry install` (Python). `h1dr4/`: `npm install`.

## Test
`python3 web3/lance/scripts/normalize_targets.py` on a sample scope; eyeball the
emitted manifest. No upstream test suite shipped.

## Feeds
- **Loop:** Recon — int/ models the target → probes/ extract surface → findings
  refine the model.
- **Consumes:** scope files, target lists.
- **Emits:** a normalized target manifest + trust-boundary map → probes/web3.

## Issues
- Lance scope/intake split out of the lance skill, whose SKILL.md (now in
  proofs/web3/lance/) still references these by their old `scripts/` path. Cross-
  referenced in MANIFEST; recombination onto OPACK reunifies.
- Python (against the "no Python when practical" rule) — inherited, not rewritten.
