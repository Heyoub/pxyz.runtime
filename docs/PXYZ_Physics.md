# PXYZ as Physics: Energy-Aware Computation

> *"Moving data costs 100–1,000× more energy than computing on it."*
> *"The graph is physics."*

These two statements are the same statement.

---

## 1. The Thermodynamic Interpretation of PXYZ

PXYZ's architectural constraints map directly to physical laws:

| PXYZ Constraint | Physical Principle |
|-----------------|-------------------|
| MAX_PREDICATE_STEPS = 256 | Energy budget per decision |
| MAX_STACK_DEPTH = 16 | Working memory (registers) |
| MAX_VISITED_NODES = 1000 | Total computation energy |
| DAG structure (no cycles) | Entropy always increases |
| Event log (append-only) | Time is irreversible |
| Hash chain integrity | Thermodynamic arrow of time |

The Landauer limit tells us: **erasing one bit costs kT ln(2) ≈ 2.87 × 10⁻²¹ J**.

PXYZ's reversible event log means we never erase—we only append. The system approaches the Landauer limit by design.

---

## 2. Memory Hierarchy as Coordinate Distance

The PXYZ coordinate system maps to physical memory:

```
┌────────────────────────────────────────────────────────────┐
│                     Z (Time)                                │
│                        │                                    │
│    DRAM (640 pJ)       │         Far from compute          │
│    ─────────────────   │         High energy cost          │
│                        │                                    │
│    L2 Cache (100 pJ)   │         Medium distance           │
│    ─────────────────   │                                    │
│                        │                                    │
│    L1 Cache (5 pJ)     │         Close                     │
│    ─────────────────   │                                    │
│                        │                                    │
│    Registers (1 pJ)    │         At compute                │
│    ─────────────────   │                                    │
│                        │                                    │
│         P ─────────────┼─────────────── X                  │
│      (Primitives)      │            (Operations)           │
│                        │                                    │
│                       Y│(Constraints)                       │
└────────────────────────────────────────────────────────────┘
```

**Coordinate Distance = Energy Cost**

```
distance(P₁, P₂) ∝ energy_to_move(P₁ → P₂)
```

The graph compiler should place frequently-communicating nodes **adjacent in memory**.

---

## 3. The Walker as Systolic Array

The PXYZ walker is essentially a 1D systolic array:

```
Systolic Array (TPU):
  Data flows → through → fixed → processing → elements

PXYZ Walker:
  Event flows → through → fixed → graph → nodes
```

