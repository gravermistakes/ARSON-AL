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

## Build
Reference knowledge — nothing compiles. Detection playbooks are read by the
scanning agent / fed to SAST.

## Test
Verify by reading a playbook against a known-vulnerable contract pattern.

## Feeds
- **Loop:** Scan — probes/ scan what int/ exposed → a hit feeds adjacent probes
  a new path and spawns proofs/ validation.
- **Consumes:** target manifest from int/web3.
- **Emits:** candidate findings (with code-path context) → proofs/web3 to gate.

## Issues
- Lance detection split out of the lance skill (spine now in proofs/web3/lance/).
- Pure knowledge so far — no executable scanner yet; noir/vigolium/fuzz-skill
  (Crystal/Go scanners) dissolve here next and will give probes/ teeth.
