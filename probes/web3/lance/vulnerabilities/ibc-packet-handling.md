# IBC Packet Handling Vulnerabilities

## Class
Cross-chain fund theft, permanent fund lock, or protocol integrity failure via IBC.

## Sources
- Missing or incorrect timeout handling in `OnTimeoutPacket`
- Acknowledgment written before state changes commit
- Channel ordering violations (UNORDERED channel assumed ORDERED)
- Missing denomination trace validation in receive handlers
- Escrow imbalance (mint on receive != burn on send-back)
- Relayer-exploitable packet replay or reordering
- Middleware that drops or corrupts acknowledgment data
- ICA host executing unvalidated messages from controller

## Detection

### Packet Lifecycle Audit
For every IBC-enabled module:
1. Trace `SendPacket` — is commitment stored atomically?
2. Trace `OnRecvPacket` — are state changes committed BEFORE ack is written?
3. Trace `OnAcknowledgement` — does failure ack properly revert?
4. Trace `OnTimeoutPacket` — are escrowed funds refunded?
5. Check channel capability — is the correct module claiming this port/channel?

### Automated
```bash
# Missing timeout handler
grep -rn 'OnRecvPacket' --include='*.go' | xargs -I{} sh -c 'f=$(echo {} | cut -d: -f1); grep -L "OnTimeoutPacket" "$f"'

# Ack before state commit (dangerous pattern)
grep -rn 'WriteAcknowledgement' --include='*.go' -B10 | grep -v _test.go

# Denomination trace — look for raw denom usage without ValidatePrefixedDenom
grep -rn 'packet.GetData' --include='*.go' -A5 | grep -v 'ValidatePrefixedDenom\|ParseDenomTrace'

# ICA host message validation
grep -rn 'OnRecvPacket' --include='*.go' -A20 | grep 'cosmos.authz\|MsgExec\|MsgGrant'
```

### Manual
- Draw the full packet flow for each IBC application
- Verify escrow accounting balances across send/receive/timeout/ack
- Check if middleware modifies packet data without downstream validation
- Verify channel ordering matches application requirements

## Exploit Paths

### 1. Fund Lock via Missing Timeout
- Send IBC transfer with short timeout
- Destination chain is down or slow
- Timeout fires but source chain doesn't refund escrowed tokens
- Result: permanent fund lock

### 2. Double-Spend via Ack Race
- Packet received and state updated on destination
- Acknowledgment lost or delayed
- Source chain times out and refunds
- Result: tokens exist on both chains

### 3. Denomination Spoofing
- Craft packet with forged denomination trace
- Destination chain mints tokens with wrong origin
- Result: unbacked tokens in circulation

### 4. ICA Arbitrary Execution
- Controller sends InterchainAccountPacket with MsgExec
- Host doesn't validate inner messages against allowed list
- Result: arbitrary state changes on host chain

## Impact
- **Critical**: fund theft, unbacked token minting, double-spend
- **High**: permanent fund lock, ICA privilege escalation
- **Medium**: partial accounting discrepancy, stale IBC state
