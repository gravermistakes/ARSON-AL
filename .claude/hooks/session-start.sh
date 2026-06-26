#!/bin/bash
# Activate the ponytail coding standard at session start. Emits a short pointer,
# not the full SKILL — the /ponytail skill loads the whole ladder on demand, so
# this keeps the per-session context cost to a few lines.
set -euo pipefail

STD="${CLAUDE_PROJECT_DIR:-.}/ops/standards/ponytail/skills/ponytail/SKILL.md"
[ -f "$STD" ] || exit 0   # standard not present in this tree — nothing to activate

echo "Ponytail coding standard is ACTIVE for this session (default intensity: full). Build the laziest solution that actually works — climb only to the first rung that holds: YAGNI -> reuse what's here -> stdlib -> native feature -> installed dep -> one line -> minimum code. Read the problem fully before climbing; a bug fix targets the root cause, not the symptom. Full standard: ${STD#"${CLAUDE_PROJECT_DIR:-.}/"} (or run /ponytail)."
