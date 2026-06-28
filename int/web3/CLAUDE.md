<!-- generated: 1782639068 -->
# INT/WEB3
## What's here
On-chain forensics, target dossier mapping, bounty platforms, and threat intelligence directories for Web3 targets.
- `on-chain-forensics.md` — blockchain explorers, EVM decompilers (Dedaub, Panoramix), transaction debuggers (Phalcon, samczsun, Tenderly), and rug checkers.
- `bounty-platforms.md` — major bug bounty platforms (Immunefi, Code4rena, Sherlock, Hats Finance).
- `threat-intel-feeds.md` — incident analysis postmortems and security newsletters (REKT, BlockSec, web3sec.news).

## Build
No compilation required. These resources are read by reconnaissance and intake agents during scoping and target intelligence gathering.

## Test
Verify decompiler and transaction tool links regularly. Trace test transactions on Phalcon BlockSec or samczsun's viewer to verify layout parsing.

## Feeds
- **Loop:** Recon Loop — threat intelligence highlights fresh hacks, which feeds the target modeling process. Foreground target data are parsed via decompilers and visualization tools to build the target profile.
- **Consumes:** target blockchain addresses, unverified bytecode, and exploit metadata.
- **Emits:** targets, ABIs, and transaction flows -> probes/web3 for deep SAST/vulnerability scanning.

## Issues
- Unverified contracts are hard to decompile perfectly; Dedaub and Panoramix outputs often contain missing types and dummy labels.
