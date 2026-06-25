#!/usr/bin/env bash
# startup.sh — Pull repos into staging/, install compilers, push
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
REMOTE_ORG="${ARSENAL_REMOTE_ORG:-gravermistakes}"
REMOTE_REPO="${ARSENAL_REMOTE_REPO:-arsenal}"
REMOTE_URL="git@github.com:${REMOTE_ORG}/${REMOTE_REPO}.git"

log()  { printf '\033[1;36m[arsenal]\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m[warn]\033[0m %s\n'    "$*"; }
ok()   { printf '\033[1;32m  ✓\033[0m %s\n'        "$*"; }

clone() {
  local url="$1" dir="$ROOT/staging/$2"
  [ -d "$dir/.git" ] && { ok "$2"; return; }
  mkdir -p "$(dirname "$dir")"
  git clone --depth 1 "$url" "$dir" 2>/dev/null && ok "$2" || warn "failed: $url"
}

# ── Structure ──────────────────────────────────────────────────
mkdir -p "$ROOT"/{ops,int,probes,picks,proofs,staging}

# ── Pull everything into staging ───────────────────────────────
log "Cloning into staging/..."
clone "https://github.com/shaniidev/bug-reaper.git"                      bug-reaper
clone "https://github.com/shaniidev/lance.git"                           lance
clone "https://github.com/0xazanul/fuzz-skill.git"                       fuzz-skill
clone "https://github.com/owasp-noir/noir.git"                           noir
clone "https://github.com/j3ssie/vigolium.git"                           vigolium
clone "https://github.com/filipi86/drogonsec.git"                        drogonsec
clone "https://github.com/vladko312/Research_Successful_Errors.git"      ssti-research
clone "https://github.com/infosecn1nja/awesome-mitre-attack.git"         mitre-attack
clone "https://github.com/vavkamil/awesome-bugbounty-tools.git"          bugbounty-tools
clone "https://github.com/cipher387/osint_stuff_tool_collection.git"     osint-tools
clone "https://github.com/TBlauwe/OPACK.git"                             opack
clone "https://github.com/maaamamia/shodansnipe_crew-v2.git"             shodansnipe
clone "https://github.com/nativ3ai/H1DR4.git"                            h1dr4
clone "https://github.com/OWASP/threat-dragon.git"                       threat-dragon
clone "https://github.com/OWASP/www-project-agentic-skills-top-10.git"  ast10
clone "https://github.com/OWASP/www-project-blockchain-appsec-standard.git" blockchain-appsec
clone "https://github.com/netsquare/Burp-Suite-Extension-Development.git" burp-extensions
clone "https://github.com/DietrichGebert/ponytail.git"                    ponytail

# ── Compilers ──────────────────────────────────────────────────
log "Compilers..."
MISSING=()
if command -v go &>/dev/null; then ok "go"
else
  ARCH=$([ "$(uname -m)" = "aarch64" ] && echo arm64 || echo amd64)
  curl -fsSL "https://go.dev/dl/go1.23.4.linux-${ARCH}.tar.gz" \
    | tar -C /usr/local -xz 2>/dev/null \
    && export PATH="/usr/local/go/bin:$PATH" && ok "go" \
    || { warn "go failed"; MISSING+=(go); }
fi
command -v crystal &>/dev/null && ok "crystal" \
  || { curl -fsSL https://crystal-lang.org/install.sh | bash 2>/dev/null \
       && ok "crystal" || { warn "crystal failed"; MISSING+=(crystal); }; }
command -v cmake &>/dev/null && command -v g++ &>/dev/null && ok "cmake+g++" \
  || { apt-get update -qq && apt-get install -y -qq build-essential cmake >/dev/null 2>&1 \
       && ok "cmake+g++" || { warn "c++ failed"; MISSING+=(cmake); }; }
command -v node &>/dev/null && ok "node" || { warn "no node"; MISSING+=(node); }

# ── Git ────────────────────────────────────────────────────────
[ -f "$ROOT/.gitignore" ] || printf 'staging/\n*/build/\n*/node_modules/\n**/__pycache__/\n*.o\n.env\n*.key\n*.pem\n' > "$ROOT/.gitignore"
if [ ! -d "$ROOT/.git" ]; then
  cd "$ROOT" && git init -b main && git add -A
  git commit -m "arsenal: bootstrap $(date +%s)"
fi
cd "$ROOT"
git remote get-url origin &>/dev/null && git remote set-url origin "$REMOTE_URL" \
  || git remote add origin "$REMOTE_URL"
git push -u origin main 2>/dev/null && ok "pushed" \
  || warn "create repo first: gh repo create ${REMOTE_ORG}/${REMOTE_REPO} --private"

# ── Done ───────────────────────────────────────────────────────
log "$(ls staging/ | wc -l) repos in staging/ — dissolve per claude.md"
[ ${#MISSING[@]} -gt 0 ] && warn "missing: ${MISSING[*]}"
