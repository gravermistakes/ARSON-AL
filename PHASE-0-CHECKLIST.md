# Phase 0: Foundation - Implementation Checklist

**Duration:** Weeks 1-4  
**Goal:** Establish OCaml 5 + Riot development environment and core types  
**Status:** Not Started

---

## Week 1: Environment Setup & Project Structure

### 1.1 Development Environment
- [ ] **Install OCaml 5.2+**
  - [ ] Install opam (OCaml package manager)
  - [ ] Initialize opam: `opam init`
  - [ ] Create switch: `opam switch create 5.2.0`
  - [ ] Verify: `ocaml --version` shows 5.2.0+
  - **Acceptance:** OCaml 5.2+ compiles and runs

- [ ] **Install Core Dependencies**
  - [ ] Install dune: `opam install dune`
  - [ ] Install Riot: `opam install riot`
  - [ ] Install eio: `opam install eio eio_main`
  - [ ] Install testing: `opam install alcotest`
  - [ ] Install utilities: `opam install fmt logs yojson`
  - **Acceptance:** All packages install without errors

- [ ] **IDE Setup**
  - [ ] Install LSP: `opam install ocaml-lsp-server`
  - [ ] Install formatter: `opam install ocamlformat`
  - [ ] Configure editor (VS Code/Vim/Emacs)
  - [ ] Test autocomplete and type hints
  - **Acceptance:** IDE shows types and autocompletes

### 1.2 Project Structure
- [ ] **Create Directory Layout**
  ```
  opaca/
  ├── bin/           # Executables
  ├── lib/           # Core library
  ├── test/          # Unit tests
  ├── kits/          # Kit definitions
  ├── docs/          # Documentation
  ├── dune-project  # Project config
  └── README.md
  ```
  - [ ] Create all directories
  - [ ] Initialize dune-project
  - [ ] Create initial README.md
  - **Acceptance:** `dune build` succeeds (even with empty project)

- [ ] **Configure Build System**
  - [ ] Create `dune-project` with:
    - Project name: opaca
    - OCaml version: >= 5.2
    - Dependencies: riot, eio, alcotest, fmt, logs, yojson
  - [ ] Create `lib/dune` for library
  - [ ] Create `bin/dune` for executable
  - [ ] Create `test/dune` for tests
  - **Acceptance:** `dune build @all` succeeds

### 1.3 CI/CD Setup
- [ ] **GitHub Actions Workflow**
  - [ ] Create `.github/workflows/ci.yml`
  - [ ] Add OCaml setup action
  - [ ] Add build step: `dune build`
  - [ ] Add test step: `dune runtest`
  - [ ] Add format check: `dune build @fmt`
  - **Acceptance:** CI passes on push

- [ ] **Pre-commit Hooks**
  - [ ] Create `.ocamlformat` config
  - [ ] Add format check to git hooks
  - [ ] Add build check to git hooks
  - **Acceptance:** Commits fail if code doesn't format

---

## Week 2: Core Type System

### 2.1 Finding Types
- [ ] **Define Finding Variants**
  - [ ] Create `lib/finding.ml`
  - [ ] Define type:
    ```ocaml
    type surface = {
      url : string;
      method_ : string;
      params : (string * string) list;
      headers : (string * string) list;
    }
    
    type vuln = {
      cwe : int;
      severity : severity;
      description : string;
      evidence : string list;
    }
    
    type exploit = {
      vuln : vuln;
      poc : string;
      success : bool;
    }
    
    type chain = {
      findings : finding list;
      impact : string;
      cvss : float;
    }
    
    type report = {
      title : string;
      findings : finding list;
      timestamp : float;
    }
    
    and finding =
      | Surface of surface
      | Vuln of vuln
      | Exploit of exploit
      | Chain of chain
      | Report of report
    
    and severity = P1 | P2 | P3 | P4
    ```
  - [ ] Add pretty-printing functions
  - [ ] Add JSON serialization (Yojson)
  - **Acceptance:** Types compile, serialize to JSON

- [ ] **Finding Relations**
  - [ ] Define relation types:
    ```ocaml
    type relation =
      | DerivedFrom of finding
      | ChainsWith of finding
      | EvidenceFor of finding
    ```
  - [ ] Add relation tracking to findings
  - [ ] Add relation query functions
  - **Acceptance:** Can link findings together

- [ ] **Write Tests**
  - [ ] Create `test/test_finding.ml`
  - [ ] Test finding creation
  - [ ] Test JSON round-trip
  - [ ] Test relation tracking
  - **Acceptance:** All tests pass

