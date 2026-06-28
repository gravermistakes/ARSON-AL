# CosmWasm Runtime — Security Scan Results

Target: cosmwasm/cosmwasm (main branch)
Scope: VM layer, std library, crypto package
Epoch: 1751104000

---

## FINDING-01: 9 Production Panics in Host Crypto Functions

**File:** packages/vm/src/imports.rs
**Lines:** 327, 373, 428, 537, 586, 635, 683, 737, 809
**Bug class:** Unhandled Panic / Node Crash (CWE-248)
**Severity:** MEDIUM (if triggerable), LOW (if truly unreachable)

Nine instances of `panic!("Error must not happen for this call")` in production host functions:
- bls12_381_aggregate_g1/g2 (lines 327, 373)
- bls12_381_pairing_equality (line 428)
- bls12_381_hash_to_g1/g2 (lines 537, 586)
- secp256k1_verify (line 635)
- secp256k1_recover_pubkey (line 683)
- ed25519_verify (line 737)
- ed25519_batch_verify (line 809)

These are in match arms for CryptoError variants deemed unreachable for the specific call configuration. If any path could trigger them (e.g., malformed input that passes early validation but produces an unexpected error variant), the node crashes.

**Assessment:** The panics are wrapped in `match` arms that should only fire if the crypto library returns an unexpected error variant. Gas is charged before the calls. The risk is conditional on whether a crafted contract input could produce an unexpected CryptoError variant from the underlying library. Needs deeper review of each crypto function's error space.

---

## FINDING-02: unsafe impl Send/Sync for Cache, Environment

**Files:**
- packages/vm/src/cache.rs:534-542
- packages/vm/src/environment.rs:220-222

**Bug class:** Thread Safety (CWE-362)
**Severity:** LOW (architectural trust)

`unsafe impl Send for Cache`, `unsafe impl Sync for Cache`, `unsafe impl Send/Sync for Environment`. These trust the internal state management to be thread-safe. The Cache uses an internal Mutex-like pattern. Not directly exploitable from contract code, but a bug in the VM's concurrent access patterns could cause UB.

---

## FINDING-03: allocate/deallocate Call Contract's Own Wasm Functions

**File:** packages/vm/src/instance.rs:457-472
**Bug class:** Trust Boundary (CWE-693)
**Severity:** LOW

The VM calls the contract's own `allocate` and `deallocate` exports to manage memory for data transfer. A malicious contract could provide hostile allocate/deallocate implementations. However, these run within the contract's own memory space and gas meter — they can only hurt themselves.

---

## CLEARED: Gas Metering

Gas metering is thorough throughout:
- `environment.rs:155-163`: LinearGasCost uses checked_add(checked_mul()) — overflow-safe
- `imports.rs:886-973`: db_scan/db_next/db_next_key/db_next_value all charge host_call_gas before operations
- `memory.rs:81-105`: read_region charges gas proportional to length before read
- BLS12-381 operations (imports.rs:282): MAX_AGGREGATE_SIZE = 2MB, reads charged proportionally

No gas metering gaps found.

---

## CLEARED: Wasm Feature Gating

**File:** packages/vm/src/wasm_backend/gatekeeper.rs:62-77

Default config: floats ALLOWED, bulk memory/reference types/SIMD/threads/exceptions all BLOCKED. This prevents non-determinism from threading and reference types. Floats are allowed because Wasm floats are IEEE 754 deterministic (NaN bit patterns canonicalized by the Wasm spec).

---

## CLEARED: Memory Limits

**File:** packages/vm/src/wasm_backend/limiting_tunables.rs:14-61

Memory page limits properly enforced. Validates min/max against configured limit. No bypass found.

---

## CLEARED: Region Struct Safety

**File:** packages/vm/src/memory.rs:62

`unsafe impl ValueType for Region` — Region is 12 bytes (offset/capacity/length as u32, little-endian). read_region validates max_length before reading. write_region checks capacity.

---

## Summary

| Finding | Severity | Exploitable? |
|---------|----------|-------------|
| 9 production panics in crypto | MEDIUM | Conditional — needs crypto error path analysis |
| unsafe Send/Sync | LOW | Not from contract code |
| allocate/deallocate trust | LOW | Self-contained |

CosmWasm VM is well-hardened. Gas metering, memory limits, and feature gating are solid. The production panics are the only actionable lead — requires proving a contract can trigger an unexpected CryptoError variant.
