# PXYZ System Reference Manual

## 1.0 Introduction and Core Principles

The PXYZ system is engineered to address a critical challenge in modern AI development: how to safely grant AI agents access to production systems and allow them to execute potentially dangerous workflows. This manual provides a complete technical reference for the PXYZ workflow compiler and runtime. The system's design is intentionally deliberate, prioritizing provable safety, auditability, and determinism over the flexibility of conventional programming paradigms. This section outlines the core design philosophy that underpins the entire architecture.

### 1.1 System Philosophy

The central insight of PXYZ is that business logic should be treated as data to be interpreted, not as code to be executed. A workflow is defined as a finite, auditable map of possible operations and the conditions under which they can be performed. The runtime's sole job is to walk this map within strictly enforced safety limits. This philosophy is embodied in the following core principles:

* Business logic is data, not code.
  * Analysis: By representing the entire program as a traversable graph, the system's behavior is fully described by a static data structure, making it completely analyzable at compile time.
* Predicates are bounded.
  * Analysis: Decision logic cannot contain unbounded loops or recursion. With hard limits on execution steps and stack depth, all predicate evaluations are guaranteed to terminate, preventing runaway processes.
* All IO is explicit.
  * Analysis: Every interaction with the outside world, from API calls to database writes, is declared via an explicit operation code. This eliminates hidden side effects and allows auditors to precisely identify and review every point of external contact.
* Safety at compile time.
  * Analysis: The compiler enforces pragmatic business rules, such as preventing an LLM from directly sending an email without an approval gate. This shifts safety checks "left," catching dangerous patterns before deployment, not after an incident.
* Human gates for irreversible actions.
  * Analysis: The system makes it impossible to compile a workflow that performs an irreversible action (e.g., deleting data) without an explicit human-in-the-loop confirmation step, enforcing a critical safety policy at the architectural level.
* Auditable runtime.
  * Analysis: The production runtime is a minimal, formally verifiable WebAssembly module. Its small size (~600 lines) allows a senior engineer to audit the entire execution engine in an afternoon, providing unparalleled transparency.

The PXYZ architecture is not designed for cleverness or maximum performance; it is designed for auditability and provable safety. When AI agents can send emails, delete user data, and call webhooks, this deliberate and constrained approach is not a limitation but a fundamental requirement.

### 1.2 System Architecture

The PXYZ architecture is strategically divided into three distinct components: the Rust Compiler, the WASM Runtime, and the JavaScript Host. This separation of concerns is fundamental to the system's security, auditability, and portability goals. The compiler builds a secure, static artifact; the runtime executes it in a sandbox; and the host provides a controlled bridge to the outside world.

The three primary components are:

1. Rust Compiler: The compiler's sole responsibility is to transform human-readable PXYZ XML definitions into a compact, secure, and portable binary format called graph.bin. It performs all syntactic, semantic, and pragmatic validation, compiles predicate logic into bytecode, and runs optimization passes. The compiler does not execute any workflow logic itself.
2. WASM Runtime (pxyz.wat): This is the crown jewel of the system. It is a minimal (~500-700 lines), auditable, and formally verifiable engine for traversing the graph.bin and executing its embedded Predicate VM. It is written in WebAssembly Text Format (.wat) to maximize readability for security auditors. Critically, the runtime makes zero direct system calls and is completely sandboxed, relying entirely on the host for all I/O.
3. JS Host (IO Adapter): The JavaScript host acts as the bridge between the sandboxed WASM runtime and the external world. It implements the io_call interface and other required host functions. When the runtime encounters an External node, it passes an operation code to the host. The host interprets this code, resolves any necessary variables from the application context, and performs the required side effect (e.g., making an API call, writing to a database).

This dual-runtime approach provides a full-featured, debuggable Rust environment for local development and testing while ensuring a minimal, highly auditable WASM target for production deployment. Both runtimes execute the identical graph.bin artifact, guaranteeing consistency between environments.

#### Component Details

| Runtime | Language | Purpose | Lines of Code |
|---------|----------|---------|---------------|
| Native | Rust | Development, testing, and debugging | ~1500 |
| WASM | WAT | Production deployment and security auditing | ~500 |

The following sections will meticulously detail each part of this architecture, beginning with the developer-facing XML language used to define workflows.

## 2.0 PXYZ Workflow Definition Language (XML)

