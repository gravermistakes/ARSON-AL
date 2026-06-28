# Unmetered / Unbounded Computation (Cosmos SDK)

## Class
Chain halt via DoS — execution outside gas metering runs unbounded.

## Sources
- BeginBlock/EndBlock hooks executing CosmWasm contracts without gas limits
- Recursive contract calls in module-executed context
- Slow-converging math (Newton's method, iterative solvers) without iteration caps
- Token transfer hooks triggering arbitrary contract execution
- Arbitrage/MEV logic in module context without gas bounds

## Detection

### Automated
```bash
# BeginBlocker/EndBlocker calling contract execution
grep -rn 'BeginBlock\|EndBlock' --include='*.go' -A30 | grep -i 'contract\|wasm\|execute\|sudo' | grep -v _test.go

# Missing gas limit in context — look for ctx without GasMeter override
grep -rn 'WithGasMeter\|WithBlockGasMeter\|GasMeter' --include='*.go' | grep -v _test.go

# Hook implementations that could call external code
grep -rn 'Hook\|Callback\|OnRecv\|PostTransfer\|AfterTransfer' --include='*.go' -A10 | grep -v _test.go

# Iterative math without bounds
grep -rn 'for.*{' --include='*.go' -A5 | grep -i 'newton\|converge\|iterate\|approx' | grep -v _test.go

# Recursive calls
grep -rn 'func.*\(.*\).*{' --include='*.go' -A20 | grep -v _test.go | grep -B5 'self\.\|k\.\|keeper\.' | grep -i 'call\|execute\|dispatch'
```

### Manual
- Trace every BeginBlock/EndBlock path for external code execution
- Verify all contract calls in module context have gas limits via `ctx.WithGasMeter(sdk.NewGasMeter(limit))`
- Check token transfer hooks — can a malicious token's hook run arbitrary code?
- Review iterative algorithms for convergence guarantees and iteration caps
- Map all paths where user-controlled input affects loop/recursion depth

## Exploit Path
1. Deploy malicious CosmWasm contract with infinite loop (or very expensive computation)
2. Create token whose transfer hook triggers the contract
3. Arrange for token to be transferred during BeginBlock (e.g., via epoch distribution, staking reward)
4. BeginBlock runs without per-operation gas limit
5. Contract executes unbounded — block production halts
6. Result: chain halt, requires coordinated validator intervention

## Impact
- **Severity**: Critical
- **CVSS**: 9.0+ (full availability loss)
- Chain halt affects all users, applications, IBC channels
- Particularly dangerous in chains with CosmWasm + custom token hooks

## Mitigations (what good code looks like)
```go
// Wrap risky calls in gas-limited context
gasLimitCtx := ctx.WithGasMeter(sdk.NewGasMeter(maxGas))
err := k.executeContract(gasLimitCtx, contractAddr, msg)
// Catch out-of-gas panic
```

## Reference
- Fault Tolerant: Cosmos Security Handbook (2024)
- Pattern: "If a chain allows arbitrary token transfer hooks and triggers them in BeginBlock, an attacker can create a token that executes an infinitely looping CosmWasm contract"
