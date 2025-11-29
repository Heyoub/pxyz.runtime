# PXYZ Architecture: A Staff Engineer's Guide

> **For readers with aphantasia**: This document uses concrete metaphors and explicit ASCII diagrams instead of asking you to "imagine" or "picture" things. Every concept is grounded in a physical or procedural analogy.

---

## The Central Insight

PXYZ answers one question: **How do you let AI agents do dangerous things safely?**

The answer: You don't write code that *does* things. You write a *map* of what things can be done, under what conditions, and the runtime *walks* that map. The map is finite, auditable, and the walker has hard limits.

```shell
Traditional Approach          PXYZ Approach
==================           ==============

if user.is_admin:            ┌─────────────────────┐
    send_email()             │ graph.bin (the map) │
                             └──────────┬──────────┘
                                        │
Code decides → Side effect   Map exists │ Walker reads map
                                        │
                             ┌──────────▼──────────┐
                             │  Runtime (walker)   │
                             │  - bounded steps    │
                             │  - explicit IO      │
                             │  - can be audited   │
                             └─────────────────────┘
```

---

## The Two Runtimes (Why Two?)

```shell
┌─────────────────────────────────────────────────────────────────────┐
│                        YOUR WORKFLOW.XML                            │
│                                                                     │
│    <workflow id="contact_search">                                   │
│      <entry p="contact" x="search" node="validate"/>               │
│      ...                                                           │
│    </workflow>                                                      │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    RUST COMPILER (~2000 lines)                      │
│                                                                     │
│    main/src/dsl/       → Parse XML into AST                        │
│    main/src/compiler/  → Transform, validate, optimize             │
│    main/src/emit/      → Generate outputs                          │
│                                                                     │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        graph.bin (~KB)                              │
│                                                                     │
│    Compact binary: nodes, edges, predicates, string pool           │
│    Same format consumed by BOTH runtimes                           │
│                                                                     │
└───────────────────┬─────────────────────────┬───────────────────────┘
                    │                         │
        ┌───────────▼───────────┐ ┌───────────▼───────────┐
        │   RUST RUNTIME        │ │   WASM RUNTIME        │
        │   (~1500 lines)       │ │   (~600 lines)        │
        │                       │ │                       │
        │   FOR: Development    │ │   FOR: Production     │
        │        Testing        │ │        Audit          │
        │        Debugging      │ │        Browser        │
        │                       │ │                       │
        │   Rich error msgs     │ │   Minimal surface     │
        │   Full Rust tooling   │ │   Formally bounded    │
        └───────────────────────┘ └───────────────────────┘
```

**Why two?** Security auditors can read 600 lines of WAT. They cannot read 2000 lines of Rust with its abstractions. The Rust runtime is for *you*. The WASM runtime is for *them*.

---

## The Compilation Pipeline

