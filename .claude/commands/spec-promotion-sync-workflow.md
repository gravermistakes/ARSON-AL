---
name: spec-promotion-sync-workflow
description: Workflow command scaffold for spec-promotion-sync-workflow in ARSON-AL.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /spec-promotion-sync-workflow

Use this workflow when working on **spec-promotion-sync-workflow** in `ARSON-AL`.

## Goal

Promotes a revised REVISE.md draft to canonical SPEC.md, syncing and updating sections as needed.

## Common Files

- `*/SPEC.md`
- `*/REVISE.md`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Copy or merge content from REVISE.md to SPEC.md in the same directory.
- Add new sections, refine existing ones, and clarify deferred topics.
- Leave REVISE.md as the ongoing working draft.

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.