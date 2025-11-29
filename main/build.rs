//! Build Script for PXYZ
//!
//! SSOT Architecture:
//! - lib.rs types are the canonical source of truth
//! - This script validates consistency (phase 1)
//! - Future: Generate WAT constants + CLAUDE.md tables (phase 2)
//!
//! Phase 1 (Alpha): Validation only
//! Phase 2 (Post-alpha): Full codegen

fn main() {
    println!("cargo:rerun-if-changed=lib.rs");
    println!("cargo:rerun-if-changed=src/compiler/mod.rs");

    // Phase 1: Validate SSOT consistency
    validate_ssot();
}

/// Validate that lib.rs types match documented specifications
fn validate_ssot() {
    // TODO: Parse lib.rs and extract:
    // - NodeKind variants (should be 7: Transform=0..Error=6)
    // - Opcode variants
    // - node_flags constants
    // - edge_flags constants

    // For now, just verify the build system works
    println!("cargo:warning=PXYZ SSOT validation: Phase 1 (validation only)");

    // Phase 2 will generate:
    // 1. wasm/pxyz_generated.wat with (global $KIND_TRANSFORM i32 (i32.const 0)) etc.
    // 2. CLAUDE_generated.md with tables
    // 3. Optional: pxyz_meta_graph.json describing the compiler pipeline
}
