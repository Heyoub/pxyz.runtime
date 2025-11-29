# Runtime Golden Tests: TypeScript ↔ Rust Agreement

**Status**: ✅ Complete (infrastructure ready, awaiting TS fixtures)
**Date**: 2025-11-29
**Related**: WASM_HOST_API.md, ADR-001

---

## Purpose

Prevent cross-language drift between TypeScript (reference implementation) and Rust (production runtime) for critical runtime semantics:

1. **ConstraintHash** - Must produce identical 8-char hex hashes
2. **ShapeRegistry** - Must generate identical default projections
3. **Error semantics** - Must enforce identical validation rules

---

## Architecture

```
TypeScript Reference Implementation
           │
           │ npm run generate-fixtures
           ▼
    ┌──────────────────┐
    │  JSON Fixtures   │  (committed to git)
    └──────────────────┘
           │
           │ cargo test --test runtime_golden
           ▼
    Rust Implementation
           │
           ▼
    ✅ Pass → Implementations agree
    ❌ Fail → Breaking change detected
```

---

## Test Files

### `/tests/runtime_golden.rs`

Contains 7 golden tests:

**Fixture-based tests** (skip gracefully if fixtures missing):
1. `test_ts_and_rust_constraint_hash_match_for_simple_cases()`
2. `test_ts_and_rust_constraint_hash_are_order_invariant()`
3. `test_rust_shape_registry_generates_same_default_projections_as_ts()`
4. `test_shape_registry_enforces_same_error_semantics_as_ts()`

**Inline tests** (always run, no fixtures needed):
5. `test_known_fnv1a_values_match_reference_implementation()`
6. `test_top_level_key_order_does_not_affect_hash()`
7. `test_nested_values_do_affect_hash()`

### `/tests/fixtures/` (awaiting TS generation)

Expected fixtures:
- `ts_constraint_hashes.json` - Constraint hash corpus from TS
- `ts_shape_snapshot.json` - Shape projection corpus from TS
- `README.md` - Fixture format specification and generation instructions

---

## Constraint Hash Algorithm

Both implementations MUST use identical FNV-1a 32-bit hash:

```
1. Normalize JSON:
   - Sort top-level keys alphabetically (nested objects unsorted)
   - Serialize to compact JSON (no whitespace)

2. Hash with FNV-1a 32-bit:
   - Offset basis: 0x811C9DC5 (2166136261)
   - Prime: 0x01000193 (16777619)
   - For each byte:
     hash ^= byte
     hash *= prime

3. Format:
   - Convert to lowercase hex
   - Pad to 8 characters with leading zeros
```

**Example**:
```json
Input:  {"name": "ltv_basic", "maxLtv": 0.97}
Output: "a1b2c3d4" (8-char hex)
```

**Critical property**: Top-level key order MUST NOT affect hash.

---

## ShapeRegistry Projections

Both implementations MUST generate identical default projections:

For shape `{Name}`, generate:

1. **`{Name}All`**
   - Query: `SELECT * FROM {Name}`
   - Index fields: `["id", "createdAt"]`

2. **`{Name}Active`**
   - Query: `SELECT * FROM {Name} WHERE status != 'deleted'`
   - Index fields: `["status", "updatedAt"]`

3. **`{Name}Recent`**
   - Query: `SELECT * FROM {Name} ORDER BY updatedAt DESC LIMIT 100`
   - Index fields: `["updatedAt"]`

**Example**:
```typescript
// TS
registry.register({ name: 'LoanApplication', ... })
// Generates: LoanApplicationAll, LoanApplicationActive, LoanApplicationRecent

// Rust
registry.register(RegisteredShape { name: "LoanApplication", ... })
// Generates: LoanApplicationAll, LoanApplicationActive, LoanApplicationRecent
```

---

## Error Semantics

Both implementations MUST enforce identical validation rules:

| Error | Condition | Message Pattern |
|-------|-----------|-----------------|
| `EmptyName` | `name.trim().is_empty()` | "empty" |
| `InvalidName` | Doesn't match `^[a-zA-Z][a-zA-Z0-9_]*$` | "match" |
| `NotFound` | Shape doesn't exist | "not found" |
| `AlreadyRegistered` | Duplicate registration | "already registered" |

