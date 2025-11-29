#WASM Host API Specification

**Status**: Canonical contract for GraphIR ↔ Runtime bridge
**Version**: 1.0.0
**Date**: 2025-11-29

---

## Purpose

This document defines the **exact** interface between GraphIR WASM bytecode and the host runtime environment. Both TypeScript (Effect-TS) and Rust implementations MUST provide these 3 host functions with identical JSON semantics.

---

## Design Principles

1. **Minimal Surface**: Only 3 host imports - no arbitrary "do anything" calls
2. **JSON Envelopes**: All data passes as UTF-8 JSON for cross-language compatibility
3. **Side-Effect Gates**: All GraphIR side effects flow through these 3 functions
4. **Status Codes**: Simple `i32` return (0 = success, non-zero = error)
5. **Golden Tested**: TS and Rust agree on hash algorithms, projection generation, error semantics

---

## WAT Import Signatures

```wat
;; Emit compliance/audit event to PXYZStream
(import "host" "host_emit_event"
  (func $host_emit_event (param i32 i32) (result i32)))

;; Validate data against registered shape
(import "host" "host_validate_shape"
  (func $host_validate_shape (param i32 i32) (result i32)))

;; Compute canonical constraint hash
(import "host" "host_constraint_hash"
  (func $host_constraint_hash (param i32 i32) (result i32)))
```

**Parameters**:
- `param1 i32`: Pointer to UTF-8 JSON string in WASM linear memory
- `param2 i32`: Length of JSON string in bytes

**Return**:
- `0` = success
- Non-zero = error (specific error logged/persisted by host, GraphIR treats as branch failure)

---

## 1. host_emit_event

### Purpose
Emit compliance or audit events into the PXYZStream event sink.

### JSON Envelope
```json
{
  "pxyz": {
    "P": "LoanApplication",
    "X": "create",
    "Y": "ltv_basic",
    "Z": "2025-01-01T00:00:00Z"
  },
  "payload": {
    "loanAmount": 500000,
    "ltv": 0.80,
    "... custom fields ..."
  }
}
```

### Fields
- **pxyz** (required): PXYZ coordinate with P,X,Y,Z fields
  - `P`: Entity/Domain name (e.g., "LoanApplication", "Contact")
  - `X`: Operation name (e.g., "create", "update", "search")
  - `Y`: Constraint/Policy name (e.g., "ltv_basic", "compliance_check")
  - `Z`: ISO 8601 timestamp
- **payload** (optional): Arbitrary JSON object with event data

### Error Conditions
- Missing `pxyz` field → `JsonDecode` error
- Invalid PXYZ structure (missing P/X/Y/Z) → `JsonDecode` error
- Event sink failure (Kafka down, disk full, etc.) → `EventSink` error

### Semantics
- Events MUST be append-only (no mutation of prior events)
- If `encryption_enabled = true` in config, host MUST sign payload before persistence
- Events SHOULD be durably persisted (Kafka, database, log file)

---

## 2. host_validate_shape

### Purpose
Validate data against a registered shape definition (P-dimension type).

### JSON Envelope
```json
{
  "shapeName": "LoanApplication",
  "data": {
    "loanAmount": 500000,
    "ltv": 0.80,
    "borrowerIncome": 120000
  }
}
```

### Fields
- **shapeName** (required): Name of registered shape (must match `^[a-zA-Z][a-zA-Z0-9_]*$`)
- **data** (required): JSON object to validate against shape schema

### Return Value (on success)
JSON object (currently `null`, future: normalized/validated data)

### Error Conditions
- Missing `shapeName` field → `JsonDecode` error
- Shape not found → `Shape(NotFound)` error
- Invalid shape name format → `Shape(InvalidName)` error
- Validation failure → `Shape(Validation)` error

### Semantics
- Shape MUST exist in ShapeRegistry before validation
- Validation logic is pluggable (currently stubbed, future: full schema validation)
- Host SHOULD cache shape definitions for performance

---

## 3. host_constraint_hash

### Purpose
Compute canonical hash of a YCtx (constraint context) for deduplication and indexing.

### JSON Envelope
```json
{
  "yctx": {
    "name": "ltv_basic",
    "maxLtv": 0.97,
    "minCreditScore": 620
  }
}
```

### Fields
- **yctx** (required): JSON object representing constraint context

### Return Value (on success)
8-character lowercase hex string (e.g., `"a1b2c3d4"`)

### Error Conditions
- Missing `yctx` field → `JsonDecode` error

### Algorithm (MUST match across languages)

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

**Critical**: This algorithm MUST produce identical hashes for:
- TypeScript implementation (ConstraintRegistry.ts)
- Rust implementation (runtime/constraint.rs)
- Any future implementations

---

## Implementation Requirements

### TypeScript (Effect-TS)

Must implement via Effect Layers:
- `EventBusProduction` → `host_emit_event`
- `ShapeRegistryService` → `host_validate_shape`
- `ConstraintRegistry.hashYCtx()` → `host_constraint_hash`

### Rust

Must implement via `HostEnv<R, E>`:
- `EventSink trait` → `host_emit_event`
- `ShapeRegistryLike trait` → `host_validate_shape`
- `hash_yctx()` function → `host_constraint_hash`

### WASM Runtime

Must expose these 3 imports and NO others for side effects. All other GraphIR operations (graph traversal, predicate evaluation, node execution) happen inside WASM sandbox.

---

## Golden Test Requirements

Every implementation MUST pass golden tests proving:

1. **ConstraintHash equivalence**:
   - Same YCtx → same 8-char hex hash
   - Order-invariant (top-level keys only)
   - Matches TypeScript reference implementation

2. **ShapeRegistry semantics**:
   - Default projections: `{Name}All`, `{Name}Active`, `{Name}Recent`
   - Projection queries and index fields match TS
   - Error semantics (EmptyName, InvalidName, AlreadyRegistered, NotFound)

3. **JSON envelope parsing**:
   - All 3 host functions correctly extract fields
   - Missing fields trigger appropriate errors
   - Invalid JSON triggers `JsonDecode` errors

---

## Security Considerations

1. **Memory Safety**: Host MUST validate pointer/length before reading WASM memory
2. **Size Limits**: JSON payloads SHOULD be capped (e.g., 1MB max) to prevent DoS
3. **Sanitization**: Payloads containing PII MUST respect compliance config
4. **Isolation**: Host functions MUST NOT expose arbitrary system access to WASM

---

## Version Compatibility

This is version 1.0.0 of the host API. Breaking changes require:
1. Major version bump
2. Migration guide for existing bytecode
3. Dual-version support period (e.g., v1 + v2 for 6 months)

Non-breaking additions (e.g., optional fields) can use minor version bumps.

---

## Cross-Reference

- **Rust implementation**: `src/runtime/host.rs`
- **TypeScript reference**: `adherify/server/src/lib/{EventBus,ShapeRegistry,ConstraintRegistry}.ts`
- **Golden tests**: `tests/runtime_golden.rs` (planned)
- **ADR-001**: GraphIR contract (compiler side)
- **ADR-002**: Runtime services (this spec)

---

*"Three functions. Three JSON envelopes. No backdoors."*
