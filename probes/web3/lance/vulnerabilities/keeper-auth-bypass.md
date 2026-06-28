# Keeper Authorization Bypass (Cosmos SDK)

## Class
Unauthorized state mutation — fund theft, parameter manipulation, or privilege escalation.

## Sources
- `MsgServer` handler missing signer validation against `msg.GetSigners()`
- Keeper method exposed without authority check (any module can call)
- `x/authz` grants with overly broad scope (MsgSend grant covers all denoms/amounts)
- Admin/authority field not validated against governance module account
- Module message router accepting unsigned or improperly signed messages
- Custom middleware skipping `SigVerificationDecorator`

## Detection

### Automated
```bash
# MsgServer handlers — check each has signer validation
grep -rn -E 'func.*MsgServer.*context\.Context.*Msg' --include='*.go' | grep -v _test.go

# Missing authority check in governance-controlled handlers
grep -rn 'func.*MsgServer' --include='*.go' -A20 | grep -v 'authority\|GetSigners\|sdk\.AccAddress'

# Keeper methods without access control (public methods callable by any module)
grep -rn -E 'func \(k \*?Keeper\) [A-Z]' --include='*.go' | grep -v '_test.go'

# Authz grant validation
grep -rn 'authz.NewGenericAuthorization\|authz.NewGrant' --include='*.go'
```

### Manual
- For every `Msg*` type: trace from `RegisterMsgServer` → handler → signer check
- List all `Keeper` public methods and their callers
- Check if any AnteDecorator conditionally skips signature verification
- Review `x/authz` grant types — does `GenericAuthorization` for a MsgType allow unintended scope?

## Exploit Path
1. Identify MsgServer handler without signer validation
2. Craft transaction with attacker address as sender
3. Handler processes without checking sender == authorized party
4. Result: unauthorized state change (fund transfer, param change, module action)

## Impact
- **Critical**: unauthorized fund transfer, minting, or burning
- **High**: parameter changes that weaken security (slashing params, unbonding period)
- **Medium**: unauthorized module actions without direct fund impact
