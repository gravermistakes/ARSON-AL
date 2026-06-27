<!-- generated: spec; sign-off epoch appended at commit -->
# Web2 request routing — intercept-proxy adapter spec

How Opaca drives web2 requests through Burp, Caido, or ZAP. One Opaca-facing
contract; per-tool adapters underneath. Drop-in, swap-out — the actor doesn't
know which tool is running.

## What this is for

A web2 actor in a recon/scan/exploit kit doesn't speak raw HTTP. It speaks
*intent*: "send this request shape through the target's auth/crypto/sig
pipeline and tell me what came back." The target's pipeline is usually
non-trivial (signed bodies, rotating headers, JWT refresh, custom encoding) —
that's exactly what intercept-proxy extensions exist to handle. So the actor
hands a typed RequestSpec to a proxy + an extension that knows the target's
quirks, and reads typed Responses + Findings back.

## The one contract

```ocaml
(* shape, not syntax *)
module Proxy : sig
  type session                    (* one open proxy, scoped to a target *)
  type ext                        (* installed extension: handles target quirks *)

  val start    : target -> session
  val install  : session -> ext_pack -> ext      (* crypto/auth/sig handler *)
  val send     : session -> RequestSpec.t -> Response.t
  val findings : session -> Finding.t list       (* passive matches the proxy collected *)
  val stop     : session -> unit
end
```

`ext_pack` is a per-target blob (signing key, auth fn, encoding rules) the
actor or the Agent assembled during recon. The proxy applies it on every
outgoing request without the actor having to know.

## Per-tool adapters (same contract, different backend)

| backend | api | what the adapter wraps |
|---------|-----|------------------------|
| **Burp Montoya** | Java, gradle | the recommended target. Adapter loads as a Montoya extension; exposes `send`/`findings` over a local socket to Opaca. |
| **Burp Extender (legacy)** | Java, maven | the older API the existing Jython scripts use. Same socket, narrower feature set. Keep for the encrypted-Intruder scripts already vendored. |
| **Caido** | JS/TS via `@caido/sdk-{frontend,backend}` (npm) | newer; Rust core + JS plugins. Adapter is a Caido plugin that proxies the same socket protocol. Note: the github.com/caido/caido vendored here is brand/docs — the SDK is npm. |
| **OWASP ZAP** | Java, gradle | full daemon mode with its own REST API and gRPC. Adapter is the thinnest of the three: ZAP already does most of the protocol; we wrap it. |

The contract is the **socket protocol**, not a code library. Each backend ships
an extension/plugin that listens on a unix socket Opaca connects to. The
extension is small (delegates to its native API); Opaca's adapter just speaks
the protocol.

## Wire format

Newline-delimited JSON over unix socket. One frame per direction per request.

```
> {"op":"send","req":{"method":"POST","url":"…","headers":{…},"body_b64":"…"},
   "ext":["sig-v2","jwt-refresh"]}
< {"resp":{"status":200,"headers":{…},"body_b64":"…","timing_ms":143},
   "findings":[{"kind":"passive","class":"reflected","ctx":"…"}]}
```

`body_b64` keeps binary cleanly. `ext` lists ext-packs to apply (order matters).
The proxy returns the response *plus* anything its passive rules caught (a
finding stream, not just the HTTP reply).

## Lifecycle

- **One session per target surface** (matches `TargetSupervisor` per-surface).
  The same session serves all actors hunting that surface; their requests
  multiplex over it. Faster than spinning a proxy per actor; isolates by
  surface, which is the security boundary.
- **Extensions installed lazily** the first time a target needs them. Cached
  per session. Re-applied on session restart.
- **Crash recovery**: the session dies, the supervisor restarts it, the
  registry replays the install list. Actors retry their pending sends.

## Why this and not raw HTTP

- Recon already runs through the proxy → consistent passive findings
- Custom auth/crypto solved once per target, not per kit
- Replay/repeater "for free" via session log → debugging + Agent context
- Same kit code works whether the operator chose Burp, Caido, or ZAP

## Build order

1. **Define the socket protocol** (this file, then JSON schema).
2. **Burp Montoya adapter** first (most mature API; easiest to validate).
3. **ZAP adapter** next (already has a REST API; adapter is mostly translation).
4. **Caido adapter** when its SDK stabilizes (currently npm-only, no public
   git surface).
5. **Burp Extender legacy** only if the existing Jython scripts move forward
   intact; otherwise port them to Montoya.

## Open

- Should ext-packs live as kit data (declarative) or compiled OCaml modules?
  Lean declarative — they're target-specific blobs assembled at recon time.
- Passive vs. active findings boundary: the proxy emits passive-matched
  findings on every response. Active scans (Burp Scanner, ZAP active) are a
  separate `scan` op, not `send`.
- This cluster's directory is misnamed: `picks/burp-extensions/` now holds
  three proxies. Rename to `picks/intercept-proxies/` when convenient.
