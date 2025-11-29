# PXYZ Compiler & Runtime Architecture

> **Purpose**: Complete reference for the PXYZ/OMAR workflow compiler and runtime system.

---

## Overview

PXYZ is a workflow compiler that transforms XML-defined business logic into a compact binary format (`graph.bin`) that can be executed by a minimal runtime. The system is designed for:

- **Auditability**: ~700 lines of auditable runtime code
- **Safety**: Compile-time constraint validation, bounded execution
- **Portability**: Binary format runs on WASM, native, or any platform

---

## Architecture

```mermaid
workflow.xml → [Compiler] → graph.bin → [Runtime] → Execution
                   ↓
            graph.audit.json
```

### Compilation Pipeline

```mermaid
XML → Parse → AST → Lower → IR → Validate → Optimize → Emit
                                    ↓
                    [Syntactic, Semantic, Pragmatic checks]
```

---

## Binary Format (graph.bin)

### Header (96 bytes)

| Offset | Size | Field |
|--------|------|-------|
| 0x00 | 4 | Magic: `0x504E5958` ("PXYZ") |
| 0x04 | 2 | Version major |
| 0x06 | 2 | Version minor |
| 0x08 | 4 | Node count |
| 0x0C | 4 | Edge count |
| 0x10 | 4 | Predicate count |
| 0x14 | 4 | String pool size |
| 0x18 | 4 | Entry count |
| 0x20 | 32 | Source hash (SHA-256) |
| 0x40 | 4 | Nodes offset |
| 0x44 | 4 | Edges offset |
| 0x48 | 4 | Predicates offset |
| 0x4C | 4 | Strings offset |
| 0x50 | 4 | Entries offset |

### Node Entry (16 bytes)

| Offset | Size | Field |
|--------|------|-------|
| 0x00 | 4 | Node ID |
| 0x04 | 1 | Kind (0-6) |
| 0x05 | 1 | Flags |
| 0x06 | 2 | Op code |
| 0x08 | 4 | Data offset (string pool) |
| 0x0C | 2 | Edge start index |
| 0x0E | 2 | Edge count |

### Edge Entry (12 bytes)

| Offset | Size | Field |
|--------|------|-------|
| 0x00 | 4 | Target node ID |
| 0x04 | 2 | Predicate ID (0 = always) |
| 0x06 | 2 | (reserved) |
| 0x08 | 2 | Weight |
| 0x0A | 2 | Flags |

### Entry Point (8 bytes)

| Offset | Size | Field |
|--------|------|-------|
| 0x00 | 4 | PX hash (FNV-1a of P+X) |
| 0x04 | 4 | Node ID |

---

## Node Kinds

| Value | Kind | Purpose |
|-------|------|---------|
| 0 | Transform | Validate/transform data |
| 1 | External | Call IO handler (op code) |
| 2 | Render | Generate HTML output |
| 3 | Signal | Emit UI signals |
| 4 | Auth | Check predicate, fail if false |
| 5 | Terminal | End traversal |
| 6 | Error | Error handling node |

---

## Predicate VM

Stack-based bytecode interpreter with bounded execution.

### Opcodes

| Code | Name | Stack Effect | Description |
|------|------|--------------|-------------|
| 0x00 | NOOP | - | No operation |
| 0x01 | PUSH_INT | → val | Push i32 (4 bytes follow) |
| 0x02 | PUSH_STR | → val | Push string (4-byte offset) |
| 0x03 | LOAD_VAR | → val | Load from context (4-byte path offset) |
| 0x04 | LOAD_FIELD | obj → val | Get field from object |
| 0x10 | EQ | a, b → bool | Equal |
| 0x11 | NEQ | a, b → bool | Not equal |
| 0x12 | GT | a, b → bool | Greater than |
| 0x13 | GTE | a, b → bool | Greater or equal |
| 0x14 | LT | a, b → bool | Less than |
| 0x15 | LTE | a, b → bool | Less or equal |
| 0x20 | AND | a, b → bool | Logical and |
| 0x21 | OR | a, b → bool | Logical or |
| 0x22 | NOT | a → bool | Logical not |
| 0x30 | CONTAINS | haystack, needle → bool | String/array contains |
| 0x31 | MATCHES | str → bool | Pattern match (4-byte pattern offset) |
| 0x32 | STARTS_WITH | str, prefix → bool | String prefix |
| 0x33 | ENDS_WITH | str, suffix → bool | String suffix |
| 0x40 | LEN | val → int | Length of string/array |
| 0x41 | GET | arr, idx → val | Array index |
| 0x42 | IS_NULL | val → bool | Check null |
| 0x43 | IS_DEFINED | val → bool | Check defined |
| 0xF0 | CALL_PRED | → bool | Call predicate (2-byte ID) |
| 0xFF | RET | bool → | Return result |