The PXYZ XML format is the primary interface for defining business logic. It is a declarative language for constructing a directed acyclic graph (DAG) of operations, not an imperative programming language. Developers use this format to define the "map" of what can be done and under what conditions. This section serves as the definitive reference for the structure and elements of the PXYZ XML dialect.

### 2.1 Document Structure

The root element of any PXYZ definition file is `<omar>`. This element serves as the container for all other top-level definitions.

```xml
<omar>
  <schemas>...</schemas>        <!-- Optional: Data structures and validation rules -->
  <predicates>...</predicates>  <!-- Optional: Reusable boolean conditions -->
  <workflow>...</workflow>      <!-- Required: Primary executable graph definition -->
  <templates>...</templates>    <!-- Optional: Reusable content templates -->
</omar>
```

**Primary child elements:**

* `<schemas>` — (Optional) Defines data structures and their validation rules
* `<predicates>` — (Optional) Contains reusable, named boolean conditions
* `<workflow>` — Contains the primary executable graph definition
* `<templates>` — (Optional) Contains reusable content templates, such as for HTML rendering

### 2.2 Workflow Definition (`<workflow>`)

The `<workflow>` element is the container for a single, executable graph. It groups the nodes and edges that constitute the business logic.

```xml
<workflow>
  <entry p="..." x="..." node="..."/>
  <nodes>
    <node id="..." kind="...">...</node>
  </nodes>
  <edges>
    <edge from="..." to="...">...</edge>
  </edges>
</workflow>
```

**Main child elements:**

* `<entry>` — Defines the starting point of the workflow, mapping a (P, X) coordinate pair to a specific node ID
* `<nodes>` — A container for all `<node>` definitions within the workflow
* `<edges>` — A container for all `<edge>` definitions that connect the nodes

### 2.3 Node Definition (`<node>`)

The `<node>` element represents a single unit of work or a control point in the graph. Each node has a unique `id` within its workflow and a `kind` that determines its behavior.

```xml
<node id="node_id" kind="transform" />
<node id="external_op" kind="external" op="0x0340" />
<node id="auth_check" kind="auth" predicate="is_admin" />
```

| Attribute | Description |
|-----------|-------------|
| `id` | Required. A unique identifier for the node within the workflow. |
| `kind` | Required. The type of the node. See Section 6.1 for a full list of kinds (e.g., transform, external). |
| `op` | The operation code for external nodes, specified in hex (e.g., `0x0340`) or decimal. |
| `template` | A reference to a named template for render nodes. |
| `schema` | A reference to a named schema used to validate the node's output. |
| `predicate` | A reference to a named predicate for an auth node. |
| `selector` | A JSONPath-like string to select data from the node's output. |
| `status` | An integer status code to be returned by a terminal or error node. |
| `message` | A human-readable message, often associated with error or terminal nodes. |
| `actor` | Specifies the required actor (human or agent). If human, requires host confirmation. |
| `confirmation` | The confirmation status of the data produced by this node (e.g., suggested, confirmed). |
| `async` | `true` or `false`. Indicates that the node may block and can be executed asynchronously. |
| `cacheable` | `true` or `false`. Indicates that the result of this node's execution can be memoized. |

### 2.4 Edge Definition (`<edge>`)

The `<edge>` element defines a directed connection between two nodes. Traversal across an edge can be made conditional by including a predicate.

| Attribute | Description |
|-----------|-------------|
| `from` | Required. The id of the source node. |
| `to` | Required. The id of the target node. |
| `weight` | An integer priority for the edge. Higher-weighted edges are evaluated first. |
| `parallel` | `true` or `false`. Indicates the edge can be traversed in parallel with its siblings. |
| `fallback` | `true` or `false`. Indicates the edge should only be taken if other paths fail. |

An edge can contain an inline `<when>` element, which in turn contains a predicate expression. If the predicate evaluates to true, the edge is traversed. If no predicate is specified, the edge is considered unconditional.

```xml
<edge from="node_a" to="node_b" weight="10">
  <when>
    <eq left="$token.perms" right="admin" />
  </when>
</edge>
```

### 2.5 Predicate Definition (`<predicate>`)

Predicates are reusable, named boolean conditions used to control graph traversal. They are defined within the top-level `<predicates>` block, and each individual condition is defined by a `<predicate>` element with a unique `id` attribute. These named predicates can be referenced from edges or other predicates.

