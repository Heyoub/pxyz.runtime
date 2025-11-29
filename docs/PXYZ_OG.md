# PXYZ: A Mathematical Foundation for Coordinate-Addressed Computation

> *"State is a lie. Events are truth. The graph is physics."*

---

## 1. The Core Insight

Traditional computation conflates three distinct concerns:

1. **What** can be computed (the operations)
2. **Whether** it should be computed (the constraints)
3. **When** it was computed (the temporal record)

PXYZ separates these into orthogonal dimensions, creating a **4-dimensional coordinate space** where every computation has a unique address.

```
           Z (Time)
           │
           │    ┌─────────────────┐
           │   ╱                 ╱│
           │  ╱    Computation  ╱ │
           │ ╱      Space      ╱  │
           │┌─────────────────┐   │
           ││                 │   │
           ││                 │  ╱ Y (Constraints)
           ││                 │ ╱
           │└─────────────────┘╱
           └──────────────────────── X (Operations)
          ╱
         ╱
        P (Primitives)
```

---

## 2. The Four Dimensions

### P — Primitives (The Nouns)

P is the dimension of **what exists**. These are the atomic values and entities that operations transform.

**Properties:**
- Primitives are **immutable values**, not mutable state
- Each primitive has a **schema** (its shape and constraints)
- Primitives are **observed**, never directly modified
- The set of all P defines your **domain vocabulary**

**Coordinate notation:** `P.EntityName`

**Example thinking:**
- "What are the fundamental building blocks?"
- "What values flow through the system?"
- "What can be observed at a point in time?"

---

### X — Operations (The Verbs)

X is the dimension of **what happens**. Operations are pure transformations from inputs to outputs.

**Properties:**
- Operations are **functions**: `X: P → P'`
- Operations form a **DAG** (directed acyclic graph)
- Operations have **declared dependencies**
- Operations are **referentially transparent** (same input → same output)

**Coordinate notation:** `X.OperationName`

**Naming convention:**
- Domain operations: `X.CalculateSomething` (PascalCase, the "what")
- Meta operations: `x.meta.generated` (lowercase dotted, the "how")

**Example thinking:**
- "What transformations are possible?"
- "What is the dependency graph between operations?"
- "Which operations are irreversible?"

---

### Y — Constraints (The Adjectives)

Y is the dimension of **under what conditions**. Constraints define the boundaries within which operations are valid.

**Properties:**
- Constraints are **predicates**: `Y: Context → Boolean`
- Constraints are **versioned regions**, not point values
- Constraints use **geometric fit**, not boolean pass/fail
- Constraints are **evaluated by a single engine** (the Y-layer)

**Coordinate notation:** `Y.RegionName.vVersion`

**The Y-Layer Unification:**

This is the key architectural insight. Rather than building separate systems for different constraint checks, PXYZ uses **one evaluation engine** applied at **five different points**:

| Application Point | Question Answered | Context Available |
|-------------------|-------------------|-------------------|
| **Edge Traversal** | "Can we take this path?" | `$input`, `$state` |
| **Authorization** | "Is this actor permitted?" | `$token`, `$entity` |
| **Validation** | "Is this data well-formed?" | `$input` |
| **Merge Resolution** | "Which version wins?" | `$a`, `$b`, `$candidate` |
| **Projection Filtering** | "Should this appear?" | `$entity`, `$token` |

**Why this matters:**
- One language to learn
- One security surface to audit
- Predicates are reusable across all contexts
- Safety bounds apply universally

**Example thinking:**
- "Under what conditions is this operation valid?"
- "Can I reuse this constraint in multiple places?"
- "What's the geometric distance from full compliance?"

---

### Z — Time (The Adverbs)

Z is the dimension of **when**. Every computation occurs at a specific temporal coordinate.

**Properties:**
- Z coordinates are **ISO 8601 timestamps** with monotonic ordering
- Events at Z coordinates are **immutable** (append-only log)
- Z enables **time-travel queries**: "What was the value at Z?"
- Z coordinates form a **hash chain** for integrity

**Coordinate notation:** `Z = 2025-01-17T10:30:00.000Z`

**Temporal causality:**
```
Event₁ ──hash──→ Event₂ ──hash──→ Event₃
  │                │                │
  Z₁              Z₂              Z₃
```

**Example thinking:**
- "What was true at this point in time?"
- "What is the causal chain of events?"
- "Can I prove this sequence wasn't tampered with?"

---

## 3. Coordinates as Addresses

A full PXYZ coordinate uniquely addresses a computation:

