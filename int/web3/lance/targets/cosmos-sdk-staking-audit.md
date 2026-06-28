# Cosmos SDK x/staking Keeper — Security Audit Results

Target: cosmos-sdk/x/staking/keeper/
Scope: Bug bounty — HackerOne, Cosmos SDK core
Epoch: 1751104000

---

## HOT LEAD: CancelUnbondingDelegation Bypasses OnHold (ICS Slashing Evasion)

**File:** msg_server.go, lines 469-596
**Bug class:** Missing Validation / ICS Bypass (CWE-862)
**Severity:** HIGH
**CVSS:** 6.5 (AV:N/AC:L/PR:L/UI:N/S:U/C:N/I:H/A:N)

CancelUnbondingDelegation does NOT check `entry.OnHold()`. Every other path that interacts with unbonding entries checks OnHold:
- CompleteUnbonding (delegation.go:1190): `entry.IsMature(ctxTime) && !entry.OnHold()`
- Slash unbonding (slash.go:255): `entry.IsMature(now) && !entry.OnHold()`
- Slash redelegation (slash.go:322,352): same pattern

OnHold is set by external modules (ICS, IBC) via `PutUnbondingOnHold(id)` to prevent unbonding completion until the module releases it. CancelUnbondingDelegation skips this entirely.

Attack path:
1. Delegate to validator V on ICS provider chain
2. V commits infraction on consumer chain C
3. Undelegate from V (unbonding entry created)
4. ICS calls PutUnbondingOnHold on the entry (prevents completion until slash resolves)
5. Call MsgCancelUnbondingDelegation — bypasses OnHold, re-delegates tokens back
6. Consumer chain's slash packet arrives but the original unbonding entry is modified/removed
7. Delegator evades consumer chain slash

**Verified:** Lines 537-555 show the only checks are CreationHeight match, balance sufficiency, and CompletionTime not matured. No OnHold check. No refcount decrement. No notification to the holding module.

**Action:** Verify against current ICS implementation. If ICS relies on staking's OnHold to prevent premature unbonding for cross-chain slashing, this is a HIGH severity bypass. Prepare PoC.

---

## FINDING-01: InitialBalance Desync in Partial Cancel After Slash

**File:** msg_server.go, lines 564-571
**Bug class:** Accounting Inconsistency (CWE-682)
**Severity:** MEDIUM-HIGH
**CVSS:** 5.3 (AV:N/AC:H/PR:L/UI:N/S:U/C:N/I:H/A:N)

After slashing reduces Balance below InitialBalance, a partial CancelUnbondingDelegation subtracts msg.Amount from BOTH Balance and InitialBalance:
```go
unbondEntry.Balance = amount  // Balance - msg.Amount
unbondEntry.InitialBalance = unbondEntry.InitialBalance.Sub(msg.Amount.Amount)
```
This changes the effective slash ratio. If a second slash hits at the same infraction height, the slash amount (calculated from InitialBalance) is lower than intended.

---

## FINDING-02: Slash remainingSlashAmount Can Go Negative

**File:** slash.go, lines 90-143
**Bug class:** Integer Logic Error (CWE-191)
**Severity:** MEDIUM
**CVSS:** 4.3

Deductions from remainingSlashAmount use IDEAL amounts (from InitialBalance), not actual burned amounts (capped at Balance). The defensive `math.MaxInt(tokensToBurn, math.ZeroInt())` on line 148 catches this — validator may lose nothing from direct delegation.

---

## FINDING-03: SlashRedelegation Picks Up Unrelated Unbonding Entries

**File:** slash.go, lines 338-367
**Bug class:** Logic Error (CWE-697)
**Severity:** MEDIUM
**CVSS:** 4.0

The inner loop over destination unbonding entries can catch entries from a direct delegation to B (unrelated to redelegation from A).

---

## FINDING-04: Swallowed Errors in BlockValidatorUpdates

**File:** val_state_change.go, lines 60-62, 95-101
**Bug class:** Silent Failure (CWE-754)
**Severity:** LOW-MEDIUM
**CVSS:** 3.7

CompleteUnbonding/CompleteRedelegation errors silently `continue`. The DVPair was already dequeued from the time queue, so tokens become permanently locked.

---

## FINDING-05: Consensus Key Rotation Slashing Evasion Window

**File:** rotation.go, lines 274-323, 389-429
**Bug class:** Race Condition (CWE-362)
**Severity:** MEDIUM
**CVSS:** 4.9

After rotation (2-block delay), old cons addr index is deleted. Slash() uses GetValidatorByConsAddr — evidence against old key post-rotation may fail to resolve. Depends on whether x/evidence does its own historical resolution.

---

## Severity Summary

| Finding | CVSS | Exploitable? | Bounty Potential |
|---------|------|-------------|-----------------|
| OnHold bypass | 6.5 | YES — direct MsgCancelUnbondingDelegation | HIGH |
| InitialBalance desync | 5.3 | YES — requires slash + cancel + slash | MEDIUM |
| Rotation slashing evasion | 4.9 | Depends on ICS impl | MEDIUM |
| SlashRedelegation wrong entries | 4.0 | Requires multi-delegation | LOW |
| Swallowed errors | 3.7 | Requires bank failure | LOW |
