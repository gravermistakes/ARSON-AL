# Crash Triage and False-Positive Control

Use this file when a fuzzer produces a crash, timeout, sanitizer report, or suspicious input. The goal is to separate confirmed memory bugs from duplicates, harness artifacts, unstable reproducers, and low-evidence leads.

## Triage Pipeline

```
crashes -> reproduce -> deduplicate -> classify -> minimize -> confirm reachability -> report
```

Do not skip reproduction or minimization before calling a finding confirmed.

## Stage 1: Reproduce

Run the crash against the same instrumented binary and keep stderr/stdout.

```bash
ASAN_OPTIONS="symbolize=1:halt_on_error=1" ./target_asan crash_input
```

If the crash is flaky:

- Re-run several times with the same input.
- Check for uninitialized state, wall-clock randomness, threads, temp-file collisions, or harness cleanup.
- Label as `UNCONFIRMED` until the same crash class is stable.

## Stage 2: Deduplicate

Deduplicate by evidence, not by filenames. Use sanitizer class plus the first target-owned stack frames.

### ASan/UBSan Token

```bash
ASAN_OPTIONS="symbolize=1:halt_on_error=1:dedup_token_length=3" \
    ./target_asan crash_input 2>&1 | sed -n '/DEDUP_TOKEN/p'
```

### Stack-Based Grouping

```bash
ASAN_OPTIONS="symbolize=1:halt_on_error=1" ./target_asan crash_input 2>&1 \
    | awk '/^    #0|^    #1|^    #2|^    #3/ {print}'
```

Use `afl-cmin` to reduce coverage-equivalent crash corpora, but do not rely on it as the only root-cause deduplication method. Two inputs can share coverage while failing for different memory reasons, and two duplicate bugs can have different filenames.

Keep a small crash ledger while triaging:

| Field | Meaning |
| --- | --- |
| crash id | Original artifact path or fuzzer id |
| reproduces | Stable, flaky, or not reproduced |
| signal | Sanitizer class, signal, timeout, or OOM |
| stack key | Top target-owned frames |
| minimized | Path and size, or reason minimization failed |
| root cause | Source line or unknown |
| status | Confirmed, unconfirmed, duplicate, harness-only, or fixed |

## Stage 3: Classify

Read the sanitizer report and classify the primitive before assigning severity.

| Evidence | Bug class | Default severity |
| --- | --- | --- |
| `heap-buffer-overflow` with WRITE | Heap OOB write | Critical |
| `stack-buffer-overflow` with WRITE | Stack OOB write | Critical |
| `global-buffer-overflow` with WRITE | Global OOB write | High to Critical |
| `heap-use-after-free` with WRITE | UAF write | Critical |
| `double-free` or `bad-free` | Invalid free class | High to Critical |
| OOB or UAF READ | Memory disclosure or crash primitive | Medium to High |
| Controlled `SEGV` WRITE | Likely arbitrary write path | Critical |
| Controlled `SEGV` READ | Wild read or null dereference | Low to High |
| Integer overflow followed by undersized allocation, bounds bypass, or copy | Memory-impacting integer overflow | High |
| Standalone UBSan arithmetic warning | Undefined behavior without proven memory impact | Low to Medium |
| Leak only | Resource exhaustion or cleanup issue | Low to Medium |
| Timeout or OOM | Potential DoS or harness problem | Unconfirmed until minimized |

Severity depends on attacker control, target exposure, and whether the primitive affects confidentiality, integrity, or availability.

## Stage 4: Eliminate Bad Evidence

Do not call these confirmed memory-corruption findings without extra proof:

| Pattern | Treatment | Verification |
| --- | --- | --- |
| Crash exists only with `allocator_may_return_null=1` | Usually robustness or null-check issue, not memory corruption | Reproduce without the option and inspect allocation path |
| Crash stack is only in harness cleanup, temp-file code, or test scaffolding | Harness artifact until proven otherwise | Simplify harness or reproduce through product code |
| Timeout/OOM depends on an unrealistically huge max input | Potential DoS, not memory corruption by default | Minimize size and show realistic attacker control |
| Leak report only at process exit | Low priority unless attacker can trigger repeated growth | Re-run with leak options and inspect ownership |
| Stack overflow from recursion or parser nesting | DoS unless it becomes overwrite or controlled memory corruption | Minimize nesting depth and identify recursion guard failure |
| Mismatched C++ allocation/deallocation | Real bug class, exploitability context-dependent | Confirm type, stack, ownership, and attacker reachability |
| Static review shows `free(NULL)` | Not a bug by itself | Only report if sanitizer proves a different invalid-free path |

Required gates for `CONFIRMED`:

- Same input reproduces the same sanitizer class.
- Minimized input preserves the same sanitizer class and target-owned stack.
- Root-cause source line is identified.
- Crash path is reachable from attacker-controlled or user-controlled input.
- Behavior is not introduced solely by the harness.

