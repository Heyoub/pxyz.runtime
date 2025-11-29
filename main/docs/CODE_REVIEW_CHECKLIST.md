# Code Review Checklist: Compiler & Safety

Use this checklist for PRs touching compiler code, validators, or IR structures.

---

## 🔒 Safety Contract (ADR-001)

### Pipeline Integrity
- [ ] **Does this code call `lower()` or `validate()` directly?**
  - ✅ Good: Uses `compile_pipeline()`
  - ❌ Bad: Bypasses canonical pipeline
  - Exception: Unit tests may construct IR directly if they call `assign_edge_indices()`

- [ ] **If this modifies IR, does it assert invariants after?**
  ```rust
  #[cfg(debug_assertions)]
  ir.assert_invariants();
  ```
  - Required after: lowering, optimization, node/edge removal
  - Location: End of function that modifies IR structure

### Validator Robustness
- [ ] **Does this validator compute from graph structure?**
  - ✅ Good: `ir.edges.iter().filter(|e| e.from == node_id)`
  - ❌ Bad: Relies solely on `node.edge_count` or `node.edge_start`
  - Why: Cached fields might not be set if pipeline is bypassed

- [ ] **If this uses `get_edges_from()` or `get_edges_to()`, does it handle empty results?**
  - These methods iterate dynamically, so they're safe
  - But check: what if a node has zero edges? Does logic still work?

### Test Coverage
- [ ] **Does every new validator have BOTH tests?**
  - [ ] Unit test (direct IR construction)
  - [ ] Integration test (XML fixture via `expect_codes!`)

  Example:
  ```rust
  // Unit test
  #[test]
  fn test_sem999_direct_ir() {
      let mut ir = make_test_ir();
      ir.assign_edge_indices(); // Don't forget!
      let diags = check(&ir);
      assert!(diags.iter().any(|d| d.code == "SEM999"));
  }

  // Integration test
  #[test]
  fn test_sem999_xml_fixture() {
      expect_codes!("fixtures/sem999.xml" => ["SEM999"]);
  }
  ```

- [ ] **Are XML fixtures checked into `tests/fixtures/`?**
  - Fixtures should be minimal (< 50 lines)
  - File name should match error code: `sem004_cycle.xml`

### Cross-Language Golden Tests
- [ ] **If this modifies runtime semantics (ConstraintHash, ShapeRegistry), did you regenerate TS fixtures?**
  ```bash
  cd adherify/server
  npm run generate-fixtures
  ```
  - Generates: `ts_constraint_hashes.json`, `ts_shape_snapshot.json`
  - Location: `pxyz/main/tests/fixtures/`
  - Test file: `tests/runtime_golden.rs`

- [ ] **Do the 4 golden tests still pass?**
  ```bash
  cargo test --test runtime_golden
  ```
  - ✅ Constraint hash basic cases
  - ✅ Constraint hash order invariance
  - ✅ ShapeRegistry default projections
  - ✅ ShapeRegistry error semantics

- [ ] **If golden tests fail, is it a breaking change?**
  - TS changed → Rust MUST match TS
  - Rust changed → Revert or update TS + fixtures
  - Both changed → RED ALERT: coordinate the change

---

## 📐 Code Quality

### Documentation
- [ ] **Are function contracts documented?**
  - What does this function require? (preconditions)
  - What does it guarantee? (postconditions)
  - Example:
    ```rust
    /// Emit graph.bin
    ///
    /// REQUIRES: IR with invariants satisfied (assign_edge_indices called)
    /// GUARANTEES: Valid binary format with SHA-256 hash
    pub fn emit(ir: &GraphIR, source_xml: &str) -> Result<Vec<u8>, CompileError>
    ```

- [ ] **If this adds a new SEM/PRAG code, is it documented in `CLAUDE.md`?**
  - Section: "Constraint System"
  - Include: code, description, severity

