# Fuzzing Toolchain Reference

Use these as templates. Always replace placeholder paths, source files, corpus directories, and binary names with repository-specific values before running commands.

## Tool Discovery

```bash
command -v clang
command -v llvm-symbolizer
command -v llvm-profdata
command -v llvm-cov
command -v afl-fuzz
command -v honggfuzz
```

If a tool is missing, state the limitation and use the best available path. Do not claim results from unavailable tools.

## Sanitizer Build Configurations

### Build-System Integration

Prefer the project's native build system over direct compiler commands.

```bash
# Autotools or Make-style projects.
CC=clang CXX=clang++ \
CFLAGS="-g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer" \
CXXFLAGS="-g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer" \
LDFLAGS="-fsanitize=address,undefined" \
make

# CMake projects.
cmake -S . -B build-asan \
    -DCMAKE_C_COMPILER=clang \
    -DCMAKE_CXX_COMPILER=clang++ \
    -DCMAKE_C_FLAGS="-g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer" \
    -DCMAKE_CXX_FLAGS="-g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer" \
    -DCMAKE_EXE_LINKER_FLAGS="-fsanitize=address,undefined"
cmake --build build-asan
```

Do not discard project-required defines, include paths, generated headers, or linker inputs. If the direct compile command fails, inspect the native build or `compile_commands.json` and adapt it.

### libFuzzer ASan + UBSan

Use for the first in-process fuzz target.

```bash
clang -g -O1 -fsanitize=fuzzer,address,undefined \
    -fno-omit-frame-pointer \
    -fno-sanitize-recover=all \
    -fsanitize-address-use-after-scope \
    -o fuzz_asan harness.c target.c
```

For multi-file projects, compile shared objects with `-fsanitize=fuzzer-no-link,address,undefined` and link only the final harness binary with `-fsanitize=fuzzer,address,undefined`.

### Standalone ASan + UBSan

Use for file-oriented reproducers, AFL++, Honggfuzz, and minimized crash replay.

```bash
clang -g -O1 -fsanitize=address,undefined \
    -fno-omit-frame-pointer \
    -fno-sanitize-recover=all \
    -fsanitize-address-use-after-scope \
    -o target_asan harness_or_driver.c target.c
```

### MSan

Use only when uninitialized reads are in scope and every linked dependency can be MSan-compatible.

```bash
clang -g -O1 -fsanitize=fuzzer,memory \
    -fsanitize-memory-track-origins=2 \
    -fno-omit-frame-pointer \
    -o fuzz_msan harness.c target.c
```

### Coverage Build

Use to verify reachability and minimize corpus by coverage.

```bash
clang -g -O1 -fprofile-instr-generate -fcoverage-mapping \
    -o target_cov harness_or_driver.c target.c
```

### Production-Like Replay Build

Use for impact evidence, not as the only confirmation path.

```bash
cc -g -O2 -o target_prod harness_or_driver.c target.c
```

## Sanitizer Runtime Options

Use one `ASAN_OPTIONS` value per run; merge required options instead of overwriting earlier exports.

```bash
# General crash quality.
ASAN_OPTIONS="detect_stack_use_after_return=1:strict_string_checks=1:halt_on_error=1:symbolize=1"

# Leak detection when relevant; can be noisy for fuzz harnesses with intentional process lifetime allocations.
ASAN_OPTIONS="detect_leaks=1:halt_on_error=1:symbolize=1"

# Dedup token emission for stack-based grouping.
ASAN_OPTIONS="dedup_token_length=3:symbolize=1:halt_on_error=1"

# Allocator failure exploration. Do not classify allocator-failure-only null derefs as memory corruption.
ASAN_OPTIONS="allocator_may_return_null=1:symbolize=1:halt_on_error=1"
```

Use separate build directories for ASan/UBSan, MSan, coverage, and production-like replay. Do not mix MSan objects with non-MSan dependencies and do not reuse stale object files across sanitizer modes.

Set symbolizer explicitly when sanitizer output is unsymbolized:

