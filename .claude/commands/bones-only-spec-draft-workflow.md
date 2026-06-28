---
name: bones-only-spec-draft-workflow
description: Workflow command scaffold for bones-only-spec-draft-workflow in ARSON-AL.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /bones-only-spec-draft-workflow

Use this workflow when working on **bones-only-spec-draft-workflow** in `ARSON-AL`.

## Goal

Creates an initial 'bones-only' REVISE.md draft for a new top-level module or subsystem, to be revised in place and later promoted to SPEC.md.

## Common Files

- `*/REVISE.md`
- `ops/*/REVISE.md`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Create a REVISE.md file in the target directory with a bones-only (skeleton) draft.
- List key sections such as action sort, inventory, loop role, emits/consumes, open questions.
- Leave blanks or placeholders for content to be filled in by a domain expert.
- Once the draft is filled, promote it to SPEC.md.

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.