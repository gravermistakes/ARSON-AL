<!-- generated: 1782416720 -->
# PICKS

## What's here
Ways in — injection techniques, bypass payloads, exploitation playbooks. What
turns a detected surface into a working exploit.
- `web2/bug-reaper/` — 19 Web2 exploitation-class playbooks (XSS, SQLi, NoSQLi,
  SSRF, IDOR, auth-bypass, RCE, SSTI, LFI, XXE, CORS, CSRF, prototype pollution,
  subdomain takeover, HTTP smuggling, open redirect, API/GraphQL, biz-logic) and
  `waf-bypass.md` (WAF evasion technique).

## Build
Reference playbooks — nothing compiles. Read by the hunting agent to build an
attack from a probe hit.

## Test
Verify a playbook against a known-vulnerable lab (DVWA/Juice Shop) target.

## Feeds
- **Loop:** Scan/Chain — a probe hit picks the matching technique here; a working
  exploit feeds proofs/ for validation and chaining.
- **Consumes:** candidate surface + class from probes/web2.
- **Emits:** working exploit paths / PoCs → proofs/web2 to gate and report.

## Issues
- Knowledge only — no executable fuzzers/payload-runners yet. fuzz-skill,
  drogonsec, and ssti-research dissolve here next and add teeth.
- `subdomain-takeover.md` straddles detect/exploit; kept with its class peers.