Both share the same physics-optimal properties:
- **No random memory access** (data flows, doesn't jump)
- **Local communication only** (node to adjacent node)
- **Minimal control overhead** (simple fetch-execute)
- **Predictable energy** (bounded by structure)

The key insight: **the graph topology IS the data flow pattern**.

---

## 4. Predicates as Energy Barriers

In thermodynamics, reactions occur when:
```
ΔG = ΔH - TΔS < 0
```

In PXYZ, edge traversal occurs when:
```
predicate(context) = true
```

These are isomorphic. A predicate evaluation is asking: "Is this transition thermodynamically favorable?"

**The Y-layer is a Boltzmann machine:**
- Each predicate has an "energy" (evaluation cost)
- High-energy predicates (complex) are less likely to be worth evaluating
- The system naturally finds low-energy paths (short predicates)

---

## 5. Energy Accounting Model

Every PXYZ operation has an energy cost:

| Operation | Cost (abstract units) | Physical Analog |
|-----------|----------------------|-----------------|
| Stack PUSH | 1 | Register write |
| Stack POP | 1 | Register read |
| LOAD_VAR | 10 | L1 cache access |
| LOAD_FIELD | 50 | L2 cache access |
| Edge traversal | 100 | Memory hierarchy hop |
| External I/O | 10,000 | DRAM + bus |

The bounded execution model:
```
MAX_PREDICATE_STEPS = 256  →  Max 256 energy units per predicate
MAX_VISITED_NODES = 1000   →  Max 1,000,000 energy units per traversal
```

---

## 6. Reversibility and the Event Log

Charles Bennett proved: **computation need not dissipate energy—only erasure does**.

PXYZ's event log enables reversibility:

```
Forward:   Event₁ → Event₂ → Event₃ → Result
Reverse:   Result → Event₃ → Event₂ → Event₁ → Initial

Energy cost (irreversible): O(n) × Landauer
Energy cost (reversible):   O(1) (only final copy)
```

The hash chain enables verification without re-computation:
```
verify(Event_n) = hash(Event_n) == Event_{n+1}.previous_hash
```

This is **O(1) verification** vs **O(n) re-execution**.

---

## 7. Thermal Noise as Resource

Traditional computing fights noise. Physics-native computing uses it.

**Probabilistic Predicates:**

Instead of:
```
predicate: x > threshold  →  true/false
```

Use:
```
predicate: P(true) = sigmoid((x - threshold) / temperature)
```

At temperature T:
- Low T → deterministic (traditional boolean)
- High T → exploratory (Boltzmann sampling)
- Optimal T → simulated annealing for optimization

The Y-layer can implement this with a single opcode:
```
SAMPLE_BOLTZMANN: energy → probability → 0/1
```

---

## 8. Graph Layout for Cache Efficiency

The graph.bin format should respect cache lines (64 bytes):

```
┌─────────────────────────────────────────────────────┐
│                  Cache Line (64 bytes)               │
├──────────────────────────────────────────────────────┤
│ Node₁ (16 bytes) │ Edge₁ (12 bytes) │ Edge₂ (12 bytes) │ Pred (24 bytes) │
└──────────────────────────────────────────────────────┘
```

**Principle: A node and its outgoing edges should fit in one cache line.**

Current format:
- Node: 16 bytes ✓
- Edge: 12 bytes ✓
- 16 + 12 + 12 + 12 + 12 = 64 bytes → 1 node + 4 edges per cache line

This is already optimal! The format was designed for physics.

---

## 9. The Roofline Model for PXYZ

The roofline model shows compute vs memory bounds:

```
Performance
    │
    │     ┌────────── Compute Bound ──────────
    │    ╱
    │   ╱
    │  ╱
    │ ╱ Memory Bound
    │╱
    └──────────────────────────────────────────
                Arithmetic Intensity (ops/byte)
```

For PXYZ:
```
Ops = predicate evaluations + node executions
Bytes = graph.bin reads + context reads + I/O

Arithmetic Intensity = (nodes_visited × ops_per_node) / bytes_moved
```

**Goal: Stay compute-bound by maximizing reuse:**
- Predicate results cached per traversal
- Context loaded once, used for all predicates
- Graph remains in L2/L3 cache

---

## 10. Physics-Native PXYZ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    PXYZ Physics Runtime                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────┐    ┌──────────────────┐              │
│  │   Graph Cache    │    │  Context Cache   │              │
│  │   (fits in L2)   │←──→│  (fits in L1)    │              │
│  └────────┬─────────┘    └────────┬─────────┘              │
│           │                       │                         │
│           ▼                       ▼                         │
│  ┌─────────────────────────────────────────┐               │
│  │           Predicate VM                   │               │
│  │  ┌─────────────────────────────────┐    │               │
│  │  │ Stack (16 × 8 bytes = 128 bytes)│    │  ← Registers  │
│  │  │ Fits in ONE cache line × 2      │    │               │
│  │  └─────────────────────────────────┘    │               │
│  │                                          │               │
│  │  Energy Counter: ████████░░ (200/256)   │               │
│  └─────────────────────────────────────────┘               │
│           │                                                 │
│           ▼                                                 │
│  ┌─────────────────────────────────────────┐               │
│  │           Event Log                      │               │
│  │  (Append-only, hash-chained)            │  ← Reversible │
│  │  Energy: O(1) per event                 │               │
│  └─────────────────────────────────────────┘               │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 11. Design Principles for Physics-Native Graphs

When designing PXYZ graphs, think thermodynamically:

### 11.1 Minimize Edge Crossings
```
Bad (high energy):          Good (low energy):
    A ──────────→ D             A → B → C → D
    │             ↑
    └──→ B ──→ C ─┘             (Linear = minimal wire length)
```

### 11.2 Cluster Related Operations
```
Bad:                        Good:
  Validate                    ┌── Validate ──┐
     ↓                        │              │
  (cross-chip)                │   Transform  │
     ↓                        │              │
  Transform                   └── External ──┘
     ↓
  (cross-chip)                (Clustered = cache-friendly)
     ↓
  External
```

### 11.3 Use Short Predicates
```
Energy cost ∝ predicate length

predicate: $x > 0                    →  3 ops  →  3 energy
predicate: $x > 0 AND $y < 100       →  7 ops  →  7 energy
predicate: complex_check($x, $y, $z) → 50 ops  → 50 energy

Prefer: Many simple predicates over few complex ones
```

### 11.4 Exploit Temporal Locality
```
If operation X always follows operation Y:
  - Place X and Y in adjacent nodes
  - They'll share cache lines
  - Edge traversal = 1 cache line read (not 2)
```

---

## 12. The Graph Compiler's Job

A physics-aware graph compiler should:

1. **Analyze Communication Patterns**
   - Build affinity graph between nodes
   - Weight edges by communication frequency

2. **Minimize Wire Length**
   - Topological sort for linear layout
   - Community detection for clustering
   - Min-cut partitioning for cache boundaries

3. **Balance Energy Budgets**
   - Ensure no path exceeds MAX_VISITED_NODES
   - Ensure no predicate exceeds MAX_PREDICATE_STEPS
   - Distribute computation evenly

4. **Optimize for Arithmetic Intensity**
   - Fuse operations that share data
   - Eliminate redundant loads
   - Maximize computation per memory access

---

## 13. Mapping to Hardware

| PXYZ Concept | CPU | GPU | TPU | Neuromorphic |
|--------------|-----|-----|-----|--------------|
| Node | Instruction | Warp | Systolic cell | Neuron |
| Edge | Branch | Warp divergence | Data flow | Synapse |
| Predicate | Condition | Predicate mask | Weight | Threshold |
| Event | Cache line | Shared memory | Accumulator | Spike |
| Walker | Program counter | Scheduler | Controller | N/A (async) |

The same graph.bin runs on all—only the walker changes.

---

## 14. Summary: PXYZ as Thermodynamic Computing

PXYZ isn't just "safe by construction"—it's **efficient by construction**:

| Principle | PXYZ Implementation | Physical Benefit |
|-----------|---------------------|------------------|
| Bounded execution | MAX_* constants | Energy capping |
| No cycles (DAG) | Compile-time check | Entropy increase |
| Append-only events | Hash-chained log | Reversibility |
| Unified Y-layer | Single VM | Minimal control |
| Explicit I/O | Op codes | No hidden costs |
| Minimal walker | ~500 LOC | Low overhead |
| Cache-aligned format | 64-byte structures | Memory efficiency |

The graph is not an abstraction over physics.
**The graph IS physics.**

---

## 15. Closing: Energy as the Universal Currency

Just as PXYZ uses coordinates to address computation, physics uses energy to price it:

```
PXYZ:     at(P, X, Y, Z) → Value
Physics:  work(mass, force, distance) → Energy
```

Every PXYZ operation has an energy cost.
The bounded execution model is an energy budget.
The graph topology determines data movement cost.
The event log enables thermodynamic reversibility.

**Design your graphs like you're paying for electrons—because you are.**
