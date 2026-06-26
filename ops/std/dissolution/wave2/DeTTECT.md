# Dissolution Manifest: DeTTECT

**Date:** 2026-06-26
**Repo:** /home/user/DeTTECT
**Dissolution Wave:** 2

## Units Table

| Unit Path | Verb | Category | Destination |
|-----------|------|----------|-------------|
| group_mapping.py | Model threat (threat actor → MITRE ATT&CK technique mapping; offensive reframing of defensive layer) | int | int/DeTTECT/group_mapping.py |
| threat-actor-data/*.yaml | Threat intel corpus (actor group TTP profiles, sourced from MITRE ATT&CK/public intelligence) | int | int/DeTTECT/actor-corpus/threat-actor-data/ |
| technique_mapping.py | Detect/visibility mapping (which detection methods cover which ATT&CK techniques; **reframed: evasion surface map**) | probes | probes/DeTTECT/detection-coverage/technique_mapping.py |
| data_source_mapping.py | Detect/visibility mapping (data sources → techniques; coverage gaps = evasion opportunities) | probes | probes/DeTTECT/visibility/data_source_mapping.py |
| dettect.py | Orchestration + query engine (main DeTTECT CLI; orchestrates mapping/scoring/export) | ops | ops/DeTTECT/tooling/dettect.py |
| interactive_menu.py | Orchestration (interactive REPL for layer navigation, technique browsing) | ops | ops/DeTTECT/tooling/interactive_menu.py |
| generic.py | Substrate (STIX/JSON parsers, MITRE download, file I/O, generic utilities) | ops | ops/DeTTECT/tooling/generic.py |
| eql_yaml.py | Substrate (EQL query loader and YAML serializer; detection query codec) | ops | ops/DeTTECT/tooling/eql_yaml.py |
| constants.py | Substrate (MITRE technique/tactic constants, reference data) | ops | ops/DeTTECT/substrate/constants.py |
| health.py | Substrate (health check, data quality validation, layer consistency checks) | ops | ops/DeTTECT/substrate/health.py |
| upgrade.py | Orchestration (versioning, layer migration, upgrade workflows) | ops | ops/DeTTECT/substrate/upgrade.py |
| mitre-data/ | STIX/JSON technique mappings | int/DeTTECT/actor-corpus/mitre-data |
| sample-data/ | Detection layers for testing | int/DeTTECT/threat-mapping/sample-data |
| editor/ (Vue SPA) | Consumer UI for layer visualization | make sure we port parts as needed by dashboard |


## Rationale

**DeTTECT** is a **defensive tool reframed for offensive use**: it maps threat actors' behaviors (TTPs) against detection coverage. Dissolution applied a **defensive→offensive transformation**:

**Original Use:** "Which detections cover the MITRE ATT&CK techniques we care about?" (Blue team visibility map)

**Arsenal Use:** "Which techniques have weak detection coverage?" → Attack path planning. "Which actors use technique X?" → Threat mimicry. "What data sources detect this technique?" → Evasion surface analysis.

**Sorted by action:**
- **Threat model** (actor profiles, group mapping) → **int/** (offensive threat intel)
- **Detection coverage** (technique→detection, data source→technique) → **probes/** (reframed as "what detection gaps exist?" for evasion planning)
- **Query engine + CLI** (dettect.py, interactive_menu.py) → **ops/** (orchestration/tooling)
- **Constants + parsing** (generic.py, constants.py, eql_yaml.py) → **ops/substrate/** (codec/reference data)
- **Health + upgrade** → **ops/substrate/** (versioning/validation)

**Split notes:** technique_mapping.py and data_source_mapping.py remain in **probes/** under two subdirs to emphasize the evasion-surface-map framing: "detection-coverage" (what detections exist), "visibility" (what data sources are available). An attacker queries these to find blind spots.

---

**Status:** COMPLETE. 11 functional units placed. 3 bulk items deferred for follow-up (PLACEMENT.md link created).

## Retention
The Vue.js editor (`editor/`), previously listed as a drop, is **retained as
reference** at `ops/DeTTECT/editor/` (src + public + manifest; `node_modules`/
`dist` excluded). It is authoring tooling for the ATT&CK YAML data model — kept
for reference per "every repo was added for a reason; nothing dropped".

## Duplicates & homomorphs
Homomorph of int/mitre-attack (ATT&CK technique mapping). UNVERIFIED
Full dedup index: ../DUPLICATES.md