Think of this as an assembly line where XML enters one end and `graph.bin` exits the other. Each station does one job.

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│  STATION 1: PARSE                                                           │
│  main/src/dsl/parser.rs                                                     │
│                                                                             │
│  INPUT:  Raw XML text                                                       │
│  OUTPUT: Abstract Syntax Tree (AST)                                         │
│                                                                             │
│  Job: Turn angle brackets into structured data. No validation yet.         │
│       Just "does this XML make sense as XML?"                              │
│                                                                             │
│  ┌──────────────────────┐         ┌──────────────────────────────────┐     │
│  │ <node id="auth"      │         │ Node {                           │     │
│  │       kind="auth">   │   ───►  │   id: "auth",                    │     │
│  │   <require           │         │   kind: "auth",                  │     │
│  │     predicate="x"/>  │         │   predicate: Some("x")           │     │
│  │ </node>              │         │ }                                │     │
│  └──────────────────────┘         └──────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  STATION 2: LOWER                                                           │
│  main/src/compiler/lower.rs                                                 │
│                                                                             │
│  INPUT:  AST (structured but still "XML-shaped")                           │
│  OUTPUT: Graph IR (structured for machine processing)                       │
│                                                                             │
│  Job: Assign numeric IDs, resolve references, compute hashes.              │
│       Turn names into numbers. Turn strings into offsets.                  │
│                                                                             │
│  ┌────────────────────────┐       ┌──────────────────────────────────┐     │
│  │ Node {                 │       │ GNode {                          │     │
│  │   id: "auth",          │       │   id: 3,           // numeric    │     │
│  │   kind: "auth",        │ ───►  │   kind: Auth,      // enum       │     │
│  │   predicate: "is_admin"│       │   auth_predicate: Some(1),       │     │
│  │ }                      │       │   data_offset: 42, // string pool│     │
│  └────────────────────────┘       │ }                                │     │
│                                   └──────────────────────────────────┘     │
│                                                                             │
│  KEY INSIGHT: The "lowerer" also builds the ENTRY TABLE.                   │
│               Entry = hash(P, X) → starting node ID                        │
│               When you call execute("contact", "search"), this is how      │
│               the runtime finds where to start.                            │
└─────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  STATION 3: COMPILE PREDICATES                                              │
│  main/src/compiler/bytecode.rs                                              │
│                                                                             │
│  INPUT:  Predicate expressions (AST form)                                  │
│  OUTPUT: Bytecode (stack machine instructions)                              │
│                                                                             │
│  Job: Turn human-readable conditions into machine-executable instructions. │
│                                                                             │
│  ┌────────────────────────────┐   ┌──────────────────────────────────┐     │
│  │ <eq left="$token.role"     │   │ LOAD_VAR "token.role"  // push   │     │
│  │     right="admin"/>        │   │ PUSH_STR "admin"       // push   │     │
│  │                            │──►│ EQ                     // pop 2, │     │
│  │ (if token.role == "admin") │   │                        // push 1 │     │
│  └────────────────────────────┘   │ RET                    // return │     │
│                                   └──────────────────────────────────┘     │
│                                                                             │
│  The bytecode is a STACK MACHINE. Think of it like RPN on a calculator:   │
│                                                                             │
│     Stack: []                                                               │
│     LOAD_VAR "token.role"  →  Stack: ["admin"]     (from context)          │
│     PUSH_STR "admin"       →  Stack: ["admin", "admin"]                    │
│     EQ                     →  Stack: [true]        (they match!)           │
│     RET                    →  Returns true                                  │
└─────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  STATION 4: VALIDATE                                                        │
│  main/src/compiler/syntactic.rs  (SYN001-007)                              │
│  main/src/compiler/semantic.rs   (SEM001-007)                              │
│  main/src/compiler/pragmatic.rs  (PRAG001-005)                             │
│                                                                             │
│  INPUT:  Graph IR                                                          │
│  OUTPUT: List of diagnostics (errors, warnings)                            │
│                                                                             │
│  Job: Catch mistakes at compile time, not runtime.                         │
│                                                                             │
│  THREE LAYERS OF VALIDATION:                                                │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ SYNTACTIC: "Is the structure valid?"                                │   │
│  │                                                                     │   │
│  │   SYN001: Edge targets exist       SYN005: Has entry points        │   │
│  │   SYN002: Entry nodes exist        SYN006: No duplicate entries    │   │
│  │   SYN003: Predicates exist         SYN007: Edge sources exist      │   │
│  │   SYN004: No duplicate node IDs                                    │   │
│  │                                                                     │   │
│  │   These catch typos. "You said go to node X but X doesn't exist."  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                              │                                              │
│                              ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ SEMANTIC: "Does the logic make sense?"                              │   │
│  │                                                                     │   │
│  │   SEM001: Auth nodes have predicates    SEM005: All nodes reachable│   │
│  │   SEM002: External nodes have opcodes   SEM006: Error nodes used   │   │
│  │   SEM003: Terminals have no outgoing    SEM007: Renders have HTML  │   │
│  │   SEM004: No cycles (DAG only)                                     │   │
│  │                                                                     │   │
│  │   These catch logic errors. "You made an auth gate with no lock."  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                              │                                              │
│                              ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ PRAGMATIC: "Is this actually safe?"                                 │   │
│  │                                                                     │   │
│  │   PRAG001: LLM → irreversible needs validation gate                │   │
│  │   PRAG002: Write operations should have error branches             │   │
│  │   PRAG003: Irreversible actions require human in path              │   │
│  │   PRAG004: Suggested data needs confirmation before irreversible   │   │
│  │   PRAG005: Quarantined data can't escape to external operations    │   │
│  │                                                                     │   │
│  │   These catch DANGEROUS patterns. "AI generated this email draft.  │   │
│  │   Are you sure you want to send it without a human looking first?" │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  The pragmatic checks are what make PXYZ different from a generic         │
│  workflow engine. They encode POLICY about AI safety into the compiler.   │
└─────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  STATION 5: OPTIMIZE                                                        │
│  main/src/compiler/optimize.rs                                              │
│                                                                             │
│  INPUT:  Validated Graph IR                                                │
│  OUTPUT: Optimized Graph IR                                                 │
│                                                                             │
│  Job: Make the output smaller and faster without changing meaning.         │
│                                                                             │
│  THREE OPTIMIZATIONS:                                                       │
│                                                                             │
│  1. DEAD CODE ELIMINATION                                                   │
│     Remove nodes that can't be reached from any entry point.               │
│     (Why ship code that can never run?)                                    │
│                                                                             │
│     ┌─────┐     ┌─────┐     ┌─────┐         ┌─────┐     ┌─────┐           │
│     │START│────►│  A  │────►│ END │         │START│────►│  A  │───►│END│  │
│     └─────┘     └─────┘     └─────┘   ───►  └─────┘     └─────┘           │
│                                                                             │
│     ┌─────┐ (orphan, unreachable)           (removed)                      │
│     │DEAD │                                                                 │
│     └─────┘                                                                 │
│                                                                             │
│  2. PREDICATE DEDUPLICATION                                                 │
│     If two predicates compile to identical bytecode, keep only one.        │
│     (Why store the same thing twice?)                                      │
│                                                                             │
│  3. EDGE ORDERING                                                           │
│     Sort edges by weight so high-priority paths are checked first.         │
│     (Faster common-case execution)                                         │
└─────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  STATION 6: EMIT                                                            │
│  main/src/emit/binary.rs   → graph.bin                                     │
│  main/src/emit/audit.rs    → graph.audit.json                              │
│  main/src/emit/text.rs     → human-readable dump                           │
│  main/src/emit/mermaid.rs  → flowchart diagram                             │
│                                                                             │
│  INPUT:  Optimized Graph IR                                                │
│  OUTPUT: Final artifacts                                                    │
│                                                                             │
│  Job: Serialize the IR into formats the world can use.                     │
│                                                                             │
│  BINARY FORMAT (graph.bin):                                                 │
│                                                                             │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  HEADER (96 bytes)                                                 │    │
│  │  ┌────────┬────────┬────────┬────────┬────────┬────────────────┐  │    │
│  │  │ MAGIC  │VERSION │ COUNTS │OFFSETS │ SHA256 │   (padding)    │  │    │
│  │  │ "PXYZ" │ 1.0    │ n,e,p  │ to data│ of src │                │  │    │
│  │  └────────┴────────┴────────┴────────┴────────┴────────────────┘  │    │
│  ├────────────────────────────────────────────────────────────────────┤    │
│  │  NODES (16 bytes each)                                            │    │
│  │  ┌────────┬────────┬────────┬────────┬────────┬────────────────┐  │    │
│  │  │ ID     │ KIND   │ OP CODE│ DATA   │ EDGE   │ EDGE COUNT     │  │    │
│  │  │ u32    │ u8     │ u16    │ OFFSET │ START  │ u16            │  │    │
│  │  └────────┴────────┴────────┴────────┴────────┴────────────────┘  │    │
│  ├────────────────────────────────────────────────────────────────────┤    │
│  │  EDGES (12 bytes each)                                            │    │
│  │  ┌────────┬────────┬────────┬────────┬────────────────────────┐   │    │
│  │  │ TARGET │ PRED ID│RESERVED│ WEIGHT │ FLAGS                  │   │    │
│  │  │ u32    │ u16    │ u16    │ u16    │ u16                    │   │    │
│  │  └────────┴────────┴────────┴────────┴────────────────────────┘   │    │
│  ├────────────────────────────────────────────────────────────────────┤    │
│  │  PREDICATES (variable length bytecode)                            │    │
│  ├────────────────────────────────────────────────────────────────────┤    │
│  │  STRING POOL (null-terminated strings)                            │    │
│  ├────────────────────────────────────────────────────────────────────┤    │
│  │  ENTRY TABLE (8 bytes each: hash → node_id)                       │    │
│  └────────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## The Graph Model

