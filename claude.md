# ARSENAL

Three files at root: `claude.md`, `startup.sh`, `README.md`.
Everything else sorted by what it *mechanically does*.

## Structure

```
ops/        orchestration, standards, substrate, build tooling
int/        target modeling, threat mapping, dossier building, scope parsing
probes/     enumeration, fingerprinting, endpoint extraction, surface discovery
picks/      fuzzers, bypass techniques, injection payloads — ways in
proofs/     PoC templates, exploit chains, triage validation, report generation
staging/    raw clones before dissolution (gitignored)
```

## What goes where

**ops/** — coding standards (ponytail ladder), compliance frameworks (AST10),
ECS substrate (OPACK), orchestration scripts, CI/CD tooling.

**int/** — threat diagrams, ATT&CK technique mappings, scope parsers,
investigation case files, social media profilers, target dossier builders,
company/person/infrastructure research tools.

**probes/** — subdomain enumerators, port scanners, endpoint extractors,
content discoverers, tech fingerprinters, SPA spiders, secret detectors,
SAST analyzers, SCA dependency checkers, DNS resolvers.

**picks/** — fuzzers, WAF bypass payloads, injection techniques (SSTI/SQLi/XSS/XXE),
credential testers, auth bypass methods, deserialization exploits,
request smuggling tools, prototype pollution, custom attack extension generators.

**proofs/** — PoC templates, exploit chain builders, triage simulators,
economic impact models, OAST blind confirmations, CVSS scorers,
platform-specific report generators (Immunefi/Bugcrowd/Intigriti/YWH).

A tool's *name* and *origin repo* are irrelevant to placement. Only the behavior matters.

## Dissolution

Every repo clones into `staging/`. Then gets cracked open.
Each component — a script, a methodology, a payload set, a scanner module,
a reference doc — gets one question: **ops, int, probe, pick, or proof?**

If a component does two things, split it. A WAF bypass doc with both
bypass techniques (pick) and detection signatures (probe) becomes two files.
If it does something new, hold that until the end.

After dissolution, `staging/` can be deleted.

### How to dissolve

1. Clone into `staging/{name}/`
2. Read everything end to end
3. For each functional unit, sort by verb
4. Place it with its peers from other repos
5. Log what went where in `{category}/MANIFEST.md`
6. When all components are placed, `rm -rf staging/{name}/`

## Compilers

| What for | Lang | Install |
|----------|------|---------|
| endpoint extraction | Crystal | `curl -fsSL https://crystal-lang.org/install.sh \| bash` |
| scanning, SAST | Go 1.25+ | `golang.org/dl` |
| ECS substrate | C++ | `apt install build-essential cmake` |
| investigation platform | Node 18+ | system |

## Loop Topology

Not a pipeline. Concurrent loops that branch, merge, and prune.

**Perpetual** — ops/ standards. Always enforced.

**Recon loop** — int/ models target → probes/ extract surface →
findings refine the model → probes/ extract more.

**Scan loop** — probes/ scan what's exposed → findings spawn new
targets → loop. A hit feeds adjacent probes a new path.

**Validation loop** — proofs/ gate for exploitability → ops/ check
 feeds back to recon or scan. Nothing exits without PoC.

**Chain loop** — background. Collects low-severity findings from
scope. Failed any loop, braids into critical chains. Success = report.

**Branching:** new surface → spawn probe. finding implies adjacent
surface → branch to recon. source found → spawn SAST probe.
chain succeeds → report.

**Pruning:** hypothesis dies. patched. 3 PoC approaches exhausted.

Loops run concurrently. probes/ doesn't finish before picks/ starts.

## Bounty Work

- **Immunefi** (Web3), **Bugcrowd** (Web2). (Other sources are valid)
- Start recon loop. Other loops join early.
- lance 7-gate for Web3. bug-reaper 4-phase for Web2.
- PoC's or it didn't happen.

## OPACK Migration

Dissolve shodansnipe the same way as everything else. Gamify thru OPACK.
Its agents and modules become cells. Recombine onto OPACK ECS. Score it.
Compiled C++ handles branching/merging/pruning/state.
LLM for triage, attack paths, report prose, problem solving.

## Deeper claude.md

When entering a subdirectory, if no `claude.md`, create one:

```
# {DIRECTORY}
## What's here — one paragraph
## Build — exact commands
## Test — verification
## Feeds — what loops, what consumed, what emitted
## Issues — what's broken
```

Under 50 lines. Tag `<!-- generated: {epoch} -->`.

## Rules

- No Python when practical. GPL preferred; MIT/BSD ok.
- Unix epoch timestamps.
- Findings: severity, CWE, CVSS, exploit path, PoC, remediation.
- SARIF for CI/CD. CycloneDX for SBOM.
- Think like the attacker. Find the seam. Prove the impact.
- Don't ask permission to act on completed reasoning.
