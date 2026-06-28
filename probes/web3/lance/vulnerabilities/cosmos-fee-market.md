# Fee Market & Gas Mispricing (Cosmos SDK)

## Class
DoS via fee underpayment, client instability, or economic extraction.

## Sources
- State-creating operations not charging proportional gas
- AnteHandler `simulate` parameter creating gas estimate divergence
- Contract-called functions with fixed fees that break composability
- Missing dynamic fee adjustment during congestion (no EIP-1559 equivalent)
- SendCoins batch operations where single failure panics entire batch

## Detection

### Automated
```bash
# AnteHandler checking simulate boolean (gas divergence risk)
grep -rn 'simulate\|isSimulate\|ctx.IsCheckTx\|ctx.IsReCheckTx' --include='*.go' -A5 | grep -i 'gas\|fee\|charge' | grep -v _test.go

# Fixed fee charges (break when contracts call on behalf of users)
grep -rn 'DeductFee\|ChargeGas\|ConsumeGas' --include='*.go' -B5 | grep -i 'fixed\|constant\|param' | grep -v _test.go

# SendCoins (batch) vs SendCoin (individual) — batch panics on single failure
grep -rn 'SendCoins\b' --include='*.go' | grep -v _test.go | grep -v vendor

# State creation without proportional gas
grep -rn 'SetKey\|Set(\|Store.Set\|KVStore.*Set' --include='*.go' -B10 | grep -v 'ConsumeGas\|GasConfig' | grep -v _test.go

# Unbounded state iteration cost not charged to creator
grep -rn 'Iterator\|Iterate' --include='*.go' -B5 | grep -v 'GasMeter\|ConsumeGas' | grep -v _test.go
```

### Manual
- Trace every state-creating operation (new account, new token, new position) for gas cost
- Check if gas cost scales with future iteration cost the new state imposes
- Compare gas estimates from `simulate=true` vs actual execution
- Review fee handling when contracts call governance-fee-bearing functions
- Check for SendCoins in BeginBlock/EndBlock (panic risk on invalid coin)

## Exploit Paths

### 1. State Spam via Mispriced Creation
- Operation creates new state entry (e.g., new token denom, new account)
- Gas charged is flat, not proportional to future iteration cost
- Attacker creates millions of entries cheaply
- Future operations iterating over entries become unaffordable
- Result: DoS on iteration-dependent operations

### 2. Gas Estimate Divergence
- AnteHandler has conditional logic: `if simulate { skip expensive check }`
- Client simulates tx: estimates low gas
- Client submits with estimated gas: tx fails (real execution needs more)
- Result: client UX degradation, repeated failed txs, wasted fees

### 3. Contract Fee Bubbling
- Governance sets fixed fee on operation X
- Contract C calls operation X on behalf of user
- Fee is charged to contract C, not end user
- Contract cannot pass fee through
- Result: composability break, contracts cannot integrate with fee-bearing operations

### 4. Batch Panic via SendCoins
- BeginBlock distributes rewards using `SendCoins` (batch)
- One coin in the batch is invalid (e.g., zero amount after rounding)
- `SendCoins` panics on validation failure
- Result: chain halt from BeginBlock panic

## Impact
- **High**: chain halt via batch panic in BeginBlock
- **Medium**: DoS via state spam, client instability from gas divergence
- **Low**: composability break (functional but limits protocol integration)

## Reference
- Fault Tolerant: Cosmos Security Handbook (2024)
- "Each new token can potentially push an increasing cost onto the system. Additional and scaling gas must be charged."
