<!-- generated: session memory; epoch appended at commit -->
# Patterns of practice — session memory

Reusable patterns learned building the arsenal (dissolution + dedup + the ops
reorg + the Opaca spec). Memorized here because the repo is the durable store;
an ephemeral memory CLI is not. Read before the next dissolution/dedup pass.

## Dissolution

- **Sort by verb, not by name or origin repo.** Only what a component
  *mechanically does* decides its bucket (ops/int/probes/picks/proofs).
- **Split two-job units.** A WAF doc with bypass (pick) + detection signatures
  (probe) becomes two files.
- **Hold-then-place, never skip.** Odd-fit / "not a security tool" material is
  *held until the end, then placed* — it is not dropped. The rule is written in
  the root CLAUDE.md; "skip non-security repos" was an agent-invented heuristic
  and was wrong. Every repo was added for a reason.
- **Defensive→offensive reframe.** A defensive tool earns an offensive home:
  DeTTECT detection-coverage → an *evasion-surface* map (probes/); ATT&CK
  group→technique → an attack playbook (int/).
- **Log every placement** in a MANIFEST; cross-reference splits rather than
  physically duplicating.

## Deduplication

- **Locate by content, not by name.** Name scans find same-*named* dirs, not
  duplicate *bytes*. Hash: `find … | xargs sha256sum | sort | uniq -w64 -D`.
  Reach for a dedicated tool (`jdupes`/`fdupes`) first (ladder rung 5); stdlib
  hash is the rung-3 fallback. Don't hand-roll a `join` pipeline — `uniq -w64 -D`
  groups duplicate-content lines directly.
- **Two kinds:** *literal* (identical bytes — prune to one copy) and
  *homomorph* (functionally equivalent, different bytes — not hashable; a
  recombination target, pick one canonical).
- **Trivial dups dominate by count, components by bytes.** Most duplicate *files*
  are repeated `.claude` command packs / READMEs / configs that ride inside every
  vendored monorepo; most duplicate *bytes* are whole re-vendored components
  (e.g. a QuDAG bundle).
- **Intra-tree before cross-tree.** Dedup *within* each component first (no
  cross-coupling); then handle cross-repo dups.
- **Surface copies are canon.** For a component vendored at multiple depths, the
  shallowest (top-level) copy is canonical; deeper nested copies become
  `*.POINTER.md` stubs naming the canonical path. Content survives in the kept
  path; everything is git-reversible.

## ops/ structure (four buckets)

- **actors/** — the engine + agent/swarm runtimes (opack/Opaca, ruv-fann,
  synaptic-mesh, neural-bridge, loki-mode, advanced_evolution, guild-hall).
- **mem/** — memory / state / scoring (loki-mode memory; advanced_evolution
  learning loop pointer).
- **std/** — standards of practice (ponytail ladder, research + dissolution
  methodology, this file).
- **hyg/** — hygiene / compliance / coverage (ast10, loki quality-gates, DeTTECT).
- Placement calls that stuck: *gamification + substrate → actors; optimizer →
  actors (canonical) + mem (pointer); C2 → picks* (ways in, not engine meta).

## Working method

- **Act on git-reversible moves; report; let the human correct.** `git mv` /
  content-dedup are cheap to undo — prefer doing + reporting over a wall of
  questions. Confirm only genuinely-ambiguous *structural* forks.
- **Ponytail is active.** Laziest solution that works: reuse, stdlib, the right
  tool over hand-rolling. Read fully before climbing; fix root cause.
- **No upselling.** State what's done; don't pitch a menu of next steps.
- **Reorg breaks path-coupled functional files** (the SessionStart hook pointed
  at `ops/standards/ponytail` → broke when it moved to `ops/std`). After any
  move, grep authored files for stale paths and fix the functional ones.

## Identity

- Signature: **Cassius von Opus** (Co-Authored-By trailer). "Claude"/"Opus" are
  the brand/substrate; the working name is Cassius von Opus.
