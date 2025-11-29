PXYZ Architecture: A Briefing on the Compiler and Runtime System

Executive Summary

The PXYZ system is an architecture designed to address a critical challenge: enabling AI agents to perform potentially dangerous tasks safely. Its core philosophy is to treat business logic as data, not code. Workflows are defined in an XML-based Domain-Specific Language (DSL) and compiled into a compact, auditable, and portable binary format called graph.bin. This binary represents the workflow as a finite, traversable map—a Directed Acyclic Graph (DAG).

The architecture is composed of three distinct, non-overlapping components:

1. The Rust Compiler: A command-line tool responsible for parsing the XML source, validating it against a rigorous multi-layered constraint system, optimizing the resulting graph, and emitting the final graph.bin binary. It does not execute the workflow.
2. The WASM Runtime (pxyz.wat): The primary production runtime, consisting of approximately 600 lines of human-readable WebAssembly Text. Designed for maximum auditability and formal verification, this sandboxed engine "walks" the graph.bin map, enforces hard safety limits, and delegates all I/O operations to a host environment.
3. The JavaScript Host: An I/O adapter that implements the actual side effects (e.g., sending emails, calling APIs) requested by the WASM runtime. It provides the necessary context, such as actor permissions and data confirmation status, effectively acting as the runtime's hands to interact with the outside world.

Safety is paramount and is enforced through several mechanisms: a tiny, stack-based Predicate VM with bounded execution to prevent infinite loops; explicit opcodes for all I/O operations to make side effects auditable; and a powerful three-tier compile-time constraint system (Syntactic, Semantic, and Pragmatic) that detects dangerous patterns before deployment, such as an LLM directly triggering an irreversible action without a human-in-the-loop. A parallel Rust runtime exists purely for development and testing, but the lightweight WAT runtime is the definitive, auditable execution engine.

1. Core Philosophy and Architectural Model

The PXYZ architecture is built on a foundational insight designed to manage the risks associated with autonomous AI agents.

The Central Insight

The system is engineered to answer a single question: "How do you let AI agents do dangerous things safely?"

The solution is not to write imperative code that performs actions, but rather to define a declarative, finite, and auditable map of what actions are possible and under what conditions. The runtime then walks this map, operating within strict, unchangeable limits. This approach transforms business logic from executable code into analyzable data.

"This architecture is not about being clever. It's about being auditable. When the AI can send emails, delete data, and call webhooks on behalf of users, 'move fast and break things' is not an option. PXYZ moves deliberately and proves things."

A core tenet of the system is that "State is a lie. Events are truth. The graph is physics." The only real elements are events (what happened), constraints (rules), and the graph (the flow of operations).

The Three-Part Architecture

The system's responsibilities are cleanly separated across three components, a resolution reached after initial confusion about where functionality should reside. The mental model is analogous to a board game: the XML is the rulebook, the Rust Compiler is a factory that prints the game board, graph.bin is the board itself, the WAT runtime is the physics engine that moves the pieces, and the JS Host is the player's hands that interact with the world.

Component Technology Primary Role
Compiler Rust Transforms human-readable XML into the graph.bin binary. Enforces all safety constraints at compile time.
Runtime WebAssembly Text (WAT) Executes the graph.bin in a minimal, auditable, and sandboxed environment. Delegates all I/O.
Host JavaScript Implements the I/O operations requested by the runtime and provides external context.

The Two-Runtime System

PXYZ employs a dual-runtime strategy to balance developer productivity with production security. Both runtimes execute the identical graph.bin format, allowing workflows to be compiled once and run anywhere.

The decision for two runtimes is justified by auditability: "Security auditors can read 600 lines of WAT. They cannot read 2000 lines of Rust with its abstractions. The Rust runtime is for you. The WASM runtime is for them."

Component Language Lines of Code (approx.) Purpose
Native Runtime Rust ~1500 Development, testing, and debugging. Full-featured and easier to instrument.
WASM Runtime WAT ~500-700 Production deployment. Minimal, formally verifiable, and designed for security audits.

2. The Compilation Pipeline and Graph Model

The PXYZ compiler transforms an XML definition into a compact binary. This process involves multiple stages and operates on a clearly defined graph structure.

From XML to Binary

