<!-- generated: 1782639068 -->
# PROBES/WEB3
## What's here
Smart contract detection, static analysis, and chain-specific guidance tools and references.
- `sast-tools.md` — canonical EVM SAST scanners (Slither, Mythril, Manticore) and visualization utilities (Surya, Solgraph).
- `lance/chains/cosmos.md` — Cosmos SDK chain guidance (non-determinism, ABCI, key design).
- `lance/vulnerabilities/` — expanded EVM playbooks including Cosmos playbooks (`cosmos-nondeterminism.md`, `cosmos-abci-panic.md`, `cosmos-kv-key-collision.md`).

## Build
No compilation required. These are used by SAST scanning engines and auditing agents to inspect candidate code surfaces.

## Test
Verify SAST scanners on target workspaces using their respective CLI:
```bash
slither .
```

## Feeds
- **Loop:** Recon/Scan — reconnaissance yields target code repositories which are probed via SAST scanners and chain-specific playbooks to expose vulnerable interfaces.
- **Consumes:** raw repository source code.
- **Emits:** vulnerable sinks and interfaces -> picks/web3 to build working exploit vectors.

## Issues
- SAST tools are purely advisory and generate high false-positive rates. Use proofs/ and picks/ to filter and prove.
