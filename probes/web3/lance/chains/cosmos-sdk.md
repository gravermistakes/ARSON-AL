# Cosmos SDK / IBC / CosmWasm Chain Guide

## Architecture

Cosmos SDK apps are Go modules composed of:
- **Modules** (x/bank, x/staking, x/gov, x/authz, x/ibc, custom) — each owns state and message handlers
- **Keeper** — module's state access layer (reads/writes to KVStore)
- **MsgServer** — handles transactions (state-mutating)
- **Querier** — handles queries (read-only)
- **BeginBlocker / EndBlocker** — per-block hooks
- **AnteHandler chain** — tx validation pipeline before execution
- **IBC** — Inter-Blockchain Communication (packet relay between chains)
- **CosmWasm** — optional smart contract runtime (Rust → Wasm)

## Cosmos-Specific Vulnerability Classes

### 1. Keeper Authorization Bypass
- Missing `msg.GetSigners()` validation in MsgServer handlers
- Keeper methods callable without proper authority checks
- `x/authz` grant scope too broad (granting MsgSend covers all denominations)
- Admin-only functions missing `authority` field check against governance

**Check**: every `Msg*` handler must validate signer == authorized party.

### 2. State Machine Non-Determinism
- Use of `map` iteration (Go maps iterate in random order)
- Floating-point arithmetic in state transitions
- Time-dependent logic using wall clock instead of `ctx.BlockTime()`
- External HTTP calls in message handlers
- Goroutines in ABCI handlers

**Impact**: consensus failure — validators produce different state roots, chain halts.

### 3. IBC Packet Handling
- Missing packet timeout handling (funds locked permanently)
- Incorrect acknowledgment writing (ack written before state commit)
- Channel ordering assumptions (UNORDERED channels deliver out-of-order)
- Missing `OnChanOpenTry` / `OnChanOpenConfirm` validation
- Relayer-exploitable packet sequences
- IBC middleware that modifies packets without re-validating
- Missing denomination trace validation in `x/ibc-transfer`

### 4. Integer Overflow / Token Arithmetic
- `sdk.Int` / `math.Int` operations without overflow checks on older SDK versions
- Decimal truncation in `sdk.Dec` division (rounding down steals dust)
- Negative coin amounts not rejected (pre-SDK v0.46 edge cases)
- Share-to-token conversion rounding in staking/LP (vault share inflation variant)

### 5. Governance Attack Vectors
- Proposals that execute arbitrary module calls via `x/authz` or `x/group`
- Parameter changes that break invariants (e.g., setting unbonding period to 0)
- Software upgrade proposals injecting malicious binary hashes
- Expedited proposals with reduced voting period + low quorum

### 6. BeginBlocker / EndBlocker Bugs
- Unbounded iteration in blockers (gas exhaustion → chain halt)
- State mutation order dependencies between modules
- Missing error handling (panics in blocker halt the chain)
- Epoch-based logic that skips blocks (epoch boundary off-by-one)

### 7. AnteHandler Chain Issues
- Custom AnteDecorators that skip signature verification
- Gas metering bypass (free tx execution)
- Fee market manipulation (min gas price set to 0)
- Sequence number replay when AnteHandler doesn't increment

### 8. CosmWasm-Specific
- Unbounded loops consuming all gas (DoS on chain)
- Cross-contract reentrancy via `SubMsg::Reply`
- Instantiate2 address collision (predictable contract addresses)
- Missing migration entry points (contract stuck on old code)
- Admin key management (admin can migrate to malicious code)
- IBC-enabled contracts with incorrect channel validation

### 9. Staking / Slashing Edge Cases
- Redelegation chains that bypass unbonding period
- Slashing during unbonding (double-counted penalties)
- Validator set manipulation via rapid delegate/undelegate
- Tombstoned validator edge cases with pending rewards

### 10. Module Account Drains
- Direct sends to module accounts that bypass expected deposit flows
- Module account balance assumptions (other modules can send tokens)
- Community pool drain via governance with insufficient checks

## IBC Deep Dive

### Packet Lifecycle
```
SendPacket → Relay → RecvPacket → WriteAcknowledgement → AcknowledgePacket
                                                    or → TimeoutPacket
```

### Critical IBC Checks
- Packet commitment stored BEFORE execution (atomicity)
- Acknowledgment written AFTER state changes succeed
- Timeout height AND timeout timestamp both validated
- Channel capability claimed correctly (one module per channel)
- Port binding authenticated (only owner module can bind)
- Denomination traces validated on receive (no spoofed origins)
- Escrow accounting balanced (mint on receive == burn on send-back)

### IBC Middleware Risks
- Middleware wrapping that drops or modifies ack data
- Fee middleware charging incorrect relayer fees
- Interchain Accounts (ICA) host module executing arbitrary msgs
- Interchain Queries (ICQ) returning stale or manipulated data

## Cosmos Audit Tooling

### Static Analysis
- `gosec` — Go security linter (catches some crypto/auth issues)
- `staticcheck` — Go static analysis (catches non-determinism patterns)
- `golangci-lint` — aggregator (enable `govet`, `gosec`, `ineffassign`, `staticcheck`)
- `semgrep` — custom rules for Cosmos patterns (write your own)

### Cosmos-Specific
- `cosmosvisor` — upgrade handler testing
- Manual: grep for `map` iteration in state handlers, `time.Now()`, `go func`, `math/rand`
- Manual: trace every `Msg*` handler for authorization checks
- Invariant checkers: `x/crisis` module's `InvariantRoute` — run after each block in testing

### CosmWasm
- `cosmwasm-check` — validates Wasm binary compatibility
- `cargo audit` — Rust dependency CVE check
- `clippy` with custom lints for CosmWasm patterns
- `cw-orchestrator` — integration testing framework

## Non-Determinism Quick Grep

```bash
# Map iteration in state handlers (consensus-breaking)
grep -rn 'range.*map\[' --include='*.go' | grep -v '_test.go' | grep -v vendor

# Wall clock usage (consensus-breaking)
grep -rn 'time\.Now()' --include='*.go' | grep -v '_test.go' | grep -v vendor

# Goroutines in ABCI (consensus-breaking)
grep -rn 'go func\|go .*(' --include='*.go' | grep -v '_test.go' | grep -v vendor

# Math/rand (consensus-breaking — use crypto/rand or deterministic source)
grep -rn 'math/rand' --include='*.go' | grep -v '_test.go' | grep -v vendor

# Float usage in state (consensus-breaking)
grep -rn 'float32\|float64' --include='*.go' | grep -v '_test.go' | grep -v vendor

# Missing error checks
grep -rn 'err :=' --include='*.go' -A1 | grep -v 'if err' | grep -v '_test.go'
```

## Reportability

Report when:
- State machine non-determinism is proven (chain halt = critical)
- Fund theft/lock via keeper bypass, IBC packet manipulation, or token arithmetic
- Governance manipulation with realistic attack path
- Chain halt via unbounded iteration or panic in blockers
- Gas bypass allowing free execution

Severity mapping:
- **Critical**: chain halt, arbitrary fund theft, consensus break
- **High**: specific fund theft with prerequisites, governance takeover
- **Medium**: DoS with economic cost, dust theft via rounding, stale data exploitation
- **Low**: theoretical with unrealistic prerequisites
