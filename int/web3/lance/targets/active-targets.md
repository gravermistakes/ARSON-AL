# Active Bounty Targets — Hunt Board

## TRACK A: Sky (MakerDAO) — $10M, NO KYC

| Field | Value |
|-------|-------|
| Platform | Immunefi |
| Max Payout | $10,000,000 |
| KYC | **NO** |
| Chain | EVM (Ethereum mainnet) |
| Language | Solidity 0.6.12 (core), mixed (periphery) |
| Payout | DAI/USDS via governance spell |
| Scope | Smart contracts — Vat, Dog, Clipper, Pot, Jug, End, DaiJoin, adapters, oracles |

### Triage Status
- **Vat.sol**: CLEARED. Fully hardened. Math library belt-and-suspenders. Auth model clean. No novel finding. _mul post-check ordering is safe (values bounded). Dust bypass only via auth-only grab (known). Rate zeroing governance-gated only.
- **Dog.sol**: CLEARED. Hole overshoot via dusty-remainder full liquidation is known+acknowledged in code comments. Bounded by `ceiling(dust * chop / WAD)`. No novel finding.
- **Clip.sol**: LEAD — callback window. take() sends collateral before collecting DAI. Callee can call Spotter.poke() and Dog.bark() during callback. Requires oracle timing or manipulation to exploit. Lock prevents same-Clipper reentrancy but not cross-ilk. Most promising DSS vector.
- **Clip.sol redo()**: CLEARED. Incentive farming possible but bounded by tail timer and governance params. Not net-profitable at current gas costs unless tip/chip misconfigured.
- **Cat.sol**: DEPRECATED. Dusty leftover bug exists but Cat superseded by Dog. Only relevant if any ilk still routes through Cat.
- **Flip.sol**: CLEARED. No callback to usr, no reentrancy vector.
- **Pot/Jug**: NOT SCANNED. DSR accumulation, rate computation.
- **End.sol**: NOT SCANNED. Emergency shutdown — global settlement math.
- **DaiJoin/GemJoin adapters**: NOT SCANNED. Token adapter edge cases.
- **Oracles (OSM/Median)**: NOT SCANNED. Key dependency for Clipper callback exploit chain.
- **SparkLend**: SEPARATE PROGRAM. Check Spark's own bounty scope.

### Known Out-of-Scope (from Immunefi page)
- First depositor share inflation (known)
- GatedRedemptionQueueSharesWrapperLib with sharesActionTimelock (known)
- GMX V2 adjustedClaimable < claimAmount (known)
- Missing wrapped-native fallback in redeemFromQueue dispersal (known)
- Malicious vault owner (trust assumption)
- External position removal with negative value (known)
- Tokens sent directly outside protocol flows (no valid entry path)
- Front-running init (factory deploys atomically)
- Griefing redemption queues (skip mechanism exists)
- autoProtocolFeeSharesBuyback share price issue (known, accepted, mitigated)

- **End.sol**: CLEARED. snip/skip truncation negligible. skim cascaded rmul sub-dust. flow double truncation negligible. bag/out multi-ilk claiming is by design.
- **Cure.sol**: LEAD — thaw() DoS. permissionless load() + post-cage drop() disabled (live==0) = permanent settlement block if cure source compromised. Requires pre-existing source vuln.
- **End.cage(ilk)**: MARGINAL. Oracle timing via OSM queued price. Anyone can call post-shutdown. Limited by OSM 1-hour delay.
- **Pot/Jug**: CLEARED. drip() precision accumulation is sub-wei. Jug._diff overflow requires governance-set extreme duty.
- **Join adapters**: CLEARED. GemJoin state-before-transfer is design-known. Only exploitable with ERC-777 or fee-on-transfer tokens (vetted out by governance).
- **Spot.sol**: CLEARED. poke() permissionless by design. Oracle returns has=false → spot=0 → mass liquidation is intended defensive behavior.
- **Dai.sol**: CLEARED. permit() signature malleability is griefing-only (no funds at risk).
- **Auth model**: CLEARED. Governance trust boundary is the security model. Cat/Dog ward can set malicious clip/flip but this is governance compromise (out of scope).

### Verdict
DSS core is exhausted. No novel Critical/High across entire codebase. Best leads require chaining with another vulnerability (oracle compromise, cure source vuln, ERC-777 collateral). Track A should shift to SparkLend or newer Sky periphery for fresh attack surface.

### Next Actions
1. Investigate SparkLend (separate Immunefi program) — newer code, less scrutinized
2. Check D3M (Direct Deposit Module) — cross-protocol integration surface
3. Review newer Sky governance modules (SubDAO architecture)

---

## TRACK B: Cosmos SDK / IBC-go — No Published Cap

| Field | Value |
|-------|-------|
| Platform | HackerOne |
| Max Payout | No published cap ("no maximum program reward") |
| KYC | YES (HackerOne ID, crypto payout available) |
| Chain | Cosmos SDK (Go) |
| Scope | cosmos-sdk, ibc-go, CometBFT |

