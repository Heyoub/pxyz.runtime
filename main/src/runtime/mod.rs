//! PXYZ Runtime Specification
//!
//! This module defines the **contract** for PXYZ runtime services.
//! Both TypeScript (Effect-TS) and Rust/WASM implementations must satisfy
//! these interfaces to be considered PXYZ-compatible.
//!
//! # Architecture
//!
//! - GraphIR (compiler) produces WASM bytecode
//! - WASM runtime imports **exactly 3 host functions** (emit_event, validate_shape, constraint_hash)
//! - Host environment implements these via RuntimeServices
//! - All side effects flow through these gates
//!
//! # Cross-Language Guarantee
//!
//! Golden tests ensure TS and Rust agree on:
//! - Constraint hash algorithm (FNV-1a with sorted keys)
//! - Shape projection generation (All/Active/Recent)
//! - Error semantics (duplicate names, invalid identifiers)

pub mod constraint;
pub mod shape;
pub mod host;
pub mod pxyz;

// Re-exports for convenience
pub use constraint::{ConstraintHash, hash_yctx};
pub use shape::{ShapeRegistry, ShapeRegistryLike, ShapeError, RegisteredShape};
pub use host::{HostEnv, EventSink, HostError};
pub use pxyz::Pxyz;
