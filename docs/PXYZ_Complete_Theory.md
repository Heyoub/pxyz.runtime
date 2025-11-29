# PXYZ: The Complete Theory

> *"The graph is not an abstraction over physics. The graph IS physics."*

---

## The Three Documents

This project now contains three foundational documents that form a complete theory:

1. **PXYZ_OG.md** — The Mathematical Foundation
   - The 4D coordinate system (P, X, Y, Z)
   - Unified Y-constraint layer
   - Bounded execution model
   - Events over state
   - How to think in coordinates

2. **PXYZ_Physics.md** — The Thermodynamic Interpretation
   - Energy accounting for operations
   - Cache hierarchy as coordinate distance
   - Walker as systolic array
   - Predicates as energy barriers
   - Thermal noise as computational resource

3. **Physical Foundations of Computation** — The Scientific Basis
   - Landauer limit: kT ln(2) per bit erased
   - Horowitz measurements: DRAM = 6400× register
   - Bounded computation prevents unbounded energy
   - Reversibility approaches theoretical minimum

---

## The Unified Insight

These three documents are actually saying the same thing from different angles:

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│   MATHEMATICS          PHYSICS              IMPLEMENTATION          │
│   (PXYZ_OG.md)        (PXYZ_Physics.md)    (Rust + WASM)           │
│                                                                     │
│   Coordinates     ←→   Memory Addresses  ←→   Byte Offsets          │
│                                                                     │
│   Bounded Steps   ←→   Energy Budget     ←→   MAX_* Constants       │
│                                                                     │
│   DAG Structure   ←→   Entropy Increase  ←→   No Cycles Check       │
│                                                                     │
│   Event Log       ←→   Reversibility     ←→   Append-Only           │
│                                                                     │
│   Y-Layer         ←→   Energy Barrier    ←→   Predicate VM          │
│                                                                     │
│   Graph Locality  ←→   Cache Efficiency  ←→   Layout Optimizer      │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Key Equations

### 1. The Coordinate Equation
```
Value = PXYZ.at(P, X, Y, Z)
```
*Everything is an observation at a coordinate.*

### 2. The Energy Equation
```
Energy(operation) = f(memory_tier) × base_cost

Where memory_tier:
  Register → 1×
  L1 Cache → 5×
  L2 Cache → 100×
  L3 Cache → 500×
  DRAM     → 6400×
```
*Distance in memory space = energy cost.*

### 3. The Landauer Equation
```
E_min = kT ln(2) ≈ 2.87 × 10⁻²¹ J per bit erased
```
*The universe charges for forgetting.*

### 4. The Locality Equation
```
locality_score = Σ(edges in same cache line) / Σ(all edges)
```
*Higher locality = less energy = faster execution.*

### 5. The Reversibility Equation
```
Energy(irreversible) = O(n) × Landauer
Energy(reversible)   = O(1)
```
*The event log makes everything reversible.*

---

## The Implementation Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│                         XML Workflow DSL                            │
│                    (Human-readable specification)                   │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Rust Compiler                               │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐          │
│  │    Parser     │→ │  Validator    │→ │   Optimizer   │          │
│  │  (XML→AST)    │  │ (SYN/SEM/PRAG)│  │ (Layout, DCE) │          │
│  └───────────────┘  └───────────────┘  └───────────────┘          │
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │                    Physics Module                              │ │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐          │ │
│  │  │ Energy  │  │  Cache  │  │ Layout  │  │ Thermal │          │ │
│  │  │Accounting│  │Alignment│  │Optimizer│  │ Noise   │          │ │
│  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘          │ │
│  └───────────────────────────────────────────────────────────────┘ │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         graph.bin                                   │
│            (Cache-aligned, locality-optimized binary)               │
│                                                                     │
│  Header (96 bytes, 1.5 cache lines)                                │
│  Nodes  (16 bytes each, 4 per cache line)                          │
│  Edges  (12 bytes each, ~5 per cache line)                         │
│  Predicates (variable, cache-line aligned)                          │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                    ┌───────────┴───────────┐
                    ▼                       ▼
┌─────────────────────────────┐  ┌─────────────────────────────┐
│    Rust Runtime (~1500 LOC) │  │   WASM Runtime (~500 LOC)   │
│    (Development/Testing)    │  │   (Production/Audit)        │
│                             │  │                             │
│  • Full debugging           │  │  • Energy accounting        │
│  • Rich error messages      │  │  • Cache tracking           │
│  • Profiling hooks          │  │  • Thermal predicates       │
│                             │  │  • Minimal attack surface   │
└─────────────────────────────┘  └─────────────────────────────┘
```

---

## What the Physics Module Provides

### 1. Energy Accounting (`physics/energy.rs`)
```rust
pub struct EnergyBudget {
    total: Energy,      // Total available (from MAX_* limits)
    consumed: Energy,   // Used so far
    costs: EnergyCosts, // Cost model (Horowitz-derived)
}

