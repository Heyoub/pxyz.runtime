//! # Energy Accounting for PXYZ Operations
//!
//! Every operation has an energy cost. The bounded execution model
//! is fundamentally an energy budget.
//!
//! ## Energy Units
//!
//! We use abstract "energy units" (EU) that map to physical picojoules:
//! - 1 EU ≈ 1 pJ for register operations
//! - Scaling based on Horowitz 2014 ISSCC measurements
//!
//! ## Budget Model
//!
//! ```text
//! MAX_PREDICATE_STEPS = 256  →  256 EU budget per predicate
//! MAX_VISITED_NODES = 1000   →  ~1,000,000 EU total traversal budget
//! ```

use std::ops::{Add, AddAssign, Sub};

/// Energy unit (abstract, maps to ~1 picojoule)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct Energy(u64);

impl Energy {
    pub const ZERO: Energy = Energy(0);

    /// Landauer limit at room temperature (in our abstract units)
    /// Real value: 2.87 × 10⁻²¹ J ≈ 0.00287 pJ
    /// We round up to 1 for integer math
    pub const LANDAUER_LIMIT: Energy = Energy(1);

    /// Create energy from raw units
    #[inline]
    pub const fn new(units: u64) -> Self {
        Energy(units)
    }

    /// Get raw energy value
    #[inline]
    pub const fn as_units(&self) -> u64 {
        self.0
    }

    /// Convert to picojoules (approximate)
    #[inline]
    pub const fn as_picojoules(&self) -> f64 {
        self.0 as f64
    }

    /// Check if within budget
    #[inline]
    pub const fn within_budget(&self, budget: Energy) -> bool {
        self.0 <= budget.0
    }
}

impl Add for Energy {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Energy(self.0.saturating_add(rhs.0))
    }
}

impl AddAssign for Energy {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.saturating_add(rhs.0);
    }
}

impl Sub for Energy {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Energy(self.0.saturating_sub(rhs.0))
    }
}

/// Energy costs for PXYZ operations
///
/// Based on Horowitz 2014 measurements, scaled to abstract units.
/// Key insight: DRAM access costs 6,400× a 32-bit add.
#[derive(Debug, Clone, Copy)]
pub struct EnergyCosts {
    // Stack operations (register-level)
    pub stack_push: Energy,
    pub stack_pop: Energy,

    // Predicate VM operations
    pub compare: Energy,
    pub logic_op: Energy,
    pub arithmetic: Energy,

    // Memory hierarchy
    pub load_register: Energy,
    pub load_l1: Energy,
    pub load_l2: Energy,
    pub load_l3: Energy,
    pub load_dram: Energy,
    pub load_hbm: Energy,

    // Graph traversal
    pub edge_eval: Energy,
    pub node_visit: Energy,

    // External I/O
    pub io_call: Energy,
}

impl Default for EnergyCosts {
    fn default() -> Self {
        Self::STANDARD
    }
}

impl EnergyCosts {
    /// Standard energy costs based on Horowitz 2014
    ///
    /// Reference: 32-bit add = 0.1 pJ, DRAM = 640 pJ
    /// We normalize: add = 1 EU, DRAM = 6400 EU
    pub const STANDARD: EnergyCosts = EnergyCosts {
        // Register operations: ~1 pJ
        stack_push: Energy::new(1),
        stack_pop: Energy::new(1),

        // ALU operations: ~0.1-4 pJ
        compare: Energy::new(1),
        logic_op: Energy::new(1),
        arithmetic: Energy::new(10),  // FP multiply ~37 pJ = 37 EU

        // Memory hierarchy (from Horowitz)
        load_register: Energy::new(1),      // ~1 pJ
        load_l1: Energy::new(5),            // ~5 pJ for 8KB
        load_l2: Energy::new(100),          // ~100 pJ for 1MB
        load_l3: Energy::new(500),          // ~500 pJ
        load_dram: Energy::new(6400),       // ~640 pJ × 10 (bus overhead)
        load_hbm: Energy::new(3200),        // ~half DRAM (shorter wires)

        // Graph traversal (includes edge lookup + predicate)
        edge_eval: Energy::new(50),         // L1-L2 range
        node_visit: Energy::new(100),       // L2 access

        // External I/O (DRAM + bus + syscall)
        io_call: Energy::new(100_000),      // Very expensive
    };

