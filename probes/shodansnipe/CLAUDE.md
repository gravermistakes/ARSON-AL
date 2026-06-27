<!-- generated -->
# probes/shodansnipe

## What's here
ShodanSnipe's enum/scrape tools — the working modules behind the recon agents.
Split here per the action sort (maps/enum/scrapers/crawlers = probes).

- `tools/shodan_query.py` — Shodan API queries
- `tools/subdomain_finder.py` — subdomain enumeration
- `tools/http_validate_tool.py` — liveness/fingerprint
- `tools/nmap_tool.py` — nmap wrapper
- `tools/archive_tool.py` — historical-archive scraping
- `tools/query_advisor.py`, `scope_advisor.py`, `doctrine.py` — query planning
  helpers (cross-ref with int/shodansnipe/core).

## Feeds
- Loop: Scan — invoked by int/shodansnipe agents to map attack surface; hits
  feed back to refine the recon dossier.
- Consumes: scope + queries from int/shodansnipe.
- Emits: surface findings (Shodan facets, subdomains, live hosts, nmap data).
