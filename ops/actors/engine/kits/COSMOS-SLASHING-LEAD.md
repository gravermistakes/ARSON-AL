# Cosmos SDK x/staking CancelUnbondingDelegation Hot Lead

## The Vulnerability

**Target:** Cosmos SDK `x/staking` module (all chains using Cosmos/Tendermint)
**Finding Type:** `SLASHING_EVASION` (CWE-827: unauthorized bypass)
**Lance Gate:** 5 (Exploit Development)
**Severity:** P1 / 9.8 CVSS (if chain relies on slashing for security)

### Attack Surface

The `CancelUnbondingDelegation` message in Cosmos `x/staking` module allows a delegator to cancel an unbonding delegation that is still pending (not yet mature).

**The seam:** The `OnHold()` method that marks funds as ineligible for withdrawal is **sometimes bypassed** if:

1. Delegation is in "unbonding" state (initiated CancelUnbondingDelegation)
2. Validator commits a double-sign (slashable offense)
3. Slashing module attempts to slash the unbonding tokens
4. OnHold() is not properly checked before slash execution

**Impact:** Delegator evades slashing penalties, draining validator rewards or escaping punishment for ICS (Interchain Security) equivocation.

### Vector: ICS Validator Equivocation

On ICS chains (e.g., Stride, Quicksilver), validators are **shared across consumer chains**. A validator that double-signs on one consumer chain triggers slashing on the provider chain (Cosmos Hub).

**Attack sequence:**
```
1. Validator delegates 1M ATOM to self on provider chain
2. Initiates unbonding (21-day wait)
3. Day 7: Validator intentionally equivocates on consumer chain (double-sign)
4. Consumer chain reports equivocation to provider
5. Slashing logic activates: should slash 1M ATOM unbonding tokens
6. BUT: CancelUnbondingDelegation + OnHold() bypass allows cancellation
7. Validator re-delegates same 1M atoms; wash trade complete
8. Equivocation penalty evaded; tokens safe
```

## Kit Mapping

This lead spans **multiple kits** in sequence:

### Gate 2: Surface Discovery
**Kit:** `COSMOS_PROBE`
- Tool: `probes/web3/cosmos-module-enum` → enumerate all `x/staking` module params
- Tool: `probes/web3/staking-state-snapshot` → dump all unbonding delegations + maturity times
- **Finding:** `new_surface` (identify chains with vulnerable unbonding patterns)
- **Next:** VALIDATOR_CHECK to audit slashing conditions

### Gate 3: Detail Gathering
**Kit:** `CHAIN_ANALYSIS`
- Tool: `int/web3/fund-flow-tracer` → identify validator self-delegation patterns
- Tool: `int/web3/entity-labeler` → tag validators + known MEV operators
- **Finding:** `slashing_evasion` (pattern of cancel-unbond→unbond→double-sign cycles detected)

### Gate 5: Exploit Development
**Kit:** `ORACLE_EXPLOIT` (repurposed for state manipulation) or new kit `COSMOS_EXPLOIT`
- Tool: `picks/web3/mint-validation-fuzzer` (adapted) → fuzz CancelUnbondingDelegation + slash ordering
- Tool: `picks/web3/state-transition-analyzer` → identify state machine race condition
- Manual PoC:
  ```rust
  // Exploit template (Rust, Cosmos SDK)
  
  // 1. Delegate with cosmosvaloper address
  let msg_delegate = MsgDelegate {
      delegator_address: attacker,
      validator_address: attacker_validator,
      amount: Coin { denom: "atom", amount: "1000000" },
  };
  
  // 2. Initiate unbonding
  let msg_undelegate = MsgUndelegate {
      delegator_address: attacker,
      validator_address: attacker_validator,
      amount: Coin { denom: "atom", amount: "1000000" },
  };
  
  // 3. Wait until day 10 of 21-day unbonding period
  // 4. Broadcast double-sign from validator on consumer chain
  //    (or bribe other validator to sign same block height twice)
  
  // 5. When slashing triggers, race CancelUnbondingDelegation:
  let msg_cancel = MsgCancelUnbondingDelegation {
      delegator_address: attacker,
      validator_address: attacker_validator,
      creation_height: original_unbonding_height,
      amount: Coin { denom: "atom", amount: "1000000" },
  };
  
  // If CancelUnbondingDelegation executes BEFORE slash:
  //   - Unbonding entry removed from state
  //   - Slash tries to find unbonding entry; not found (or wrong index)
  //   - Funds safely re-delegated
  // If slash is deferred (common in Tendermint):
  //   - Race window extends to multiple blocks
  //   - High success probability with fast relay
  ```