```
PXYZ = (P, X, Y, Z)
     = (Primitive, Operation, Constraint, Time)
```

**Reading a coordinate:**
```
(Amount, Calculate, Compliant.v2025, 2025-01-17T10:30:00Z)
     │        │            │                  │
     │        │            │                  └── "at this moment"
     │        │            └── "under these constraints"
     │        └── "applying this operation"
     └── "to this primitive"
```

**The fundamental operation:**

```
PXYZ.at(P, X, Y, Z) → Value
```

This reads: "Give me the value of primitive P, after operation X, under constraints Y, as of time Z."

There is no mutable state. There is only **observation at coordinates**.

---

## 4. The Graph as Physics

### 4.1 Structure

The computation space is a **Directed Acyclic Graph (DAG)**:

```
    ┌─────┐         ┌─────┐
    │  A  │────────→│  B  │
    └─────┘         └─────┘
       │               │
       │    ┌─────┐    │
       └───→│  C  │←───┘
            └─────┘
               │
            ┌─────┐
            │  D  │
            └─────┘
```

**Properties:**
- **Nodes** are operations (X coordinates)
- **Edges** are transitions with optional predicates (Y evaluation)
- **No cycles** — computation always moves forward
- **Finite paths** — all traversals terminate

### 4.2 Traversal

Traversal is the execution of computation through the graph:

```
START → Node₁ → [predicate?] → Node₂ → ... → TERMINAL
```

**Edge evaluation:**
1. Evaluate edge predicate using Y-layer
2. If true, traverse to target node
3. If false, try next edge or fail

**Bounded traversal:**
| Limit | Value | Purpose |
|-------|-------|---------|
| MAX_VISITED_NODES | 1000 | Prevents runaway traversal |
| MAX_PREDICATE_STEPS | 256 | Prevents infinite loops in predicates |
| MAX_STACK_DEPTH | 16 | Prevents stack overflow |
| MAX_CALL_DEPTH | 4 | Prevents infinite recursion |

### 4.3 Entry Points

The graph has explicit **entry points** — (P, X) pairs that identify where traversal can begin:

```
Entry: hash(P + X) → NodeId
```

This is a **coordinate lookup**, not a function call. You're asking: "Where in the graph do I start for this (P, X) pair?"

---

## 5. The Y-Layer: Unified Constraint Evaluation

### 5.1 The Predicate VM

All constraints evaluate on a tiny, bounded virtual machine:

```
┌────────────────────────────────────┐
│         Predicate VM               │
│                                    │
│  ┌──────────┐    ┌──────────┐     │
│  │  Stack   │    │ Context  │     │
│  │ (16 max) │    │ $token   │     │
│  │          │    │ $entity  │     │
│  └──────────┘    │ $input   │     │
│        ↑         │ $a, $b   │     │
│        │         └──────────┘     │
│   ┌────┴────┐                     │
│   │ Opcodes │  (~25 operations)   │
│   └─────────┘                     │
└────────────────────────────────────┘
              │
              ↓
         true / false
```

### 5.2 Opcode Categories

| Category | Operations | Purpose |
|----------|------------|---------|
| Stack | PUSH, LOAD_VAR, LOAD_FIELD | Move values onto stack |
| Comparison | EQ, NEQ, GT, GTE, LT, LTE | Compare values |
| Logic | AND, OR, NOT | Combine predicates |
| String | CONTAINS, MATCHES, STARTS_WITH | Text operations |
| Utility | IS_NULL, IS_DEFINED, LEN | Type checks |
| Merge | TIMESTAMP, ORIGIN, VCLOCK_GT | Conflict resolution |
| Control | CALL_PRED, RET | Predicate composition |

### 5.3 Why One Engine?

```
                    ┌─────────────┐
                    │ Predicate   │
                    │ Definition  │
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
           ↓               ↓               ↓
    ┌──────────┐    ┌──────────┐    ┌──────────┐
    │   Edge   │    │   Auth   │    │  Merge   │
    │ Traversal│    │  Check   │    │ Policy   │
    └──────────┘    └──────────┘    └──────────┘
           │               │               │
           └───────────────┼───────────────┘
                           ↓
                    ┌──────────────┐
                    │ Same Safety  │
                    │   Bounds     │
                    └──────────────┘
```

One definition, multiple applications, uniform guarantees.

---

## 6. Events, Not State

### 6.1 The Event Model

There is no mutable state. Everything is an **event** at a **Z coordinate**:

```
Event = {
  id:       UUID,
  pxyz:     (P, X, Y, Z),
  payload:  Value,
  checksum: SHA256(content),
  previous: SHA256(Event_{n-1})
}
```

