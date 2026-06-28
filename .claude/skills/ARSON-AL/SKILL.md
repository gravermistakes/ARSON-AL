```markdown
# ARSON-AL Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill introduces the core development patterns and workflows used in the ARSON-AL TypeScript codebase. It covers file naming, import/export conventions, commit styles, and the structured approach to drafting and promoting module specifications. The repository emphasizes clarity, modularity, and a formalized spec-driven workflow for subsystem design.

## Coding Conventions

- **File Naming:**  
  Use kebab-case for all file names.  
  _Example:_  
  ```
  action-sort.ts
  inventory-manager.ts
  ```

- **Import Style:**  
  Prefer alias imports for clarity and maintainability.  
  _Example:_  
  ```typescript
  import actionSort from 'modules/action-sort'
  import { InventoryItem as Item } from 'inventory/types'
  ```

- **Export Style:**  
  Use default exports for modules.  
  _Example:_  
  ```typescript
  // inventory-manager.ts
  const InventoryManager = { /* ... */ }
  export default InventoryManager
  ```

- **Commit Patterns:**  
  - Freeform messages, sometimes prefixed (e.g., `spec`, `lance`)
  - Average commit message length: ~61 characters

## Workflows

### Bones-Only Spec Draft Workflow
**Trigger:** When starting to formalize the design/spec for a new subsystem or top-level directory  
**Command:** `/new-bones-spec`

1. Create a `REVISE.md` file in the target directory (e.g., `ops/inventory/REVISE.md`).
2. Populate it with a skeleton draft, including key sections:
    - Action sort
    - Inventory
    - Loop role
    - Emits/consumes
    - Open questions
3. Use placeholders or leave blanks for content to be filled in by a domain expert.
4. Revise the draft collaboratively until it is ready for promotion.

_Example skeleton:_
```markdown
# Inventory Subsystem Spec (Draft)

## Action Sort
<!-- TODO: Describe main actions -->

## Inventory
<!-- TODO: List key inventory items -->

## Loop Role
<!-- TODO: Explain subsystem's role in main loop -->

## Emits/Consumes
<!-- TODO: List events/messages -->

## Open Questions
<!-- TODO: List unresolved issues -->
```

### Spec Promotion Sync Workflow
**Trigger:** When a `REVISE.md` draft is ready to become the canonical specification  
**Command:** `/promote-spec`

1. Copy or merge content from `REVISE.md` to `SPEC.md` in the same directory.
2. Add new sections, refine existing ones, and clarify any deferred topics.
3. Leave `REVISE.md` as the ongoing working draft for future changes.

_Example:_
```sh
cp ops/inventory/REVISE.md ops/inventory/SPEC.md
# Edit SPEC.md to refine and finalize the spec
```

## Testing Patterns

- **Test File Naming:**  
  Test files follow the `*.test.*` pattern.  
  _Example:_  
  ```
  inventory-manager.test.ts
  ```

- **Testing Framework:**  
  The specific testing framework is unknown, but tests are colocated with source files using the above pattern.

## Commands

| Command         | Purpose                                                         |
|-----------------|-----------------------------------------------------------------|
| /new-bones-spec | Start a skeleton REVISE.md draft for a new module or subsystem. |
| /promote-spec   | Promote a filled REVISE.md to canonical SPEC.md.                |
```
