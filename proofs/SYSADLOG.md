<!-- generated: 1782416720 -->
# PROOFS — SYSADLOG

Sysadmin/dissolution error log for proofs/. Each entry: what broke, how it was
found, the fix. Newest first. Linked from CLAUDE.md.

## 1782416720d — batch 5 review (mitre-attack, threat-dragon, ssti-research, osint-tools)  [CLEAN]

Review cadence after batch-5 pushes. No errors — merged.
- **verify:** threat-dragon sample JS `node --check` 20/20 clean (full npm
  build skipped — heavy monorepo); ssti/mitre/osint are docs/HTML.
- **security:** no real credentials. threat-dragon `example.env` holds
  obvious placeholder values (`asdfasdfasdf`, `0123...`) — upstream template.

## 1782416720c — batch 3 review (noir, ast10, bofhound)  [CLEAN]

Review cadence after batch-3 pushes. No errors — merged.
- **verify:** bofhound `py_compile` clean; ast10 `validate.sh` `bash -n` clean;
  noir structure sane (shard.yml + src/noir.cr entrypoint) — full Crystal build
  skipped (crystal not installed in env).
- **security:** no real credentials. `noir/src/passive_scan/false_positive.cr`
  flagged by the scanner but matches are detector *signatures in comments*
  (PEM markers), not secrets.
- **accepted observation:** `int/bofhound/tests/test_data/` ships ~8.9M of
  upstream AD log fixtures (ldapsearch/BOF/Brute-Ratel sample objects, may
  include sample hashes). Public upstream test corpus — kept per policy, NOT a
  leak. noir is ~75M (vendored tree-sitter grammars) — intentional.

## 1782416720b — batch 2 review (drogonsec, fuzz-skill, h1dr4)  [CLEAN]

Review cadence after batch-2 pushes. No errors — nothing to fix, merged.
- **verify:** `drogonsec` `go build ./...` → exit 0 (go 1.26, all deps resolve);
  `gofmt` clean; `h1dr4` index.js + mcp-smoke.mjs `node --check` clean; fuzz
  harnesses are legit ELF (not stripped, debug_info).
- **security:** no hardcoded credentials in the new Go/JS/MD/JSON; `internal/leaks`
  hits are detector *signatures*, not secrets; prebuilt fuzz binaries inspected
  (`file`) — ordinary x86-64 PIE executables.
- **code-review:** placements coherent; `go.mod/go.sum` already complete (build
  left the tree clean). Kept docs/corpus/fuzzers per policy.

## 1782416720 — spine references broken by verb-split  [FIXED]

**Severity:** high (methodology non-navigable).
**Found by:** review-cadence VERIFY step — grepped the dissolved `SKILL.md`
spines for `references/` + `scripts/` paths after splitting lance/bug-reaper.

**Error:** Dissolving lance (7-gate) and bug-reaper (4-phase) by verb moved each
SKILL spine into `proofs/` while its referenced files scattered to `int/`,
`probes/`, and `picks/`. Result: ~30 inline links per spine resolved to nothing.
Self-inflicted aggravation — proofs-local refs were placed under `refs/` but the
spine still said `references/`, so even same-dir links missed.

**Fix:** Prepended a "Dissolution reference map" block to each spine
(`proofs/web3/lance/SKILL.md`, `proofs/web2/bug-reaper/SKILL.md`) mapping the
original `references/`/`scripts/` prefixes to their true ARSENAL locations. One
block per spine — no per-line relative-path coupling between category dirs
(which would fight the dissolution model). Cross-links also recorded in the
int/probes/picks/proofs MANIFEST.yml `cross_refs`.

**Residual:** inline paths themselves are still written in upstream form; the map
is the index. Full reunification happens when the gates recombine onto the OPACK
ECS substrate (see CLAUDE.md "OPACK Migration").

**Verified clean alongside:** all dissolved Python scripts `py_compile` OK; no
`eval`/`exec`/`os.system`/`subprocess` sinks; no hardcoded credentials anywhere
in the dissolved tree; opack CMake free of dangling dropped-dir refs.