### Safety Limits

| Limit | Value | Purpose |
|-------|-------|---------|
| MAX_PREDICATE_STEPS | 256 | Prevent infinite loops |
| MAX_STACK_DEPTH | 16 | Prevent stack overflow |
| MAX_CALL_DEPTH | 4 | Prevent infinite recursion |
| MAX_PREDICATE_BYTECODE | 256 bytes | Limit predicate size |
| MAX_VISITED_NODES | 1000 | Prevent runaway traversal |

---

## IO Operation Codes

### Entity (0x01xx)

- `0x0100` ENTITY_CREATE
- `0x0101` ENTITY_READ
- `0x0102` ENTITY_UPDATE
- `0x0103` ENTITY_DELETE
- `0x0104` ENTITY_LIST
- `0x0105` ENTITY_SEARCH

### Google Workspace (0x03xx)

- `0x0300` GOOGLE_CONTACTS_SEARCH
- `0x0301` GOOGLE_CONTACTS_GET
- `0x0302` GOOGLE_CONTACTS_CREATE
- `0x0310` GOOGLE_CALENDAR_LIST
- `0x0320` GOOGLE_DRIVE_SEARCH
- `0x0330` GOOGLE_GMAIL_SEARCH
- `0x0332` GOOGLE_GMAIL_SEND ⚠️ IRREVERSIBLE

### Communication - IRREVERSIBLE (0x034x-0x036x)

- `0x0340` EMAIL_SEND ⚠️
- `0x0350` SMS_SEND ⚠️
- `0x0360` WEBHOOK_CALL ⚠️

### HTTP (0x04xx)

- `0x0400` HTTP_GET
- `0x0401` HTTP_POST
- `0x0402` HTTP_PUT
- `0x0403` HTTP_DELETE

### Vector/RAG (0x07xx)

- `0x0700` QDRANT_SEARCH
- `0x0701` QDRANT_INDEX
- `0x0702` EMBEDDING_GENERATE

### AI/LLM (0x08xx)

- `0x0800` LLM_COMPLETE
- `0x0801` LLM_CLASSIFY
- `0x0802` LLM_STRUCTURED
- `0x0810` LOCAL_MODEL_RUN

### Storage (0x09xx)

- `0x0900` STORAGE_GET
- `0x0901` STORAGE_SET
- `0x0910` EVENT_LOG_APPEND
- `0x0911` EVENT_LOG_QUERY

---

## Constraint System

### Syntactic (SYN) - Structure validation

| Code | Check |
|------|-------|
| SYN001 | Edge targets exist |
| SYN002 | Entry points reference existing nodes |
| SYN003 | Predicate references exist |
| SYN004 | No duplicate node IDs |
| SYN005 | At least one entry point |
| SYN006 | No duplicate entry points |
| SYN007 | Edge sources exist |

### Semantic (SEM) - Logic validation

| Code | Check |
|------|-------|
| SEM001 | Auth nodes have predicates |
| SEM002 | External nodes have op codes |
| SEM003 | Terminal nodes shouldn't have outgoing edges |
| SEM004 | No cycles in graph |
| SEM005 | All nodes reachable from entry |
| SEM006 | Error nodes have incoming edges |
| SEM007 | Render nodes have templates |

### Pragmatic (PRAG) - Business rules

| Code | Check |
|------|-------|
| PRAG001 | LLM → Irreversible requires validation gate |
| PRAG002 | Write operations should have error branches |
| PRAG003 | Irreversible actions require human in path |
| PRAG004 | Irreversible actions require confirmed inputs |
| PRAG005 | Quarantined data cannot escape to external |

