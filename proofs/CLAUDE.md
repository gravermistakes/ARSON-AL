<!-- generated: 1782416571 -->
# PROOFS

## What's here
Exploit validation, economic models, triage simulation, CVSS scoring, and
platform-specific report generation. The validation loop — nothing exits
without a PoC.
- `web3/lance/` — the lance 7-gate spine (`SKILL.md`, `workflow.md`) plus its
  validation gates G3-G6: exploit/economic/false-positive/triage refs and the
  Web3 severity guide; `platforms/` (Immunefi, HackenProof, HackerOne, Bugcrowd
  triage rules); `templates/` (per-platform report templates + finding schema);
  `scripts/` (scoring_engine, triage_simulator, generate_web3_report,
  invariant_output_adapter); `agents/` (per-agent skill adapters).

## Build
Python scripts + markdown, no build. `python3 web3/lance/scripts/scoring_engine.py --help`.

## Test
`python3 web3/lance/scripts/triage_simulator.py` on a sample finding; confirm an
Accepted/Needs-Evidence/Rejected verdict. No upstream test suite shipped.

## Feeds
- **Loop:** Validation — gate candidates from probes/ for exploitability +
  economics; only Medium+ that pass triage simulation become reports.
- **Consumes:** candidate findings from probes/web3, scope from int/web3.
- **Emits:** platform-ready reports (Immunefi/Bugcrowd/HackenProof/HackerOne).

## Issues
- The lance SKILL spine lives here but references G0/G1 scripts that dissolved
  to int/web3/lance/scripts/ — stale relative paths, cross-referenced in
  MANIFEST. Recombination onto OPACK reunifies the gates.
- Python (no-Python rule is for new code; inherited tools kept).