```xml
<predicates>
  <predicate id="is_admin">
    <contains left="$token.perms" right="admin" />
  </predicate>
  <predicate id="is_owner">
    <eq left="$token.sub" right="$entity.owner_id" />
  </predicate>
</predicates>
```

**Available predicate expression elements:**

| Element | Attributes | Description |
|---------|------------|-------------|
| `<always/>` | — | Always evaluates to true. |
| `<fail/>` | — | Always evaluates to false. |
| `<eq>` | `left, right` | Checks if left is equal to right. |
| `<neq>` | `left, right` | Checks if left is not equal to right. |
| `<gt>` | `left, right` | Checks if left is greater than right. |
| `<gte>` | `left, right` | Checks if left is greater than or equal to right. |
| `<lt>` | `left, right` | Checks if left is less than right. |
| `<lte>` | `left, right` | Checks if left is less than or equal to right. |
| `<contains>` | `left, right` | Checks if the string or array at left contains the value at right. |
| `<matches>` | `left, pattern` | Checks if the string at left matches the regular expression in pattern. |
| `<startsWith>` | `left, prefix` | Checks if the string at left starts with the prefix. |
| `<endsWith>` | `left, suffix` | Checks if the string at left ends with the suffix. |
| `<and>` | (child elements) | Logical AND. Evaluates to true if all child predicate elements are true. |
| `<or>` | (child elements) | Logical OR. Evaluates to true if any child predicate element is true. |
| `<not>` | (child element) | Logical NOT. Negates the result of the single child predicate element. |
| `<ref>` | `predicate` | References a named predicate defined in the `<predicates>` block. |
| `<fn>` | `name, arg` | Invokes a built-in boolean function (e.g., `is_defined`) on the arg. |

### 2.6 Variable Paths

Data from the host environment and workflow state is accessed within predicates using variable paths, which always begin with a `$` prefix.

```shell
$token.sub              → User ID from auth token
$token.perms            → Array of permissions
$entity.owner_id        → Owner of current entity
$state.node_id.field    → Output from previous node
```

| Path | Description |
|------|-------------|
| `$token.sub` | The subject (e.g., user ID) from the authentication token. |
| `$token.perms` | An array of permissions from the authentication token. |
| `$token.tenant` | The tenant ID from the authentication token. |
| `$entity.owner_id` | The owner ID of the current entity being processed. |
| `$entity.status` | The status of the current entity. |
| `$input.query` | An input field, typically from a user query. |
| `$state.node_id.field` | State data produced by a previously executed node. |

This XML definition serves as the high-level input to the PXYZ compilation pipeline, which transforms it into a secure and executable binary artifact.

## 3.0 The Compilation Pipeline

The compilation process is a multi-stage pipeline that transforms the high-level XML DSL into graph.bin, a secure, compact, and executable binary artifact. Each stage performs a specific transformation or validation, ensuring that the final output is safe and correct by construction. This section details each stage of that transformation.

### 1. Parsing (XML to AST)

* **Input:** Raw XML text (`workflow.xml`)
* **Output:** An in-memory Abstract Syntax Tree (AST), represented by the `OmarDocument` struct
* **Function:** This initial stage parses the XML document, validating its basic structure and converting the text into a tree-like data structure that the compiler can understand and manipulate.

### 2. Lowering (AST to IR)

* **Input:** The Abstract Syntax Tree (AST)
* **Output:** The Graph Intermediate Representation (IR)
* **Function:** This is a crucial step where the high-level, developer-friendly AST is transformed into a lower-level, graph-centric representation (`GraphIR`, `GNode`, `GEdge`). The IR resolves symbolic names (like node IDs) into numerical IDs, flattens the structure for efficient processing, and serves as the primary data structure for all subsequent validation and optimization passes.

### 3. Predicate Compilation

* **Input:** Predicate expressions from the AST and IR
* **Output:** Predicate bytecode appended to the IR
* **Function:** All predicate expressions, whether named or defined inline on an edge, are compiled into a compact bytecode format. This bytecode is designed for safe and efficient execution by the sandboxed Predicate VM within the PXYZ runtime.

### 4. Validation

