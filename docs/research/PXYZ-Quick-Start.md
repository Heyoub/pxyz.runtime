# PXYZ Quick Start Guide

## What is PXYZ?

PXYZ is a **workflow compiler and runtime system** designed to safely grant AI agents access to production systems. Unlike traditional imperative programming, PXYZ treats business logic as **data to be interpreted**, not code to be executed.

### Core Philosophy

- **Business logic is data**: Workflows are finite, auditable graphs of operations
- **Predicates are bounded**: No unbounded loops or recursion; guaranteed termination
- **All IO is explicit**: Every external interaction is declared and auditable
- **Safety at compile time**: Dangerous patterns are caught before deployment
- **Human gates for irreversible actions**: Critical actions require human approval
- **Auditable runtime**: ~600 lines of WebAssembly for complete transparency

---

## System Architecture

PXYZ consists of three components:

### 1. Rust Compiler (~1500 LOC)
Transforms your XML workflow definitions into `graph.bin`:
- Parses XML into Abstract Syntax Tree (AST)
- Lowers AST to Graph Intermediate Representation (IR)
- Compiles predicates to bytecode
- Validates with 3-layer constraint system
- Optimizes for efficiency
- Emits portable binary artifact

### 2. WASM Runtime (~600 LOC)
Executes `graph.bin` in a sandboxed environment:
- Written in WebAssembly Text (.wat) for auditability
- Completely isolated with no direct system access
- Enforces strict execution limits (1000 visited nodes, 256 predicate steps)
- Embedded Predicate VM for safe condition evaluation
- All side effects mediated through host imports

### 3. JavaScript Host
Provides the bridge to the outside world:
- Implements host imports (`io_call`, `io_resolve_var`, etc.)
- Handles all external I/O operations
- Manages variable resolution from application context
- Returns responses to the sandboxed runtime

---

## Workflow Definition (XML)

### Document Structure

```xml
<omar>
  <schemas><!-- Data structures and validation rules --></schemas>
  <predicates><!-- Reusable boolean conditions --></predicates>
  <workflow><!-- Executable graph definition --></workflow>
  <templates><!-- Reusable content templates --></templates>
</omar>
```

### Nodes

Nodes are units of work or control points. Each has a **kind** that determines its behavior:

| Kind | Purpose | Example |
|------|---------|---------|
| **Transform** | Validate/transform data (no side effects) | Data validation, field mapping |
| **External** | Call external I/O handler | API calls, database operations |
| **Render** | Generate user-facing output | HTML generation, form rendering |
| **Signal** | Emit UI framework signals | Client-side state updates |
| **Auth** | Authorization check | Permission validation |
| **Terminal** | End workflow successfully | Return success status |
| **Error** | Handle errors | Return error status |

### Edges

Edges connect nodes and can have conditional predicates:

```xml
<edge from="nodeA" to="nodeB">
  <when>
    <eq left="$token.role" right="admin"/>
  </when>
</edge>
```

### Predicates

Boolean conditions used to control graph traversal:

```xml
<predicate id="is_admin">
  <contains left="$token.perms" right="admin"/>
</predicate>
```

**Available operations:**
- Comparisons: `<eq>`, `<neq>`, `<gt>`, `<gte>`, `<lt>`, `<lte>`
- String ops: `<contains>`, `<matches>`, `<startsWith>`, `<endsWith>`
- Logic: `<and>`, `<or>`, `<not>`
- Functions: `<fn name="is_defined" arg="..."/>`

---

## Example: Approval Workflow

