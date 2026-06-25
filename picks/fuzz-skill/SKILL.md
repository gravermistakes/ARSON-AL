---
name: fuzzing-for-memory-bugs
description: "Use when auditing authorized C or C++ code for memory-safety bugs with fuzzing, sanitizer builds, harness design, corpus planning, crash reproduction, crash minimization, deduplication, exploitability assessment, or confirmed vulnerability reporting. Applies to parsers, codecs, deserializers, file and network protocol handlers, and suspected buffer overflow, use-after-free, double-free, integer overflow, format string, or null-dereference findings."
---

# Fuzzing for Memory Bugs

## Proof Standard

Find real memory-safety bugs with harnessed, sanitizer-backed fuzzing.

- Treat static review as a way to choose harnesses, seed inputs, dictionaries, and root causes. Do not treat static review alone as proof of a vulnerability.
- Confirm a finding only when there is a reproducible sanitizer report or equivalent deterministic crash, a minimized input, a reachable target code path, and a clear affected source location.
- Report "no confirmed findings" only with coverage, runtime, harness scope, and tool limitations. Never claim "no bugs" from a smoke run or a narrow harness.
- If the user asks only for strategy or commands, provide commands and rationale without launching long fuzzing jobs.

## Operating Loop

1. Scope the target: identify the exact repository or files, authorization boundary, entry point, build system, platform, time budget, and acceptable artifact directory.
2. Inventory before editing: use `rg --files`, `rg`, build files, existing tests, existing fuzz harnesses, sample inputs, and `command -v` for relevant fuzzing tools.
3. Rank targets by attacker reachability, parser complexity, unsafe operations, historical bug density, and ease of harnessing.
4. Select the workflow variation from the table below.
5. Build the minimum useful instrumented targets.
6. Prepare or minimize the seed corpus.
7. Run bounded fuzzing based on the user's time budget.
8. Triage every crash before reporting.
9. After any confirmed root cause, search for sibling entry points or variant bugs.
10. Report confirmed findings, unconfirmed leads, and explicit limitations separately.

## Workflow Variations

| Situation | First action | Output |
| --- | --- | --- |
| Existing harness and seeds | Build with ASan+UBSan, run a smoke pass, then expand runtime or fuzzers as budget allows | Confirmed setup, crashes, or limitations |
| No harness | Read the target API and write the smallest deterministic harness for the parser or handler | Harness diff plus build and run commands |
| Existing crash corpus | Reproduce, deduplicate by sanitizer class and stack, then minimize | Unique crash list and minimized reproducers |
| Suspected code-review bug | Write a targeted harness and input to prove reachability before claiming impact | Confirmed finding or unconfirmed lead |
| Limited local tooling | Use the available sanitizer/compiler path, state missing tools, and avoid inventing ensemble results | Best-effort result with caveats |
| User wants a report only | Do not mutate the repo. Reproduce from existing artifacts and format evidence | Vulnerability report or evidence gap list |
| Large codebase | Build a target map first, then fuzz the top 1-3 highest-risk reachable parsers | Prioritized campaign plan and first-pass evidence |

## Campaign Design

- Create a target map before writing code in large projects: entry point, input format, trust boundary, build target, seed source, expected risky operations, and harness cost.
- Prefer many narrow harnesses over one application-sized harness. Split by format, protocol message type, decompression stage, or parser mode when state machines hide deeper paths.
- Use a smoke pass to prove build, determinism, corpus loading, and crash artifact paths. Use standard or thorough runs only after the smoke pass produces stable coverage.
- Escalate only when evidence says to: add dictionaries for compare-heavy parsers, custom mutators for structured formats, MSan for suspicious uninitialized-state behavior, and AFL++ CmpLog when magic values or length checks block progress.
- Stop or pivot when coverage is flat, crash classes are all harness-owned, seeds are not reaching target code, or the fuzzer is spending time in setup code instead of parser code.

## Reference Loading

- Read `fuzzing-toolchain.md` when selecting build flags, harness patterns, fuzzer commands, corpus handling, coverage commands, or custom mutator approaches.
- Read `crash-triage.md` when a crash, timeout, sanitizer report, minimized input, severity decision, false-positive check, or final report is needed.
- Do not load large generated crash directories into context. List them, sample filenames, and inspect only representative reproducers or reports needed for the current decision.

## Tool-Calling Rules

