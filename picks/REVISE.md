# picks — revision draft

Bones. Revise in place; `___` = fill me.

## Action
fuzzers, crackers, extractors — the firing tools

## What's here (species)
- `fuzz-skill/` — C/C++ memory-bug fuzzing + 5 sanitizer harnesses
- `ssti-research/` — SSTI payloads + research
- `burp-extensions/` — Burp/Caido/ZAP intercept proxies + CrisisActor SPEC + 3 Jython scripts
- `web2/bug-reaper/` — 19 web2 exploitation playbooks
- `red-team-scripts/` — payloads + evasion (smuggler, gen-chm, LNK, BYOVD, Lockbit, ASR)
- `awesome-bugbounty-tools/` — 138 picks-class species (INTEGRATION-TABLE)
- `shodansnipe/` (split: tools went to probes; nothing here yet)
- ___

## Loop role
scan/chain — probe hit picks the matching technique; working exploit feeds proofs/

## Emits
- working exploit paths / PoCs
- ___

## Consumes
- candidate surface + class from probes/
- ___

## Open
- CrisisActor (Burp/Caido/ZAP) is the load-bearing pick — build order in burp-extensions/SPEC.md
- `picks/burp-extensions/` → rename `picks/intercept-proxies/` (flagged in SPEC)?
- ___