If any required gate is missing, label the issue `UNCONFIRMED` and state what evidence is missing.

## Stage 5: Analyze Attacker Control

For confirmed or near-confirmed crashes, map input bytes to the failing operation:

- Length control: Which field controls allocation, copy, loop bound, recursion depth, or decompression size?
- Offset control: Which field selects index, pointer arithmetic, object id, table slot, or seek position?
- Content control: Does attacker-controlled content reach the overwritten bytes or only trigger a fixed overwrite?
- Lifetime control: Which input branch frees, reuses, swaps, or aliases the object?
- Reachability control: Is the path exposed by normal parser configuration, feature flags, file type, protocol state, or authentication boundary?

Use this mapping for severity. A sanitizer class without control analysis is evidence of a bug, not enough evidence for an exploitability claim.

## Stage 6: Minimize

### libFuzzer

```bash
./fuzz_asan -minimize_crash=1 -exact_artifact_path=minimized.bin crash_input
ASAN_OPTIONS="symbolize=1:halt_on_error=1" ./fuzz_asan minimized.bin
```

### AFL++

```bash
afl-tmin -i crash_input -o minimized.bin -- ./target_afl @@
ASAN_OPTIONS="symbolize=1:halt_on_error=1" ./target_asan minimized.bin
```

### Manual Minimization

Use this when automatic minimization breaks format invariants:

1. Identify mandatory envelope fields: magic, version, length, checksum, terminator, nesting.
2. Preserve the envelope.
3. Remove or zero nonessential payload regions.
4. Re-run after every reduction.
5. Stop when further reduction changes the crash class or target stack.

Target small reproducers, but preserve clarity over arbitrary byte-count goals. A 200-byte structured reproducer is better than a 20-byte input that no longer explains the path.

## Stage 7: Confirm and Root Cause

### Symbolized Sanitizer Report

```bash
ASAN_SYMBOLIZER_PATH="$(command -v llvm-symbolizer)" \
ASAN_OPTIONS="symbolize=1:halt_on_error=1" \
./target_asan minimized.bin 2>&1 | tee crash_report.txt
```

### Production-Like Replay

```bash
./target_prod minimized.bin
echo "exit code: $?"
```

Production replay is impact evidence. If the sanitized crash does not crash a production-like binary, it can still be a real memory bug; label it sanitizer-detected and explain the observed production behavior.

### Root-Cause Checklist

- Which source line performs the invalid read, write, free, allocation, or bounds calculation?
- Which input bytes control the relevant length, offset, object lifetime, branch, or pointer?
- Which validation check is missing, wrong, too late, or bypassed?
- Does the bug survive normal parser error handling, or is it created by the harness?
- Is the primitive write, read, free, leak, timeout, or null dereference?

## Stage 8: Variant Search

After identifying root cause, look for variants before finalizing:

- Same helper function called by other parsers or message types.
- Same length field pattern used for allocation and copy in sibling code.
- Same ownership rule violated on alternate error paths.
- Same integer cast, signedness conversion, truncation, or multiplication pattern.
- Same missing terminator or boundary check in encode and decode paths.

Do not report variants as confirmed unless each has its own reproducer or direct evidence that the same minimized input reaches the variant path.

## Stage 9: Report Template

````markdown
### Bug: [TYPE] in [function] ([file]:[line])

Status: CONFIRMED
Severity: [Critical/High/Medium/Low]
CWE: [CWE id if clear]

Summary:
[One paragraph describing the invalid operation, attacker-controlled input, and impact.]

Reproducer:
[path to minimized input, plus base64 if binary content must be embedded]

Build and run:
```bash
[exact build command]
[exact reproducer command]
```

Sanitizer evidence:
```text
[symbolized report or key stack]
```

Root cause:
[file:line and why validation/lifetime/bounds handling fails]

Reachability:
[how untrusted or user-controlled input reaches the vulnerable code]

Fix direction:
[specific bounds check, lifetime fix, allocation-size check, parser rejection, or ownership change]

Limitations:
[anything not proven, such as production exploitability, platform dependence, or sanitizer-only behavior]
````

## Severity Rules

Use the observed primitive and control relationship:

```text
OOB/UAF write with attacker-controlled offset or content -> Critical
OOB/UAF write with limited control -> High
OOB/UAF read with disclosure or parser state impact -> High
OOB/UAF read that only crashes -> Medium
Integer overflow that causes undersized allocation or copy -> High
Null dereference reachable from untrusted input -> Low to Medium
Leak, timeout, or OOM -> Low to Medium unless repeated remote trigger is proven
Harness-only crash -> Unconfirmed
```

Avoid absolute exploitability claims unless the evidence supports them. Prefer precise language: "attacker controls the length field used in `memcpy`" is stronger than "possibly exploitable."
