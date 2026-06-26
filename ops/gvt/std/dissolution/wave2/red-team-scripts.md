# Dissolution Manifest: red-team-scripts

**Date:** 2026-06-26
**Repo:** /home/user/red-team-scripts
**Dissolution Wave:** 2

## Units Table

| Unit Path | Action | Category | Destination |
|-----------|------|----------|-------------|
| smuggler.py | Generate payloads (HTTP request smuggling attack templates) | picks | picks/red-team-scripts/payloads/ |
| gen-chm.py | Generate payloads (CHM/help file attack vector payloads) | picks | picks/red-team-scripts/payloads/ |
| Generate-Mustang-Panda-LNK.ps1 | Generate payloads (LNK shortcut attack; APT group TTP emulation) | picks | picks/red-team-scripts/payloads/ |
| ASR Rules Bypass.vba | Bypass evasion (Attack Surface Reduction bypass via VBA macro) | picks | picks/red-team-scripts/evasion/ |
| BYOVD_kill_av_edr.c | Bypass evasion (Bring Your Own Vulnerable Driver to kill AV/EDR processes) | picks | picks/red-team-scripts/evasion/ |
| plugx.profile | C2 traffic profile (Sliver C2 beacon configuration/evasion profile) | picks | picks/red-team-scripts/c2/ |
| Lockbit_Ransomware_Atomic_Simulation.ps1 | Evasion + persistence playbook (LockBit ransomware TTP chain) | picks | picks/red-team-scripts/evasion/ |
| setup_havoc_teamserver.yml | Orchestration (Havoc C2 infrastructure provisioning) | ops | ops/c2-infrastructure/ |
| sliver.md (sections: server-install, systemd) | Orchestration (C2 server setup and lifecycle management) | ops | ops/c2-infrastructure/sliver-c2-ops.md |
| sliver.md (sections: recon, lateral, privesc, persistence, evasion) | Evasion + post-exploitation playbooks (post-compromised-host techniques) | picks | picks/red-team-scripts/ (cross-ref in manifest note) |
| Invoke-AtomicEnterpriseLayer.ps1 (ATT&CK group→technique mapping section) | Model threat (fetch MITRE ATT&CK group layer, extract techniques) | int | int/red-team-scripts/attack-mapping/atomic-enterprise-mapper.ps1 |
| Invoke-AtomicEnterpriseLayer.ps1 (Atomic execution loop section) | Execute attack techniques (invoke Atomic tests by technique ID) | picks | picks/red-team-scripts/ (note: execution coordination) |

## Drop List

- LICENSE (boilerplate, not functional)
- README.md (project intro, not functional arsenal content)
- .git/ (repository metadata, excluded by rule)

## Rationale

**red-team-scripts** is a collection of **attack payloads, evasion techniques, and C2 infrastructure templates**. Dissolution sorted by action:

- **Generate/Craft payloads** (smuggler, gen-chm, LNK) → **picks/** (attack payload toolkit)
- **Bypass AV/EDR/ASR** (VBA bypass, BYOVD, profiles) → **picks/evasion/** (detection evasion methods)
- **C2 infrastructure** (Havoc setup, Sliver server config) → **ops/c2-infrastructure/** (red-team substrate)
- **Post-exploitation choreography** (Sliver lateral/privesc/persistence sections) → **picks/** (attack playbooks)
- **Threat modeling** (Atomic Enterprise Layer mapper) → **int/** (ATT&CK technique→group correlation)

The sliver.md file is a **split unit**: server-provisioning sections go to ops/ (infrastructure), post-exploitation sections go to picks/ (evasion/persistence playbooks). Invoke-AtomicEnterpriseLayer.ps1 is also **split**: the mapping logic (fetch + parse MITRE layer) goes to int/ (threat modeling), the execution loop goes to picks/ (orchestrated technique invocation).

**Key principle applied:** The repo's origin/name is irrelevant. Only the **action** (what it mechanically does) matters. A tool named "red team scripts" is sorted into arsenal categories by its functional behavior, not its package name.

---

**Status:** COMPLETE. All 11 functional units placed. No bulk deferred items.
