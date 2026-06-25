#!/bin/bash
# Build the vulnerable parser with different sanitizer configurations
set -e

cd "$(dirname "$0")"

# ASan + UBSan build (primary fuzzing target)
clang -g -fsanitize=address,undefined -fno-omit-frame-pointer \
    -DSTANDALONE -o vuln_parser_asan vuln_parser.c

# MSan build (uninitialized reads)
clang -g -fsanitize=memory -fno-omit-frame-pointer \
    -DSTANDALONE -o vuln_parser_msan vuln_parser.c 2>/dev/null || \
    echo "MSan build skipped (may need MSan-instrumented libc)"

# Coverage build (for corpus minimization)
clang -g -fprofile-instr-generate -fcoverage-mapping \
    -DSTANDALONE -o vuln_parser_cov vuln_parser.c

# Production-like build (no sanitizers, for crash triage)
gcc -g -O2 -DSTANDALONE -o vuln_parser_prod vuln_parser.c

echo "=== Build complete ==="
echo "ASan binary: vuln_parser_asan"
echo "Coverage binary: vuln_parser_cov"
echo "Production binary: vuln_parser_prod"