* **Input:** The Graph Intermediate Representation (IR)
* **Output:** A set of diagnostics (errors and warnings)
* **Function:** The IR undergoes a rigorous, three-layer validation process to ensure correctness and safety before the binary is emitted. This is a cornerstone of the PXYZ safety philosophy. The checks are:
  * **Syntactic:** Verifies structural integrity (e.g., all edges point to existing nodes)
  * **Semantic:** Verifies logical correctness (e.g., no cycles in the graph, auth nodes have predicates)
  * **Pragmatic:** Enforces business safety rules (e.g., irreversible actions require a human-in-the-loop)

### 5. Optimization

* **Input:** The validated Graph IR
* **Output:** An optimized Graph IR
* **Function:** A series of optimization passes are performed on the IR to produce a more efficient binary. These include dead code elimination (removing nodes that are unreachable from any entry point) and predicate deduplication (merging identical predicate bytecode to reduce binary size).

### 6. Emission

* **Input:** The validated and optimized Graph IR
* **Output:** The final `graph.bin` binary file
* **Function:** This final stage serializes the IR into the precisely defined `graph.bin` binary format. It lays out the header, data sections, and string pool according to the specification.

The final output of this pipeline is the `graph.bin` file. Its precise binary layout is specified in the next section.

## 4.0 Binary Format Specification: `graph.bin`

The graph.bin file is the portable, auditable, and executable output of the PXYZ compiler. It is a self-contained, platform-agnostic artifact designed to be loaded and executed by a compatible PXYZ runtime. This section provides the definitive byte-level specification for the format, which is essential for implementing a compatible runtime or building inspection and analysis tools.

### 4.1 Header (96 bytes)

Every graph.bin file begins with a 96-byte header that contains metadata about the graph and offsets to its various data sections. All multi-byte values are little-endian.

| Offset (Hex) | Size (bytes) | Field | Description |
|--------------|--------------|-------|-------------|
| `0x00` | 4 | Magic | The constant value `0x504E5958` (ASCII for "PXYZ") |
| `0x04` | 2 | Version major | The major version of the binary format |
| `0x06` | 2 | Version minor | The minor version of the binary format |
| `0x08` | 4 | Node count | The total number of nodes in the graph |
| `0x0C` | 4 | Edge count | The total number of edges in the graph |
| `0x10` | 4 | Predicate count | The total number of compiled predicates |
| `0x14` | 4 | String pool size | The total size of the String Pool in bytes |
| `0x18` | 4 | Entry count | The number of entry points defined |
| `0x20` | 32 | Source hash | SHA-256 hash of the source XML file |
| `0x40` | 4 | Nodes offset | Byte offset from the start of the file to the Node section |
| `0x44` | 4 | Edges offset | Byte offset from the start of the file to the Edge section |
| `0x48` | 4 | Predicates offset | Byte offset from the start of the file to the Predicate Table |
| `0x4C` | 4 | Strings offset | Byte offset from the start of the file to the String Pool |
| `0x50` | 4 | Entries offset | Byte offset from the start of the file to the Entry Points section |

### 4.2 Data Sections

The header contains direct byte offsets to the five primary data sections of the file:

* Nodes: A contiguous array of Node Entry structures.
* Edges: A contiguous array of Edge Entry structures, sorted by source node ID.
* Predicates: A table of compiled predicate bytecode chunks.
* Strings: A pool of all unique, null-terminated UTF-8 strings.
* Entry Points: A table for mapping (P, X) hashes to entry node IDs.

### 4.3 Node Entry (16 bytes)

Each node in the graph is represented by a 16-byte structure in the Nodes section.

| Offset (Hex) | Size (bytes) | Field | Description |
|--------------|--------------|-------|-------------|
| `0x00` | 4 | Node ID | The unique identifier for this node (its index in the node array) |
| `0x04` | 1 | Kind | The node's type (0-6). See Section 6.1 for a full list |
| `0x05` | 1 | Flags | A bitfield of node properties. See Section 6.2 |
| `0x06` | 2 | Op code | The I/O operation code for External nodes |
| `0x08` | 4 | Data offset | An offset into the String Pool, typically for the node's name |
| `0x0C` | 2 | Edge start index | The starting index in the Edges section for this node's outgoing edges |
| `0x0E` | 2 | Edge count | The number of outgoing edges for this node |

### 4.4 Edge Entry (12 bytes)

Each edge connecting two nodes is represented by a 12-byte structure in the Edges section.

