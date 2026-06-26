# awesome-bugbounty-tools — probes

Subset of vavkamil/awesome-bugbounty-tools — classes that sort to **probes** (maps / enum / scrapers / crawlers). See INTEGRATION-TABLE.md for the species roster with live GitHub metadata.

### Subdomain Takeover

- [subjack](https://github.com/haccer/subjack) - Subdomain Takeover tool written in Go
- [SubOver](https://github.com/Ice3man543/SubOver) - A Powerful Subdomain Takeover Tool
- [autoSubTakeover](https://github.com/JordyZomer/autoSubTakeover) - A tool used to check if a CNAME resolves to the scope address. If the CNAME resolves to a non-scope address it might be worth checking out if subdomain takeover is possible.
- [NSBrute](https://github.com/shivsahni/NSBrute) - Python utility to takeover domains vulnerable to AWS NS Takeover
- [can-i-take-over-xyz](https://github.com/EdOverflow/can-i-take-over-xyz) - "Can I take over XYZ?" — a list of services and how to claim (sub)domains with dangling DNS records.
- [cnames](https://github.com/cybercdh/cnames) - take a list of resolved subdomains and output any corresponding CNAMES en masse.
- [subHijack](https://github.com/vavkamil/old-repos-backup/tree/master/subHijack-master) - Hijacking forgotten & misconfigured subdomains
- [tko-subs](https://github.com/anshumanbh/tko-subs) - A tool that can help detect and takeover subdomains with dead DNS records
- [HostileSubBruteforcer](https://github.com/nahamsec/HostileSubBruteforcer) - This app will bruteforce for existing subdomains and provide information if the 3rd party host has been properly setup.
- [second-order](https://github.com/mhmdiaa/second-order) - Second-order subdomain takeover scanner
- [takeover](https://github.com/mzfr/takeover) - A tool for testing subdomain takeover possibilities at a mass scale.
- [dnsReaper](https://github.com/punk-security/dnsReaper) - DNS Reaper is yet another sub-domain takeover tool, but with an emphasis on accuracy, speed and the number of signatures in our arsenal!
- [subzy](https://github.com/PentestPad/subzy) - Subdomain takeover tool which works based on matching response fingerprints from `can-i-take-over-xyz`.


### JSON Web Token

- [jwt_tool](https://github.com/ticarpi/jwt_tool) - A toolkit for testing, tweaking and cracking JSON Web Tokens
- [c-jwt-cracker](https://github.com/brendan-rius/c-jwt-cracker) - JWT brute force cracker written in C
- [jwt-heartbreaker](https://github.com/wallarm/jwt-heartbreaker) - The Burp extension to check JWT (JSON Web Tokens) for using keys from known from public sources
- [jwtear](https://github.com/KINGSABRI/jwtear) - Modular command-line tool to parse, create and manipulate JWT tokens for hackers
- [jwt-key-id-injector](https://github.com/dariusztytko/jwt-key-id-injector) - Simple python script to check against hypothetical JWT vulnerability.
- [jwt-hack](https://github.com/hahwul/jwt-hack) - jwt-hack is tool for hacking / security testing to JWT.
- [jwt-cracker](https://github.com/lmammino/jwt-cracker) - Simple HS256 JWT token brute force cracker


### Screenshots

- [EyeWitness](https://github.com/FortyNorthSecurity/EyeWitness) - EyeWitness is designed to take screenshots of websites, provide some server header info, and identify default credentials if possible.
- [aquatone](https://github.com/michenriksen/aquatone) - Aquatone is a tool for visual inspection of websites across a large amount of hosts and is convenient for quickly gaining an overview of HTTP-based attack surface.
- [screenshoteer](https://github.com/vladocar/screenshoteer) - Make website screenshots and mobile emulations from the command line.
- [gowitness](https://github.com/sensepost/gowitness) - gowitness - a golang, web screenshot utility using Chrome Headless
- [WitnessMe](https://github.com/byt3bl33d3r/WitnessMe) - Web Inventory tool, takes screenshots of webpages using Pyppeteer (headless Chrome/Chromium) and provides some extra bells & whistles to make life easier.
- [eyeballer](https://github.com/BishopFox/eyeballer) - Convolutional neural network for analyzing pentest screenshots
- [scrying](https://github.com/nccgroup/scrying) - A tool for collecting RDP, web and VNC screenshots all in one place
- [Depix](https://github.com/beurtschipper/Depix) - Recovers passwords from pixelized screenshots
- [httpscreenshot](https://github.com/breenmachine/httpscreenshot/) - HTTPScreenshot is a tool for grabbing screenshots and HTML of large numbers of websites.
- [invisible-playwright](https://github.com/feder-cr/invisible_playwright) - Playwright wrapper for a stealth-patched Firefox 150 binary, useful for screenshotting and recon against targets with anti-bot detection (reCAPTCHA v3, FingerprintPro, Cloudflare).


### Content Discovery

- [gobuster](https://github.com/OJ/gobuster) - Directory/File, DNS and VHost busting tool written in Go
- [recursebuster](https://github.com/C-Sto/recursebuster) - rapid content discovery tool for recursively querying webservers, handy in pentesting and web application assessments
- [feroxbuster](https://github.com/epi052/feroxbuster) - A fast, simple, recursive content discovery tool written in Rust.
- [dirsearch](https://github.com/maurosoria/dirsearch) - Web path scanner
- [dirsearch](https://github.com/evilsocket/dirsearch) - A Go implementation of dirsearch.
- [filebuster](https://github.com/henshin/filebuster) - An extremely fast and flexible web fuzzer
- [dirstalk](https://github.com/stefanoj3/dirstalk) - Modern alternative to dirbuster/dirb
- [dirbuster-ng](https://github.com/digination/dirbuster-ng) - dirbuster-ng is C CLI implementation of the Java dirbuster tool
- [gospider](https://github.com/jaeles-project/gospider) - Gospider - Fast web spider written in Go
- [hakrawler](https://github.com/hakluke/hakrawler) - Simple, fast web crawler designed for easy, quick discovery of endpoints and assets within a web application
- [crawley](https://github.com/s0rg/crawley) - fast, feature-rich unix-way web scraper/crawler written in Golang.
- [katana](https://github.com/projectdiscovery/katana) - A next-generation crawling and spidering framework
- [kiterunner](https://github.com/assetnote/kiterunner) - Fast API endpoint bruteforcer and content discovery tool for modern web applications.
- [vaf](https://github.com/andreiverse/vaf) - Vaf is a cross-platform very advanced and fast web fuzzer written in nim .
- [uncover](https://github.com/projectdiscovery/uncover) - uncover is a go wrapper using APIs of well known search engines to quickly discover exposed hosts on the internet.


### Secrets

- [git-secrets](https://github.com/awslabs/git-secrets) - Prevents you from committing secrets and credentials into git repositories
- [gitleaks](https://github.com/zricethezav/gitleaks) - Scan git repos (or files) for secrets using regex and entropy
- [truffleHog](https://github.com/dxa4481/truffleHog) - Searches through git repositories for high entropy strings and secrets, digging deep into commit history
- [gitGraber](https://github.com/hisxo/gitGraber) - gitGraber: monitor GitHub to search and find sensitive data in real time for different online services
- [talisman](https://github.com/thoughtworks/talisman) - By hooking into the pre-push hook provided by Git, Talisman validates the outgoing changeset for things that look suspicious - such as authorization tokens and private keys.
- [GitGot](https://github.com/BishopFox/GitGot) - Semi-automated, feedback-driven tool to rapidly search through troves of public data on GitHub for sensitive secrets.
- [git-all-secrets](https://github.com/anshumanbh/git-all-secrets) - A tool to capture all the git secrets by leveraging multiple open source git searching tools
- [github-search](https://github.com/gwen001/github-search) - Tools to perform basic search on GitHub.
- [git-vuln-finder](https://github.com/cve-search/git-vuln-finder) - Finding potential software vulnerabilities from git commit messages
- [commit-stream](https://github.com/x1sec/commit-stream) - #OSINT tool for finding Github repositories by extracting commit logs in real time from the Github event API
- [gitrob](https://github.com/michenriksen/gitrob) - Reconnaissance tool for GitHub organizations
- [repo-supervisor](https://github.com/auth0/repo-supervisor) - Scan your code for security misconfiguration, search for passwords and secrets.
- [GitMiner](https://github.com/UnkL4b/GitMiner) - Tool for advanced mining for content on Github
- [shhgit](https://github.com/eth0izzle/shhgit) - Ah shhgit! Find GitHub secrets in real time
- [detect-secrets](https://github.com/Yelp/detect-secrets) - An enterprise friendly way of detecting and preventing secrets in code.
- [rusty-hog](https://github.com/newrelic/rusty-hog) - A suite of secret scanners built in Rust for performance. Based on TruffleHog
- [whispers](https://github.com/Skyscanner/whispers) - Identify hardcoded secrets and dangerous behaviours
- [yar](https://github.com/nielsing/yar) - Yar is a tool for plunderin' organizations, users and/or repositories.
- [dufflebag](https://github.com/BishopFox/dufflebag) - Search exposed EBS volumes for secrets
- [secret-bridge](https://github.com/duo-labs/secret-bridge) - Monitors Github for leaked secrets
- [earlybird](https://github.com/americanexpress/earlybird) - EarlyBird is a sensitive data detection tool capable of scanning source code repositories for clear text password violations, PII, outdated cryptography methods, key files and more.
- [Trufflehog-Chrome-Extension](https://github.com/trufflesecurity/Trufflehog-Chrome-Extension) - Trufflehog-Chrome-Extension
- [noseyparker](https://github.com/praetorian-inc/noseyparker) - Nosey Parker is a command-line program that finds secrets and sensitive information in textual data and Git history.
- [GitHound](https://github.com/tillson/git-hound) - Recon tool leveraging Code Search API. Scans for exposed API keys across all of GitHub, not just known repos and orgs. Support for GitHub dorks.
- [cariddi](https://github.com/edoardottt/cariddi) - Take a list of domains, crawl urls and scan for endpoints, secrets, api keys, file extensions, tokens and more...
- [SecretFinder](https://github.com/m4ll0k/SecretFinder) - A python script for finding sensitive data (apikeys, accesstoken,jwt,..) and search anything on javascript files.
- [js-snitch](https://github.com/vavkamil/js-snitch) - Scans remote JavaScript files with Trufflehog + Semgrep to detect leaked secrets.
- [keyhacks](https://github.com/streaak/keyhacks) - KeyHacks shows methods to validate different API keys found on a Bug Bounty Program or a pentest.
- [keyFinder](https://github.com/momenbasel/keyFinder) - A Chrome extension that passively scans web pages for API keys, tokens, and secrets using 80+ regex patterns and Shannon entropy analysis across 10 attack surfaces.



### CMS

- [wpscan](https://github.com/wpscanteam/wpscan) - WPScan is a free, for non-commercial use, black box WordPress security scanner
- [WPSpider](https://github.com/cyc10n3/WPSpider) - A centralized dashboard for running and scheduling WordPress scans powered by wpscan utility.
- [wprecon](https://github.com/blackcrw/wprecon) - Wordpress Recon
- [Temodar Agent](https://github.com/xeloxa/temodar-agent) - AI-powered WordPress plugin/theme security analysis platform with Semgrep-based static analysis and agent-assisted investigation workflows
- [CMSmap](https://github.com/Dionach/CMSmap) -  CMSmap is a python open source CMS scanner that automates the process of detecting security flaws of the most popular CMSs.
- [joomscan](https://github.com/OWASP/joomscan) - OWASP Joomla Vulnerability Scanner Project
- [pyfiscan](https://github.com/fgeek/pyfiscan) - Free web-application vulnerability and version scanner
- [aemhacker](https://github.com/0ang3el/aem-hacker) - Tools to identify vulnerable Adobe Experience Manager (AEM) webapps.
- [aemscan](https://github.com/Raz0r/aemscan) - Adobe Experience Manager Vulnerability Scanner



### Useful

- [anew](https://github.com/tomnomnom/anew) -  A tool for adding new lines to files, skipping duplicates 
- [gf](https://github.com/tomnomnom/gf) -  A wrapper around grep, to help you grep for things 
- [uro](https://github.com/s0md3v/uro) -  declutters url lists for crawling/pentesting 
- [unfurl](https://github.com/tomnomnom/unfurl) -  Pull out bits of URLs provided on stdin 
- [qsreplace](https://github.com/tomnomnom/qsreplace) -  Accept URLs on stdin, replace all query string values with a user-supplied value 
- [interactsh](https://github.com/projectdiscovery/interactsh) - Interactsh is an open-source tool for detecting out-of-band interactions. It is a tool designed to detect vulnerabilities that cause external interactions.
- [CyberChef](https://github.com/gchq/CyberChef) - The Cyber Swiss Army Knife - a web app for encryption, encoding, compression and data analysis
- [notify](https://github.com/projectdiscovery/notify) - Notify is a Go-based assistance package that enables you to stream the output of several tools (or read from a file) and publish it to a variety of supported platforms.


### Origin IP

- [CloudRip](https://github.com/staxsum/CloudRip) - A tool that helps you find the real IP addresses hiding behind Cloudflare by checking subdomains.
- [hakoriginfinder](https://github.com/hakluke/hakoriginfinder) - Tool for discovering the origin host behind a reverse proxy. Useful for bypassing WAFs and other reverse proxies.
 

### Fuzzing

- [wfuzz](https://github.com/xmendez/wfuzz) - Web application fuzzer
- [ffuf](https://github.com/ffuf/ffuf) -  Fast web fuzzer written in Go
- [fuzzdb](https://github.com/fuzzdb-project/fuzzdb) - Dictionary of attack patterns and primitives for black-box application fault injection and resource discovery.
- [IntruderPayloads](https://github.com/1N3/IntruderPayloads) - A collection of Burpsuite Intruder payloads, BurpBounty payloads, fuzz lists, malicious file uploads and web pentesting methodologies and checklists.
- [fuzz.txt](https://github.com/Bo0oM/fuzz.txt) - Potentially dangerous files
- [fuzzilli](https://github.com/googleprojectzero/fuzzilli) - A JavaScript Engine Fuzzer
- [fuzzapi](https://github.com/Fuzzapi/fuzzapi) - Fuzzapi is a tool used for REST API pentesting and uses API_Fuzzer gem
- [qsfuzz](https://github.com/ameenmaali/qsfuzz) - qsfuzz (Query String Fuzz) allows you to build your own rules to fuzz query strings and easily identify vulnerabilities.
- [vaf](https://github.com/d4rckh/vaf) - very advanced (web) fuzzer written in Nim.


### Subdomain Enumeration
- [Sublist3r](https://github.com/aboul3la/Sublist3r) - Fast subdomains enumeration tool for penetration testers
- [Amass](https://github.com/OWASP/Amass) - In-depth Attack Surface Mapping and Asset Discovery
- [massdns](https://github.com/blechschmidt/massdns) - A high-performance DNS stub resolver for bulk lookups and reconnaissance (subdomain enumeration)
- [Findomain](https://github.com/Findomain/Findomain) - The fastest and cross-platform subdomain enumerator, do not waste your time.
- [Sudomy](https://github.com/Screetsec/Sudomy) - Sudomy is a subdomain enumeration tool to collect subdomains and analyzing domains performing automated reconnaissance (recon) for bug hunting / pentesting
- [chaos-client](https://github.com/projectdiscovery/chaos-client) - Go client to communicate with Chaos DNS API.
- [domained](https://github.com/TypeError/domained) - Multi Tool Subdomain Enumeration
- [bugcrowd-levelup-subdomain-enumeration](https://github.com/appsecco/bugcrowd-levelup-subdomain-enumeration) - This repository contains all the material from the talk "Esoteric sub-domain enumeration techniques" given at Bugcrowd LevelUp 2017 virtual conference
- [shuffledns](https://github.com/projectdiscovery/shuffledns) - shuffleDNS is a wrapper around massdns written in go that allows you to enumerate valid subdomains using active bruteforce as well as resolve subdomains with wildcard handling and easy input-output…
- [puredns](https://github.com/d3mondev/puredns) - Fast domain resolver and subdomain bruteforcing with accurate wildcard filtering with wildcard(*)
- [censys-subdomain-finder](https://github.com/christophetd/censys-subdomain-finder) - Perform subdomain enumeration using the certificate transparency logs from Censys.
- [Turbolist3r](https://github.com/fleetcaptain/Turbolist3r) - Subdomain enumeration tool with analysis features for discovered domains
- [censys-enumeration](https://github.com/0xbharath/censys-enumeration) - A script to extract subdomains/emails for a given domain using SSL/TLS certificate dataset on Censys
- [tugarecon](https://github.com/LordNeoStark/tugarecon) - Fast subdomains enumeration tool for penetration testers.
- [as3nt](https://github.com/cinerieus/as3nt) - Another Subdomain ENumeration Tool
- [Subra](https://github.com/si9int/Subra) - A Web-UI for subdomain enumeration (subfinder)
- [Substr3am](https://github.com/nexxai/Substr3am) - Passive reconnaissance/enumeration of interesting targets by watching for SSL certificates being issued
- [domain](https://github.com/jhaddix/domain/) - enumall.py Setup script for Regon-ng
- [altdns](https://github.com/infosec-au/altdns) - Generates permutations, alterations and mutations of subdomains and then resolves them
- [brutesubs](https://github.com/anshumanbh/brutesubs) - An automation framework for running multiple open sourced subdomain bruteforcing tools (in parallel) using your own wordlists via Docker Compose
- [dns-parallel-prober](https://github.com/lorenzog/dns-parallel-prober) - his is a parallelised domain name prober to find as many subdomains of a given domain as fast as possible.
- [dnscan](https://github.com/rbsec/dnscan) - dnscan is a python wordlist-based DNS subdomain scanner.
- [knock](https://github.com/guelfoweb/knock) - Knockpy is a python tool designed to enumerate subdomains on a target domain through a wordlist.
- [hakrevdns](https://github.com/hakluke/hakrevdns) - Small, fast tool for performing reverse DNS lookups en masse.
- [dnsx](https://github.com/projectdiscovery/dnsx) - Dnsx is a fast and multi-purpose DNS toolkit allow to run multiple DNS queries of your choice with a list of user-supplied resolvers.
- [subfinder](https://github.com/projectdiscovery/subfinder) - Subfinder is a subdomain discovery tool that discovers valid subdomains for websites.
- [assetfinder](https://github.com/tomnomnom/assetfinder) - Find domains and subdomains related to a given domain
- [crtndstry](https://github.com/nahamsec/crtndstry) - Yet another subdomain finder
- [VHostScan](https://github.com/codingo/VHostScan) - A virtual host scanner that performs reverse lookups
- [scilla](https://github.com/edoardottt/scilla) - Information Gathering tool - DNS / Subdomains / Ports / Directories enumeration
- [sub3suite](https://github.com/3nock/sub3suite) - A research-grade suite of tools for subdomain enumeration, intelligence gathering and attack surface mapping.
- [cero](https://github.com/glebarez/cero) - Scrape domain names from SSL certificates of arbitrary hosts 
- [shosubgo](https://github.com/incogbyte/shosubgo) - Small tool to Grab subdomains using Shodan api
- [haktrails](https://github.com/hakluke/haktrails) - Golang client for querying SecurityTrails API data
- [bbot](https://github.com/blacklanternsecurity/bbot) - A recursive internet scanner for hackers
- [crt.go](https://github.com/TaurusOmar/crt.sh) - This Go script simplifies the process of efficiently saving and analyzing subdomain output from the crt.sh website.
- [github-subdomains](https://github.com/gwen001/github-subdomains) - This Go tool performs searches on GitHub and parses the results to find subdomains of a given domain.
- [gitlab-subdomains](https://github.com/gwen001/gitlab-subdomains) - This Go tool performs searches on GitLab and parses the results to find subdomains of a given domain.
- [subdominator](https://github.com/RevoltSecurities/Subdominator) - Fast and powerfull to enumerate subdomains (50+ passive results ).
- [csprecon](https://github.com/edoardottt/csprecon) - Discover new target domains using Content Security Policy 
- [related-domains](https://github.com/gwen001/related-domains) - Find related domains of a given domain. this tool search for domains that have been registered by the same peoples/companies.
- [hakip2host](https://github.com/hakluke/hakip2host) - hakip2host takes a list of IP addresses via stdin, then does a series of checks to return associated domain names.


### Monitoring

- [bbscope](https://github.com/sw33tLie/bbscope) - Scope aggregation tool for HackerOne, Bugcrowd, Intigriti, YesWeHack, Immunefi
- [jsmon](https://github.com/robre/jsmon) - A Javascript change monitoring tool for Bug Bounty.

---


### Vulnerability Scanners

- [nuclei](https://github.com/projectdiscovery/nuclei) - Nuclei is a fast tool for configurable targeted scanning based on templates offering massive extensibility and ease of use.
- [nuclei-templates](https://github.com/projectdiscovery/nuclei-templates) - Community curated list of templates for the nuclei engine to find security vulnerabilities.
- [Sn1per](https://github.com/1N3/Sn1per) - Automated pentest framework for offensive security experts
- [metasploit-framework](https://github.com/rapid7/metasploit-framework) - Metasploit Framework
- [nikto](https://github.com/sullo/nikto) - Nikto web server scanner
- [arachni](https://github.com/Arachni/arachni) - Web Application Security Scanner Framework
- [jaeles](https://github.com/jaeles-project/jaeles) - The Swiss Army knife for automated Web Application Testing
- [retire.js](https://github.com/RetireJS/retire.js) - scanner detecting the use of JavaScript libraries with known vulnerabilities
- [Osmedeus](https://github.com/j3ssie/Osmedeus) - Fully automated offensive security framework for reconnaissance and vulnerability scanning
- [Vigolium](https://github.com/vigolium/vigolium) - High-fidelity vulnerability scanner fusing agentic AI with native speed, modularity, and precision
- [getsploit](https://github.com/vulnersCom/getsploit) - Command line utility for searching and downloading exploits
- [flan](https://github.com/cloudflare/flan) - A pretty sweet vulnerability scanner
- [Findsploit](https://github.com/1N3/Findsploit) - Find exploits in local and online databases instantly
- [BlackWidow](https://github.com/1N3/BlackWidow) - A Python based web application scanner to gather OSINT and fuzz for OWASP vulnerabilities on a target website.
- [backslash-powered-scanner](https://github.com/PortSwigger/backslash-powered-scanner) - Finds unknown classes of injection vulnerabilities
- [Eagle](https://github.com/BitTheByte/Eagle) - Multithreaded Plugin based vulnerability scanner for mass detection of web-based applications vulnerabilities
- [cariddi](https://github.com/edoardottt/cariddi) - Take a list of domains, crawl urls and scan for endpoints, secrets, api keys, file extensions, tokens and more...
- [OWASP ZAP](https://github.com/zaproxy/zaproxy) -  World’s most popular free web security tools and is actively maintained by a dedicated international team of volunteers
- [SSTImap](https://github.com/vladko312/SSTImap) -  SSTImap is a penetration testing software that can check websites for Code Injection and Server-Side Template Injection vulnerabilities and exploit them, giving access to the operating system itself.
- [Lonkero](https://github.com/bountyyfi/lonkero) - Enterprise-grade web vulnerability scanner with 60+ attack modules, built in Rust for penetration testing and security assessments.
- [OWASP PTK](https://github.com/DenisPodgurskii/pentestkit) -  Browser-based vulnerability scanner for bug bounty and pentesting workflows, combining DAST, SAST, IAST, and SCA capabilities to detect runtime, source-level, interactive, and dependency-related security issues.



### Port Scanning

- [masscan](https://github.com/robertdavidgraham/masscan) - TCP port scanner, spews SYN packets asynchronously, scanning entire Internet in under 5 minutes.
- [RustScan](https://github.com/RustScan/RustScan) - The Modern Port Scanner
- [naabu](https://github.com/projectdiscovery/naabu) - A fast port scanner written in go with focus on reliability and simplicity.
- [nmap](https://github.com/nmap/nmap) - Nmap - the Network Mapper. Github mirror of official SVN repository.
- [sandmap](https://github.com/trimstray/sandmap) - Nmap on steroids. Simple CLI with the ability to run pure Nmap engine, 31 modules with 459 scan profiles.
- [ScanCannon](https://github.com/johnnyxmas/ScanCannon) - Combines the speed of masscan with the reliability and detailed enumeration of nmap
- [nrich](https://gitlab.com/shodan-public/nrich) - A command-line tool to quickly analyze all IPs in a file and see which ones have open ports/ vulnerabilities.
- [NimScan](https://github.com/elddy/NimScan/) - Fast Port Scanner 🚀 


### Git

- [GitTools](https://github.com/internetwache/GitTools) - A repository with 3 tools for pwn'ing websites with .git repositories available
- [gitjacker](https://github.com/liamg/gitjacker) - Leak git repositories from misconfigured websites
- [git-dumper](https://github.com/arthaud/git-dumper) - A tool to dump a git repository from a website
- [GitHunter](https://github.com/digininja/GitHunter) - A tool for searching a Git repository for interesting content
- [dvcs-ripper](https://github.com/kost/dvcs-ripper) - Rip web accessible (distributed) version control systems: SVN/GIT/HG...
- [Gato (Github Attack TOolkit)](https://github.com/praetorian-inc/gato) - GitHub Self-Hosted Runner Enumeration and Attack Tool 
- [zizmor](https://github.com/zizmorcore/zizmor) - Static analysis tool for GitHub Actions 


### Content Filtering
- [Hacker-Scoper](https://github.com/ItsIgnacioPortal/Hacker-Scoper) - CLI tool for filtering a mixed list of targets (URLs/IPs) according to the bug-bounty program's scope. The scope can be supplied manually, or it can also be detected automatically by just giving hacker-scoper the name of the targeted company. Hacker-Scoper supports IPs, URLs, wildcards, CIDR ranges, Nmap octet ranges, and even full Regex scopes.


### Links

- [LinkFinder](https://github.com/GerbenJavado/LinkFinder) - A python script that finds endpoints in JavaScript files
- [JS-Scan](https://github.com/zseano/JS-Scan) - a .js scanner, built in php. designed to scrape urls and other info
- [LinksDumper](https://github.com/arbazkiraak/LinksDumper) - Extract (links/possible endpoints) from responses & filter them via decoding/sorting
- [GoLinkFinder](https://github.com/0xsha/GoLinkFinder) - A fast and minimal JS endpoint extractor
- [BurpJSLinkFinder](https://github.com/InitRoot/BurpJSLinkFinder) - Burp Extension for a passive scanning JS files for endpoint links.
- [urlgrab](https://github.com/IAmStoxe/urlgrab) - A golang utility to spider through a website searching for additional links.
- [waybackurls](https://github.com/tomnomnom/waybackurls) - Fetch all the URLs that the Wayback Machine knows about for a domain
- [gau](https://github.com/lc/gau) - Fetch known URLs from AlienVault's Open Threat Exchange, the Wayback Machine, and Common Crawl.
- [getJS](https://github.com/003random/getJS) -  A tool to fastly get all javascript sources/files
- [linx](https://github.com/riza/linx) - Reveals invisible links within JavaScript files
- [waymore](https://github.com/xnl-h4ck3r/waymore) -  Find way more from the Wayback Machine!
- [xnLinkFinder](https://github.com/xnl-h4ck3r/xnLinkFinder) -  A python tool used to discover endpoints, potential parameters, and a target specific wordlist for a given target 
- [URLFinder](https://github.com/projectdiscovery/urlfinder) - A high-speed tool for passively gathering URLs, optimized for efficient web asset discovery without active scanning.
- [github-endpoints](https://github.com/gwen001/github-endpoints) - This Go tool performs searches on GitHub and parses the results to find endpoints of a given domain.
- [jsleak](https://github.com/byt3hx/jsleak) - jsleak is a tool to find secret , paths or links in JavaScript files or source code.
- [jsfinder](https://github.com/kacakb/jsfinder) - A tool that scans web pages to find JavaScript file URLs linked in the HTML source code.
- [jsluice](https://github.com/BishopFox/jsluice) - This tool extracts URLs, paths, secrets, and other interesting bits from JavaScript files. Values are extracted based not just on how they look, but also based on how they are used.


### Technologies

- [wappalyzer](https://github.com/AliasIO/wappalyzer) - Identify technology on websites.
- [webanalyze](https://github.com/rverton/webanalyze) - Port of Wappalyzer (uncovers technologies used on websites) to automate mass scanning.
- [python-builtwith](https://github.com/claymation/python-builtwith) - BuiltWith API client
- [whatweb](https://github.com/urbanadventurer/whatweb) - Next generation web scanner
- [retire.js](https://github.com/RetireJS/retire.js) - scanner detecting the use of JavaScript libraries with known vulnerabilities
- [httpx](https://github.com/projectdiscovery/httpx) - httpx is a fast and multi-purpose HTTP toolkit allows to run multiple probers using retryablehttp library, it is designed to maintain the result reliability with increased threads.
- [fingerprintx](https://github.com/praetorian-inc/fingerprintx) - fingerprintx is a standalone utility for service discovery on open ports that works well with other popular bug bounty command line tools.
- [graphw00f](https://github.com/dolevf/graphw00f) - graphw00f is GraphQL Server Engine Fingerprinting utility for software security professionals looking to learn more about what technology is behind a given GraphQL endpoint.
- [wafw00f](https://github.com/EnableSecurity/wafw00f) - wafw00f allows one to identify and fingerprint Web Application Firewall (WAF) products protecting a website.
- [cdncheck](https://github.com/projectdiscovery/cdncheck) - cdncheck is a tool for identifying the technology associated with dns / ip network addresses.
- [tlsx](https://github.com/projectdiscovery/tlsx) - A fast and configurable TLS grabber focused on TLS based data collection and analysis.
- [MurMurHash](https://github.com/Viralmaniar/MurMurHash) - This little tool is to calculate a MurmurHash value of a favicon. This favicon hash can be used to look for similar websites on various search engines.


### Permutation

- [alterx](https://github.com/projectdiscovery/alterx) - Fast and customizable subdomain wordlist generator using DSL. alterx takes patterns as input and generates subdomain permutation wordlist based on that pattern.
- [gotator](https://github.com/Josue87/gotator) - Gotator is a tool to generate DNS wordlists through permutations.
- [ripgen](https://github.com/resyncgg/ripgen) - Rust-based high performance domain permutation generator.
- [dnsgen](https://github.com/AlephNullSK/dnsgen) - DNSGen is a powerful and flexible DNS name permutation tool designed for security researchers and penetration testers. It generates intelligent domain name variations to assist in subdomain discovery and security assessments.
- [goaltdns](https://github.com/subfinder/goaltdns) - A permutation generation tool written in golang.
- [altdns](https://github.com/infosec-au/altdns) - Generates permutations, alterations and mutations of subdomains and then resolves them.


### Buckets

- [S3Scanner](https://github.com/sa7mon/S3Scanner) - Scan for open AWS S3 buckets and dump the contents
- [AWSBucketDump](https://github.com/jordanpotti/AWSBucketDump) - Security Tool to Look For Interesting Files in S3 Buckets
- [CloudScraper](https://github.com/jordanpotti/CloudScraper) - CloudScraper: Tool to enumerate targets in search of cloud resources. S3 Buckets, Azure Blobs, Digital Ocean Storage Space.
- [s3viewer](https://github.com/SharonBrizinov/s3viewer) - Publicly Open Amazon AWS S3 Bucket Viewer
- [festin](https://github.com/cr0hn/festin) - FestIn - S3 Bucket Weakness Discovery
- [s3reverse](https://github.com/hahwul/s3reverse) - The format of various s3 buckets is convert in one format. for bugbounty and security testing.
- [mass-s3-bucket-tester](https://github.com/random-robbie/mass-s3-bucket-tester) - This tests a list of s3 buckets to see if they have dir listings enabled or if they are uploadable
- [S3BucketList](https://github.com/AlecBlance/S3BucketList) - Firefox plugin that lists Amazon S3 Buckets found in requests
- [dirlstr](https://github.com/cybercdh/dirlstr) - Finds Directory Listings or open S3 buckets from a list of URLs
- [Burp-AnonymousCloud](https://github.com/codewatchorg/Burp-AnonymousCloud) - Burp extension that performs a passive scan to identify cloud buckets and then test them for publicly accessible vulnerabilities
- [kicks3](https://github.com/abuvanth/kicks3) - S3 bucket finder from html,js and bucket misconfiguration testing tool
- [2tearsinabucket](https://github.com/Revenant40/2tearsinabucket) - Enumerate s3 buckets for a specific target.
- [s3_objects_check](https://github.com/nccgroup/s3_objects_check) - Whitebox evaluation of effective S3 object permissions, to identify publicly accessible files.
- [s3tk](https://github.com/ankane/s3tk) - A security toolkit for Amazon S3
- [CloudBrute](https://github.com/0xsha/CloudBrute) - Awesome cloud enumerator
- [s3cario](https://github.com/0xspade/s3cario) - This tool will get the CNAME first if it's a valid Amazon s3 bucket and if it's not, it will try to check if the domain is a bucket name.
- [S3Cruze](https://github.com/JR0ch17/S3Cruze) - All-in-one AWS S3 bucket tool for pentesters.
- [s3dns](https://github.com/olizimmermann/s3dns) - Passive DNS-based discovery of S3 (and other cloud) buckets by resolving CNAMEs and IPs during recon—ideal for stealthy and early identification of cloud storage exposures 


### Parameters

- [parameth](https://github.com/maK-/parameth) - This tool can be used to brute discover GET and POST parameters
- [param-miner](https://github.com/PortSwigger/param-miner) - This extension identifies hidden, unlinked parameters. It's particularly useful for finding web alterx poisoning vulnerabilities.
- [ParamPamPam](https://github.com/Bo0oM/ParamPamPam) - This tool for brute discover GET and POST parameters.
- [Arjun](https://github.com/s0md3v/Arjun) - HTTP parameter discovery suite.
- [ParamSpider](https://github.com/devanshbatham/ParamSpider) - Mining parameters from dark corners of Web Archives.
- [x8](https://github.com/Sh1Yo/x8) - Hidden parameters discovery suite written in Rust.


