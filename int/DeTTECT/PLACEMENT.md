# DeTTECT Bulk Placement Notes

## Deferred Items (Bulk, Follow-Up Copy)

The following DeTTECT components are large or asset-heavy and should be copied on follow-up:

| Item | Path | Reason | Size | Status |
|------|------|--------|------|--------|
| **MITRE ATT&CK data** | `mitre-data/` | Pre-computed STIX/JSON technique mappings; used by group_mapping.py and technique_mapping.py | ~2-5 MB | copy on follow-up |
| **Sample data** | `sample-data/` | Example detection/technique mappings for testing; | ~500 KB | copy on follow-up |
| **Editor UI** | `editor/` (Vue SPA) | React/Vue single-page app for visual threat layer editing; frontend only | ~3+ MB | UI scaffold, preserved in-repo |
| **Requirements.txt** | `requirements.txt` | Python dependencies for DeTTECT tooling | ~1 KB | keep reference in ops/ until rebuild |
| **Dockerfile** | `Dockerfile` | Container image for DeTTECT deployment | ~1 KB | unknown usefulness |

## Recommended Follow-Up Dissolution

After initial placement, copy these items into:
- `mitre-data/` → `/home/user/ARSON-AL/int/mitre` (reference data for actor->technique mapping)
- `sample-data/` → `/home/user/ARSON-AL/batteries


