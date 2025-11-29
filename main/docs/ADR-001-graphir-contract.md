# ADR-001: GraphIR Contract & Canonical Pipeline

**Status**: Accepted
**Date**: 2025-11-29
**Deciders**: Engineering Team
**Context**: Preventing silent safety violations during refactors and extensions

---

## Context

The PXYZ compiler transforms XML workflows into validated GraphIR structures, then emits binary `.wasm` executables. Safety properties (no cycles, auth gates, human-in-loop for irreversible actions) are enforced at compile time.

**Problem**: Early implementation had multiple paths from XML → IR, leading to:
- Validators passing on directly-constructed IR but failing on XML-derived IR
- Cached state (`edge_count`, `edge_start`) not being set consistently
- Binary emission writing garbage when IR invariants weren't satisfied
- No clear contract about what "valid IR" means

**Risk**: As the codebase grows, someone could bypass the pipeline and introduce silent safety violations.

---

## Decision

We establish a **canonical compilation pipeline** with explicit contracts at each boundary.

### 1. Single Source of Truth: `compile_pipeline()`

**Location**: `lib.rs`

```rust
/// Canonical compilation pipeline
///
/// IMPORTANT: This is the ONLY path from XML → validated IR.
/// All tests and front-ends MUST use this to ensure consistency.
pub fn compile_pipeline(
    xml: &str,
    optimize: bool
) -> Result<(GraphIR, OmarDocument, Vec<Diagnostic>), CompileError>
```

**Contract**:
1. Parse XML → AST (fail fast on syntax errors)
2. Lower AST → IR
3. Compile predicates to bytecode
4. **`assign_edge_indices()`** ← IR invariant, MUST run before validation
5. Optimize (optional, preserves invariants)
6. Validate (syntactic, semantic, pragmatic)

**Rules**:
- All front-ends (CLI, tests, API servers) MUST use this function
- No direct calls to `lower()` + `validate()` in production paths
- Tests that construct IR directly must call `ir.assign_edge_indices()` explicitly

---

### 2. IR Invariants (Structural Guarantees)

Valid IR satisfies these invariants (enforced by `ir.assert_invariants()` in debug builds):

#### Structural Integrity
- All `edge.from` and `edge.target` reference existing node IDs
- All `entry.node_id` reference existing node IDs
- No duplicate node IDs

#### Index Coherence
- Edges are sorted by `(from, ...)` (either by ID or by weight)
- Each node's `edge_start` and `edge_count` correctly index into the sorted edge array
- `assign_edge_indices()` establishes this invariant

#### String Pool
- All `node.data_offset` values are valid offsets into `strings.data`
- All interned strings are null-terminated

**Enforcement**:
```rust
#[cfg(debug_assertions)]
ir.assert_invariants();
```

Called:
- After `lower()` completes
- After each optimization pass
- Before `emit()` writes binary

---

### 3. Validator Robustness

**Principle**: Validators MUST NOT depend solely on cached fields that might not be set.

#### Good (Robust)
```rust
// SEM003: Compute outgoing edges dynamically
let outgoing_edges: Vec<_> = ir.edges.iter()
    .filter(|e| e.from == node.id)
    .collect();
```

#### Bad (Fragile)
```rust
// Don't do this - depends on cached edge_count being set
if node.edge_count > 0 { /* ... */ }
```

**Why**: If someone bypasses `assign_edge_indices()`, cached fields will be 0 or stale. Validators would silently pass invalid graphs.

**Current Status**: All validators (SEM001-007, PRAG001-005) are already robust ✅

---

### 4. Test Requirements

Every new validator MUST have:

#### A. Unit Test (Direct IR Construction)
```rust
#[test]
fn test_sem999_whatever() {
    let mut ir = GraphIR::new();
    // ... construct IR directly
    ir.assign_edge_indices(); // MUST call this!

    let diags = check(&ir);
    assert!(diags.iter().any(|d| d.code == "SEM999"));
}
```

#### B. Integration Test (XML → IR or AST → IR)
```rust
// Option 1: XML fixture (when parser coverage exists)
#[test]
fn test_sem999_xml_fixture() {
    expect_codes!("fixtures/sem999_case.xml" => ["SEM999"]);
}

// Option 2: AST construction (for focused lowering tests)
#[test]
fn test_sem999_ast_lowering() {
    let doc = build_test_ast_with_sem999_violation();
    let ir = lower(&doc).unwrap();
    ir.assign_edge_indices();
    let diags = check_semantic(&ir);
    assert!(diags.iter().any(|d| d.code == "SEM999"));
}
```