### 6.2 Hash Chain Integrity

Events form a **hash chain** (like Git commits):

```
┌─────────┐    ┌─────────┐    ┌─────────┐
│ Event₁  │───→│ Event₂  │───→│ Event₃  │
│ hash: a │    │ hash: b │    │ hash: c │
│ prev: ∅ │    │ prev: a │    │ prev: b │
└─────────┘    └─────────┘    └─────────┘
```

**Properties:**
- Append-only (events are never modified)
- Tamper-evident (hash chain detects changes)
- Auditable (complete history is preserved)
- Replayable (state can be reconstructed)

### 6.3 Reconstructing "State"

What appears to be state is actually a **projection of events**:

```
State(P, Z) = fold(events.filter(e => e.P == P && e.Z <= Z))
```

Or in words: "The 'state' of P at time Z is the result of replaying all events for P up to Z."

---

## 7. Conflict Resolution (CRDT Merge)

When the same primitive is modified concurrently (e.g., offline edits), the Y-layer resolves conflicts:

### 7.1 Merge Context

```
Merge = {
  $a:         Version₁,
  $b:         Version₂,
  $candidate: ProposedMerge
}
```

### 7.2 Built-in Policies

| Policy | Semantics | Use Case |
|--------|-----------|----------|
| `lww` | Last Writer Wins (higher timestamp) | Default for most fields |
| `fww` | First Writer Wins (lower timestamp) | Immutable fields |
| `max` / `min` | Numeric comparison | Counters, versions |
| `union` | Combine sets | Tags, collections |
| `vclock` | Vector clock dominance | True causality |
| `human-review` | Flag for manual resolution | Critical conflicts |

### 7.3 The Same Engine

Merge policies are **predicates**. They run on the same bounded VM:

```xml
<predicate id="prefer_owner">
  <eq left="$candidate.author" right="$entity.owner"/>
</predicate>
```

This predicate can be used for auth, edge traversal, OR merge resolution.

---

## 8. Compile-Time Safety

### 8.1 The Three-Layer Constraint System

Before a graph can execute, it must pass validation:

```
┌────────────────────────────────────────┐
│           PRAGMATIC (PRAG)             │  ← Business rules
│  "Does this follow safety policies?"   │
├────────────────────────────────────────┤
│           SEMANTIC (SEM)               │  ← Logic coherence
│  "Does this make logical sense?"       │
├────────────────────────────────────────┤
│           SYNTACTIC (SYN)              │  ← Structure
│  "Is this well-formed?"                │
└────────────────────────────────────────┘
```

### 8.2 Example Constraints

**Syntactic (structure):**
- SYN001: Edge targets must exist
- SYN004: No duplicate node IDs
- SYN005: At least one entry point

**Semantic (logic):**
- SEM002: External nodes must have operation codes
- SEM004: No cycles in graph (must be DAG)
- SEM005: All nodes reachable from entry

**Pragmatic (policy):**
- PRAG001: Probabilistic output → irreversible action requires validation gate
- PRAG003: Irreversible actions require human in path
- PRAG005: Quarantined data cannot reach external nodes

### 8.3 Shifting Left

```
Traditional:     Compile → Run → Discover Bug → Fix → Repeat
PXYZ:           Compile → [Constraints Block Bad Patterns] → Run (safe by construction)
```

---

## 9. Self-Proving Convergence

### 9.1 The Fixed-Point Property

A generator is **convergent** if:
```
hash(generate(N)) === hash(generate(N+1))
```

When consecutive generations produce identical output, we've reached a **fixed point**.

### 9.2 The Role of Normalization

Raw generation may produce semantically identical but textually different output:
```
Iteration 1: const x=1;
Iteration 2: const x = 1;
```

These are the same, but hashes differ. **Normalization** (canonical form) solves this:

```
normalize("const x=1;")     → "const x = 1;"
normalize("const x = 1;")   → "const x = 1;"
```

Now: `hash(normalize(gen(N))) === hash(normalize(gen(N+1)))`

### 9.3 The Formatter Participates in the Proof

```
┌───────────┐    ┌────────────┐    ┌────────┐    ┌──────────┐
│ Generator │───→│ Normalizer │───→│ Hasher │───→│ Compare  │
└───────────┘    └────────────┘    └────────┘    └──────────┘
                       │                              │
                       │         Convergence?         │
                       └──────────────────────────────┘
```

When convergence is detected:
- The generator is **deterministic** (not random)
- The output is in **canonical form**
- The system has reached a **fixed point**
- Therefore: the generated artifact is **self-consistent**

---

