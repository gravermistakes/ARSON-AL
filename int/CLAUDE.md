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

## Build
Python scripts, no build. `python3 web3/lance/scripts/parse_web3_scope.py --help`.

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