### Naming & Clarity
- [ ] **Do function names reveal intent?**
  - ✅ `reassign_edge_indices_preserving_order()` - clear why it exists
  - ❌ `reassign_edge_indices()` - looks like duplicate of `ir.assign_edge_indices()`

- [ ] **Are "weird" design choices explained with loud comments?**
  - If code looks like duplication but isn't, say so:
    ```rust
    /// This is different from `ir.assign_edge_indices()` which sorts edges.
    /// The optimizer has already sorted edges by weight, so we preserve order.
    ```

---

## 🧪 Testing

### Debug Assertions
- [ ] **Are tests run with debug assertions enabled?**
  ```bash
  RUSTFLAGS="-C debug-assertions=on" cargo test
  ```
  - CI should always run this
  - Catches invariant violations that release builds skip

### Edge Cases
- [ ] **Does this handle empty graphs?**
  - Zero nodes? Zero edges? Zero entries?

- [ ] **Does this handle disconnected components?**
  - Node reachable from one entry but not another?

- [ ] **Does this handle self-loops or parallel edges?**
  - Graph structure should prevent these, but validators should be safe

---

## 🚀 Performance

### Optimization Correctness
- [ ] **If this optimizes IR, does it preserve semantics?**
  - Dead code elimination: removes unreachable nodes
  - Predicate dedup: only merges identical bytecode
  - Edge ordering: only changes traversal order, not reachability

- [ ] **Are optimization stats tested?**
  ```rust
  let stats = compute_stats(&before, &after);
  assert_eq!(stats.nodes_removed, expected_count);
  ```

### Binary Size
- [ ] **Does this change binary format?**
  - If yes: Update `CLAUDE.md` binary format table
  - If yes: Bump `VERSION_MINOR` in `lib.rs`
  - If yes: Add migration test (old format → new format)

---

## 🔧 Common Mistakes

### ❌ Constructing IR without indices
```rust
// BAD
let mut ir = GraphIR::new();
ir.nodes.push(...);
ir.edges.push(...);
let diags = semantic::check(&ir); // edge_start/edge_count not set!
```

```rust
// GOOD
let mut ir = GraphIR::new();
ir.nodes.push(...);
ir.edges.push(...);
ir.assign_edge_indices(); // ← Must call this!
#[cfg(debug_assertions)]
ir.assert_invariants();
let diags = semantic::check(&ir);
```

### ❌ Depending on cached state
```rust
// BAD
if node.edge_count > 0 {
    // Fragile: edge_count might not be set
}
```

```rust
// GOOD
let has_outgoing = ir.edges.iter().any(|e| e.from == node.id);
if has_outgoing {
    // Robust: computes from source
}
```

### ❌ Skipping invariant checks
```rust
// BAD
pub fn optimize(mut ir: GraphIR) -> GraphIR {
    ir.nodes.retain(|n| reachable.contains(&n.id));
    ir // No invariant check!
}
```

```rust
// GOOD
pub fn optimize(mut ir: GraphIR) -> GraphIR {
    ir.nodes.retain(|n| reachable.contains(&n.id));
    ir.assign_edge_indices();
    #[cfg(debug_assertions)]
    ir.assert_invariants(); // ← Catch bugs early
    ir
}
```

---

## ✅ Approval Criteria

**Minimum requirements for merge**:

1. All tests pass (unit + integration + NASA-grade)
2. Debug assertions enabled in at least one CI job
3. New validators have both IR and XML tests
4. IR modifications followed by invariant checks
5. No bypasses of canonical pipeline (unless justified)

**Optional but recommended**:
- Snapshot tests for complex transformations
- Fuzzing for parser changes
- Benchmark regression tests for optimizer

---

## 📚 References

- **ADR-001**: GraphIR contract and canonical pipeline
- **ARCHITECTURE_REVIEW.md**: Recent audit findings
- **CLAUDE.md**: Full system documentation

---

*Copy this checklist into PR descriptions and check off items as you review.*