## 10. Thinking in Coordinates

### 10.1 Questions to Ask

When designing a system with PXYZ:

**For P (Primitives):**
- What are my fundamental value types?
- What schemas constrain them?
- How are they observed vs. mutated?

**For X (Operations):**
- What transformations exist?
- What is the dependency graph?
- Which operations have side effects?
- Which are irreversible?

**For Y (Constraints):**
- What conditions must hold?
- Can I express this as a predicate?
- Can I reuse this constraint elsewhere?
- What's the fit score, not just pass/fail?

**For Z (Time):**
- What's the temporal ordering?
- How do I handle concurrent modifications?
- What's my audit trail?
- Can I reconstruct any past state?

### 10.2 Coordinate-First Design

Instead of designing objects and methods, design **coordinates and transitions**:

```
Traditional OOP:
  class Entity {
    calculate() { ... }
    validate() { ... }
  }

PXYZ Coordinates:
  P.Entity at X.Calculate under Y.Valid at Z.now
  │             │                │           │
  └── what     └── how          └── when ok └── as of
```

### 10.3 The Graph is the Specification

The graph isn't an implementation detail — it IS the specification:

```
┌─────────────────────────────────────────────────┐
│                   The Graph                      │
│                                                  │
│  - Every possible path is visible               │
│  - Every constraint is declared                 │
│  - Every side effect is explicit                │
│  - The behavior IS the structure                │
│                                                  │
│  "I can audit the graph in an afternoon"        │
└─────────────────────────────────────────────────┘
```

---

## 11. Implementation Principles

### 11.1 Logic as Data

```
Code:    function calculate(x) { return x * 2; }
Data:    { type: "multiply", operand: "x", factor: 2 }
```

When logic is data:
- It can be analyzed before execution
- It can be optimized by the compiler
- It can be serialized and transmitted
- It can be formally verified

### 11.2 Bounded Computation

Every computation has hard limits:

```
╔═══════════════════════════════════════╗
║         FUSES, NOT TRUST              ║
╠═══════════════════════════════════════╣
║ Predicate steps:     max 256          ║
║ Stack depth:         max 16           ║
║ Call depth:          max 4            ║
║ Bytecode size:       max 256 bytes    ║
║ Visited nodes:       max 1000         ║
╚═══════════════════════════════════════╝
```

These aren't configurable. They're physical laws of the system.

### 11.3 Explicit I/O

Every interaction with the external world has an **operation code**:

```
0x0100  ENTITY_CREATE
0x0400  HTTP_GET
0x0340  EMAIL_SEND    ⚠️ IRREVERSIBLE
```

Want to find every place the system sends email? `grep 0x0340`.

### 11.4 The Walker is Dumb

The runtime (the "walker") is intentionally minimal:

```
┌─────────────────────────────────────┐
│           Complex Graph             │  ← All the logic
│      (compiled, validated)          │
└─────────────────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────┐
│          Simple Walker              │  ← Just follows instructions
│     (~500 lines, auditable)         │
└─────────────────────────────────────┘
```

The walker doesn't know what the graph means. It just:
1. Looks up entry point
2. Evaluates edge predicates
3. Follows matching edges
4. Executes nodes
5. Repeats until terminal

---

## 12. Summary: The PXYZ Laws

1. **Separation of Concerns**
   - P: What exists
   - X: What happens
   - Y: What's allowed
   - Z: When it happened

2. **Unified Constraints**
   - One predicate engine
   - Five application points
   - Same safety bounds everywhere

3. **Events Over State**
   - No mutable state
   - Append-only event log
   - State is a projection of events

4. **Bounded Execution**
   - Hard limits on all computation
   - Provable termination
   - No infinite loops by construction

5. **Explicit Effects**
   - All I/O has operation codes
   - Side effects are declared, not hidden
   - Irreversible actions are marked

6. **Compile-Time Safety**
   - Three-layer validation
   - Dangerous patterns blocked before deployment
   - Safety is a prerequisite, not an afterthought

7. **Auditability**
   - The graph is readable
   - The runtime is minimal
   - The event log is tamper-evident

---

## 13. Closing Thought

> *"This architecture is not about being clever. It's about being auditable."*

PXYZ is a coordinate system for computation. When you think in coordinates, you stop asking "how do I implement this?" and start asking "where does this live in the space?"

The graph is not code. It's a map.
The walker doesn't execute logic. It follows the map.
The constraints don't validate data. They define the shape of valid space.
The events don't change state. They record observations at coordinates.

Everything is an address. Everything is explicit. Everything is bounded.

That's PXYZ.