The compilation pipeline is an assembly line that converts high-level logic into an executable binary:

1. Parse XML: The source workflow.xml is parsed into an Abstract Syntax Tree (AST), representing all defined workflows, nodes, edges, and predicates.
2. Lower to IR: The AST is "lowered" into a Graph Intermediate Representation (IR). This step resolves names, assigns unique IDs, and creates a structured graph that is easier to validate and optimize.
3. Compile Predicates: Predicate expressions are compiled into bytecode for the system's simple, stack-based Predicate VM.
4. Validate: The IR is subjected to the comprehensive three-layer constraint system (see Section 3).
5. Optimize: The IR undergoes optimization passes, including dead code elimination and predicate deduplication.
6. Emit Binary: The final, optimized IR is serialized into the graph.bin format.

The Graph as a Directed Acyclic Graph (DAG)

The core data structure is a Directed Acyclic Graph (DAG), which is explicitly a "map, not code." This structure is composed of nodes that represent actions and edges that represent the flow between them.

Node Kinds and Flags

Nodes are the stations on the map where work is done. Each node has a specific kind and can be modified by bitfield flags.

Node Kinds | Value | Kind | Purpose | | :--- | :--- | :--- | | 0 | Transform | Validates or transforms data in the workflow context. | | 1 | External | Calls an I/O handler in the host, identified by an op_code. | | 2 | Render | Generates HTML or other user-facing output. | | 3 | Signal | Emits UI signals for front-end frameworks. | | 4 | Auth | Checks a predicate and fails the traversal if it returns false. | | 5 | Terminal | Ends the graph traversal, returning a final status. | | 6 | Error | A designated node for handling errors from other nodes. |

Node Flags (Bitfield) | Bit | Flag | Meaning | | :--- | :--- | :--- | | 0 | ASYNC | Indicates the node's operation may block and can be executed asynchronously. | | 1 | REQUIRES_AUTH | The node execution must pass an authentication predicate. | | 2 | HAS_SIDE_EFFECTS | The node produces side effects (e.g., writes to a database). | | 3 | IRREVERSIBLE | The node's action cannot be undone (e.g., sending an email). | | 4 | REQUIRES_HUMAN | A human actor is required for this node to execute. | | 5 | CACHEABLE | The result of this node's execution can be memoized. |

Edge Structure and Flags

Edges connect nodes and define the possible paths of traversal. An edge's traversal can be conditional based on the result of a predicate evaluation.

Edge Flags (Bitfield) | Bit | Flag | Meaning | | :--- | :--- | :--- | | 0 | PARALLEL | The edge can be traversed in parallel with its siblings. | | 1 | FALLBACK | The edge is taken only if other sibling paths fail. | | 2 | ERROR_EDGE | The edge leads to an error handling node. |

Binary Format (graph.bin)

The graph.bin file is the portable, compiled artifact. It consists of a header followed by data sections for nodes, edges, predicates, strings, and entry points.

Header (96 bytes) | Offset | Size (bytes) | Field | Description | | :--- | :--- | :--- | :--- | | 0x00 | 4 | Magic Number | 0x504E5958 ("PXYZ") | | 0x04 | 2 | Version Major | The major version of the binary format. | | 0x06 | 2 | Version Minor | The minor version of the binary format. | | 0x08 | 4 | Node Count | Total number of nodes in the graph. | | 0x0C | 4 | Edge Count | Total number of edges in the graph. | | 0x10 | 4 | Predicate Count | Total number of compiled predicates. | | 0x14 | 4 | String Pool Size | Total size of the string pool in bytes. | | 0x18 | 4 | Entry Count | Number of entry points into the graph. | | 0x20 | 32 | Source Hash | SHA-256 hash of the source XML file. | | 0x40 | 4 | Nodes Offset | File offset to the start of the node table. | | 0x44 | 4 | Edges Offset | File offset to the start of the edge table. | | 0x48 | 4 | Predicates Offset | File offset to the start of the predicate data. | | 0x4C | 4 | Strings Offset | File offset to the start of the string pool. | | 0x50 | 4 | Entries Offset | File offset to the start of the entry point table. |