Your workflow is a **directed acyclic graph (DAG)**. Not code. Not a state machine. A map.

```shell
                         ┌─────────────────────────────────────────────┐
                         │              CONCEPTUAL MODEL               │
                         └─────────────────────────────────────────────┘

    NODES are stations where things happen.
    EDGES are paths between stations, guarded by predicates.
    PREDICATES are questions with yes/no answers.

                    ┌──────────────────────────────────────────┐
                    │                 WORKFLOW                 │
                    │                                          │
                    │  ┌─────────┐    is_admin?   ┌─────────┐ │
         entry      │  │TRANSFORM│───────────────►│  AUTH   │ │
    ────(P,X)──────►│  │(validate│    !is_admin?  │(check   │ │
                    │  │ input)  │────────┐       │ perms)  │ │
                    │  └─────────┘        │       └────┬────┘ │
                    │                     │            │      │
                    │                     ▼            ▼      │
                    │              ┌─────────┐   ┌─────────┐  │
                    │              │  ERROR  │   │EXTERNAL │  │
                    │              │(403 deny│   │(call API│  │
                    │              └─────────┘   └────┬────┘  │
                    │                                 │       │
                    │                                 ▼       │
                    │                           ┌─────────┐   │
                    │                           │ RENDER  │   │
                    │                           │(HTML out│   │
                    │                           └────┬────┘   │
                    │                                │        │
                    │                                ▼        │
                    │                           ┌─────────┐   │
                    │                           │TERMINAL │   │
                    │                           │(done!)  │   │
                    │                           └─────────┘   │
                    └──────────────────────────────────────────┘
```

### Node Kinds (What Stations Do)

```shell
┌─────────────┬─────────────────────────────────────────────────────────────┐
│ KIND        │ WHAT IT DOES                                                │
├─────────────┼─────────────────────────────────────────────────────────────┤
│ Transform   │ Validate or reshape data. Pure computation. No side effects│
│             │ Example: Check that email is valid format, parse JSON      │
├─────────────┼─────────────────────────────────────────────────────────────┤
│ External    │ Call the outside world via IO handler. Has an OP CODE.     │
│             │ Example: 0x0300 = Google Contacts Search                   │
│             │          0x0340 = Send Email (IRREVERSIBLE!)               │
├─────────────┼─────────────────────────────────────────────────────────────┤
│ Render      │ Generate HTML output to send to browser/UI.                │
│             │ Has a template reference and optional CSS selector.        │
├─────────────┼─────────────────────────────────────────────────────────────┤
│ Signal      │ Emit events to the UI (loading spinners, progress, etc.)   │
│             │ Fire-and-forget notifications.                             │
├─────────────┼─────────────────────────────────────────────────────────────┤
│ Auth        │ Check a predicate. If false, traversal STOPS here.         │
│             │ This is a GATE. You can't pass without the right key.      │
├─────────────┼─────────────────────────────────────────────────────────────┤
│ Terminal    │ End of the line. Return a status code. Done.               │
│             │ Example: 200 OK, 404 Not Found                             │
├─────────────┼─────────────────────────────────────────────────────────────┤
│ Error       │ Error handler. Something went wrong. Clean up here.        │
│             │ Example: Log failure, show error page                      │
└─────────────┴─────────────────────────────────────────────────────────────┘
```

### Node Flags (Properties)

```shell
┌──────────────────────────────────────────────────────────────────────────┐
│ FLAGS ARE BITS. Each flag is a single bit in a byte.                    │
│ A node can have multiple flags set simultaneously.                       │
│                                                                          │
│  Bit 0 (ASYNC):           Node may block, await IO                      │
│  Bit 1 (REQUIRES_AUTH):   Must pass predicate first                     │
│  Bit 2 (HAS_SIDE_EFFECTS): Does something that changes the world        │
│  Bit 3 (IRREVERSIBLE):    Cannot be undone (email sent, webhook called) │
│  Bit 4 (REQUIRES_HUMAN):  Human must be in the loop                     │
│  Bit 5 (CACHEABLE):       Result can be memoized                        │
│                                                                          │
│  Example: A node with flags = 0b00001100 (decimal 12)                   │
│           Has: HAS_SIDE_EFFECTS (bit 2) + IRREVERSIBLE (bit 3)          │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## The Predicate VM

Predicates answer yes/no questions. They run in a tiny **stack-based virtual machine** with hard safety limits.

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PREDICATE VM EXECUTION MODEL                        │
└─────────────────────────────────────────────────────────────────────────────┘

    The VM has:
    - A STACK (max 16 values)
    - A PROGRAM COUNTER (where are we in the bytecode?)
    - Access to CONTEXT (variables like $token.role, $entity.owner)
    - HARD LIMITS (max 256 steps, max 4 call depth)

    ┌─────────────────────────────────────────────────────────────────────┐
    │                                                                     │
    │  BYTECODE: [LOAD_VAR "token.role", PUSH_STR "admin", EQ, RET]       │
    │                                                                     │
    │  STEP 1: LOAD_VAR "token.role"                                      │
    │          Look up token.role in context → "admin"                    │
    │          Push onto stack                                            │
    │                                                                     │
    │          Stack: ["admin"]                                           │
    │                  ▲                                                  │
    │                  │                                                  │
    │  STEP 2: PUSH_STR "admin"                                           │
    │          Push literal string "admin"                                │
    │                                                                     │
    │          Stack: ["admin", "admin"]                                  │
    │                  ▲        ▲                                         │
    │                  │        │                                         │
    │  STEP 3: EQ                                                         │
    │          Pop 2 values, compare, push result                         │
    │                                                                     │
    │          Stack: [true]                                              │
    │                  ▲                                                  │
    │                  │                                                  │
    │  STEP 4: RET                                                        │
    │          Pop top of stack, return it                                │
    │                                                                     │
    │          RESULT: true                                               │
    │                                                                     │
    └─────────────────────────────────────────────────────────────────────┘
```