- Adapt every command to the repository's build system and paths. Do not paste template commands with unresolved `target.c`, `corpus/`, or placeholder binary names.
- Prefer bounded commands in an interactive agent session: smoke fuzzing, reproduction, minimization, coverage, and targeted triage. Ask before starting multi-hour fuzzing, dependency installation, or high-CPU jobs.
- Put new fuzzing artifacts under an isolated directory such as `fuzz-work/<target>/` unless the user specifies another path. Do not delete existing corpora, crashes, or findings unless explicitly asked.
- Capture the exact compiler command, sanitizer options, fuzzer command, crash input path, and minimized reproducer path for every confirmed result.
- If a tool is missing, say which tool is missing and choose the next viable path. Do not imply AFL++, Honggfuzz, MSan, coverage, or production reproduction was used unless it actually ran.
- Use the agent's normal patch or edit mechanism for harnesses, build files, dictionaries, and report files. Use shell commands for building, running, minimizing, and inspecting results.
- Keep source edits scoped to harnesses, build glue, or requested fixes. Do not refactor application code while trying to prove a bug.
- Maintain an evidence ledger in the response or report: target, harness, sanitizer, fuzzer, corpus source, runtime, coverage signal, crash id, minimization status, and confidence label.

## Build Selection

- Use ASan+UBSan as the default first build for C/C++ memory corruption and undefined behavior.
- Add a coverage build when deciding whether the harness reaches the intended parser, decoder, or handler.
- Add MSan only when uninitialized reads are in scope and dependencies can be built with MSan-compatible instrumentation.
- Add a no-sanitizer or production-like build only for impact evidence. A sanitizer-only crash can still be a real bug, but it should be labeled accordingly.
- Use the project's native build system first. Fall back to direct `clang` commands only for small standalone targets or proof-of-concept harnesses.

## Fuzzer Selection

- Prefer libFuzzer when an in-process API harness is practical and the target can be linked into one binary.
- Prefer AFL++ when the target is naturally file-oriented, persistent mode is available, compare-heavy logic benefits from CmpLog, or multiple mutation schedules are useful.
- Prefer Honggfuzz when it is already installed or the target benefits from its persistent mode and hardware/software feedback options.
- For standard or thorough audits, use more than one fuzzer family when available. If only one fuzzer is available, proceed and state the limitation instead of blocking.

## Harness Quality Gates

- Fuzz the smallest input-processing unit that accepts attacker-controlled bytes: parser, decoder, deserializer, message handler, or file loader.
- Keep the hot loop deterministic, fast, and side-effect bounded. Reset mutable global state between iterations.
- Return normally for invalid inputs. Do not call `exit`, `abort`, assertions, network services, wall-clock randomness, or uncontrolled filesystem writes from the fuzz path.
- Bound maximum input size when the target can allocate or recurse based on length.
- For file-only APIs, write inputs to a temporary file inside the artifact directory and remove or overwrite it per iteration.
- For structured formats, add a dictionary or structure-aware mutator after basic coverage stops improving.
- Add a standalone replay path for each harness when practical so minimized inputs can be reproduced without invoking the fuzzing engine.

## Corpus Rules

- Start with valid examples from tests, fixtures, specs, sample files, or captured payloads.
- Add edge seeds only when they match the format's real boundary conditions: empty body, maximum length field, missing terminator, nested object, duplicate field, invalid checksum, truncated header.
- Minimize corpus by coverage before long runs. Keep the smallest input that preserves each covered path.
- Track coverage against the intended target code. If risky functions are unreachable, improve the harness before drawing conclusions.

## Crash Triage Gates

Before calling a crash confirmed:

1. Reproduce it locally with the same instrumented binary.
2. Deduplicate by sanitizer class plus top target-owned stack frames, not by filename alone.
3. Minimize the input while preserving the same crash class and stack.
4. Confirm the stack reaches target code rather than harness cleanup or unrelated process teardown.
5. Identify the root-cause source line and attacker-controlled data relationship.
6. Assign severity from observed primitive, control, and reachability: write corruption usually outranks read corruption; controlled UAF/OOB outranks null dereference; sanitizer-only impact must be labeled.

After root cause:

- Search for sibling bugs using the same unsafe pattern, parser state transition, length calculation, ownership rule, or missing validation.
- Re-run the minimized reproducer after any harness or build change; do not carry confirmation across binaries without replay.
- Keep fix verification separate from discovery unless the user explicitly asks for remediation.

## Reporting Language

Use these labels consistently:

- `CONFIRMED`: Reproducible crash, minimized input, target-owned stack, source location, and reachability evidence exist.
- `UNCONFIRMED`: Suspicious code, non-minimized crash, unstable repro, missing symbols, missing reachability, or harness-only behavior.
- `NO CONFIRMED FINDINGS`: Fuzzing produced no confirmed issue within stated coverage, runtime, and harness limits.

A confirmed report must include the minimized reproducer, sanitizer output or crash signal, exact build/run commands, affected source location, root cause, reachability, severity, and a concrete fix direction.