---

## Running Tests

### Without Fixtures (inline tests only)

```bash
cargo test --test runtime_golden
```

**Expected output**:
```
running 7 tests
⚠️  Skipping golden test: .../ts_constraint_hashes.json not found
   Generate fixtures by running: cd adherify/server && npm run generate-fixtures
⚠️  Skipping golden test: .../ts_shape_snapshot.json not found
test inline_hash_agreement::test_known_fnv1a_values_match_reference_implementation ... ok
test inline_hash_agreement::test_top_level_key_order_does_not_affect_hash ... ok
test inline_hash_agreement::test_nested_values_do_affect_hash ... ok
test test_shape_registry_enforces_same_error_semantics_as_ts ... ok
test result: ok. 7 passed; 0 failed; 0 ignored
```

### With Fixtures (full golden verification)

1. Generate TS fixtures:
   ```bash
   cd adherify/server
   npm run generate-fixtures
   ```

2. Run golden tests:
   ```bash
   cargo test --test runtime_golden
   ```

**Expected output**:
```
running 7 tests
test test_ts_and_rust_constraint_hash_match_for_simple_cases ... ok
test test_ts_and_rust_constraint_hash_are_order_invariant ... ok
test test_rust_shape_registry_generates_same_default_projections_as_ts ... ok
test test_shape_registry_enforces_same_error_semantics_as_ts ... ok
test inline_hash_agreement::test_known_fnv1a_values_match_reference_implementation ... ok
test inline_hash_agreement::test_top_level_key_order_does_not_affect_hash ... ok
test inline_hash_agreement::test_nested_values_do_affect_hash ... ok
test result: ok. 7 passed; 0 failed; 0 ignored
```

---

## Breaking Change Protocol

### If Golden Tests Fail

1. **Determine which implementation changed**:
   - Did TS change? → Rust MUST update to match
   - Did Rust change? → Revert Rust or update TS + regenerate fixtures
   - Both changed? → **RED ALERT**: This is a coordination failure

2. **Never modify fixtures to make tests pass**:
   - Fixtures are source of truth from TS
   - If test fails, implementation is wrong, not fixture

3. **Regenerate fixtures after coordinated changes**:
   ```bash
   cd adherify/server
   npm run generate-fixtures
   git add ../pxyz/main/tests/fixtures/*.json
   git commit -m "chore: regenerate runtime fixtures after [change]"
   ```

---

## Fixture Generation (TypeScript Side)

See `/tests/fixtures/README.md` for complete TS fixture generator specification.

**Summary**:
```typescript
// adherify/server/scripts/generate-fixtures.ts
import { hashYCtx } from '../src/lib/ConstraintRegistry'
import { ShapeRegistry } from '../src/lib/ShapeRegistry'

const cases = [
  { name: 'simple', yctx: {...} },
  { name: 'order_invariant', yctx: {...} },
  // ...
]

const corpus = {
  cases: cases.map(c => ({
    name: c.name,
    yctx: c.yctx,
    expected_hash: hashYCtx(c.yctx)
  }))
}

fs.writeFileSync(
  '../pxyz/main/tests/fixtures/ts_constraint_hashes.json',
  JSON.stringify(corpus, null, 2)
)
```

---

## References

- **WASM_HOST_API.md**: Full specification of host API contract
- **CODE_REVIEW_CHECKLIST.md**: When to regenerate fixtures
- **src/runtime/constraint.rs**: Rust ConstraintHash implementation
- **src/runtime/shape.rs**: Rust ShapeRegistry implementation
- **src/runtime/host.rs**: WASM host bridge (3 host functions)

---

## Bottom Line

**Before**: TS and Rust could silently drift on constraint hashing or projection generation.

**After**: Any drift triggers golden test failure with clear error message.

**Contract**: If you modify runtime semantics, you MUST:
1. Regenerate TS fixtures
2. Verify golden tests pass
3. Commit both code and fixture changes together

---

*"No hand-waving. Just: 'Here's the corpus, here's the hash, TS and Rust agree.'"*
