# Cosmos SDK x/gov Keeper — Security Audit Results

Target: cosmos-sdk/x/gov/keeper/
Scope: Bug bounty — HackerOne, Cosmos SDK core
Epoch: 1751104000

---

## FINDING-01: Zero Quorum Allowed — Governance Takeover Chain

**File:** types/v1/params.go, lines 129-138
**Bug class:** Improper Access Control (CWE-284)
**Severity:** HIGH (conditional — requires param update)
**CVSS:** 7.5 (AV:N/AC:L/PR:H/UI:N/S:U/C:N/I:H/A:H)

Quorum validation allows zero:
```go
if quorum.IsNegative() { return error }  // but 0 passes
if quorum.GT(sdkmath.LegacyOneDec()) { return error }  // 0 < 1, passes
```
With Quorum="0", tally check `percentVoting.LT(quorum)` evaluates `0.LT(0) = false`, so any proposal passes with a single vote. Two-step attack: (1) param update setting quorum=0, (2) malicious proposal passing with minimal participation.

Combined with MsgUpdateParams (lines 270-285) accepting any params that pass ValidateBasic, this enables: Quorum=0, Threshold=0.000000000000000001, VetoThreshold=1.0, ProposalCancelRatio=0.

---

## FINDING-02: Expedited-to-Regular Conversion Timing Bug

**File:** abci.go, lines 218-233
**Bug class:** Incorrect Calculation (CWE-682)
**Severity:** LOW-MEDIUM
**CVSS:** 4.3

When expedited proposal fails and converts to regular, new end time = VotingStartTime + VotingPeriod. Since expedited period already elapsed, remaining time = VotingPeriod - ExpeditedVotingPeriod. If params narrowed (expedited ~= regular), converted proposal gets negligible voting time.

Fix: calculate from conversion time, not original start time.

---

## FINDING-03: ConsensusParams Authority Override

**File:** types/authority.go, lines 11-20
**Bug class:** Incorrect Authorization (CWE-863)
**Severity:** MEDIUM-HIGH (conditional)
**CVSS:** 7.2

ValidateAuthority checks consensus params first: if `cp.Authority.Authority != ""`, it overrides the keeper's authority. Whoever controls consensus param updates controls all module authorities.

---

## FINDING-04: Silently Discarded Error in Weight Parsing During Tally

**File:** tally.go, lines 95 and 132
**Bug class:** Unchecked Return Value (CWE-252)
**Severity:** MEDIUM
**CVSS:** 5.9

Vote option weights parsed with `weight, _ := math.LegacyNewDecFromStr(option.Weight)`. If stored weight is malformed, vote power silently zeroes out.

---

## Bounty Assessment

**Most reportable:** FINDING-01 (zero quorum) chained with FINDING-03 (authority override). A governance proposal setting quorum to zero + consensus param redirect = governance takeover. Requires first passing a param update, but social engineering or burying in a large proposal is realistic.

**Second:** FINDING-02 (conversion timing). Concrete logic error with clear fix. May be treated as design-level but the param narrowing attack is novel.