### 2.2 Kit Types
- [ ] **Define Kit Structure**
  - [ ] Create `lib/kit.ml`
  - [ ] Define type:
    ```ocaml
    type tool = {
      name : string;
      path : string;
      args : string list;
      timeout : float;
    }
    
    type switch_rule =
      | OnSurface of string  (* kit name *)
      | OnVuln of string
      | OnExploit of string
      | OnChain of string
    
    type t = {
      name : string;
      tools : tool list;
      switch_rules : switch_rule list;
    }
    ```
  - [ ] Add kit validation
  - [ ] Add kit loading from file
  - **Acceptance:** Can load kit from JSON/S-exp

- [ ] **Kit Registry**
  - [ ] Create `lib/kit_registry.ml`
  - [ ] Implement registry type:
    ```ocaml
    type t = {
      kits : (string, Kit.t) Hashtbl.t;
    }
    ```
  - [ ] Add `register : t -> Kit.t -> unit`
  - [ ] Add `lookup : t -> string -> Kit.t option`
  - [ ] Add `list : t -> Kit.t list`
  - **Acceptance:** Can register and lookup kits

- [ ] **Write Tests**
  - [ ] Create `test/test_kit.ml`
  - [ ] Test kit creation
  - [ ] Test kit validation
  - [ ] Test registry operations
  - **Acceptance:** All tests pass

### 2.3 Actor Types
- [ ] **Define Actor State**
  - [ ] Create `lib/actor.ml`
  - [ ] Define type:
    ```ocaml
    type seed = int64
    
    type state = {
      seed : seed;
      current_kit : string;
      findings : Finding.t list;
      score : float;
      prng : Random.State.t;
    }
    ```
  - [ ] Add state initialization
  - [ ] Add state serialization
  - **Acceptance:** State can be saved/restored

- [ ] **Actor Messages**
  - [ ] Define message type:
    ```ocaml
    type msg =
      | Finding of Finding.t
      | SwitchKit of string
      | GetState of state Riot.Ref.t
      | Shutdown
    ```
  - [ ] Add message serialization
  - **Acceptance:** Messages can be sent/received

- [ ] **Write Tests**
  - [ ] Create `test/test_actor.ml`
  - [ ] Test state creation
  - [ ] Test state serialization
  - [ ] Test message types
  - **Acceptance:** All tests pass

### 2.4 Score Types
- [ ] **Define Scoring System**
  - [ ] Create `lib/score.ml`
  - [ ] Define type:
    ```ocaml
    type event =
      | FindingConfirmed of Finding.severity
      | ChainCompleted of int  (* num findings *)
      | FalsePositiveAvoided
      | ToolCrash
      | TimeToFinding of float
    
    type t = {
      total : float;
      events : event list;
    }
    ```
  - [ ] Add scoring functions
  - [ ] Add score calculation
  - **Acceptance:** Can calculate scores

- [ ] **Write Tests**
  - [ ] Create `test/test_score.ml`
  - [ ] Test score calculation
  - [ ] Test event tracking
  - **Acceptance:** All tests pass

---

## Week 3: Minimal Actor Runtime

### 3.1 Basic Actor Spawn
- [ ] **Implement Actor Process**
  - [ ] Create `lib/actor_runtime.ml`
  - [ ] Implement spawn function:
    ```ocaml
    val spawn : seed:int64 -> kit:string -> Riot.Pid.t
    ```
  - [ ] Implement basic message loop
  - [ ] Add logging (Logs library)
  - **Acceptance:** Actor spawns and logs startup

- [ ] **Message Passing**
  - [ ] Implement send function:
    ```ocaml
    val send : Riot.Pid.t -> Actor.msg -> unit
    ```
  - [ ] Implement receive in actor loop
  - [ ] Add message queue monitoring
  - **Acceptance:** Messages sent and received

- [ ] **Write Tests**
  - [ ] Create `test/test_actor_runtime.ml`
  - [ ] Test actor spawn
  - [ ] Test message passing
  - [ ] Test actor shutdown
  - **Acceptance:** All tests pass

### 3.2 Supervision
- [ ] **Implement Supervisor**
  - [ ] Create `lib/supervisor.ml`
  - [ ] Implement supervisor process:
    ```ocaml
    type strategy = OneForOne | OneForAll
    
    val start : strategy -> (unit -> Riot.Pid.t) list -> Riot.Pid.t
    ```
  - [ ] Add crash detection
  - [ ] Add restart logic
  - **Acceptance:** Supervisor restarts crashed actors

- [ ] **Crash Recovery**
  - [ ] Implement state restoration
  - [ ] Add crash logging
  - [ ] Add restart limits (max 3 restarts/minute)
  - **Acceptance:** Actor restarts with last state

