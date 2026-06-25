<!-- generated: 1782416720 -->
# PICKS

## What's here
Ways in — injection techniques, bypass payloads, exploitation playbooks. What
turns a detected surface into a working exploit.
- `web2/bug-reaper/` — 19 Web2 exploitation-class playbooks (XSS, SQLi, NoSQLi,
  SSRF, IDOR, auth-bypass, RCE, SSTI, LFI, XXE, CORS, CSRF, prototype pollution,
  subdomain takeover, HTTP smuggling, open redirect, API/GraphQL, biz-logic) and
  `waf-bypass.md` (WAF evasion technique).
- `fuzz-skill/` — C/C++ memory-bug fuzzing: methodology (SKILL, fuzzing-toolchain,
  crash-triage) + `test-vuln/` runnable reference (5 sanitizer-built fuzz
  harnesses, `seeds/` corpus, `fuzz_output/` crashes). `cd fuzz-skill/test-vuln
  && ./build.sh` to rebuild the harnesses.
- `ssti-research/` — "Successful Errors" SSTI payloads + research (paper,
  slides, images): server-side template injection techniques.
- `burp-extensions/` — Burp Suite extension examples (Jython): runtime
  encryption in Intruder/Scanner, editor-tab crypt. Custom attack extensions.

## Build
- Web2 playbooks: nothing compiles — read by the hunting agent to build an
  attack from a probe hit.
- `fuzz-skill/test-vuln/`: `./build.sh` (needs clang + sanitizers/AFL++) rebuilds
  the fuzz harnesses; prebuilt binaries ship in-tree.

## Test
- Playbooks: verify against a known-vulnerable lab (DVWA/Juice Shop).
- Fuzzers: `./fuzz-skill/test-vuln/vuln_parser_libfuzzer -runs=10000 seeds/`.

## Feeds
- **Loop:** Scan/Chain — a probe hit picks the matching technique here; a working
  exploit feeds proofs/ for validation and chaining.
- **Consumes:** candidate surface + class from probes/web2.
- **Emits:** working exploit paths / PoCs → proofs/web2 to gate and report.

## Issues
- fuzz-skill gives picks/ its first runnable teeth (memory-bug fuzzing).
  ssti-research, drogonsec attack-extensions, burp-extensions dissolve here next.
- `subdomain-takeover.md` straddles detect/exploit; kept with its class peers.
- `test-vuln/` ships ~13M of prebuilt fuzz binaries (kept deliberately); rebuild
  from source with `build.sh` if you don't trust the prebuilts.
