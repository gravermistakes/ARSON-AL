# State Machine Non-Determinism (Cosmos SDK)

## Class
Consensus-breaking bug. Validators compute different state roots → chain halt.

## Sources
- Go `map` iteration in state-mutating code (random order per Go spec)
- `float32`/`float64` arithmetic (platform-dependent rounding)
- `time.Now()` instead of `ctx.BlockTime()` (wall clock varies per validator)
- `math/rand` without deterministic seed from block state
- Goroutines in ABCI handlers (non-deterministic execution order)
- External HTTP/network calls in message handlers
- Environment-dependent string sorting (locale-sensitive)

## Detection

### Automated
```bash
# Map iteration — highest frequency Cosmos consensus bug
grep -rn 'for.*range' --include='*.go' | xargs grep -l 'map\[' | grep -v _test.go | grep -v vendor

# Float arithmetic
grep -rn 'float32\|float64' --include='*.go' | grep -v _test.go | grep -v vendor

# Wall clock
grep -rn 'time\.Now\(\)' --include='*.go' | grep -v _test.go | grep -v vendor

# Goroutines in handlers
grep -rn 'go func\|go [a-zA-Z]' --include='*.go' | grep -v _test.go | grep -v vendor

# Non-deterministic random
grep -rn 'math/rand' --include='*.go' | grep -v _test.go | grep -v vendor
```

### Manual
- Trace every `BeginBlocker`, `EndBlocker`, and `MsgServer` handler
- Check if any sorted output depends on map-derived input
- Look for conditional logic based on system state (env vars, file existence)

## Exploit Path
1. Identify non-deterministic operation in state-mutating code path
2. Show two validators can produce different state after same block
3. Result: chain halt, requiring coordinated validator restart with patched binary

## Impact
- **Severity**: Critical
- **CVSS**: 9.0+ (availability loss for entire chain)
- Chain halt affects all users, all applications, all IBC channels
- Emergency governance proposal or hard fork required to resume

## Reportability
- Must demonstrate reachability from a real transaction
- Map iteration in dead code or test files doesn't count
- Must be in a state-mutating path (queries are fine)