| Offset (Hex) | Size (bytes) | Field | Description |
|--------------|--------------|-------|-------------|
| `0x00` | 4 | Target node ID | The ID of the node this edge points to |
| `0x04` | 2 | Predicate ID | The ID of the predicate to evaluate for this edge. A value of 0 means the edge is always taken |
| `0x06` | 2 | (reserved) | Reserved for future use |
| `0x08` | 2 | Weight | The priority of this edge. Higher values are evaluated first |
| `0x0A` | 2 | Flags | A bitfield of edge properties. See Section 6.3 |

### 4.5 Entry Point Entry (8 bytes)

Each defined entry point is represented by an 8-byte structure in the Entry Points section.

| Offset (Hex) | Size (bytes) | Field | Description |
|--------------|--------------|-------|-------------|
| `0x00` | 4 | PX hash | An FNV-1a hash of the concatenated P and X coordinates for fast lookup |
| `0x04` | 4 | Node ID | The ID of the node where traversal begins for this entry point |

### 4.6 String Pool

The String Pool is a single, contiguous block of null-terminated UTF-8 strings located at the offset specified in the header. Various other data structures, such as the Node Entry, contain offsets that point into this pool. This method de-duplicates all strings (node names, variable paths, etc.) to significantly reduce the final binary size.

```shell
Offset 0x00: "node_a\0"
Offset 0x08: "node_b\0"
Offset 0x10: "$token.sub\0"
...
```

### 4.7 Predicate Table

The Predicate Table contains the compiled bytecode for all predicates. It is a series of variable-length chunks. Each chunk begins with a 2-byte unsigned integer specifying the length of the bytecode that follows. The runtime uses the predicate ID as an index to find the correct bytecode chunk.

```shell
Predicate 0: [len: 2 bytes][bytecode: N bytes]
Predicate 1: [len: 2 bytes][bytecode: N bytes]
Predicate 2: [len: 2 bytes][bytecode: N bytes]
```

This binary format is loaded and executed by the PXYZ WASM runtime, which is detailed next.

## 5.0 The PXYZ Runtime Environment

The PXYZ Runtime is the execution engine responsible for interpreting graph.bin files. The definitive, production-grade runtime is the WebAssembly Text (.wat) version, which is engineered for maximum security, auditability, and formal verifiability. This section details its architecture, execution flow, and the embedded Predicate VM that safely evaluates conditional logic.

### 5.1 WASM Runtime (`pxyz.wat`)

The core runtime is a small, self-contained WebAssembly module designed to be sandboxed and controlled entirely by a host environment.

#### Design Goals

* **Auditability:** The entire runtime is approximately 600 lines of human-readable WebAssembly Text, enabling a thorough security review in a single afternoon.
* **Formal Verifiability:** The simple, state-machine-like design with no complex abstractions makes it suitable for formal methods verification.
* **Sandboxing:** As a WASM module, it runs in a completely isolated memory space and has no direct access to system resources. All external communication is explicitly mediated through host imports.

#### Execution Flow

1. **Load graph.bin:** The host loads the binary into the WASM module's memory. The runtime validates the header's magic number and version.
2. **Find Entry Point:** The runtime receives P and X coordinates from the host, computes an FNV-1a hash, and searches the entry point table to find the starting node ID.
3. **Begin Traversal:** Starting with the entry node, the runtime enters a loop. In each iteration, it marks the current node as visited to detect cycles.
4. **Execute Node:** The runtime executes the logic for the current node based on its kind (e.g., for an External node, it invokes the `io_call` host function).
5. **Evaluate Edges:** It then iterates through the node's outgoing edges, evaluating the associated predicate for each one. The first predicate that returns true determines the next node to visit.
6. **Terminate:** The loop continues until a Terminal or Error node is reached, or if no valid outgoing edge is found. The final result code is then returned to the host.

#### Host Imports

The runtime is completely sandboxed and communicates with the outside world exclusively through a minimal, well-defined set of imported functions provided by the host. The `io_call` function is the single, critical gateway for all external side effects.