    /// Low-power mode (aggressive caching assumed)
    pub const LOW_POWER: EnergyCosts = EnergyCosts {
        stack_push: Energy::new(1),
        stack_pop: Energy::new(1),
        compare: Energy::new(1),
        logic_op: Energy::new(1),
        arithmetic: Energy::new(5),
        load_register: Energy::new(1),
        load_l1: Energy::new(3),
        load_l2: Energy::new(50),
        load_l3: Energy::new(200),
        load_dram: Energy::new(3000),
        load_hbm: Energy::new(1500),
        edge_eval: Energy::new(25),
        node_visit: Energy::new(50),
        io_call: Energy::new(50_000),
    };
}

/// Energy budget for a computation
#[derive(Debug, Clone)]
pub struct EnergyBudget {
    /// Total budget
    pub total: Energy,
    /// Consumed so far
    pub consumed: Energy,
    /// Per-predicate budget
    pub per_predicate: Energy,
    /// Cost model in use
    pub costs: EnergyCosts,
}

impl EnergyBudget {
    /// Create from PXYZ limits
    ///
    /// Maps bounded execution limits to energy:
    /// - MAX_PREDICATE_STEPS = 256 → 256 EU per predicate
    /// - MAX_VISITED_NODES = 1000 → 1000 × node_visit EU total
    pub fn from_pxyz_limits(costs: EnergyCosts) -> Self {
        let per_predicate = Energy::new(256 * costs.compare.as_units());
        let total = Energy::new(1000 * costs.node_visit.as_units());

        Self {
            total,
            consumed: Energy::ZERO,
            per_predicate,
            costs,
        }
    }

    /// Spend energy, return false if over budget
    #[inline]
    pub fn spend(&mut self, amount: Energy) -> bool {
        if self.consumed + amount <= self.total {
            self.consumed += amount;
            true
        } else {
            false
        }
    }

    /// Remaining energy
    #[inline]
    pub fn remaining(&self) -> Energy {
        self.total - self.consumed
    }

    /// Efficiency ratio (0.0 - 1.0)
    #[inline]
    pub fn efficiency(&self) -> f64 {
        if self.total.as_units() == 0 {
            1.0
        } else {
            1.0 - (self.consumed.as_units() as f64 / self.total.as_units() as f64)
        }
    }
}

/// Energy-aware operation result
#[derive(Debug)]
pub struct EnergyResult<T> {
    pub value: T,
    pub energy_spent: Energy,
}

impl<T> EnergyResult<T> {
    pub fn new(value: T, energy: Energy) -> Self {
        Self {
            value,
            energy_spent: energy,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> EnergyResult<U> {
        EnergyResult {
            value: f(self.value),
            energy_spent: self.energy_spent,
        }
    }
}

/// Trait for operations that consume energy
pub trait EnergyAware {
    /// Execute with energy tracking
    fn execute_with_energy(&self, budget: &mut EnergyBudget) -> Result<Energy, EnergyExhausted>;
}

/// Error when energy budget is exhausted
#[derive(Debug, Clone, Copy)]
pub struct EnergyExhausted {
    pub required: Energy,
    pub available: Energy,
}

impl std::fmt::Display for EnergyExhausted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Energy exhausted: required {} EU, only {} EU available",
            self.required.as_units(),
            self.available.as_units()
        )
    }
}

impl std::error::Error for EnergyExhausted {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_budget() {
        let costs = EnergyCosts::STANDARD;
        let mut budget = EnergyBudget::from_pxyz_limits(costs);

        // Should be able to spend within budget
        assert!(budget.spend(Energy::new(100)));
        assert!(budget.spend(Energy::new(100)));

        // Check efficiency decreases
        let eff = budget.efficiency();
        assert!(eff < 1.0);
        assert!(eff > 0.0);
    }

    #[test]
    fn test_dram_is_expensive() {
        let costs = EnergyCosts::STANDARD;

        // DRAM should be 6400× register cost (per Horowitz)
        let ratio = costs.load_dram.as_units() / costs.load_register.as_units();
        assert_eq!(ratio, 6400);
    }
}
