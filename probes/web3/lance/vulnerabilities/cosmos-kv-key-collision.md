# Cosmos KV Store Key Collision

## Hunt Targets

- Use of simple string concatenation for KV store keys (e.g. `[]byte(poolID + tokenDenom)`) without delimiter separation.
- Variable-length parameter concatenation in KV store key composition.
- Collision patterns in composite keys (e.g. key `12` + `abc` colliding with key `1` + `2abc` if concatenated as `12abc`).

## Exploit Checks

- Identify composite KV store key construction using variable-length fields.
- Construct two distinct domain inputs (e.g. pool ID `12` + denom `abc` vs pool ID `1` + denom `2abc`) that resolve to the identical concatenated byte array.
- Prove that writing state for the second input overwrites or reads state from the first input, enabling unauthorized access, position balance theft, or privilege escalation.

## Reject Conditions

- Inputs are fixed-length (e.g. fixed-length hash or integer serialization) or padded to prevent length manipulation.
- Key parts are cleanly separated with a distinct non-colliding byte delimiter (e.g. `\x00`).

## Evidence Required

- Concatenation code block for composite keys.
- Concrete inputs showing collision path and the affected state parameters.