```bash
ASAN_SYMBOLIZER_PATH="$(command -v llvm-symbolizer)" \
ASAN_OPTIONS="symbolize=1:halt_on_error=1" \
./target_asan minimized.bin
```

## libFuzzer Commands

```bash
# Smoke run.
./fuzz_asan -max_len=4096 -runs=10000 corpus/ -artifact_prefix=fuzz-work/crashes/

# Time-bounded run.
./fuzz_asan -max_len=4096 -rss_limit_mb=4096 -max_total_time=3600 \
    corpus/ -artifact_prefix=fuzz-work/crashes/

# Parallel workers. Choose workers based on available CPU and user-approved runtime.
./fuzz_asan -jobs=8 -workers=8 corpus/ -artifact_prefix=fuzz-work/crashes/

# Dictionary-assisted run.
./fuzz_asan -dict=protocol.dict corpus/ -artifact_prefix=fuzz-work/crashes/

# Compare-heavy target. Useful after coverage stalls on magic values or integer checks.
./fuzz_asan -use_value_profile=1 corpus/ -artifact_prefix=fuzz-work/crashes/

# Corpus merge/minimization.
./fuzz_asan -merge=1 corpus_min/ corpus/

# Crash minimization to an explicit output path.
./fuzz_asan -minimize_crash=1 -exact_artifact_path=minimized.bin crash_input
```

Exchange corpora between engines instead of treating each fuzzer as isolated:

```bash
# Import AFL++ queue into libFuzzer corpus.
./fuzz_asan -merge=1 corpus_from_afl/ findings/main/queue/

# Feed libFuzzer corpus back into AFL++ after minimization.
afl-cmin -i corpus/ -o afl_in_min/ -- ./target_afl @@
```

## AFL++ Commands

Build AFL++ targets with the repository's real source list and build system when possible.

```bash
# One-off direct build.
AFL_USE_ASAN=1 afl-clang-fast -g -O1 -fno-omit-frame-pointer \
    -o target_afl harness_or_driver.c target.c

# Main fuzzer.
afl-fuzz -i corpus/ -o findings/ -M main -- ./target_afl @@

# Secondary fuzzer with a different schedule.
afl-fuzz -i corpus/ -o findings/ -S secondary1 -p explore -- ./target_afl @@

# CmpLog build and run.
AFL_LLVM_CMPLOG=1 AFL_USE_ASAN=1 afl-clang-fast -g -O1 \
    -o target_cmplog harness_or_driver.c target.c
afl-fuzz -i corpus/ -o findings/ -S cmplog -c ./target_cmplog -- ./target_afl @@

# Minimize coverage-equivalent corpus.
afl-cmin -i corpus/ -o corpus_min/ -- ./target_afl @@

# Minimize a single crashing input.
afl-tmin -i crash_input -o minimized.bin -- ./target_afl @@
```

Use AFL++ persistent mode only when the harness is written for `__AFL_LOOP`.

Advanced AFL++ choices:

- Use `-p explore` for broad early discovery and switch schedules only when coverage stalls.
- Use CmpLog for parsers gated by magic constants, checksums, length comparisons, or nested tags.
- Use persistent mode only after proving per-iteration state reset. A fast but state-leaking persistent harness creates misleading crashes.
- Keep ASan builds for crash quality, but compare execution speed against non-ASan instrumentation when campaigns are long and crashes are not the immediate goal.

## Honggfuzz Commands

```bash
# File-oriented target.
hfuzz-clang -g -O1 -fsanitize=address,undefined \
    -fno-omit-frame-pointer \
    -o target_hfuzz harness_or_driver.c target.c
honggfuzz -i corpus/ -o findings/ -- ./target_hfuzz ___FILE___

# Persistent-mode target, only with a compatible harness.
honggfuzz -i corpus/ -o findings/ -P -- ./target_hfuzz
```

## Harness Patterns

### libFuzzer In-Process Harness

```c
#include <stddef.h>
#include <stdint.h>

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size > MAX_INPUT_SIZE) return 0;
    target_reset_state();
    target_parse(data, size);
    return 0;
}
```

### AFL++ Persistent Harness

