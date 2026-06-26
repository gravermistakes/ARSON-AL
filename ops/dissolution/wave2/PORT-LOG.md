# Wave-2 full repo port

The bulky monorepos that wave-2 placed as `PLACEMENT.md` pointers are now
**fully ported** — complete repo trees vendored into their primary ops/ homes.

Excluded everywhere (regenerable / VCS metadata, per the arsenal's wave-1
"build artifacts only" drop convention — see vigolium): `.git`, `node_modules`,
`target`, `dist`, `.next`, `build`, `coverage`, `.turbo`.

| Source repo | Canonical home (full tree) | Size |
|-------------|----------------------------|------|
| ruv-FANN | `ops/substrate/ruv-fann/` | 71M |
| ruv-fann-neural-bridge | `ops/substrate/neural-bridge/` | 416K |
| Synaptic-Mesh | `ops/substrate/synaptic-mesh/` | 248M |
| advanced_evolution | `ops/optimizer/advanced_evolution/` | 1.1M |
| loki-mode | `ops/orchestration/loki-mode/` | 45M |

## Cross-reference dirs (not duplicates)

The verb-split secondary placements from the wave-2 manifests are
cross-reference pointers into the canonical trees above, not separate copies:

- `ops/orchestration/ruv-fann/` → `ops/substrate/ruv-fann/ruv-swarm/`
- `ops/optimizer/ruv-fann/` → `ops/substrate/ruv-fann/` (ml/training crates)
- `ops/orchestration/synaptic-mesh/` → `ops/substrate/synaptic-mesh/` (daa-swarm)
- `ops/standards/loki-mode/` → `ops/orchestration/loki-mode/` (quality-gates)
- `ops/substrate/loki-mode-memory/` → `ops/orchestration/loki-mode/memory/`

## Notes

- Synaptic-Mesh vendors two QuDAG copies (`src/rs/QuDAG/` and
  `src/rs/daa/.../qudag/`) plus prebuilt test binaries — kept as-is per the
  full-port directive and the project's keep-vendored-trees convention.
- Source repos at `/home/user/<repo>` remain untouched.