| Host Import | Description |
|-------------|-------------|
| `io_call` | Executes an external operation identified by an op code |
| `io_resolve_var` | Resolves a variable path (e.g., `$token.sub`) from the host's context |
| `io_is_human` | Returns true if the current actor is a human, used for permission checks |
| `io_is_confirmed` | Returns true if the specified entity's data is confirmed, not suggested |
| `io_str_contains` | Host-provided implementation for string contains checks |
| `io_str_matches` | Host-provided implementation for regular expression matching |
| `io_str_starts_with` | Host-provided implementation for string startsWith checks |
| `io_str_ends_with` | Host-provided implementation for string endsWith checks |
| `io_log` | Passes a log message to the host environment |
| `emit_event` | Emits a structured event to the host for auditing and tracing |

#### Safety Limits

To prevent unbounded execution and resource exhaustion, the runtime enforces strict, non-configurable safety limits. Tripping any of these limits immediately terminates the traversal.

| Limit | Value | Purpose |
|-------|-------|----------|
| `MAX_VISITED_NODES` | 1000 | Prevents runaway graph traversal and terminates long-running workflows |
| `MAX_PREDICATE_STEPS` | 256 | Prevents infinite loops within a single predicate evaluation |
| `MAX_STACK_DEPTH` | 16 | Prevents stack overflow within the Predicate VM |
| `MAX_CALL_DEPTH` | 4 | Prevents infinite recursion from predicates calling other predicates |
| `MAX_PREDICATE_BYTECODE` | 256 bytes | Limits the complexity of a single predicate at compile time |

### 5.2 The Predicate VM

Embedded within the WASM runtime is a simple, stack-based bytecode interpreter designed to safely evaluate the boolean conditions on graph edges. It has a tiny instruction set, operates on a small, fixed-size stack, and adheres to the strict execution step limits, guaranteeing that all predicate evaluations will terminate quickly and safely.

The following table is a comprehensive reference for all Predicate VM opcodes.

| Opcode (Hex) | Name | Stack Effect | Description |
|--------------|------|--------------|-------------|
| `0x00` | NOOP | — | No operation |
| `0x01` | PUSH_INT | → val | Push an immediate 32-bit integer value onto the stack |
| `0x02` | PUSH_STR | → val | Push a string reference (a 32-bit offset into the String Pool) |
| `0x03` | LOAD_VAR | → val | Load a variable from the host context via `io_resolve_var` |
| `0x04` | LOAD_FIELD | obj → val | Get a field from an object reference on the stack |
| `0x10` | EQ | a, b → bool | Pop two values, push 1 if they are equal, 0 otherwise |
| `0x11` | NEQ | a, b → bool | Pop two values, push 1 if they are not equal |
| `0x12` | GT | a, b → bool | Pop a and b, push 1 if a > b |
| `0x13` | GTE | a, b → bool | Pop a and b, push 1 if a >= b |
| `0x14` | LT | a, b → bool | Pop a and b, push 1 if a < b |
| `0x15` | LTE | a, b → bool | Pop a and b, push 1 if a <= b |
| `0x20` | AND | a, b → bool | Pop two booleans, push their logical AND |
| `0x21` | OR | a, b → bool | Pop two booleans, push their logical OR |
| `0x22` | NOT | a → bool | Pop a boolean, push its logical NOT |
| `0x30` | CONTAINS | haystack, needle → bool | Check if a string/array contains a value (delegates to host) |
| `0x31` | MATCHES | str → bool | Check if a string matches a regex pattern (delegates to host) |
| `0x32` | STARTS_WITH | str, prefix → bool | Check if a string starts with a prefix (delegates to host) |
| `0x33` | ENDS_WITH | str, suffix → bool | Check if a string ends with a suffix (delegates to host) |
| `0x40` | LEN | val → int | Push the length of a string or array |
| `0x41` | GET | arr, idx → val | Get an element from an array at a given index |
| `0x42` | IS_NULL | val → bool | Push 1 if the value is null or undefined, 0 otherwise |
| `0x43` | IS_DEFINED | val → bool | Push 1 if the value is not null or undefined |
| `0x44` | IS_CONFIRMED | val → bool | Check if an entity's data is confirmed (delegates to host) |
| `0xF0` | CALL_PRED | → bool | Call another predicate by its 16-bit ID |
| `0xFF` | RET | bool → | Return the boolean value from the top of the stack and terminate execution |

The runtime uses a well-defined set of data structures and codes to operate. These are cataloged for reference in the following section.

## 6.0 Core System Reference