Node Entry (16 bytes) | Offset | Size (bytes) | Field | | :--- | :--- | :--- | | 0x00 | 4 | Node ID | | 0x04 | 1 | Kind (0-6) | | 0x05 | 1 | Flags | | 0x06 | 2 | Op Code | | 0x08 | 4 | Data Offset (in string pool) | | 0x0C | 2 | Edge Start Index | | 0x0E | 2 | Edge Count |

Edge Entry (12 bytes) | Offset | Size (bytes) | Field | | :--- | :--- | :--- | | 0x00 | 4 | Target Node ID | | 0x04 | 2 | Predicate ID (0 = always) | | 0x06 | 2 | (reserved) | | 0x08 | 2 | Weight | | 0x0A | 2 | Flags |

3. Safety, Validation, and Bounded Execution

The PXYZ architecture is defined by its multi-layered approach to safety, which combines a bounded execution model with a comprehensive compile-time constraint system.

The Predicate VM

Predicates are simple yes/no questions that control graph traversal. They execute in a tiny, stack-based virtual machine with hard safety limits designed to prevent runaway execution.

Predicate VM Safety Limits | Limit | Value | Purpose | | :--- | :--- | :--- | | MAX_PREDICATE_STEPS | 256 | Prevents infinite loops within a single predicate. | | MAX_STACK_DEPTH | 16 | Prevents stack overflow from deeply nested expressions. | | MAX_CALL_DEPTH | 4 | Prevents infinite recursion from predicates calling other predicates. | | MAX_PREDICATE_BYTECODE | 256 bytes | Limits the complexity and size of any single predicate. | | MAX_VISITED_NODES | 1000 | Prevents runaway graph traversal and ensures termination. |

Predicate VM Opcodes The VM supports a small set of opcodes for stack manipulation, comparison, logic, and host interaction. | Category | Opcodes | | :--- | :--- | | Stack/Load | NOOP, PUSH_INT, PUSH_STR, LOAD_VAR, LOAD_FIELD | | Comparison | EQ, NEQ, GT, GTE, LT, LTE | | Logical | AND, OR, NOT | | String/Array | CONTAINS, MATCHES, STARTS_WITH, ENDS_WITH, LEN, GET | | Utility | IS_NULL, IS_DEFINED, IS_CONFIRMED | | Merge/CRDT | TIMESTAMP (0x50), IS_FLAGGED (0x51), ORIGIN (0x52), VCLOCK_GT (0x53), MERGE_FIELD (0x54) | | Control | CALL_PRED, RET |

**The Y-Constraint Insight**: The predicate VM is not just for edge traversal. It is a **unified constraint evaluation engine** called from five different application points:

| Application Point | Context Variables | Purpose |
|-------------------|-------------------|---------|
| Edge Traversal | $token, $entity, $input, $state | Decide which path to take |
| Auth Nodes | $token, $entity | Check user permissions |
| Input Validation | $input | Validate incoming data |
| CRDT Merge | $a, $b, $candidate | Resolve data conflicts |
| Projection Views | $entity, $token | Filter query results |

This unification means one predicate definition can be reused across all contexts, and all contexts benefit from the same safety bounds.

The Three-Layer Constraint System

The compiler validates the graph against a hierarchy of checks to catch errors ranging from structural mistakes to dangerous business logic patterns.

Syntactic (SYN) - Structure Validation These checks ensure the graph is well-formed. | Code | Check Description | | :--- | :--- | | SYN001 | Edge targets must reference existing nodes. | | SYN002 | Entry points must reference existing nodes. | | SYN003 | Predicate references must be valid. | | SYN004 | No duplicate node IDs. | | SYN005 | At least one entry point must be defined. | | SYN006 | No duplicate entry points (same P, X coordinates). | | SYN007 | Edge sources must reference existing nodes. |

Semantic (SEM) - Logic Validation These checks ensure the graph is logically coherent. | Code | Check Description | | :--- | :--- | | SEM001 | Auth nodes must have associated predicates. | | SEM002 | External nodes must have op codes. | | SEM003 | Terminal nodes should not have outgoing edges. | | SEM004 | The graph must contain no cycles (must be a DAG). | | SEM005 | All nodes must be reachable from an entry point. | | SEM006 | Error nodes should have incoming edges. | | SEM007 | Render nodes should have associated templates. |