---

## Node Flags (Bitfield)

| Bit | Flag | Meaning |
|-----|------|---------|
| 0 | ASYNC | Node may block |
| 1 | REQUIRES_AUTH | Must pass auth predicate |
| 2 | HAS_SIDE_EFFECTS | Produces side effects |
| 3 | IRREVERSIBLE | Cannot be undone |
| 4 | REQUIRES_HUMAN | Human actor required |
| 5 | CACHEABLE | Result can be memoized |

## Edge Flags (Bitfield)

| Bit | Flag | Meaning |
|-----|------|---------|
| 0 | PARALLEL | Can execute with siblings |
| 1 | FALLBACK | Only if others fail |
| 2 | ERROR_EDGE | Leads to error node |

---

## XML DSL Structure

```xml
<?xml version="1.0" encoding="UTF-8"?>
<omar version="1.0.0">
  
  <!-- Type definitions -->
  <schemas>
    <schema name="Contact">
      <field name="id" type="uuid" required="true"/>
      <field name="email" type="string" required="true" pattern="^[^@]+@[^@]+$"/>
    </schema>
  </schemas>
  
  <!-- Reusable predicates -->
  <predicates>
    <predicate id="is_admin">
      <contains left="$token.perms" right="admin"/>
    </predicate>
    
    <predicate id="can_write">
      <or>
        <ref predicate="is_admin"/>
        <eq left="$entity.owner_id" right="$token.sub"/>
      </or>
    </predicate>
  </predicates>
  
  <!-- Workflow definition -->
  <workflow id="contact_search">
    <entry p="contact" x="search" node="validate"/>
    
    <nodes>
      <node id="validate" kind="transform">
        <schema ref="SearchQuery"/>
      </node>
      
      <node id="auth" kind="auth">
        <require predicate="can_write"/>
      </node>
      
      <node id="search" kind="external" op="0x0300"/>
      
      <node id="render" kind="render">
        <template ref="contact_list"/>
        <selector>#content</selector>
      </node>
      
      <node id="done" kind="terminal" status="200"/>
      
      <node id="error" kind="error" status="500"/>
    </nodes>
    
    <edges>
      <edge from="validate" to="auth">
        <when><always/></when>
      </edge>
      <edge from="validate" to="error" fallback="true">
        <when><fail/></when>
      </edge>
      <edge from="auth" to="search">
        <when><always/></when>
      </edge>
      <edge from="search" to="render"/>
      <edge from="render" to="done"/>
    </edges>
  </workflow>
  
  <!-- HTML templates -->
  <templates>
    <template id="contact_list">
      <![CDATA[
      <div class="contact-list">
        {{#each contacts}}
        <div class="card">{{name}}</div>
        {{/each}}
      </div>
      ]]>
    </template>
  </templates>
  
</omar>
```

---

## Predicate Expressions

| Element | Attributes | Description |
|---------|------------|-------------|
| `<always/>` | - | Always true |
| `<fail/>` | - | Always false |
| `<eq>` | left, right | Equality |
| `<neq>` | left, right | Not equal |
| `<gt>` | left, right | Greater than |
| `<gte>` | left, right | Greater or equal |
| `<lt>` | left, right | Less than |
| `<lte>` | left, right | Less or equal |
| `<contains>` | left, right | Contains substring/element |
| `<matches>` | left, pattern | Regex match |
| `<and>` | (children) | All conditions true |
| `<or>` | (children) | Any condition true |
| `<not>` | (child) | Negate condition |
| `<ref>` | predicate | Reference named predicate |

### Variable Paths

- `$token.sub` - Token subject (user ID)
- `$token.perms` - Token permissions array
- `$token.tenant` - Tenant ID
- `$entity.owner_id` - Entity owner
- `$entity.status` - Entity status
- `$input.query` - Input field
- `$state.node_id.field` - State from previous node

---

## CLI Commands

```bash
# Compile workflow
pxyz compile -i workflow.xml -o graph.bin [--audit] [--strict]

# Inspect binary
pxyz inspect -i graph.bin --format text|json|mermaid

# Validate only
pxyz check -i workflow.xml

# Create new project
pxyz init myproject
```