This section provides detailed reference tables for the core data types and codes used throughout the PXYZ system, from the XML definition down to the binary format and runtime execution. These tables serve as a definitive guide for developers and auditors.

### 6.1 Node Kinds

Each node in a workflow has a Kind that defines its fundamental behavior.

| Value (integer) | Kind Name | Purpose |
|-----------------|-----------|----------|
| 0 | Transform | Validates, transforms, or structures data without external side effects |
| 1 | External | Calls an external I/O handler, specified by an op code, to produce side effects |
| 2 | Render | Generates HTML or other user-facing output from a template |
| 3 | Signal | Emits a signal to the UI framework to trigger client-side changes |
| 4 | Auth | Performs an authorization check using a predicate; traversal fails if the predicate is false |
| 5 | Terminal | Gracefully ends the graph traversal and returns a success status |
| 6 | Error | A designated node for handling errors; ends traversal and returns an error status |

### 6.2 Node Flags

The Flags field in a Node Entry is an 8-bit bitfield that specifies boolean properties of the node.

| Bit | Flag Name | Meaning |
|-----|-----------|----------|
| 0 | ASYNC | The node may block for a long time and should be executed asynchronously |
| 1 | REQUIRES_AUTH | The node requires a successful authentication check before execution |
| 2 | HAS_SIDE_EFFECTS | The node produces side effects in an external system |
| 3 | IRREVERSIBLE | The node's action cannot be easily undone (e.g., sending an email) |
| 4 | REQUIRES_HUMAN | The node's execution requires confirmation from a human actor |
| 5 | CACHEABLE | The result of the node's execution can be memoized for future use |

### 6.3 Edge Flags

The Flags field in an Edge Entry is a 16-bit bitfield that specifies properties of the connection.

| Bit | Flag Name | Meaning |
|-----|-----------|----------|
| 0 | PARALLEL | The edge can be traversed in parallel with its sibling edges |
| 1 | FALLBACK | The edge should only be taken if other non-fallback paths from the source node fail |
| 2 | ERROR_EDGE | The edge leads to a designated error handling node |

### 6.4 IO Operation Codes

When an External node is executed, it invokes the io_call host function with a specific operation code (op code) that dictates the action to be performed by the host. Irreversible actions are marked with a warning.

#### Entity (0x01xx)

* `0x0100` ENTITY_CREATE
* `0x0101` ENTITY_READ
* `0x0102` ENTITY_UPDATE
* `0x0103` ENTITY_DELETE
* `0x0104` ENTITY_LIST
* `0x0105` ENTITY_SEARCH

#### Google Workspace (0x03xx)

* `0x0300` GOOGLE_CONTACTS_SEARCH
* `0x0301` GOOGLE_CONTACTS_GET
* `0x0302` GOOGLE_CONTACTS_CREATE
* `0x0310` GOOGLE_CALENDAR_LIST
* `0x0320` GOOGLE_DRIVE_SEARCH
* `0x0330` GOOGLE_GMAIL_SEARCH
* `0x0332` GOOGLE_GMAIL_SEND ⚠️ **IRREVERSIBLE**

#### Communication (0x034x-0x036x)

* `0x0340` EMAIL_SEND ⚠️ **IRREVERSIBLE**
* `0x0350` SMS_SEND ⚠️ **IRREVERSIBLE**
* `0x0360` WEBHOOK_CALL ⚠️ **IRREVERSIBLE**

#### HTTP (0x04xx)

* `0x0400` HTTP_GET
* `0x0401` HTTP_POST
* `0x0402` HTTP_PUT
* `0x0403` HTTP_DELETE

#### Vector/RAG (0x07xx)

* `0x0700` QDRANT_SEARCH
* `0x0701` QDRANT_INDEX
* `0x0702` EMBEDDING_GENERATE

#### AI/LLM (0x08xx)

* `0x0800` LLM_COMPLETE
* `0x0801` LLM_CLASSIFY
* `0x0802` LLM_STRUCTURED
* `0x0810` LOCAL_MODEL_RUN

#### Storage (0x09xx)

* `0x0900` STORAGE_GET
* `0x0901` STORAGE_SET
* `0x0910` EVENT_LOG_APPEND
* `0x0911` EVENT_LOG_QUERY

### 6.5 Constraint System

The PXYZ compiler uses a three-layer constraint system to validate every workflow before compilation. This ensures that only structurally sound, logically coherent, and pragmatically safe graphs can be deployed.

