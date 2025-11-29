//! # PXYZ Physics Module
//!
//! Energy-aware computation primitives based on thermodynamic principles.
//!
//! Key insight: PXYZ's bounded execution model IS an energy budget model.
//! Every operation has a cost. The limits are energy caps.
//!
//! References:
//! - Landauer (1961): kT ln(2) ≈ 2.87 × 10⁻²¹ J minimum per bit erasure
//! - Horowitz (2014): DRAM access = 6,400× energy of 32-bit add
//! - Bennett (1973): Reversible computation avoids Landauer cost

pub mod energy;
pub mod cache;
pub mod layout;
pub mod thermal;

pub use energy::*;
pub use cache::*;
pub use layout::*;
pub use thermal::*;