impl EnergyBudget {
    pub fn spend(&mut self, amount: Energy) -> bool;
    pub fn efficiency(&self) -> f64;
}
```

### 2. Cache-Aware Allocation (`physics/cache.rs`)
```rust
pub struct CacheAlignedBuffer { ... }  // 64-byte aligned
pub struct CachePackedNodeUnit { ... } // Node + 4 edges = 1 cache line

pub enum MemoryTier { Hot, Warm, Cold, External }
```

### 3. Layout Optimization (`physics/layout.rs`)
```rust
pub struct LayoutOptimizer { ... }

impl LayoutOptimizer {
    pub fn compute_layout(&mut self) -> Vec<u32>;  // Locality-aware topo sort
    pub fn locality_score(&self) -> f64;           // 0.0 - 1.0
    pub fn estimate_energy(&self) -> u64;          // Abstract units
}
```

### 4. Thermal Predicates (`physics/thermal.rs`)
```rust
pub struct BoltzmannSampler {
    temperature: f64,
}

impl BoltzmannSampler {
    pub fn sample(&mut self, value: f64, threshold: f64) -> bool;
    pub fn metropolis(&mut self, current: f64, proposed: f64) -> bool;
    pub fn anneal(&mut self, rate: f64);
}
```

---

## What the WASM Walker Provides

### Energy-Aware Execution
```wat
;; Every operation costs energy
(func $spend_energy (param $amount i64) (result i32)
  ;; Returns 1 if within budget, 0 if exhausted
)

;; Energy costs based on Horowitz 2014
(global $COST_STACK_OP i32 (i32.const 1))      ;; Register
(global $COST_LOAD_VAR i32 (i32.const 10))     ;; L1
(global $COST_LOAD_FIELD i32 (i32.const 50))   ;; L2
(global $COST_IO_CALL i32 (i32.const 100000))  ;; DRAM + bus
```

### Cache Tracking
```wat
;; Track cache hits/misses
(func $track_cache_access (param $addr i32)
  ;; Compares to last cache line accessed
)

;; Get hit rate for diagnostics
(func $get_cache_hit_rate (export "get_cache_hit_rate") (result f64))
```

### Thermal Predicates
```wat
;; Boltzmann sampling for probabilistic edges
(func $boltzmann_sample (param $delta_e f64) (param $temp f64) (result i32)
  ;; Uses host entropy
  ;; Returns 1 with P = exp(-delta_e / T)
)
```

---

## Design Principles (Summary)

From **PXYZ_OG.md**:
1. Coordinates as addresses
2. Unified Y-layer
3. Events over state
4. Bounded execution
5. Explicit I/O
6. Minimal walker

From **PXYZ_Physics.md**:
7. Energy budgets, not infinite loops
8. Cache locality = energy efficiency
9. Predicates as energy barriers
10. Thermal noise as resource
11. Reversibility via event log
12. The graph IS the data layout

From **Physical Foundations**:
13. Moving data > computing on it (100-1000×)
14. Landauer limit is real and measurable
15. Roofline model is physics, not abstraction
16. Biological brains got it right: collocated memory/compute

---

## The Final Synthesis

PXYZ is not a workflow engine. It's a **coordinate system for energy-efficient computation**.

The bounded execution model isn't about preventing bugs (though it does). It's about **capping energy expenditure**.

The graph structure isn't about visualizing logic (though it does). It's about **optimizing data locality**.

The event log isn't about auditing (though it does). It's about **approaching the Landauer limit through reversibility**.

The Y-layer isn't about authorization (though it does). It's about **unified constraint evaluation with shared energy bounds**.

The minimal WASM walker isn't about security (though it is). It's about **minimizing control overhead like a systolic array**.

```
┌──────────────────────────────────────────────────────────────────┐
│                                                                  │
│   "Design your graphs like you're paying for electrons—        │
│    because you are."                                            │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Files Created

### Documentation
- `docs/PXYZ_OG.md` — Mathematical foundation
- `docs/PXYZ_Physics.md` — Thermodynamic interpretation
- `docs/PXYZ_Complete_Theory.md` — This synthesis

### Rust Implementation
- `src/physics/mod.rs` — Module entry
- `src/physics/energy.rs` — Energy accounting (EnergyCosts, EnergyBudget)
- `src/physics/cache.rs` — Cache alignment (CacheAlignedBuffer, MemoryTier)
- `src/physics/layout.rs` — Graph layout optimization
- `src/physics/thermal.rs` — Boltzmann sampling, simulated annealing

### WASM Implementation
- `wasm/pxyz_physics.wat` — Energy-aware, cache-tracking walker

---

## Next Steps

1. **Integrate physics module with compiler**
   - Layout optimization pass before binary emission
   - Energy estimates in audit.json

2. **Extend predicate VM with thermal opcodes**
   - `SAMPLE_BOLTZMANN`: Probabilistic edge traversal
   - `TEMPERATURE`: Get current annealing temperature

3. **Add roofline analysis tool**
   - Compute arithmetic intensity of graph
   - Identify memory-bound vs compute-bound paths

4. **Benchmark against physical predictions**
   - Measure actual cache hit rates
   - Compare energy estimates to reality

The theory is complete. The implementation continues.