```xml
<omar>
  <workflow>
    <entry p="approval" x="submit" node="start"/>
    
    <nodes>
      <!-- Accept and validate the approval request -->
      <node id="start" kind="transform" schema="ApprovalRequest"/>
      
      <!-- Check if user is approver -->
      <node id="check_auth" kind="auth" predicate="is_approver"/>
      
      <!-- Send approval email (irreversible - requires human-in-the-loop) -->
      <node id="send_email" kind="external" op="0x0340" actor="human"/>
      
      <!-- Success response -->
      <node id="success" kind="terminal" status="200" 
            message="Approval sent successfully"/>
      
      <!-- Error response -->
      <node id="access_denied" kind="error" status="403" 
            message="Not authorized to approve"/>
    </nodes>
    
    <edges>
      <!-- Happy path: auth check → send email → success -->
      <edge from="start" to="check_auth" weight="10"/>
      <edge from="check_auth" to="send_email" weight="10"/>
      <edge from="send_email" to="success" weight="10"/>
      
      <!-- Error path: auth check fails → error node -->
      <edge from="check_auth" to="access_denied" weight="0" fallback="true"/>
    </edges>
  </workflow>
  
  <predicates>
    <predicate id="is_approver">
      <and>
        <contains left="$token.perms" right="approve"/>
        <eq left="$token.tenant" right="$entity.tenant_id"/>
      </and>
    </predicate>
  </predicates>
</omar>
```

---

## Compilation Pipeline

The PXYZ compiler transforms your XML through 6 stages:

1. **Parsing**: XML → Abstract Syntax Tree (AST)
2. **Lowering**: AST → Graph IR (resolves names, flattens structure)
3. **Predicate Compilation**: Expressions → Predicate VM bytecode
4. **Validation**: Three-layer constraint checking
5. **Optimization**: Dead code elimination, predicate deduplication
6. **Emission**: Graph IR → Binary `graph.bin`

### Validation Layers

**Syntactic (SYN)**: Structure is well-formed
- All edges point to existing nodes
- No duplicate node IDs
- At least one entry point defined

**Semantic (SEM)**: Logic is coherent
- Auth nodes have predicates
- External nodes have operation codes
- No cycles in graph (must be DAG)
- All nodes reachable from entry points

**Pragmatic (PRAG)**: Safety rules enforced
- **PRAG001**: LLM → Irreversible paths require validation gate
- **PRAG002**: Write operations have error branches
- **PRAG003**: Irreversible actions require human-in-the-loop
- **PRAG004**: Irreversible actions require confirmed inputs
- **PRAG005**: Quarantined data cannot escape to I/O

---

## Binary Format (graph.bin)

The compiled workflow is a portable binary with this structure:

```
96-byte Header
├─ Magic: 0x504E5958 ("PXYZ")
├─ Version: major.minor
├─ Counts: nodes, edges, predicates
├─ Offsets: to all data sections
└─ Source hash: SHA-256 of input XML

Data Sections
├─ Nodes (16 bytes each)
├─ Edges (12 bytes each)
├─ Predicates (variable-length bytecode)
├─ Strings (null-terminated UTF-8 pool)
└─ Entry Points (8 bytes each)
```

---

## Predicate VM Bytecode

The embedded Predicate VM safely evaluates conditions with 26 opcodes:

**Stack Operations:**
- `0x01 PUSH_INT` - Push integer
- `0x02 PUSH_STR` - Push string reference
- `0x03 LOAD_VAR` - Load variable from host

**Comparisons:**
- `0x10 EQ`, `0x11 NEQ`, `0x12 GT`, `0x13 GTE`, `0x14 LT`, `0x15 LTE`

**Logic:**
- `0x20 AND`, `0x21 OR`, `0x22 NOT`

**String Operations:**
- `0x30 CONTAINS`, `0x31 MATCHES`, `0x32 STARTS_WITH`, `0x33 ENDS_WITH`

**Other:**
- `0x40 LEN`, `0x41 GET`, `0x42 IS_NULL`, `0x43 IS_DEFINED`, `0x44 IS_CONFIRMED`
- `0xF0 CALL_PRED`, `0xFF RET`

---

## IO Operation Codes

External nodes call operations via operation codes:

**Entity Operations (0x01xx):**
- `0x0100` ENTITY_CREATE
- `0x0101` ENTITY_READ
- `0x0102` ENTITY_UPDATE
- `0x0103` ENTITY_DELETE ⚠️ IRREVERSIBLE
- `0x0104` ENTITY_LIST
- `0x0105` ENTITY_SEARCH

**Communication (0x034x-0x036x):**
- `0x0340` EMAIL_SEND ⚠️ IRREVERSIBLE
- `0x0350` SMS_SEND ⚠️ IRREVERSIBLE
- `0x0360` WEBHOOK_CALL ⚠️ IRREVERSIBLE

**HTTP (0x04xx):**
- `0x0400` HTTP_GET
- `0x0401` HTTP_POST
- `0x0402` HTTP_PUT
- `0x0403` HTTP_DELETE

**AI/LLM (0x08xx):**
- `0x0800` LLM_COMPLETE
- `0x0801` LLM_CLASSIFY
- `0x0802` LLM_STRUCTURED

**Plus:** Google Workspace, Vector/RAG, and Storage operations

---

## CLI Commands

### Compile Workflow
```bash
pxyz compile --input workflow.xml --output graph.bin --audit
```

### Inspect Binary
```bash
pxyz inspect --input graph.bin --format mermaid
```

### Validate Workflow
```bash
pxyz check workflow.xml --strict
```

### Initialize Project
```bash
pxyz init --name my-workflow
```

---

## Safety Limits

The WASM runtime enforces strict, non-configurable limits:

| Limit | Value | Purpose |
|-------|-------|---------|
| MAX_VISITED_NODES | 1000 | Prevent runaway traversal |
| MAX_PREDICATE_STEPS | 256 | Prevent infinite loops in conditions |
| MAX_STACK_DEPTH | 16 | Prevent stack overflow |
| MAX_CALL_DEPTH | 4 | Prevent infinite recursion |
| MAX_PREDICATE_BYTECODE | 256 bytes | Limit condition complexity |

---

## Variable Paths

Access data from the host context in predicates:

| Path | Description |
|------|-------------|
| `$token.sub` | User ID from auth token |
| `$token.perms` | Permissions array |
| `$token.tenant` | Tenant ID |
| `$entity.owner_id` | Owner of current entity |
| `$entity.status` | Current entity status |
| `$input.query` | User input |
| `$state.node_id.field` | Output from previous node |

---

## Design Principles

### Why Not Traditional Programming?

PXYZ's constraints are **intentional**:

1. **Auditability**: A senior engineer can audit the entire runtime in an afternoon
2. **Provability**: With bounded execution, termination is guaranteed
3. **Safety by Construction**: Dangerous patterns are impossible to compile
4. **Determinism**: Same input → same execution path always
5. **Transparency**: Every external interaction is explicit and reviewable

### When to Use PXYZ

✅ **Good Use Cases:**
- AI agents executing workflows
- Approval processes
- Data validation pipelines
- Multi-step integrations
- Systems where audit trails matter

❌ **Not Ideal For:**
- Real-time high-frequency operations
- Complex algorithms (use external services instead)
- Stateful long-running processes

---

## Key References

- **Magic Number**: `0x504E5958` (ASCII: "PXYZ")
- **Binary Version**: 1.0
- **WASM Memory**: Linear memory shared with host
- **Entry Points**: (P, X) coordinate pairs mapped to start nodes
- **Node IDs**: 0-indexed, resolved at compile time
- **Predicate IDs**: 0 means "always true" (unconditional edge)

---

## Next Steps

1. Review the Architecture Overview in PXYZ Explorer
2. Study example workflows for your use case
3. Write your first workflow XML
4. Compile with `pxyz compile`
5. Inspect the binary with `pxyz inspect --format mermaid`
6. Test with the WASM runtime
7. Deploy to production

---

*For the complete technical reference, see the full PXYZ System Reference Manual.*