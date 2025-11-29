//! # PXYZ Scientific Hypotheses Tests
//!
//! Falsifiable hypotheses about PXYZ's design that map to real physics/engineering principles.
//! Pattern: ml-vm/tests/e2e_hypotheses.rs

use pxyz::*;
use pxyz::physics::{Energy, EnergyCosts};
use pxyz::limits::*;

/// H1: Graph Constraints Are Unrepresentable-Illegal
///
/// The type system prevents illegal states from being constructed.
/// If it compiles, the graph structure is valid.
#[test]
fn h1_type_system_prevents_illegal_states() {
    // NodeKind is repr(u8) with exhaustive match
    for kind_byte in 0..=6 {
        assert!(NodeKind::from_byte(kind_byte).is_some(),
            "NodeKind byte {} must map to valid variant", kind_byte);
    }

    // Invalid bytes are unrepresentable
    assert!(NodeKind::from_byte(7).is_none(), "Invalid NodeKind byte 7 should return None");
    assert!(NodeKind::from_byte(255).is_none(), "Invalid NodeKind byte 255 should return None");
}

/// H2: Bounded Execution IS an Energy Budget
///
/// MAX_VISITED_NODES and MAX_PREDICATE_STEPS are not arbitrary limits.
/// They map to real energy consumption based on Horowitz 2014.
#[test]
fn h2_bounded_execution_maps_to_energy_budget() {
    let costs = EnergyCosts::STANDARD;

    // Hypothesis: MAX_PREDICATE_STEPS × stack_op_cost < reasonable budget
    let predicate_budget: Energy = Energy::new(MAX_PREDICATE_STEPS as u64 * costs.stack_push.as_units());

    // Each predicate step costs at least 1 EU (stack op)
    // 256 steps = reasonable complexity without runaway computation
    assert_eq!(MAX_PREDICATE_STEPS, 256, "Predicate step limit must be 256");
    assert!(predicate_budget.as_units() < 10_000,
        "Predicate budget {} exceeds reasonable threshold", predicate_budget.as_units());

    // Hypothesis: MAX_VISITED_NODES × node_cost = full traversal budget
    let traversal_budget: Energy = Energy::new(MAX_VISITED_NODES as u64 * costs.node_visit.as_units());

    // 1000 nodes × 100 EU/node = 100k EU budget
    assert_eq!(MAX_VISITED_NODES, 1000, "Max visited nodes must be 1000");
    assert!(traversal_budget.as_units() > 0, "Traversal budget must be non-zero");
    assert!(traversal_budget.as_units() < 10_000_000,
        "Traversal budget {} too large (DoS risk)", traversal_budget.as_units());
}

/// H3: Pragmatic Constraints Enforce Safety Invariants
///
/// The compiler validates that dangerous patterns (LLM→Irreversible, unconfirmed writes)
/// are structurally impossible or require explicit gates.
#[test]
fn h3_pragmatic_checks_prevent_dangerous_patterns() {
    // Test that irreversible op codes are correctly identified
    let irreversible_ops = [
        0x0332,  // GoogleGmailSend
        0x0340,  // EmailSend
        0x0350,  // SmsSend
        0x0360,  // WebhookCall
    ];

    for &op in &irreversible_ops {
        assert!(is_irreversible_op(op),
            "Op code 0x{:04x} should be marked irreversible", op);
    }

    // Test that LLM ops are in the correct range
    assert!(is_llm_op(0x0800), "LLM complete should be detected");
    assert!(is_llm_op(0x08FF), "LLM range end should be detected");
    assert!(!is_llm_op(0x0700), "Vector ops are not LLM ops");
    assert!(!is_llm_op(0x0900), "Storage ops are not LLM ops");
}

/// H4: Energy Costs Reflect Physical Reality
///
/// DRAM access costs ~6400× a register operation (Horowitz 2014).
/// Our energy model must preserve this ratio.
#[test]
fn h4_energy_costs_match_horowitz_2014() {
    let costs = EnergyCosts::STANDARD;

    // Key ratios from Horowitz 2014:
    // DRAM / Register ≈ 6400×
    let dram_cost = costs.load_dram.as_units();
    let register_cost = costs.load_register.as_units();

    let ratio = dram_cost as f64 / register_cost as f64;

    // Allow some tolerance, but must be in the ballpark
    assert!(ratio > 1000.0, "DRAM must be >> register (got {}×)", ratio);
    assert!(ratio < 100_000.0, "Ratio must be physically plausible (got {}×)", ratio);

    // L1 << L2 << L3 << DRAM (monotonic increasing)
    assert!(costs.load_l1.as_units() < costs.load_l2.as_units(),
        "L1 must be faster than L2");
    assert!(costs.load_l2.as_units() < costs.load_l3.as_units(),
        "L2 must be faster than L3");
    assert!(costs.load_l3.as_units() < costs.load_dram.as_units(),
        "L3 must be faster than DRAM");
}

/// H5: Hash Function Has Good Distribution
///
/// FNV-1a hash for (P, X) entry lookup must avoid collisions.
#[test]
fn h5_hash_px_has_good_distribution() {
    let test_cases = vec![
        ("contact", "search"),
        ("contact", "create"),
        ("contact", "update"),
        ("loan", "apply"),
        ("loan", "approve"),
        ("user", "login"),
        ("user", "logout"),
    ];

    let mut hashes = std::collections::HashSet::new();

    for (p, x) in &test_cases {
        let hash = hash_px(p, x);
        assert!(!hashes.contains(&hash),
            "Hash collision for ({}, {}): 0x{:08x}", p, x, hash);
        hashes.insert(hash);
    }

    // Same input = same hash (determinism)
    assert_eq!(hash_px("contact", "search"), hash_px("contact", "search"));

    // Different inputs = different hashes (no trivial collisions)
    assert_ne!(hash_px("contact", "search"), hash_px("contact", "create"));
    assert_ne!(hash_px("contact", "search"), hash_px("loan", "search"));
}

/// H6: Binary Format Constants Match Specification
///
/// The binary header layout must match the documented spec exactly.
#[test]
fn h6_binary_format_constants_match_spec() {
    // Magic must be "PXYZ" in little-endian
    assert_eq!(MAGIC, 0x504E5958, "Magic number must be 'PXYZ'");

    // Header size must be 96 bytes (enough for all metadata)
    assert_eq!(HEADER_SIZE, 96, "Header must be 96 bytes");

    // Version format
    assert_eq!(VERSION_MAJOR, 1, "Major version should be 1");
    assert_eq!(VERSION_MINOR, 0, "Minor version should be 0");
}