**Note**: Current golden tests use AST-based approach to isolate lowering semantics from XML parser behavior. XML fixtures are used where parser coverage exists (NASA tests).

#### C. Golden Test (Snapshot)
For complex transformations, assert IR equivalence:
```rust
let ir_from_xml = compile_pipeline(xml, false)?.0;
let ir_from_direct = build_ir_directly();
assert_ir_equivalent(&ir_from_xml, &ir_from_direct);
```

---

### 5. Contract Boundaries

#### `lower()` → `GraphIR`
**Guarantees**:
- All nodes, edges, entries created
- Predicate IDs resolved
- String pool populated
- **Does NOT guarantee**: `edge_start`/`edge_count` are set

**Consumer must**: Call `assign_edge_indices()` before validation/emission

---

#### `assign_edge_indices()` → Modified `GraphIR`
**Guarantees**:
- Edges sorted by `(from, id)`
- All `edge_start`/`edge_count` correctly set

**Why it exists**: Runtime needs O(1) edge lookup. Binary format encodes this.

**Special case**: `optimize_edge_order()` uses `reassign_edge_indices_preserving_order()` to maintain weight-based sort.

---

#### `validate()` → `Vec<Diagnostic>`
**Requires**:
- IR with valid structure (invariants satisfied)

**Guarantees**:
- Returns all syntactic, semantic, pragmatic violations
- Does NOT modify IR

**Robustness**: Validators compute from graph structure, not cached fields

---

#### `emit()` → Binary
**Requires**:
- IR with invariants satisfied (checked in debug builds)
- `assign_edge_indices()` must have been called

**Guarantees**:
- Valid `graph.bin` binary format
- SHA-256 hash of source XML embedded

**Why invariants matter**: Binary format relies on `edge_start`/`edge_count` being correct. If not set, binary is corrupt.

---

## Consequences

### Positive
- **One path to production**: Eliminates "works in tests, fails in prod" scenarios
- **Fail fast**: Debug builds catch contract violations immediately
- **Auditable**: Exactly one function to review for safety properties
- **Refactor-safe**: Validators don't silently break when lowering changes

### Negative
- **Performance**: Debug builds pay cost of invariant checking (acceptable - release builds skip it)
- **Developer discipline required**: New code must follow the pipeline (enforced by review checklist)

### Neutral
- **Tests must be explicit**: Can't construct invalid IR and expect validators to work

---

## Enforcement Mechanisms

### 1. Code Review Checklist
Every PR touching compiler code must answer:

- [ ] Does this bypass `compile_pipeline()`? (No = good)
- [ ] Does this validator rely on cached fields rather than graph structure? (No = good)
- [ ] Does this new validator have both IR unit test + XML integration test? (Yes = good)
- [ ] Are invariants asserted after IR modifications? (Yes = good)

### 2. CI Gates
```yaml
# .github/workflows/ci.yml
- name: Run tests with debug assertions
  run: cargo test --all-features
  env:
    RUSTFLAGS: "-C debug-assertions=on"

- name: Verify NASA-grade tests
  run: |
    cargo test nasa_grade
    # Ensure at least 11 tests run (not all ignored)
```

### 3. Documentation Updates
- New validators must update `CLAUDE.md` with SEM/PRAG code descriptions
- Breaking changes to IR must update this ADR

---

## Compliance Status

### Current Codebase ✅
- [x] Canonical pipeline exists (`compile_pipeline()`)
- [x] All validators are robust (SEM001-007, PRAG001-005)
- [x] Invariant checks at boundaries
- [x] Test DSL (`expect_codes!`, `assert_ir_equivalent`)

### Future Work 📋
- [ ] **XML golden tests for all NASA scenarios** - Current golden tests are AST-based to isolate lowering semantics; XML→AST parser has separate test coverage but exhaustive golden coverage is planned
- [ ] Snapshot testing for complex IR transformations (strict structural isomorphism)
- [ ] API documentation with contract examples
- [ ] XML parser fuzzing for edge cases

---

## References

- **Implementation**: `lib.rs:compile_pipeline()`
- **Invariants**: `src/compiler/ir.rs:assert_invariants()`
- **Review doc**: `ARCHITECTURE_REVIEW.md`
- **Test DSL**: `tests/test_utils.rs`

---

## Revision History

| Date | Change | Reason |
|------|--------|--------|
| 2025-11-29 | Initial version | Formalize compiler safety contract |

---

*"Lock down contracts so they can't break at 3am."*
