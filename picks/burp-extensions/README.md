# Burp Suite extensions

Custom attack extensions for Burp Suite (a `picks/` species: the firing tool
that drives crafted requests through Burp's intercept pipeline).

## What's here
- `extender-api/` — legacy **Extender API** (PortSwigger/burp-extender-api).
  Java interfaces the older Jython scripts here program against.
- `montoya-api/` — current **Montoya API** (PortSwigger/burp-extensions-montoya-api),
  the recommended target for new extensions.
- `IHttpListener_for_Intruder_Scanner_Runtime_Encryption.py` — Jython 2.7,
  runtime crypto in Intruder + Scanner. Legacy Extender API.
- `Text-editor_Tab_Runtime_Encryption_Decryption.py` — Jython 2.7, custom
  editor tab with runtime crypt. Legacy Extender API.
- `exceptions_fix.py` — shared exception scaffolding for the Jython scripts.

## Build/use
- Jython 2.7 scripts load directly into Burp's Extender tab.
- Extender API extensions: Maven (`pom.xml`).
- Montoya API extensions: Gradle (`build.gradle`).

## Migration
The Jython scripts use the legacy Extender API. Montoya is the path forward —
PortSwigger is freezing Extender features. New extensions should program against
`montoya-api/` directly.