- [ ] **Write Tests**
  - [ ] Create `test/test_supervisor.ml`
  - [ ] Test crash detection
  - [ ] Test restart logic
  - [ ] Test restart limits
  - **Acceptance:** All tests pass

### 3.3 Multicore Verification
- [ ] **Domain Scheduling**
  - [ ] Create test with 10 actors
  - [ ] Verify actors run on different domains
  - [ ] Measure message throughput
  - **Acceptance:** Actors utilize multiple cores

- [ ] **Performance Baseline**
  - [ ] Measure actor spawn time
  - [ ] Measure message latency
  - [ ] Measure memory per actor
  - [ ] Document results in `docs/PERFORMANCE.md`
  - **Acceptance:** Metrics documented

- [ ] **Write Tests**
  - [ ] Create `test/test_multicore.ml`
  - [ ] Test concurrent actors
  - [ ] Test message ordering
  - **Acceptance:** All tests pass

---

## Week 4: Documentation & Integration

### 4.1 Type System Documentation
- [ ] **Create Type Reference**
  - [ ] Create `docs/TYPES.md`
  - [ ] Document Finding types with examples
  - [ ] Document Kit types with examples
  - [ ] Document Actor types with examples
  - [ ] Document Score types with examples
  - **Acceptance:** All types documented

- [ ] **API Documentation**
  - [ ] Add odoc comments to all modules
  - [ ] Generate HTML docs: `dune build @doc`
  - [ ] Review generated docs
  - **Acceptance:** Docs build without warnings

### 4.2 Example Programs
- [ ] **Minimal Actor Example**
  - [ ] Create `examples/minimal_actor.ml`
  - [ ] Spawn actor, send message, shutdown
  - [ ] Add comments explaining each step
  - **Acceptance:** Example runs successfully

- [ ] **Supervision Example**
  - [ ] Create `examples/supervision.ml`
  - [ ] Spawn supervisor with 3 actors
  - [ ] Crash one actor, verify restart
  - **Acceptance:** Example demonstrates restart

- [ ] **Kit Loading Example**
  - [ ] Create `examples/kit_loading.ml`
  - [ ] Load kit from file
  - [ ] Print kit details
  - **Acceptance:** Example loads kit

### 4.3 Integration Testing
- [ ] **End-to-End Test**
  - [ ] Create `test/test_e2e.ml`
  - [ ] Spawn actor with kit
  - [ ] Send finding message
  - [ ] Verify kit switch
  - [ ] Verify state persistence
  - **Acceptance:** Full workflow works

- [ ] **Stress Test**
  - [ ] Create `test/test_stress.ml`
  - [ ] Spawn 100 actors
  - [ ] Send 1000 messages
  - [ ] Verify no crashes
  - **Acceptance:** System handles load

### 4.4 Phase 0 Review
- [ ] **Code Review**
  - [ ] Review all code for style
  - [ ] Check test coverage (aim for 80%+)
  - [ ] Run `dune build @fmt` and fix issues
  - **Acceptance:** Code passes review

- [ ] **Documentation Review**
  - [ ] Review all docs for completeness
  - [ ] Check examples work
  - [ ] Fix any broken links
  - **Acceptance:** Docs are complete

- [ ] **Deliverables Checklist**
  - [ ] `opaca/lib/types.ml` — Core types ✓
  - [ ] `opaca/lib/actor.ml` — Basic actor spawn/message ✓
  - [ ] `opaca/test/test_actor.ml` — Unit tests ✓
  - [ ] `docs/TYPES.md` — Type system documentation ✓
  - [ ] CI/CD passing ✓
  - **Acceptance:** All deliverables complete

---

## Success Criteria (Phase 0)

- [x] OCaml 5.2+ compiles and runs on target platform
- [ ] Riot actors spawn and communicate
- [ ] Supervision restarts crashed actors
- [ ] Tests pass on CI (80%+ coverage)
- [ ] Core types defined and documented
- [ ] Basic actor runtime works
- [ ] Examples demonstrate key features

---

## Risks & Mitigation

| Risk | Mitigation |
|------|------------|
| Riot API changes | Pin version to 0.0.9, track upstream |
| OCaml 5 effects bugs | Use stable eio 1.0, report issues |
| Learning curve | Pair programming, study examples |
| Timeline slip | Buffer week 4 for catch-up |

---

## Next Steps After Phase 0

1. **Week 5 Checkpoint**
   - Review Phase 0 deliverables
   - Go/No-Go decision for Phase 1
   - Adjust timeline if needed

2. **Phase 1 Prep**
   - Design kit definition format
   - Plan tool adapter interface
   - Set up Phase 1 project board

---

**Status:** Ready to begin  
**Owner:** Development Team  
**Start Date:** TBD  
**Target Completion:** 4 weeks from start
