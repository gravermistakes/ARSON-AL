#!/bin/bash
# Activate the ponytail coding standard at session start. Emits a short pointer
# + ladder summary, not the full SKILL — keeps per-session context cost to one
# line; Read the linked SKILL.md for the complete standard when needed.
set -euo pipefail

STD="${CLAUDE_PROJECT_DIR:-.}/ops/std/ponytail/skills/ponytail/SKILL.md"
[ -f "$STD" ] || exit 0   # standard not present in this tree — nothing to activate

echo "Ponytail coding standard is ACTIVE for this session (default intensity: full). Build the laziest solution that actually works — climb only to the first rung that holds: YAGNI -> reuse what's here -> stdlib -> native feature -> installed dep -> one line -> minimum code. Read the problem fully before climbing; a bug fix targets the root cause, not the symptom. Full standard (Read for the complete ladder, rules, and intensity levels): ${STD#"${CLAUDE_PROJECT_DIR:-.}/"}."