---

## Crate Structure

```shell
pxyz/
├── CLAUDE.md               # This documentation
├── MENTAL_MODEL.md         # Design philosophy
├── README.md               # Project overview
│
├── main/                   # Rust compiler crate
│   ├── Cargo.toml          # Dependencies
│   ├── lib.rs              # Public API: compile(), validate()
│   ├── main.rs             # CLI entry point
│   │
│   └── src/
│       ├── compiler/       # Core compilation & validation
│       │   ├── mod.rs      # Module exports
│       │   ├── bytecode.rs # Predicate AST → bytecode
│       │   ├── ir.rs       # GraphIR, GNode, GEdge, StringPool
│       │   ├── lower.rs    # AST → IR transformation
│       │   ├── optimize.rs # Dead code elimination, dedup
│       │   ├── syntactic.rs # SYN001-007 structure checks
│       │   ├── semantic.rs  # SEM001-007 logic checks
│       │   └── pragmatic.rs # PRAG001-005 business rules
│       │
│       ├── dsl/            # XML parsing
│       │   ├── mod.rs      # Module exports
│       │   ├── ast.rs      # AST types
│       │   └── parser.rs   # XML → AST
│       │
│       └── emit/           # Output generation
│           ├── mod.rs      # Module exports
│           ├── binary.rs   # IR → graph.bin
│           ├── audit.rs    # IR → audit.json
│           ├── text.rs     # Human-readable output
│           └── mermaid.rs  # Flowchart diagram
│
└── wasm/                   # WASM runtime (production)
    └── pxyz.wat            # ~500 lines auditable runtime
```

---

## Key Design Principles

1. **Business logic is data** - Workflows compile to traversable graphs, not code
2. **Predicates are bounded** - VM has step limits, no unbounded loops
3. **All IO is explicit** - External nodes with op codes, no hidden side effects
4. **Safety at compile time** - Pragmatic constraints catch dangerous patterns
5. **Human gates for irreversible** - PRAG003/004 enforce confirmation flows

---

## Hash Function (Entry Point Lookup)

FNV-1a hash combining P and X coordinates:

```rust
fn hash_px(p: &str, x: &str) -> u32 {
    const FNV_PRIME: u32 = 16777619;
    const FNV_OFFSET: u32 = 2166136261;
    
    let mut hash = FNV_OFFSET;
    for byte in p.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash ^= 0xFF; // separator
    hash = hash.wrapping_mul(FNV_PRIME);
    for byte in x.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
```

---

## Runtime Execution Flow

1. **Load graph.bin** - Parse header, build node/edge/predicate tables
2. **Find entry** - Hash (P, X), lookup in entry table
3. **Traverse** - Visit nodes, evaluate edge predicates, follow matches
4. **Execute nodes** - Transform validates, External calls IO, Auth checks predicate
5. **Terminate** - Hit Terminal/Error node, return result

---

## Safety Guarantees

- **Traversal terminates**: MAX_VISITED_NODES + cycle detection
- **Predicates terminate**: MAX_PREDICATE_STEPS + MAX_CALL_DEPTH
- **No unbounded allocation**: Fixed stack size
- **Explicit IO boundary**: All side effects through IoHandler
- **Compile-time safety**: Pragmatic constraints block dangerous patterns

---

## WASM Runtime (pxyz.wat)

The core runtime is ~500 lines of WebAssembly Text format, designed to be:

- **Auditable**: Small enough to read in one sitting
- **Formally verifiable**: No hidden complexity
- **Sandboxed**: WASM memory isolation

### Memory Layout

```shell
0x0000 - 0x0FFF: VM Stack (4KB)
0x1000 - 0x1FFF: Execution state (visited bitmap, etc)
0x2000 - onwards: Graph data (loaded from graph.bin)
```

### Host Imports

```wat
(import "host" "io_call" (func $io_call (param i32 i32 i32) (result i32)))
(import "host" "log" (func $log (param i32 i32 i32)))
```

The runtime makes **zero** direct system calls. All IO goes through `io_call` which the host implements.

### Exports