### Why a Stack Machine?

```shell
┌────────────────────────────────────────────────────────────────────────────┐
│                                                                            │
│  REGISTER MACHINES:              STACK MACHINES:                           │
│  - Faster on real CPUs           - Simpler to implement                    │
│  - Harder to verify              - Easier to verify                        │
│  - Variable instruction size     - Compact bytecode                        │
│                                                                            │
│  For predicates (simple boolean logic), stack machines are ideal.          │
│  The bytecode is tiny, the VM is tiny, auditors can understand it.         │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### Predicate Opcodes

```shell
┌────────┬─────────────────┬────────────────────────────────────────────────┐
│ OPCODE │ NAME            │ WHAT IT DOES                                   │
├────────┼─────────────────┼────────────────────────────────────────────────┤
│  0x01  │ PUSH_INT        │ Push a 32-bit integer onto stack               │
│  0x02  │ PUSH_STR        │ Push string (by offset) onto stack             │
│  0x03  │ LOAD_VAR        │ Load variable from context onto stack          │
│  0x04  │ LOAD_FIELD      │ Get field from object on stack                 │
├────────┼─────────────────┼────────────────────────────────────────────────┤
│  0x10  │ EQ              │ Pop 2, push (a == b)                           │
│  0x11  │ NEQ             │ Pop 2, push (a != b)                           │
│  0x12  │ GT              │ Pop 2, push (a > b)                            │
│  0x13  │ GTE             │ Pop 2, push (a >= b)                           │
│  0x14  │ LT              │ Pop 2, push (a < b)                            │
│  0x15  │ LTE             │ Pop 2, push (a <= b)                           │
├────────┼─────────────────┼────────────────────────────────────────────────┤
│  0x20  │ AND             │ Pop 2, push (a && b)                           │
│  0x21  │ OR              │ Pop 2, push (a || b)                           │
│  0x22  │ NOT             │ Pop 1, push (!a)                               │
├────────┼─────────────────┼────────────────────────────────────────────────┤
│  0x30  │ CONTAINS        │ Pop 2, push (haystack contains needle)         │
│  0x31  │ MATCHES         │ Pop 1 + pattern, push (regex match)            │
│  0x32  │ STARTS_WITH     │ Pop 2, push (string starts with prefix)        │
│  0x33  │ ENDS_WITH       │ Pop 2, push (string ends with suffix)          │
├────────┼─────────────────┼────────────────────────────────────────────────┤
│  0x40  │ LEN             │ Pop 1, push length (string/array)              │
│  0x42  │ IS_NULL         │ Pop 1, push (value is null)                    │
│  0x43  │ IS_DEFINED      │ Pop 1, push (value is defined)                 │
│  0x44  │ IS_CONFIRMED    │ Pop 1, push (data confirmed by human)          │
├────────┼─────────────────┼────────────────────────────────────────────────┤
│  0x50  │ TIMESTAMP       │ Pop 1, push timestamp (for LWW merge)          │
│  0x51  │ IS_FLAGGED      │ Pop 1, push (flagged for human review)         │
│  0x52  │ ORIGIN          │ Pop 1, push origin/author ID                   │
│  0x53  │ VCLOCK_GT       │ Pop 2, push (a's vclock dominates b)           │
│  0x54  │ MERGE_FIELD     │ Load field from merge context ($a/$b/$cand)    │
├────────┼─────────────────┼────────────────────────────────────────────────┤
│  0xF0  │ CALL_PRED       │ Call another predicate by ID                   │
│  0xFF  │ RET             │ Return top of stack as result                  │
└────────┴─────────────────┴────────────────────────────────────────────────┘
```

---

## The WASM Runtime Deep Dive

The WASM runtime (`wasm/pxyz.wat`) is the crown jewel. ~600 lines of WebAssembly Text format, designed to be read by security auditors.

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                        WASM RUNTIME MEMORY LAYOUT                           │
└─────────────────────────────────────────────────────────────────────────────┘

    Linear memory is divided into fixed regions:

    0x00000 ┌─────────────────────────────────────┐
            │         GRAPH DATA (64 KB)          │
            │  Header, nodes, edges, predicates   │
            │  String pool, entry table           │
    0x10000 ├─────────────────────────────────────┤
            │       VISITED BITMAP (64 KB)        │
            │  1 bit per node: have we been here? │
            │  Prevents infinite loops            │
    0x20000 ├─────────────────────────────────────┤
            │        IO BUFFER (64 KB)            │
            │  Data passed to/from host IO calls  │
    0x30000 ├─────────────────────────────────────┤
            │       PREDICATE STACK (64 KB)       │
            │  Stack for predicate VM execution   │
    0x40000 └─────────────────────────────────────┘
```

### Host Imports (The Only Escape Hatches)

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  The WASM runtime has NO direct access to:                                 │
│  - Network                                                                  │
│  - File system                                                              │
│  - System calls                                                             │
│  - Anything outside its memory sandbox                                      │
│                                                                             │
│  ALL external operations go through HOST IMPORTS:                          │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │ (import "host" "io_call" (func $io_call (param i32 i32 i32) (result i32)))│
│  │                                                                      │  │
│  │  Runtime calls: io_call(op_code, input_ptr, input_len)              │  │
│  │  Host does the actual work (HTTP, database, email, etc.)            │  │
│  │  Host returns result to runtime                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │ (import "host" "resolve_var" (func $resolve_var ...))                │  │
│  │                                                                      │  │
│  │  Predicate VM calls: resolve_var("token.role")                      │  │
│  │  Host looks up value in execution context                            │  │
│  │  Host writes result to memory, returns length                        │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │ (import "host" "is_confirmed" (func $is_confirmed (param i32) (result i32)))│
│  │                                                                      │  │
│  │  For PRAG004: Has this data been confirmed by a human?              │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  This architecture means:                                                   │
│  1. The WASM code can be audited independently of the host                 │
│  2. The host controls what "send email" actually means                     │
│  3. You can mock the host for testing                                      │
│  4. The runtime is portable (browser, Node, native)                        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Execution Flow

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                     RUNTIME EXECUTION: STEP BY STEP                         │
└─────────────────────────────────────────────────────────────────────────────┘

    1. LOAD GRAPH
       ──────────
       Host loads graph.bin into memory at offset 0x0000
       Runtime parses header, validates magic number "PXYZ"
       Now we have: node table, edge table, predicate table, string pool

    2. FIND ENTRY POINT
       ─────────────────
       Given (P="contact", X="search")
       Compute hash: FNV-1a("contact" + separator + "search") → 0xABCD1234
       Binary search entry table for hash → node_id 0

    3. TRAVERSAL LOOP
       ───────────────
       current_node = entry_node
       visited = empty set

       while true:
         │
         ├─► If visited[current_node]: STOP (cycle protection)
         │
         ├─► Mark visited[current_node] = true
         │
         ├─► Execute node based on kind:
         │     Transform: validate data
         │     External:  io_call(op_code, ...)
         │     Render:    io_call(RENDER, template, ...)
         │     Auth:      evaluate predicate, STOP if false
         │     Terminal:  return status, DONE
         │     Error:     return error status, DONE
         │
         ├─► Find outgoing edges from current_node
         │
         ├─► For each edge (in weight order):
         │     Evaluate edge predicate
         │     If predicate is true:
         │       current_node = edge.target
         │       break (take this path)
         │
         └─► If no edge matched and there are fallback edges:
               Take the first fallback edge

    4. SAFETY LIMITS
       ─────────────
       MAX_VISITED_NODES = 1000   (traversal terminates)
       MAX_PRED_STEPS = 256       (predicate VM terminates)
       MAX_PRED_DEPTH = 4         (nested predicate calls)
       MAX_STACK = 16             (predicate stack size)

       These are HARD LIMITS. If exceeded, execution STOPS with error.
       There is no way to disable them. This is by design.
```

---

## IO Operation Codes

When an External node executes, it calls `io_call(op_code, ...)`. The op code identifies what to do.

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                           IO OPERATION TAXONOMY                             │
└─────────────────────────────────────────────────────────────────────────────┘

    Op codes are 16-bit values organized by category:

    0x01xx  ENTITY OPERATIONS (internal CRUD)
    ──────────────────────────────────────────
    0x0100  ENTITY_CREATE      Create a new record
    0x0101  ENTITY_READ        Read a record
    0x0102  ENTITY_UPDATE      Update a record
    0x0103  ENTITY_DELETE      Delete a record
    0x0104  ENTITY_LIST        List records
    0x0105  ENTITY_SEARCH      Search records

    0x03xx  GOOGLE WORKSPACE
    ──────────────────────────────────────────
    0x0300  GOOGLE_CONTACTS_SEARCH    Search contacts
    0x0301  GOOGLE_CONTACTS_GET       Get contact
    0x0302  GOOGLE_CONTACTS_CREATE    Create contact
    0x0310  GOOGLE_CALENDAR_LIST      List calendar events
    0x0320  GOOGLE_DRIVE_SEARCH       Search drive files
    0x0330  GOOGLE_GMAIL_SEARCH       Search emails
    0x0332  GOOGLE_GMAIL_SEND         Send email ⚠️ IRREVERSIBLE

    0x034x-0x036x  COMMUNICATION ⚠️ ALL IRREVERSIBLE
    ──────────────────────────────────────────
    0x0340  EMAIL_SEND         ⚠️ Cannot unsend
    0x0350  SMS_SEND           ⚠️ Cannot unsend
    0x0360  WEBHOOK_CALL       ⚠️ May trigger external actions

    0x04xx  HTTP
    ──────────────────────────────────────────
    0x0400  HTTP_GET           Read-only
    0x0401  HTTP_POST          May have side effects
    0x0402  HTTP_PUT           May have side effects
    0x0403  HTTP_DELETE        May have side effects

    0x07xx  VECTOR/RAG
    ──────────────────────────────────────────
    0x0700  QDRANT_SEARCH          Vector similarity search
    0x0701  QDRANT_INDEX           Index new vectors
    0x0702  EMBEDDING_GENERATE     Generate embeddings

    0x08xx  AI/LLM
    ──────────────────────────────────────────
    0x0800  LLM_COMPLETE           Text completion
    0x0801  LLM_CLASSIFY           Classification
    0x0802  LLM_STRUCTURED         Structured output (JSON mode)
    0x0810  LOCAL_MODEL_RUN        Run local model

    0x09xx  STORAGE
    ──────────────────────────────────────────
    0x0900  STORAGE_GET            Get from key-value store
    0x0901  STORAGE_SET            Set in key-value store
    0x0910  EVENT_LOG_APPEND       Append to audit log
    0x0911  EVENT_LOG_QUERY        Query audit log
```

### The Irreversibility Spectrum

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│    SAFE                                                                     │
│    ─────────────────────────────────────────────────────────────────────►   │
│                                                                  DANGEROUS  │
│                                                                             │
│    ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────────────┐  │
│    │  READ   │  │  WRITE  │  │ DELETE  │  │  SEND   │  │ IRREVERSIBLE    │  │
│    │         │  │         │  │         │  │         │  │ EXTERNAL ACTION │  │
│    └─────────┘  └─────────┘  └─────────┘  └─────────┘  └─────────────────┘  │
│                                                                             │
│    HTTP_GET     ENTITY_      ENTITY_      EMAIL_      WEBHOOK_CALL          │
│    STORAGE_GET  UPDATE       DELETE       SEND        (anything could       │
│    LLM_COMPLETE STORAGE_SET               SMS_SEND     happen downstream)   │
│                                                                             │
│    Pragmatic checks (PRAG001-005) enforce that:                             │
│    - LLM output cannot directly reach IRREVERSIBLE without a gate           │
│    - IRREVERSIBLE requires human in the path somewhere                      │
│    - Suggested data must be confirmed before IRREVERSIBLE                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## File-by-File Reference

### Core Library

```shell
main/lib.rs
───────────
The public API. Exports:
  - compile(xml) → Result<graph.bin>
  - validate(xml) → Vec<Diagnostic>
  - NodeKind, ActorKind, SideEffects, etc.
  - hash_px() for entry point lookup
  - Opcode enum for predicate bytecode

main/main.rs
────────────
CLI entry point. Commands:
  - compile: workflow.xml → graph.bin
  - inspect: analyze graph.bin
  - check:   validate without emitting
  - init:    create new project
```

### DSL Module (Parsing)

```shell
main/src/dsl/ast.rs
───────────────────
Abstract Syntax Tree types. These mirror the XML structure:
  - OmarDocument (root)
  - Workflow, Node, Edge
  - PredicateExpr (the condition language)
  - Schema, Template, FieldDef

main/src/dsl/parser.rs
──────────────────────
XML → AST transformation using quick-xml.
Handles:
  - Schemas and field definitions
  - Named predicates
  - Workflows with nodes and edges
  - Templates with CDATA content
```

### Compiler Module (Transformation)

```shell
main/src/compiler/ir.rs
───────────────────────
Graph Intermediate Representation. Machine-friendly:
  - GNode: numeric IDs, kind enum, flags byte
  - GEdge: source/target IDs, predicate ID
  - GEntry: hash → node ID mapping
  - StringPool: interned strings with offsets
  - CompiledPredicate: bytecode + metadata

main/src/compiler/lower.rs
──────────────────────────
AST → IR transformation:
  - Assign numeric node IDs
  - Resolve string references to offsets
  - Compute entry point hashes
  - Build edge adjacency structure

main/src/compiler/bytecode.rs
─────────────────────────────
Predicate expression → bytecode:
  - Stack machine instruction emission
  - String interning into pool
  - Bytecode validation
  - Disassembler for debugging

main/src/compiler/syntactic.rs
──────────────────────────────
SYN001-007: Structure validation
  - References resolve
  - No duplicates
  - Required elements present

main/src/compiler/semantic.rs
─────────────────────────────
SEM001-007: Logic validation
  - Node types complete
  - No cycles (DAG check)
  - Reachability analysis

main/src/compiler/pragmatic.rs
──────────────────────────────
PRAG001-005: Safety validation
  - LLM → irreversible gating
  - Human-in-the-loop requirements
  - Data flow taint tracking

main/src/compiler/optimize.rs
─────────────────────────────
IR → IR optimization passes:
  - Dead code elimination
  - Predicate deduplication
  - Edge ordering by weight
```

### Emit Module (Output)

```shell
main/src/emit/binary.rs
───────────────────────
IR → graph.bin serialization:
  - 96-byte header
  - Node table (16 bytes each)
  - Edge table (12 bytes each)
  - Predicate bytecode
  - String pool
  - Entry table

main/src/emit/audit.rs
──────────────────────
IR → graph.audit.json:
  - Human-readable JSON
  - Full graph structure
  - For auditing and debugging

main/src/emit/text.rs
─────────────────────
Human-readable text dump:
  - Node listings with flags
  - Edge listings with predicates
  - Entry point table

main/src/emit/mermaid.rs
────────────────────────
IR → Mermaid flowchart syntax:
  - Visual graph representation
  - Renders in GitHub, IDEs, etc.
```

### WASM Runtime

```shell
wasm/pxyz.wat
─────────────
WebAssembly Text format runtime (~600 lines):

MEMORY REGIONS:
  - 0x00000: Graph data (64KB)
  - 0x10000: Visited bitmap (64KB)
  - 0x20000: IO buffer (64KB)
  - 0x30000: Predicate stack (64KB)

HOST IMPORTS:
  - io_call(op, ptr, len) → result
  - resolve_var(path_ptr, path_len, out_ptr) → len
  - is_confirmed(node_id) → bool
  - is_human(node_id) → bool
  - log(level, msg_ptr, len)
  - emit_event(type, data_ptr, len)

EXPORTS:
  - load_graph(ptr, len) → success
  - execute(p_ptr, p_len, x_ptr, x_len, ctx_ptr, ctx_len) → status
  - get_result_ptr() → ptr
  - get_result_len() → len
  - get_visit_count() → count

SAFETY CONSTANTS:
  - MAX_VISITED = 1000
  - MAX_PRED_STEPS = 256
  - MAX_PRED_DEPTH = 4
  - MAX_STACK = 16
```

---

## Why This Architecture Matters

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                     TRADITIONAL AGENT ARCHITECTURE                          │
│                                                                             │
│    User ──► LLM ──► "Call send_email(to='...', body='...')" ──► ?????      │
│                                                                             │
│    Problems:                                                                │
│    1. LLM decides what to do (unbounded)                                   │
│    2. No compile-time safety checks                                        │
│    3. Hard to audit what CAN happen                                        │
│    4. Hard to prove what CAN'T happen                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                        PXYZ ARCHITECTURE                                    │
│                                                                             │
│    ┌──────────────────────────────────────────────────────────────┐        │
│    │                     graph.bin (THE MAP)                      │        │
│    │                                                              │        │
│    │  All possible paths are KNOWN at compile time.              │        │
│    │  All predicates are BOUNDED.                                │        │
│    │  All IO is EXPLICIT.                                        │        │
│    │  Safety constraints are VERIFIED before deployment.         │        │
│    └──────────────────────────────────────────────────────────────┘        │
│                                │                                            │
│                                ▼                                            │
│    ┌──────────────────────────────────────────────────────────────┐        │
│    │                    WASM RUNTIME (THE WALKER)                 │        │
│    │                                                              │        │
│    │  600 lines. Auditable. Bounded. Sandboxed.                  │        │
│    │  Can only do what the map allows.                           │        │
│    │  Cannot escape the sandbox.                                 │        │
│    │  Cannot exceed the limits.                                  │        │
│    └──────────────────────────────────────────────────────────────┘        │
│                                                                             │
│    Benefits:                                                                │
│    1. LLM can navigate the graph, but can't leave it                       │
│    2. Compile-time: PRAG001 says "this graph is unsafe"                    │
│    3. Audit: "here are ALL paths to irreversible actions"                  │
│    4. Prove: "no path exists from LLM to email without human gate"         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## The Y-Constraint Layer: One Engine, Five Applications

The PXYZ coordinate system has four dimensions: **P** (entities), **X** (operations), **Y** (constraints), and **Z** (temporal). The Y dimension is often misunderstood as "just predicates for edges." It's actually something more powerful.

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│    THE KEY INSIGHT:                                                         │
│                                                                             │
│    Y is not "5 different things."                                          │
│    Y is ONE evaluation engine called from FIVE different points.           │
│                                                                             │
│    The Predicate VM (~20 opcodes, ~250 lines of WAT) is the ENGINE.        │
│    The application points are CONFIGURATION.                                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

                      ┌─────────────────────────────────────┐
                      │          PREDICATE VM               │
                      │  ┌───────────────────────────────┐  │
                      │  │ Stack-based bytecode executor │  │
                      │  │ ~20 opcodes, max 256 steps   │  │
                      │  │ Host imports for context     │  │
                      │  └───────────────────────────────┘  │
                      │              ▲ ▲ ▲ ▲ ▲              │
                      └──────────────┼─┼─┼─┼─┼──────────────┘
                                     │ │ │ │ │
    ┌────────────────────────────────┘ │ │ │ └────────────────────────────────┐
    │           ┌──────────────────────┘ │ └──────────────────────────┐       │
    │           │           ┌────────────┘           │                │       │
    ▼           ▼           ▼                        ▼                ▼       │
┌─────────┐ ┌─────────┐ ┌─────────┐ ┌──────────────────────┐ ┌─────────────┐  │
│ 1. EDGE │ │ 2. AUTH │ │ 3.INPUT │ │ 4. CRDT MERGE        │ │ 5. PROJECT  │  │
│TRAVERSAL│ │ NODES   │ │VALIDATE │ │    POLICIES          │ │   VIEWS     │  │
│         │ │         │ │         │ │                      │ │             │  │
│"can we  │ │"is user │ │"is this │ │"which version wins?" │ │"include in  │  │
│ go this │ │ allowed │ │ valid   │ │"needs human review?" │ │ this query?"│  │
│ way?"   │ │ here?"  │ │ input?" │ │                      │ │             │  │
└─────────┘ └─────────┘ └─────────┘ └──────────────────────┘ └─────────────┘  │
```

### The Five Application Points

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│  POINT 1: EDGE TRAVERSAL (Syntactic Y-constraints)                          │
│  ──────────────────────────────────────────────────────────────────────     │
│                                                                             │
│  When: Runtime evaluates edges to decide which path to take                 │
│  Context: $token, $entity, $input, $state                                   │
│  Question: "Given current state, should we take this edge?"                 │
│                                                                             │
│  Example:                                                                   │
│    <edge from="validate" to="process">                                      │
│      <when><eq left="$input.status" right="approved"/></when>              │
│    </edge>                                                                  │
│                                                                             │
│  Bytecode: LOAD_VAR "input.status", PUSH_STR "approved", EQ, RET           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  POINT 2: AUTH NODES (Semantic Y-constraints)                               │
│  ──────────────────────────────────────────────────────────────────────     │
│                                                                             │
│  When: Auth node evaluates permission before allowing traversal             │
│  Context: $token (JWT claims), $entity (resource being accessed)            │
│  Question: "Does this user have permission for this action?"                │
│                                                                             │
│  Example:                                                                   │
│    <predicate id="can_edit">                                                │
│      <or>                                                                   │
│        <eq left="$entity.owner_id" right="$token.sub"/>                    │
│        <contains left="$token.perms" right="admin"/>                       │
│      </or>                                                                  │
│    </predicate>                                                             │
│                                                                             │
│  Same VM, same opcodes, different application point.                        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  POINT 3: INPUT VALIDATION (Pragmatic Y-constraints)                        │
│  ──────────────────────────────────────────────────────────────────────     │
│                                                                             │
│  When: Transform node validates incoming data                               │
│  Context: $input (the data being validated)                                 │
│  Question: "Does this input meet our requirements?"                         │
│                                                                             │
│  Example:                                                                   │
│    <predicate id="valid_email">                                             │
│      <and>                                                                  │
│        <fn name="is_defined" arg="$input.email"/>                          │
│        <matches left="$input.email" pattern="^[^@]+@[^@]+$"/>              │
│      </and>                                                                 │
│    </predicate>                                                             │
│                                                                             │
│  New opcode: IS_DEFINED (0x43) - check if field exists                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  POINT 4: CRDT MERGE POLICIES (Conflict Resolution Y-constraints)           │
│  ──────────────────────────────────────────────────────────────────────     │
│                                                                             │
│  When: Two versions of data conflict and must be merged                     │
│  Context: $a (first version), $b (second version), $candidate (proposal)    │
│  Question: "Which version wins? Or do we need human review?"                │
│                                                                             │
│  NEW OPCODES for merge context:                                             │
│    0x50 TIMESTAMP   - Get timestamp of a value (for LWW)                   │
│    0x51 IS_FLAGGED  - Check if flagged for human review                    │
│    0x52 ORIGIN      - Get origin/author of a value                         │
│    0x53 VCLOCK_GT   - Compare vector clocks for dominance                  │
│    0x54 MERGE_FIELD - Load field from merge context ($a, $b, $candidate)   │
│                                                                             │
│  Example merge policy (LWW = Last Writer Wins):                             │
│    <merge>                                                                  │
│      <entity name="Contact" default="lww">                                 │
│        <field name="email" policy="fww"/>         <!-- immutable -->       │
│        <field name="tags" policy="union"/>        <!-- combine sets -->    │
│        <field name="notes" policy="human-review"/> <!-- needs human -->    │
│      </entity>                                                              │
│    </merge>                                                                 │
│                                                                             │
│  Built-in policies compile to bytecode:                                     │
│    LWW: timestamp($a) > timestamp($b) ? return 0 : return 1                │
│    FWW: timestamp($a) < timestamp($b) ? return 0 : return 1                │
│    VClock: vclock_gt($a, $b) ? return 0 : (vclock_gt($b, $a) ? return 1 : CONFLICT)│
│                                                                             │
│  Custom policies use full predicate language.                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  POINT 5: PROJECTION VIEWS (Query-time Y-constraints)                       │
│  ──────────────────────────────────────────────────────────────────────     │
│                                                                             │
│  When: Building a view of data, filtering what to include                   │
│  Context: $entity (each record), $token (viewer's permissions)              │
│  Question: "Should this record appear in this view for this user?"          │
│                                                                             │
│  Example:                                                                   │
│    <predicate id="visible_to_user">                                         │
│      <or>                                                                   │
│        <eq left="$entity.visibility" right="public"/>                      │
│        <eq left="$entity.owner_id" right="$token.sub"/>                    │
│        <contains left="$entity.shared_with" right="$token.sub"/>           │
│      </or>                                                                  │
│    </predicate>                                                             │
│                                                                             │
│  This predicate runs for EACH entity when building a projection.           │
│  Same VM, different calling context.                                        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Why This Design Matters

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  ALTERNATIVE: 5 DIFFERENT SYSTEMS                                           │
│  ─────────────────────────────────                                          │
│                                                                             │
│  Edge predicates:    Custom DSL #1                                          │
│  Auth checks:        Custom DSL #2, or raw code                            │
│  Validation:         JSON Schema, or custom DSL #3                         │
│  Merge policies:     Hard-coded in application                             │
│  View filters:       SQL WHERE clauses, or custom DSL #4                   │
│                                                                             │
│  Problems:                                                                  │
│  - 5 languages to learn                                                     │
│  - 5 security surfaces to audit                                             │
│  - Can't reuse logic between contexts                                       │
│  - No unified bounds checking                                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  PXYZ: ONE SYSTEM, FIVE APPLICATIONS                                        │
│  ────────────────────────────────────                                        │
│                                                                             │
│  One predicate VM (~250 lines WAT)                                          │
│  One opcode set (~25 ops, all bounded)                                      │
│  One bytecode format                                                        │
│  One set of limits (256 steps, 16 stack, 4 call depth)                     │
│                                                                             │
│  Benefits:                                                                  │
│  - Learn once, apply everywhere                                             │
│  - One security surface to audit                                            │
│  - Predicates can reference each other (reuse)                             │
│  - All applications inherit the same safety bounds                          │
│  - Same tooling (disassembler, debugger) for all                           │
│                                                                             │
│  The only difference between applications is the CONTEXT:                   │
│  - What variables are available ($a, $b, $entity, $token, etc.)            │
│  - What happens with the result (take edge, grant access, merge, etc.)     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Merge Context Host Imports

For merge policies, the host provides additional functions:

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│  MERGE-SPECIFIC HOST IMPORTS                                                │
│                                                                             │
│  (import "io" "get_timestamp" (func $io_get_timestamp (param i32) (result i64)))│
│  Get the timestamp of a value. For LWW/FWW policies.                        │
│                                                                             │
│  (import "io" "is_flagged" (func $io_is_flagged (param i32) (result i32)))  │
│  Check if a value is flagged for human review.                              │
│                                                                             │
│  (import "io" "get_origin" (func $io_get_origin (param i32) (result i32)))  │
│  Get the origin/author ID of a value.                                       │
│                                                                             │
│  (import "io" "vclock_dominates" (func $io_vclock_dominates ...))           │
│  Check if one vector clock dominates another.                               │
│                                                                             │
│  (import "io" "get_merge_field" (func $io_get_merge_field ...))             │
│  Get a field from the merge context (0=a, 1=b, 2=candidate).                │
│                                                                             │
│  These are provided by the host, just like io_call and resolve_var.         │
│  The WASM runtime remains sandboxed - it just asks the host.                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### The Elegance of Unified Constraints

```shell
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  Consider a named predicate:                                                │
│                                                                             │
│    <predicate id="is_owner">                                                │
│      <eq left="$entity.owner_id" right="$token.sub"/>                      │
│    </predicate>                                                             │
│                                                                             │
│  This ONE predicate can be used:                                            │
│                                                                             │
│  1. On an edge:     <ref predicate="is_owner"/>                            │
│  2. In an auth node: <require predicate="is_owner"/>                       │
│  3. For validation: Transform checks is_owner before proceeding            │
│  4. For merge:      PreferOrigin policy checks is_owner                    │
│  5. For projection: Only show entities where is_owner is true              │
│                                                                             │
│  ONE definition. FIVE applications. ZERO duplication.                       │
│                                                                             │
│  When you update the predicate, ALL applications update.                    │
│  When you audit the predicate, you audit ALL applications.                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

This is the Y-layer insight: The predicate VM is not "for edges." It's a **universal constraint evaluation engine** that happens to be called from multiple points in the system. The power comes from the unification, not the individual features.

---

## Summary: The PXYZ Philosophy

1. **Business logic is data, not code.** The graph IS the program. The runtime is just a walker.

2. **Predicates are bounded.** You can't write `while(true)` in a predicate. Max 256 steps. Max 16 stack. Done.

3. **All IO is explicit.** No hidden network calls. Every side effect has an op code. Auditors can grep for `0x0340` to find all email sends.

4. **Safety at compile time.** PRAG001 catches "LLM → email without approval" before deployment, not after the email is sent.

5. **Human gates for irreversible actions.** The compiler enforces this. You literally cannot compile a graph that violates PRAG003.

6. **Auditable runtime.** 600 lines of WAT. A senior engineer can read it in an afternoon. A formal methods person can verify it in a week.

---

*This architecture is not about being clever. It's about being auditable. When the AI can send emails, delete data, and call webhooks on behalf of users, "move fast and break things" is not an option. PXYZ moves deliberately and proves things.*
