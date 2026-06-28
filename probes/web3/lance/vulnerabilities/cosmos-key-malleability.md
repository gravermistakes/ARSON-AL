# Store Key Malleability & Prefix Iteration (Cosmos SDK)

## Class
State corruption, unauthorized access, or data overwrite via store key design flaws.

## Sources
- Unpadded numeric IDs in composite store keys (prefix collision)
- Non-unique composite keys (data overwrite)
- Iterator bounds errors (off-by-one inclusion/exclusion)
- Missing key separator suffixes after variable-length fields

## Detection

### Automated
```bash
# Composite keys using fmt.Sprintf with %d (unpadded decimal — prefix collision)
grep -rn 'fmt.Sprintf.*%d' --include='*.go' | grep -i 'key\|prefix\|store' | grep -v _test.go

# Key construction with string concatenation of numeric IDs
grep -rn 'sdk.Uint64ToBigEndian\|binary.BigEndian.PutUint64' --include='*.go' | grep -v _test.go

# Prefix iteration — check for correct bounds
grep -rn 'storetypes.KVStorePrefixIterator\|store.Iterator\|sdk.KVStorePrefixIterator' --include='*.go' | grep -v _test.go

# Reverse iteration (must add +1 byte to end bound)
grep -rn 'storetypes.KVStoreReversePrefixIterator\|store.ReverseIterator' --include='*.go' | grep -v _test.go

# Key functions without separators
grep -rn -E 'func.*Key.*(uint64|string)' --include='*.go' | grep -v _test.go | grep -v vendor
```

### Manual
- Map every composite key schema in the module's `types/keys.go`
- Verify numeric IDs are big-endian encoded, not decimal string formatted
- Check for separator bytes between variable-length key components
- Trace every `Iterator` / `ReverseIterator` call for correct start/end bounds
- Verify composite key uniqueness — can two distinct objects produce the same key?

## Exploit Paths

### 1. Prefix Collision via Unpadded IDs
- Key uses `fmt.Sprintf("%s%d", prefix, poolId)` for poolId
- poolId=42 produces key bytes `[52 50]`
- poolId=421 produces key bytes `[52 50 49]`
- Prefix iteration over poolId=42 matches BOTH entries
- Result: unauthorized read/write of another pool's data

### 2. Data Overwrite via Non-Unique Keys
- Two distinct objects (e.g., different delegations) map to same composite key
- Second `Set()` silently overwrites the first
- Result: state corruption, lost records, accounting errors

### 3. Iterator Bounds Off-by-One
- Reverse prefix iterator missing +1 byte on end boundary
- Boundary entry included/excluded incorrectly
- Result: skipped or duplicated entries in state iteration

## Impact
- **Critical**: unauthorized fund access via prefix collision on balance keys
- **High**: state corruption via key overwrite or iterator bounds error
- **Medium**: data leak across pool/account boundaries

## Reference
- Fault Tolerant: Cosmos Security Handbook (2024)
- Pattern observed in Osmosis (poolId prefix collision), multiple appchains