```wat
(export "load_graph")    ;; (ptr, len) -> success
(export "execute")       ;; (p_ptr, p_len, x_ptr, x_len, ctx_ptr, ctx_len) -> status
(export "get_result_ptr")
(export "get_result_len")
(export "get_visit_count")
(export "memory")
```

### Browser Usage

```javascript
// Load WASM
const wasmBytes = await fetch('pxyz.wasm').then(r => r.arrayBuffer());
const imports = {
  host: {
    io_call: (op, inputPtr, inputLen) => {
      // Dispatch to appropriate handler
      return handleIO(op, readMemory(inputPtr, inputLen));
    },
    log: (level, msgPtr, msgLen) => {
      console.log(readString(msgPtr, msgLen));
    }
  }
};
const { instance } = await WebAssembly.instantiate(wasmBytes, imports);

// Load graph
const graphBin = await fetch('graph.bin').then(r => r.arrayBuffer());
const graphPtr = allocate(graphBin.byteLength);
new Uint8Array(instance.exports.memory.buffer, graphPtr, graphBin.byteLength)
  .set(new Uint8Array(graphBin));
instance.exports.load_graph(graphPtr, graphBin.byteLength);

// Execute
const p = encodeString('contact');
const x = encodeString('search');
const ctx = encodeString('{"token":{"sub":"user123"}}');
const status = instance.exports.execute(p.ptr, p.len, x.ptr, x.len, ctx.ptr, ctx.len);
```

### Compiling WAT to WASM

```bash
# Using wat2wasm from WABT
wat2wasm pxyz.wat -o pxyz.wasm

# Or using wasm-tools
wasm-tools parse pxyz.wat -o pxyz.wasm
```

---

## Two-Runtime Architecture

The PXYZ system has **two implementations**:

| Component | Language | Purpose | Lines |
|-----------|----------|---------|-------|
| **Compiler** | Rust | XML → graph.bin | ~3000 |
| **Runtime (Native)** | Rust | Development/testing | ~1500 |
| **Runtime (WASM)** | WAT | Production/audit | ~500 |

### Why Two Runtimes?

1. **Rust runtime**: Full-featured, easy to debug, runs tests
2. **WAT runtime**: Minimal, auditable, production deployment

The graph.bin format is identical - compile once, run on either.

### Build Pipeline

```mermaid
workflow.xml
     │
     ▼
┌─────────────────┐
│  Rust Compiler  │  (pxyz compile)
└────────┬────────┘
         │
         ▼
    graph.bin ─────────────────────────┐
         │                             │
         ▼                             ▼
┌─────────────────┐           ┌─────────────────┐
│  Rust Runtime   │           │  WASM Runtime   │
│  (development)  │           │  (production)   │
└─────────────────┘           └─────────────────┘
```

---

## IO Adapter Pattern

Both runtimes use the same IO adapter pattern:

```mermaid
┌──────────────────────────────────────────┐
│              PXYZ Runtime                │
│  (pure graph traversal + predicates)     │
└────────────────┬─────────────────────────┘
                 │ io_call(op_code, input)
                 ▼
┌──────────────────────────────────────────┐
│              IO Adapter                  │
│  Browser: fetch, IndexedDB, WebSocket    │
│  Node: fs, http, child_process           │
│  Native: system calls                    │
└──────────────────────────────────────────┘
```

### Browser IO Adapter (~200 lines)

```typescript
const handlers: Record<number, (input: any) => Promise<any>> = {
  // Google
  0x0300: (input) => gapi.client.people.searchContacts(input),
  0x0310: (input) => gapi.client.calendar.events.list(input),
  
  // HTTP
  0x0400: (input) => fetch(input.url).then(r => r.json()),
  0x0401: (input) => fetch(input.url, { method: 'POST', body: JSON.stringify(input.body) }),
  
  // Storage
  0x0900: (input) => localforage.getItem(input.key),
  0x0901: (input) => localforage.setItem(input.key, input.value),
  
  // LLM
  0x0800: (input) => fetch('/api/llm', { method: 'POST', body: JSON.stringify(input) }),
};

export function ioCall(op: number, input: any): Promise<any> {
  const handler = handlers[op];
  if (!handler) throw new Error(`Unknown op: 0x${op.toString(16)}`);
  return handler(input);
}
```