**Finding:** `confirmed_poc` (PoC demonstrates token escape)

### Gate 6: Validation
**Kit:** `LANCE_VALIDATE` or `POC_BUILDER`
- Tool: `proofs/web3/foundry-test-harness` (adapted) or `proofs/web3/web3py-interaction-library`
- Run on testnet against vulnerable chain binary
- Confirm: unbonding tokens NOT slashed despite validator equivocation
- Calculate $-impact: `slashing_rate × total_delegated × affected_validators`

**Finding:** `economic_exploit` (quantify total at-risk value across all validators)

### Gate 7: Reporting
**Kit:** `IMMUNEFI_REPORT`
- Tool: `proofs/web3/immunefi-report-generator`
- Severity: P1 (9.8 CVSS)
- Title: "x/staking CancelUnbondingDelegation OnHold() Bypass Enables Slashing Evasion"
- PoC: Include Rust exploit template + testnet tx hashes
- Impact narrative: Validators can evade slashing, undermining ICS consensus security
- Remediation: Check OnHold() status in CancelUnbondingDelegation validation

## Detection Rules (for COSMOS_PROBE kit)

Chains vulnerable if:
- Cosmos SDK version < v0.47.x (check git tags on chain repo)
- `x/staking/keeper/msg_server.go:CancelUnbondingDelegation()` does NOT call `k.holdKeeper.SetHoldAccount()` before removal
- `x/staking/keeper/slash.go` slashing defer is > 1 block (timing window)
- ICS integration enabled (`x/ccv` module present)

## Actor Behavior

An OPACA actor running this lead:

1. **Seed:** UUID influences kit selection; Cosmos expert actor (level 4+) biased toward COSMOS_PROBE
2. **Phase 0 (Recon):** CONTRACT_AUDIT on Cosmos Hub repo → identifies x/staking module
3. **Phase 1 (Scan):** COSMOS_PROBE → dumps unbonding state from every ICS consumer chain
4. **Phase 2 (Detail):** CHAIN_ANALYSIS → correlates validator identities, delegation patterns
5. **Phase 3 (Exploit):** Runs fuzzer on state transitions; race condition found → ORACLE_EXPLOIT (adapted)
6. **Phase 4 (Validate):** PoC on testnet → LANCE_VALIDATE confirms P1 severity
7. **Phase 5 (Report):** IMMUNEFI_REPORT generator → submits to Cosmos Hub bug bounty
8. **Reward:** If accepted: P1 bounty ($50k-$500k), actor gains 100 score points, rep boost
9. **Persistence:** If exploited in production, VALIDATOR_BACKDOOR kit could phish affected validators (rare, ethical concern)

## Cross-Chain Impact

This vulnerability affects:

- **Provider:** Cosmos Hub (primary target)
- **Consumers:** Stride, Quicksilver, Neutron, Dydx Chain, etc. (50+ ICS chains)
- **Bridges:** Any bridge that uses staking for light-client validation (Axelar, Wormhole, etc.)

Finding on ONE chain → report to Cosmos Hub → fix released → all chains benefit.

## Tools Needed (to be dissolved into ARSON-AL)

- `probes/web3/cosmos-module-enum` — parse Cosmos SDK module state
- `picks/web3/state-transition-analyzer` — race condition finder (adapt from Echidna)
- `proofs/web3/cosmwasm-test-harness` (optional) — for Contract chains

## Bounty Platform

- **Primary:** Immunefi (Cosmos Hub Bug Bounty)
- **URL:** https://immunefi.com/bug-bounty/cosmos/
- **Payout:** P1: $50,000–$500,000 (depends on complexity + impact)
- **Verification:** Requires runnable PoC + testnet tx proof
