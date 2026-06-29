# Ponytail

The laziest solution that actually works. A coding methodology that channels a
senior dev who has seen every over-engineered codebase and been paged at 3am
for one. The best code is the code never written.

MIT. By DietrichGebert.

## The Ladder

Stop at the first rung that holds:

1. **Does this need to exist at all?** Speculative need = skip it. (YAGNI)
2. **Already in this codebase?** Reuse it.
3. **Stdlib does it?** Use it.
4. **Native platform feature covers it?** CSS over JS, DB constraint over app code.
5. **Already-installed dependency solves it?** Use it. Never add a dep for what a few lines can do.
6. **Can it be one line?** One line.
7. **Only then:** the minimum code that works.

The ladder runs *after* you understand the problem, not instead of it.

## Intensity

| Level | What changes |
|-------|-------------|
| **full** | The ladder enforced. Stdlib and native first. Shortest diff, shortest explanation. Default. |
| **ultra** | YAGNI extremist. Deletion before addition. Challenge the requirement before building. |

## Skills

| Skill | Trigger | Purpose |
|-------|---------|---------|
| `ponytail` | `/ponytail [full\|ultra]` | Lazy mode. Simplest solution that works. |
| `ponytail-review` | `/ponytail-review` | Over-engineering review on a diff. One line per finding. |
| `ponytail-audit` | `/ponytail-audit` | Whole-repo audit for complexity. Ranked, biggest cut first. |
| `ponytail-debt` | `/ponytail-debt` | Harvest every `ponytail:` comment into a debt ledger. |
| `ponytail-gain` | `/ponytail-gain` | Measured impact scoreboard from benchmarks. |
| `ponytail-help` | `/ponytail-help` | Quick-reference card. |

## Review Tags

Findings use one tag per line:

- `delete:` dead code, unused flexibility. Replacement: nothing.
- `stdlib:` hand-rolled thing the standard library ships.
- `native:` dependency doing what the platform already does.
- `yagni:` abstraction with one implementation, config nobody sets.
- `shrink:` same logic, fewer lines.

End with: `net: -<N> lines possible.`

## Marking Shortcuts

Deliberate simplifications get a `ponytail:` comment naming the ceiling and
upgrade path:

```
// ponytail: global lock, per-account locks if throughput matters
```

`/ponytail-debt` collects these into a ledger so deferrals don't rot.

## Install

### Claude Code
```bash
cp -R skills/* ~/.claude/skills/
```

### Codex
```bash
cp hooks/claude-codex-hooks.json .codex/hooks.json
```

### Adapters
Cursor, Copilot, Cline, Windsurf adapters in `integration/adapters/`.

## Hooks

Runtime hooks in `hooks/` handle activation, mode tracking, statusline
display, subagent instruction injection, and config resolution. Priority
order: env var `PONYTAIL_DEFAULT_MODE` > config file > `full`.

## Benchmarks

5 everyday tasks, 3 models (Haiku, Sonnet, Opus):

| Metric | Ponytail vs baseline |
|--------|---------------------|
| Lines of code | 6-20% of baseline (80-94% reduction) |
| Cost | 23-53% of baseline (47-77% reduction) |
| Speed | 3-6x faster |

These are benchmark medians, not per-repo numbers.

## When NOT to be lazy

Never simplify away: input validation at trust boundaries, error handling
that prevents data loss, security measures, accessibility basics. User
insists on the full version = build it.

## Structure

```
skills/         SKILL.md for each ponytail skill
hooks/          runtime hooks (activation, config, statusline, subagent)
integration/    adapters (cursor, copilot, cline, windsurf), claude plugin, pi extension
examples/       12 worked examples (email validation, debounce, CSV sum, etc.)
```
