# Phase 2 — Pseudocode: Finding type system & kit definition

Types are the spine: findings, kits, tools, and switch rules. Illustrative
OCaml — shape, not final syntax. Traces to FR4, FR1, FR5, FR3, C6, NFR10.

## 1. Severity, classification, evidence (FR4, C6)

```ocaml
type severity = P1 | P2 | P3 | P4          (* scoring: 100/50/20/5 *)
type cwe   = CWE of int                     (* e.g. CWE 89 *)
type cvss  = { vector : string; score : float }   (* 3.1 base vector + score *)

type evidence =
  | Request_response of { req : string; resp : string }
  | Oast_callback    of { token : string; hit_at : int }   (* epoch *)
  | Code_path        of { file : string; line : int; snippet : string }
  | Tool_output      of { tool : string; raw : string; format : string } (* sarif|json|text *)
```

## 2. Finding variants (FR4)

A finding is one of five kinds along the loop. Each carries enough to score,
gate, and report. `id` is content-addressed (hash of kind + locus) for dedup.

```ocaml
type finding_id = string

type finding =
  | Surface  of { id : finding_id; locus : target_ref; kind : surface_kind;
                  source : string }
  | Vuln     of { id : finding_id; locus : target_ref; cwe : cwe; cvss : cvss;
                  severity : severity; hypothesis : string; evidence : evidence list }
  | Exploit  of { id : finding_id; from_vuln : finding_id; works : bool;
                  poc : string; evidence : evidence list }
  | Chain    of { id : finding_id; parts : finding_id list; severity : severity;
                  multiplier : float; narrative : string }
  | Report   of { id : finding_id; about : finding_id; platform : platform;
                  body : string; submitted : bool }

and surface_kind = Endpoint | Subdomain | Param | Header | Source_file | Service
and target_ref   = { element : string; in_scope : bool }   (* in_scope set by gate, §4 *)
and platform     = Immunefi | Bugcrowd | HackerOne | Intigriti | YWH | HackenProof
```

### Relations (the graph the chain-loop searches — NFR8)

```ocaml
type relation =
  | Derived_from of finding_id * finding_id   (* child, parent *)
  | Chains_with  of finding_id * finding_id
  | Evidence_for of finding_id * finding_id
```

### Legal-transition invariant (T-FR4)

A `Report` may only reference an `Exploit{works=true}` or a `Chain`; an
`Exploit` only references a `Vuln`. Enforce in the smart constructor, so an
illegal finding is unrepresentable:

```ocaml
val mk_report : about:finding -> platform:platform -> (finding, error) result
(* returns Error unless `about` is Exploit{works=true} or Chain *)
```

## 3. Kit definition format (FR1)

A kit = a name, an ordered tool list, and switch rules. Pure data; the registry
(Phase 3) loads and validates it.

```ocaml
type tool =
  | Binary of { name : string; path : string; args : string list;
                emits : output_shape }
  | Script of { name : string; path : string; interp : string;   (* python3|bash|node *)
                emits : output_shape }
  | Mcp    of { name : string; server : string; tool : string;
                emits : output_shape }

and output_shape = To_surface | To_vuln | To_exploit | To_manifest | To_report

type switch_rule = On of finding_pattern * action
and  finding_pattern =
  | New_surface | Vulnerability | Source_code | Exploit_works
  | Validation_confirmed | Validation_failed | Report_submitted
and  action = Switch_to of string | Stay | Spawn_child of string

type kit = {
  name        : string;
  tools       : tool list;
  switch_rules: switch_rule list;
}
```

Example (mirrors `OPACA.md`):

```ocaml
let web2_recon = {
  name = "web2-recon";
  tools = [
    Binary { name="noir"; path="probes/noir/noir";
             args=["--ai-context";"-o";"sarif"]; emits=To_surface };
    Script { name="scope-parser"; interp="python3";
             path="int/web2/bug-reaper/scripts/analyze_scope.py"; emits=To_manifest };
  ];
  switch_rules = [
    On (Vulnerability, Switch_to "web2-exploit");
    On (New_surface,   Switch_to "web2-recon");   (* loop *)
    On (Source_code,   Switch_to "sast-probe");
  ];
}
```

## 4. Scope gate (NFR10 — compiled, unbypassable)

Every tool invocation passes the gate before any I/O. The gate is the only
thing that sets `target_ref.in_scope`; a kit cannot forge it.

```ocaml
val authorize : manifest:target_manifest -> element:string -> target_ref
(* in_scope = element matches an authorized rule in the manifest *)

val invoke : tool -> target_ref -> (finding list, error) result
(* PRECONDITION (asserted): target_ref.in_scope = true.
   in_scope=false  ->  drop + log + Error Out_of_scope  (never sends) *)
```

## 5. Target manifest (FR12)

The single shape `int/` emits and all actors consume.

```ocaml
type target_manifest = {
  engagement   : string;
  scope_rules  : scope_rule list;     (* wildcard / apex / exact / address / package *)
  out_of_scope : scope_rule list;
  trust_bounds : (string * string) list;   (* role -> element, e.g. admin->0xabc *)
  emitted_at   : int;                 (* epoch *)
}
```

## 6. Serialization

- Internal persistence: a compact form for SQLite/flat files (Phase 4).
- Egress: **SARIF** for findings into CI; **CycloneDX** for SBOM (C6).
- LLM/MCP boundary: JSON (Phase 4 §1–2).

## 7. TDD anchors

- **T2.1 (T-FR4)** Every `finding` round-trips internal serialize→deserialize
  with no field loss; `id` is stable under re-hash.
- **T2.2 (T-FR4)** `mk_report` rejects a report about an unconfirmed vuln.
- **T2.3 (T-NFR10)** `invoke` with `in_scope=false` returns `Error Out_of_scope`
  and emits zero network/subprocess effects (assert via a recording mock adapter).
- **T2.4 (T-FR1)** A kit referencing a non-existent tool path fails validation
  (Phase 3) before any actor can run it.
- **T2.5** SARIF export of a `Vuln` validates against the SARIF 2.1 schema.

<!-- generated: spec; sign-off epoch appended at commit -->