```c
#include <stddef.h>
#include <stdint.h>

__AFL_FUZZ_INIT();

int main(void) {
    __AFL_INIT();
    unsigned char *buf = __AFL_FUZZ_TESTCASE_BUF;
    target_init_once();

    while (__AFL_LOOP(10000)) {
        int len = __AFL_FUZZ_TESTCASE_LEN;
        if (len < 0 || len > MAX_INPUT_SIZE) continue;
        target_reset_state();
        target_parse(buf, (size_t)len);
    }

    return 0;
}
```

### File-Oriented Driver

Use when the target only accepts filenames.

```c
#include <stdio.h>

int main(int argc, char **argv) {
    if (argc != 2) return 0;
    return target_parse_file(argv[1]);
}
```

## Harness Rules

- Initialize expensive global state once when the API allows it.
- Reset mutable state before each fuzz iteration.
- Return on invalid input. Do not abort on expected parser failures.
- Avoid network access, threads, sleeps, wall-clock randomness, and unbounded filesystem writes in the fuzz path.
- Bound input size before allocations, recursion, or decompression.
- Close handles and free per-iteration allocations so leak reports remain meaningful.
- Assert harness invariants only outside the fuzzer hot path. Parser rejection is not a crash.
- If the target uses callbacks or allocators, expose fuzzer-controlled data only through the same API surface a real caller can reach.
- For stateful protocols, model one complete message or transaction per iteration before attempting multi-step sequences.

## Harness Review Checklist

- Does every byte consumed by the target come from the fuzzer input or a fixed seed fixture?
- Is the target reset enough that input order cannot change crash behavior?
- Can a minimized input be replayed with a standalone binary?
- Are parser errors ignored while sanitizer failures still abort?
- Is the max input length low enough to avoid wasting cycles but high enough to reach deep structures?
- Does coverage show the intended target-owned files, not only the harness or framework code?

## Corpus Management

### Seed Sources

Use existing valid artifacts first:

```bash
find tests samples fixtures -type f 2>/dev/null
```

For protocols or file formats, add edge cases that preserve the envelope enough to reach parser internals: empty payload, truncated header, maximum length field, duplicate field, nested object, invalid checksum, and missing terminator.

### Coverage Check

```bash
mkdir -p profraw
for f in corpus/*; do
    LLVM_PROFILE_FILE="profraw/%p.profraw" ./target_cov "$f" || true
done
llvm-profdata merge -sparse profraw/*.profraw -o total.profdata
llvm-cov report ./target_cov -instr-profile=total.profdata
llvm-cov show ./target_cov -instr-profile=total.profdata
```

If coverage is flat or risky code is unreached, improve seeds, add a dictionary, narrow or expand the harness entry point, or add a structure-aware mutator.

### Custom Mutator Trigger

Add a custom mutator when random byte mutation stalls on checksums, length fields, magic bytes, nested structure, or required field ordering. Keep mutation format-aware and cheap.

```c
#include <stddef.h>
#include <stdint.h>
#include <string.h>

size_t LLVMFuzzerCustomMutator(
    uint8_t *data, size_t size, size_t max_size, unsigned int seed) {
    if (size < 4 || max_size < 4) return size;

    uint32_t len;
    memcpy(&len, data, sizeof(len));
    len ^= (uint32_t)(seed & 0xffffu);
    memcpy(data, &len, sizeof(len));

    return size;
}
```

### Dictionary

Create `protocol.dict` with one quoted token per line:

```text
"GET"
"POST"
"HTTP/1.1"
"Host:"
```

Then run:

```bash
./fuzz_asan -dict=protocol.dict corpus/ -artifact_prefix=fuzz-work/crashes/
```

## Runtime Scaling

| Depth | Runtime | Valid conclusion |
| --- | --- | --- |
| Smoke | seconds to 15 minutes | Tooling and harness sanity only |
| Standard | 1 to 4 hours | Useful bug-hunting pass with stated limits |
| Thorough | 24+ hours | Audit-style campaign with coverage monitoring |

Ask before launching standard or thorough campaigns from an interactive agent session.
