<!-- generated: 1782416720 -->
# PROOFS — SYSADLOG

Sysadmin/dissolution error log for proofs/. Each entry: what broke, how it was
found, the fix. Newest first. Linked from CLAUDE.md.

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
