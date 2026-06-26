#!/bin/bash
# Surface the ponytail coding standard at session start so the ladder is
# active for the whole session. SessionStart adds stdout to session context.
set -euo pipefail

STD="${CLAUDE_PROJECT_DIR:-.}/ops/standards/ponytail/skills/ponytail/SKILL.md"
[ -f "$STD" ] || exit 0   # standard not present in this tree — nothing to load

echo "The ponytail coding standard is ACTIVE for this session (default intensity: full). Follow the ladder; build the laziest solution that actually works."
echo

# Print the SKILL body, skipping the YAML frontmatter block if present.
awk 'NR==1 && $0=="---"{f=1; next} f && $0=="---"{f=0; next} !f' "$STD"