#### Syntactic (SYN) - Structure Validation

These checks ensure the graph's structure is well-formed and all references are valid.

| Code | Check | Description |
|------|-------|-------------|
| SYN001 | Edge targets exist | Validates all edge target nodes exist |
| SYN002 | Entry points reference existing nodes | Ensures entry points point to valid nodes |
| SYN003 | Predicate references exist | Validates all referenced predicates are defined |
| SYN004 | No duplicate node IDs | Ensures all node IDs are unique |
| SYN005 | At least one entry point is defined | Requires at least one entry point |
| SYN006 | No duplicate entry points (same P, X) | Prevents duplicate entry point definitions |
| SYN007 | Edge sources exist | Validates all edge source nodes exist |

#### Semantic (SEM) - Logic Validation

These checks ensure the graph's logic is coherent and follows system rules.

| Code | Check | Description |
|------|-------|-------------|
| SEM001 | Auth nodes have predicates | Auth nodes must have associated predicates |
| SEM002 | External nodes have op codes | External nodes must have operation codes |
| SEM003 | Terminal nodes shouldn't have outgoing edges | Terminal nodes must be leaf nodes |
| SEM004 | No cycles in graph (must be a DAG) | Graph must be acyclic |
| SEM005 | All nodes are reachable from an entry point | All nodes must be reachable from at least one entry |
| SEM006 | Error nodes have incoming edges | Error nodes must have at least one incoming edge |
| SEM007 | Render nodes have templates | Render nodes must reference templates |

#### Pragmatic (PRAG) - Business & Safety Rules

These checks enforce high-level safety policies and best practices.

| Code | Check | Description |
|------|-------|-------------|
| PRAG001 | LLM → Irreversible action paths require a validation gate | LLM outputs must pass validation before irreversible actions |
| PRAG002 | Write operations should have defined error-handling branches | Write operations must have error handling |
| PRAG003 | Irreversible actions require a human-in-the-loop path | Irreversible actions must include human confirmation |
| PRAG004 | Irreversible actions require inputs that have been explicitly confirmed | Irreversible actions must use confirmed data |
| PRAG005 | Data marked as 'quarantined' cannot escape to an external I/O operation | Quarantined data cannot be sent externally |

These components are managed and compiled using the PXYZ command-line interface.

## 7.0 Command-Line Interface (CLI)

The pxyz command-line tool is the primary utility for compiling, inspecting, and managing PXYZ workflows. It provides a simple interface for the entire compilation pipeline. This section documents its available commands and options.

### 7.1 `pxyz compile`

This is the core command for compiling a `workflow.xml` file into a `graph.bin` binary artifact.

```bash
pxyz compile --input workflow.xml --output graph.bin --audit --strict
```

**Options:**

* `--input <FILE>` — Specifies the path to the input `workflow.xml` file
* `--output <FILE>` — Specifies the path for the output `graph.bin` file
* `--audit` — Generates an `audit.json` file alongside the binary, containing detailed metadata and validation results
* `--strict` — Treats all compiler warnings as errors, failing the compilation if any warnings are present

### 7.2 `pxyz inspect`

This command is used to display high-level metadata about a compiled `graph.bin` file without executing it.

```bash
pxyz inspect --input graph.bin --format mermaid
pxyz inspect --input graph.bin --format json
```

**Options:**

* `--input <FILE>` — Specifies the path to the `graph.bin` file to inspect
* `--format <FORMAT>` — Sets the output format:
  * `text` — A human-readable summary (default)
  * `json` — Detailed metadata in JSON format
  * `mermaid` — Generates a complete visual flowchart of the graph in Mermaid diagram syntax. This output can be used to render a styled, color-coded diagram of all nodes, edges, and entry points, making it an invaluable tool for developers to visualize and debug their workflows

### 7.3 `pxyz check`

This command runs the complete three-layer constraint validation system on a `workflow.xml` file without producing a binary output. It is useful for integrating PXYZ validation into CI/CD pipelines.

```bash
pxyz check --input workflow.xml
```

### 7.4 `pxyz init`

This command creates a new PXYZ project directory with boilerplate files to help developers get started quickly. It generates a sample `workflow.xml` and a `build.sh` script.

```bash
pxyz init --project my_workflow
```