### Triage Status — IBC-go
- **ibc-go non-determinism**: SCANNED. Clean on hot paths (time.Now only in CLI, floats IEEE 754 deterministic).
- **ibc-go PFM (packet-forward-middleware)**: FLAGGED. `panic()` in error/timeout path (keeper.go:149). Complex escrow accounting worth deeper review.
- **ibc-go GMP module**: REVIEWED. No AllowMessages whitelist (by design for GMP). Signer verification present but less restrictive than ICA.
- **ibc-go rate-limiting**: REVIEWED. Unparseable packets silently pass (by design — non-transfer packets).
- **ibc-go callbacks module**: NOT SCANNED. IBC callbacks to smart contracts — potential unmetered computation vector.

### Triage Status — Cosmos SDK Modules (SCANNED)
- **x/staking**: HOT LEAD. CancelUnbondingDelegation bypasses OnHold() — ICS slashing evasion. CVSS 6.5. See `cosmos-sdk-staking-audit.md`.
- **x/staking (secondary)**: InitialBalance desync in partial cancel after slash. Under-slashing vector. CVSS 5.3.
- **x/staking (tertiary)**: Consensus key rotation creates slashing evasion window. CVSS 4.9.
- **x/gov**: Zero quorum allowed (Quorum="0" passes validation). Two-step governance takeover chain. CVSS 7.5. See `cosmos-sdk-gov-audit.md`.
- **x/gov (secondary)**: Expedited-to-regular conversion timing bug — negligible remaining vote time after conversion. CVSS 4.3.
- **x/gov (tertiary)**: ConsensusParams authority override escalation path. CVSS 7.2 conditional.

### Triage Status — CosmWasm (SCANNED)
- **VM runtime**: 9 production `panic!()` in host crypto functions. Conditional on triggering unexpected CryptoError variant. See `cosmwasm-scan.md`.
- **Gas metering**: CLEARED. Thorough checked arithmetic throughout. No gaps.
- **Memory/feature gates**: CLEARED. Proper limits, feature blocking (threads/SIMD/ref types).

### Vulnerability Classes to Hunt
1. **ICS slashing evasion** via CancelUnbondingDelegation OnHold bypass (HOT)
2. **Governance takeover** via zero quorum + authority override (HOT)
3. Key malleability / prefix iteration (store key design)
4. Unmetered computation in BeginBlock/EndBlock hooks
5. Fee market / gas mispricing (SendCoins batch panic)
6. IBC packet handling (escrow accounting, timeout refund)
7. CosmWasm crypto panic triggering

### Next Actions
1. **PoC: OnHold bypass** — verify ICS uses PutUnbondingOnHold, build exploit scenario
2. **PoC: Zero quorum chain** — verify param update can set quorum=0, build two-step attack
3. Deep review PFM panic path — can an attacker trigger it?
4. Scan ibc-go callbacks module for unmetered computation
5. Check appchain forks for unpatched SDK versions (dYdX, Celestia, Injective)

---

## TRACK C: Enzyme Finance — $200K, NO KYC

| Field | Value |
|-------|-------|
| Platform | Immunefi |
| Max Payout | $200,000 (critical) |
| KYC | **NO** |
| Chain | EVM (Ethereum + Polygon) |
| Language | Solidity |
| Payout | USDC on Ethereum |
| Scope | enzymefinance/protocol contracts |
| Total Paid | $635.5K historically |

### Notes
- Below $5M target but NO KYC + Solidity + active program
- On-chain asset management protocol — vaults, adapters, fee logic
- "Extensive code reviews; lighter than audits" — has QA but not heavy formal audit
- Known issues documented — check before reporting
- PoC REQUIRED on local fork (no mainnet/testnet testing)
- Secondary target — only pursue if lead found during adjacent work

---

## SECONDARY WATCHLIST

| Target | Max | KYC | Chain | Status |
|--------|-----|-----|-------|--------|
| Usual | $16M | YES | EVM | 20 prior audits, 0 findings. Hard. |
| Uniswap v4 | $15.5M | YES | EVM | Heavily audited. KYC required. |
| LayerZero | $15M | YES | Cross-chain | Full KYC + OFAC. |
| Wormhole | $10M (W token) | YES | Cross-chain | Previously paid $10M. |
| dYdX v4 | $1M | YES | Cosmos SDK | Appchain — SDK module bugs applicable. |
| Celestia | $750K (TIA) | YES | Cosmos SDK | Modular DA — novel surface. |
| Seaport | $3M | YES | EVM | OpenSea. HackerOne ID. |

---

## STRATEGY

**Primary**: Track B (Cosmos SDK/IBC-go). Rich bug classes, recent advisories, Go code is manually auditable, Agent has deep pattern knowledge. Find first, KYC at payout.

**Secondary**: Track A (Sky periphery). Vat is clean but Dog/Clipper/End/oracles are less scrutinized. $10M no-KYC makes it worth parallel effort.

**Opportunistic**: Track C (Enzyme). Only if a lead surfaces during adjacent Solidity work. $200K ceiling is below target but no-KYC and historically active.
