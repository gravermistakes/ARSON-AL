<!-- generated: spec; sign-off epoch appended at commit -->
# CrisisActor — the web2 intercept-proxy actor

How Opaca drives web2 requests through Burp, Caido, or ZAP. **One actor class
owns the proxy.** Other actors send it messages. The proxy's native API is
private to the CrisisActor.

## The shape

```
TargetSupervisor (per-surface)
├── CrisisActor                           ← long-lived; owns one proxy session
│   • state: open session + installed exts + finding log
│   • backend: one of {Burp, Caido, ZAP}, picked at start, hidden from others
├── Actor (recon kit) -─ sends → CrisisActor
├── Actor (scan kit)  -─ sends → CrisisActor
└── ChainActor       -─ subscribes → CrisisActor.findings
```

A web2 actor never speaks HTTP, never opens a socket, never knows whether
Burp or ZAP is on the other end. It sends Riot messages. The CrisisActor speaks
its backend's native API and emits typed results back.

## Message protocol (internal, not wire)

```ocaml
(* shape, not syntax *)
type req =
  | Install_ext of ext_pack            (* sig, jwt-refresh, target encoding... *)
  | Send of RequestSpec.t              (* one HTTP intent *)
  | Scan of ScanSpec.t                 (* active scan via backend's scanner *)
  | Get_findings                       (* drain the passive-finding log *)

type reply =
  | Ok of Response.t * Finding.t list  (* response + anything passive caught *)
  | Findings of Finding.t list
  | NeedsAgent of ambiguity            (* punt to the Agent tactician *)
  | Failed of err
```

`ext_pack` is target-specific (assembled at recon by the actor or Agent:
signing key, auth function, encoding rules). Installed once, reapplied on every
send.

## Backends — CrisisActor variants

All variants expose the same message interface. Internals differ:

| backend | api | how CrisisActor wraps it |
|---------|-----|-------------------------|
| **Burp Montoya** | Java, gradle | subprocess-spawn Burp with a small Montoya extension that opens a unix socket; CrisisActor talks to it via that socket. Recommended target — most mature API. |
| **Burp Extender (legacy)** | Java, maven | same shape, narrower API. Kept only for the encrypted-Intruder Jython scripts already vendored. |
| **OWASP ZAP** | Java, gradle | spawn ZAP in daemon mode; talk its REST/gRPC API directly. Adapter is thin — ZAP already does most of the work. |
| **Caido** | TS via `@caido/sdk-{frontend,backend}` (npm) | Caido plugin opens a socket; CrisisActor connects. Note: `caido/` here is brand/docs — the SDK is npm-only. |

Backend choice is one parameter at CrisisActor start; the rest of the engine
sees one actor class.

## Lifecycle (Riot supervision tree)

- **Start**: TargetSupervisor spawns CrisisActor with `(target, backend)`. The
  actor starts the backend (subprocess), waits for ready, transitions to live.
- **Install**: extension installs are replayable. CrisisActor keeps an install
  log; on restart, the supervisor restarts the actor and the actor replays the
  log against the fresh backend.
- **Send**: every `Send` returns `Ok` (or `NeedsAgent` / `Failed`). Per-target
  serialization vs. concurrent dispatch is the actor's policy.
- **Crash**: the backend dies, the actor crashes with it, the supervisor
  restarts. Other actors' pending `Send`s retry per their kit's retry rule.
- **Stop**: TargetSupervisor terminates CrisisActor at engagement end;
  backend subprocess reaped.

## Generalization — the ToolActor class

CrisisActor is one species of a broader pattern. **Any long-lived stateful
external tool earns its own actor.** The shape is always:

> *ToolActor owns the tool's session, exposes typed Riot messages, hides the
> native API.*

Candidates beyond Proxy:
- **LDAPActor** — drives BOFHound's parser engine for AD recon over long
  sessions.
- **TeamserverActor** — Sliver/Havoc beacon manager when a C2 path stands up.
- **OASTActor** — long-lived Collaborator/Interactsh listener for blind PoCs.

Short-lived one-shots (noir extract, vigolium scan, drogonsec audit) stay as
direct tool invocations from kits. The rule: **session state → actor; one-shot →
invocation.**

## Build order

1. **Define the message types** (this file → Opaca types in OCaml).
2. **Burp Montoya CrisisActor** first — write the Montoya extension that opens
   the socket; write the OCaml actor.
3. **ZAP CrisisActor** next — almost-translation around ZAP's REST API.
4. **Caido CrisisActor** when its SDK story stabilizes.
5. Generalize the ToolActor base when a second tool class shows up (LDAPActor
   is the likely next).

## Open

- **Backend choice**: operator config at start, or learned per target from
  score history? Default to config; let the learning layer override later.
- **Per-target serialization**: should CrisisActor serialize all sends to one
  surface (safe but slow) or pipeline concurrently (fast but races on stateful
  exts)? Start serialized; relax when an ext declares itself stateless.
- **Dir name**: `picks/burp-extensions/` now holds three proxies. Rename to
  `picks/intercept-proxies/` when convenient.
