<!-- generated: 1751155000 -->
# CHAINS

## What's here
Exploit chain state — the chain loop's memory store. Collects low-severity
findings that individually fail validation, braids them into composite
attack paths that reach Critical/High impact. Each chain is a directed
graph of findings with cumulative CVSS and a combined PoC.

## Build
No build. Chain state is structured data consumed by the actor loop
(ops/actors/) and validated by proofs/.

## Test
Verify a chain file parses: each entry must have `findings[]` with
CWE + severity, `chain_cvss`, `exploit_path[]`, and `status`
(brewing | viable | reported | dead).

## Feeds
- **Loop:** Chain — background. Receives rejected/low-sev findings from
  validation loop. Braids into composite chains. Viable chains feed back
  to proofs/ for full PoC + report generation.
- **Consumes:** low-severity findings from probes/, failed validations
  from proofs/, scope context from int/.
- **Emits:** composite exploit chains (finding graph + cumulative impact)
  back to proofs/ when chain reaches Medium+ combined severity.

## Issues
- No chain files yet — structure TBD as actors begin producing findings.
- Chain loop logic lives in the actor's decide phase (kit_switch rules
  detect braid opportunities via crosslinks).
