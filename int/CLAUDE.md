<!-- generated: 1782416571 -->
# INT

## What's here
Target modeling, threat mapping, dossier building — the front of
the recon loop. Models a target so probes/ know what surface to hit.
  ( `normalize_targets.py` (normalize
  addresses / repo paths / Sui packages into one manifest shape), and
  `wallet-trust-boundary.md` (admin/keeper/oracle/relayer trust boundaries).
- `web2/bug-reaper/`
  (wildcard/apex/nested-subdomain rules).
- `h1dr4/` — agent-native OSINT/investigation toolkit (MCP): case files,
  evidence/source verification, contradiction checks; docs incl SKYNET historical
  bases, case-studies, research. `node index.js` / see `examples/`. (only used for docs and ethics]
- `bofhound/` — Active Directory recon: parses ldapsearch/BOF logs into
  BloodHound graph data (operator-controlled LDAP, evades collector honeypots).
  `bofhound -i <logs> -o <out>`. (need refitting
- `mitre-attack/` — curated MITRE ATT&CK resource index; map findings/TTPs to ATT&CK techniques. (Need to gather those resources also)
- `threat-dragon/` — OWASP Threat Dragon: threat-modeling app (diagrams + rule engine) with example models under `ThreatDragonModels/`.
- `osint-tools/` — curated OSINT tool collection (1000+) + standalone HTML search widgets (court/graves/hashtag) for person/infra research. (really taking most ofvm this)
- `shodansnipe/` — actor recon/OSINT toolkit (agents/core/tools/skills).
  **Opaca coordination-migration target**: its agents+modules become ECS cells (see root
  CLAUDE.md "OPACK Migration"); lives here until recombined.
- `awesome-bugbounty-tools/`, `awesome-hacking/` — curated tool/resource index. (gimme all of it)

## Feeds
- **Consumes:** constraints, architectures, target lists.
- **Provides:** Actors with stimuli towards next steps.

## Issues
- Lance scope/intake split out of the lance skill, whose SKILL.md (now in proofs/web3/lance/) still references these by their old `scripts/` path. Cross-referenced in MANIFEST; recombination onto Opaca reunifies. (not an issue)
- Python (against the "no Python when practical" rule) — inherited, not rewritten yet, must be resolved.