Pragmatic (PRAG) - Business Rule & Safety Validation These checks enforce high-level safety policies. | Code | Check Description | | :--- | :--- | | PRAG001 | A path from an LLM node to an irreversible node requires a validation gate (e.g., an Auth or Transform node). | | PRAG002 | Data write operations should have defined error-handling branches. | | PRAG003 | Irreversible actions must have a human-in-the-loop on their execution path. | | PRAG004 | Irreversible actions require confirmed inputs and cannot be triggered by suggested data. | | PRAG005 | Data marked as "quarantined" cannot flow to external nodes with side effects. |

Explicit I/O and The Irreversibility Spectrum

All side effects are made explicit through External nodes, which call the host with a specific operation code (op_code). This allows auditors to easily identify and analyze all points where the system interacts with the outside world.

"Auditors can grep for 0x0340 to find all email sends."

Operations are categorized, and some are explicitly marked as irreversible, which subjects them to stricter pragmatic validation checks at compile time.

IO Operation Code Examples | Category | Opcodes | | :--- | :--- | | Entity | 0x0100 ENTITY_CREATE, 0x0103 ENTITY_DELETE | | Google Workspace | 0x0320 GOOGLE_DRIVE_SEARCH, 0x0332 GOOGLE_GMAIL_SEND ⚠️ IRREVERSIBLE | | Communication | 0x0340 EMAIL_SEND ⚠️ IRREVERSIBLE, 0x0360 WEBHOOK_CALL ⚠️ IRREVERSIBLE | | HTTP | 0x0400 HTTP_GET, 0x0401 HTTP_POST | | AI/LLM | 0x0800 LLM_COMPLETE, 0x0810 LOCAL_MODEL_RUN | | Storage | 0x0900 STORAGE_GET, 0x0901 STORAGE_SET |

CRDT Merge Policies

PXYZ supports conflict resolution for distributed/offline data through merge policies. These policies use the same predicate VM, with merge-specific opcodes:

| Opcode | Code | Description |
|--------|------|-------------|
| TIMESTAMP | 0x50 | Get timestamp of a value (for LWW/FWW) |
| IS_FLAGGED | 0x51 | Check if value is flagged for human review |
| ORIGIN | 0x52 | Get origin/author of a value |
| VCLOCK_GT | 0x53 | Compare vector clocks for dominance |
| MERGE_FIELD | 0x54 | Load field from merge context ($a, $b, $candidate) |

Built-in merge policies:

| Policy | Behavior |
|--------|----------|
| `lww` | Last Writer Wins - higher timestamp wins |
| `fww` | First Writer Wins - lower timestamp wins (immutable) |
| `vclock` | Vector clock dominance for true causality |
| `max` / `min` | Numeric comparison |
| `union` / `intersect` | Set operations |
| `human-review` | Flag for manual resolution |

Merge policies are declared per-entity with field-level overrides:

```xml
<merge>
  <entity name="Contact" default="lww">
    <field name="email" policy="fww"/>
    <field name="tags" policy="union"/>
  </entity>
</merge>
```

4. Tooling and Outputs

The PXYZ ecosystem includes a command-line interface and supports various output formats to aid in development, debugging, and auditing.

The PXYZ Command-Line Interface (CLI)

A CLI tool named pxyz provides the main interface for working with the compiler.

Command Description
pxyz compile Compiles a workflow.xml file into a graph.bin binary. Can optionally generate an audit.json file.
pxyz inspect Displays summary information from a compiled graph.bin file in text, JSON, or Mermaid format.
pxyz check Runs the full validation suite on a workflow.xml file without producing a binary output.
pxyz init Creates a new project directory with a minimal workflow.xml file and a build script.

Compiler Outputs

In addition to the primary graph.bin artifact, the compiler toolchain can generate several other outputs for different purposes.

* audit.json: A JSON file generated during compilation that contains metadata about the build, including source and binary hashes, graph statistics (node/edge counts), and a detailed report of the results from the Syntactic, Semantic, and Pragmatic validation checks.
* Mermaid Diagram: A text-based representation of the graph IR that can be rendered into a flowchart diagram, providing a visual map of the workflow logic.
* Text Dump: A human-readable summary of the graph IR, detailing all entry points, nodes, edges, predicates (with disassembled bytecode), and the contents of the string pool.
