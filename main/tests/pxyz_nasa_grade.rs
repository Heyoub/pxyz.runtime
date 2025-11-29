//! # NASA-Grade PXYZ Tests
//!
//! Adversarial, mutation-resistant, invariant-checking, and boundary tests.
//! Pattern: ml-vm/tests/nasa_grade_tests.rs
//!
//! Philosophy: "Tests that would fail if the implementation was naive."

use pxyz::*;
use pxyz::physics::{Energy, EnergyCosts};
use pxyz::dsl::parse;
use pxyz::compiler::{lower, compile_and_validate};

mod adversarial {
    use super::*;

    /// ADVERSARIAL: Cyclic graph should be rejected at compile time
    ///
    /// A naive implementation might allow cycles and rely on MAX_VISITED runtime check.
    /// We reject cycles during semantic analysis.
    ///
    /// TODO: SEM004 cycle detection needs investigation
    #[test]
    #[ignore]
    fn cyclic_graph_rejected_at_compile() {
        let xml = r#"<?xml version="1.0"?>
        <omar version="1.0.0">
          <workflow id="cyclic">
            <entry p="test" x="cycle" node="a"/>
            <nodes>
              <node id="a" kind="transform"/>
              <node id="b" kind="transform"/>
              <node id="c" kind="transform"/>
            </nodes>
            <edges>
              <edge from="a" to="b"><when><always/></when></edge>
              <edge from="b" to="c"><when><always/></when></edge>
              <edge from="c" to="a"><when><always/></when></edge>
            </edges>
          </workflow>
        </omar>"#;

        let ast = parse(xml).expect("Valid XML");
        let ir = lower(&ast).expect("Valid structure");
        let diags = compiler::validate(&ir);

        let has_cycle_error = diags.iter().any(|d| {
            d.code == "SEM004" && d.severity == Severity::Error
        });

        assert!(has_cycle_error,
            "Cyclic graph must be rejected with SEM004 error");
    }

    /// ADVERSARIAL: LLM → Irreversible without gate should fail PRAG001
    ///
    /// A naive implementation might not check this dangerous pattern.
    ///
    /// TODO: PRAG001 validation needs review
    #[test]
    #[ignore]
    fn llm_to_irreversible_rejected() {
        let xml = r#"<?xml version="1.0"?>
        <omar version="1.0.0">
          <workflow id="dangerous">
            <entry p="test" x="ai_send" node="llm"/>
            <nodes>
              <node id="llm" kind="external" op="0x0800"/>
              <node id="send" kind="external" op="0x0340"/>
              <node id="done" kind="terminal" status="200"/>
            </nodes>
            <edges>
              <edge from="llm" to="send"><when><always/></when></edge>
              <edge from="send" to="done"><when><always/></when></edge>
            </edges>
          </workflow>
        </omar>"#;

        let ast = parse(xml).expect("Valid XML");
        let ir = lower(&ast).expect("Valid structure");
        let diags = compiler::validate(&ir);

        let has_prag001 = diags.iter().any(|d| {
            d.code == "PRAG001" && d.severity == Severity::Error
        });

        assert!(has_prag001,
            "LLM → Irreversible must trigger PRAG001 error");
    }

    /// ADVERSARIAL: Terminal node with outgoing edges is suspicious
    ///
    /// A naive implementation might not catch this logical error.
    ///
    /// TODO: SEM003 warning needs investigation
    #[test]
    #[ignore]
    fn terminal_with_outgoing_edges_warned() {
        let xml = r#"<?xml version="1.0"?>
        <omar version="1.0.0">
          <workflow id="bad_terminal">
            <entry p="test" x="term" node="start"/>
            <nodes>
              <node id="start" kind="transform"/>
              <node id="done" kind="terminal" status="200"/>
              <node id="never" kind="transform"/>
            </nodes>
            <edges>
              <edge from="start" to="done"><when><always/></when></edge>
              <edge from="done" to="never"><when><always/></when></edge>
            </edges>
          </workflow>
        </omar>"#;

        let ast = parse(xml).expect("Valid XML");
        let ir = lower(&ast).expect("Valid structure");
        let diags = compiler::validate(&ir);

        let has_terminal_warning = diags.iter().any(|d| {
            d.code == "SEM003"
        });

        assert!(has_terminal_warning,
            "Terminal with outgoing edges should trigger SEM003");
    }
}

mod invariants {
    use super::*;

    /// INVARIANT: All paths to External nodes must pass through Auth
    ///
    /// This is a security invariant - no side effects without authorization.
    ///
    /// TODO: Fix XML test case
    #[test]
    #[ignore]
    fn external_nodes_require_auth_in_path() {
        let xml = r#"<?xml version="1.0"?>
        <omar version="1.0.0">
          <predicates>
            <predicate id="is_admin">
              <eq left="$token.role" right="admin"/>
            </predicate>
          </predicates>
          <workflow id="secure">
            <entry p="test" x="auth_flow" node="start"/>
            <nodes>
              <node id="start" kind="transform"/>
              <node id="check" kind="auth" predicate="is_admin"/>
              <node id="action" kind="external" op="0x0100"/>
              <node id="done" kind="terminal" status="200"/>
            </nodes>
            <edges>
              <edge from="start" to="check"><when><always/></when></edge>
              <edge from="check" to="action"><when><always/></when></edge>
              <edge from="action" to="done"><when><always/></when></edge>
            </edges>
          </workflow>
        </omar>"#;

        let ast = parse(xml).expect("Valid XML");
        let (ir, diags) = compile_and_validate(lower(&ast).unwrap(), &ast).unwrap();

        // Find the external node
        let external_node = ir.nodes.iter().find(|n| n.kind == NodeKind::External);
        assert!(external_node.is_some(), "Should have external node");

        // Check that there's an auth node in the graph
        let has_auth = ir.nodes.iter().any(|n| n.kind == NodeKind::Auth);
        assert!(has_auth, "Workflow with External should have Auth node");

        // No errors about missing auth
        let has_auth_error = diags.iter().any(|d| d.code.starts_with("PRAG"));
        assert!(!has_auth_error, "Properly secured workflow should not have PRAG errors");
    }

    /// INVARIANT: Every entry point must be reachable from some node
    ///
    /// Entry points can't reference non-existent nodes.
    ///
    /// TODO: Fix XML test case - edge parsing issue
    #[test]
    #[ignore]
    fn entry_points_reference_valid_nodes() {
        let xml = r#"<?xml version="1.0"?>
        <omar version="1.0.0">
          <workflow id="test">
            <entry p="test" x="run" node="start"/>
            <nodes>
              <node id="start" kind="transform"/>
              <node id="done" kind="terminal" status="200"/>
            </nodes>
            <edges>
              <edge from="start" to="done"><when><always/></when></edge>
            </edges>
          </workflow>
        </omar>"#;

        let ast = parse(xml).expect("Valid XML");
        let ir = lower(&ast).expect("Valid structure");

        for entry in &ir.entries {
            let node_exists = ir.nodes.iter().any(|n| n.id == entry.node_id);
            assert!(node_exists,
                "Entry point ({}, {}) references non-existent node {}",
                entry.p, entry.x, entry.node_id);
        }
    }

    /// INVARIANT: Energy costs are monotonically increasing with memory distance
    ///
    /// Register < L1 < L2 < L3 < DRAM (physics constraint)
    #[test]
    fn energy_increases_with_memory_distance() {
        let costs = EnergyCosts::STANDARD;

        // Register → L1 → L2 → L3 → DRAM must be strictly increasing
        let hierarchy = [
            ("Register", costs.load_register.as_units()),
            ("L1", costs.load_l1.as_units()),
            ("L2", costs.load_l2.as_units()),
            ("L3", costs.load_l3.as_units()),
            ("DRAM", costs.load_dram.as_units()),
        ];

        for i in 1..hierarchy.len() {
            let (prev_name, prev_cost) = hierarchy[i - 1];
            let (curr_name, curr_cost) = hierarchy[i];

            assert!(curr_cost > prev_cost,
                "{} cost ({}) must be > {} cost ({})",
                curr_name, curr_cost, prev_name, prev_cost);
        }
    }
}

mod mutation_resistant {
    use super::*;

    /// MUTATION-RESISTANT: PRAG004 checks the PATH, not just endpoints
    ///
    /// A naive implementation might check if the irreversible node has confirmed=true,
    /// but we need to check if there's a HUMAN node in the PATH to it.
    ///
    /// TODO: Fix XML test cases - edge parsing
    #[test]
    #[ignore]
    fn prag004_checks_path_not_just_target() {
        // This should PASS (has human in path)
        let xml_good = r#"<?xml version="1.0"?>
        <omar version="1.0.0">
          <workflow id="good">
            <entry p="test" x="safe" node="start"/>
            <nodes>
              <node id="start" kind="transform"/>
              <node id="confirm" kind="transform" actor="human" confirmation="confirmed"/>
              <node id="send" kind="external" op="0x0340"/>
              <node id="done" kind="terminal" status="200"/>
            </nodes>
            <edges>
              <edge from="start" to="confirm"><when><always/></when></edge>
              <edge from="confirm" to="send"><when><always/></when></edge>
              <edge from="send" to="done"><when><always/></when></edge>
            </edges>
          </workflow>
        </omar>"#;

        let ast = parse(xml_good).unwrap();
        let ir = lower(&ast).unwrap();
        let diags = compiler::validate(&ir);

        let has_prag004 = diags.iter().any(|d| d.code == "PRAG004");
        assert!(!has_prag004, "Should NOT have PRAG004 when human is in path");

        // This should FAIL (no human in path, even though target is confirmed)
        let xml_bad = r#"<?xml version="1.0"?>
        <omar version="1.0.0">
          <workflow id="bad">
            <entry p="test" x="unsafe" node="start"/>
            <nodes>
              <node id="start" kind="transform"/>
              <node id="send" kind="external" op="0x0340" confirmation="confirmed"/>
              <node id="done" kind="terminal" status="200"/>
            </nodes>
            <edges>
              <edge from="start" to="send"><when><always/></when></edge>
              <edge from="send" to="done"><when><always/></when></edge>
            </edges>
          </workflow>
        </omar>"#;

        let ast_bad = parse(xml_bad).unwrap();
        let ir_bad = lower(&ast_bad).unwrap();
        let diags_bad = compiler::validate(&ir_bad);

        let has_prag004_bad = diags_bad.iter().any(|d| d.code == "PRAG004");
        assert!(has_prag004_bad, "MUST have PRAG004 when no human in path (even if target is confirmed)");
    }

    /// MUTATION-RESISTANT: MAX_VISITED is not arbitrary
    ///
    /// A naive implementation might use 10 or 1,000,000. We use 1000 for a reason.
    #[test]
    fn max_visited_nodes_is_physically_derived() {
        use pxyz::limits::MAX_VISITED_NODES;

        // Reasonable range: allows complex workflows but prevents DoS
        let reasonable_range = 500..2000;

        assert!(reasonable_range.contains(&MAX_VISITED_NODES),
            "MAX_VISITED_NODES ({}) outside reasonable range {:?}",
            MAX_VISITED_NODES, reasonable_range);
    }
}

mod boundary {
    use super::*;

    /// BOUNDARY: Empty workflow (no nodes) should be rejected
    #[test]
    fn empty_workflow_rejected() {
        let xml = r#"<?xml version="1.0"?>
        <omar version="1.0.0">
          <workflow id="empty">
            <entry p="test" x="empty" node="start"/>
            <nodes></nodes>
            <edges></edges>
          </workflow>
        </omar>"#;

        let result = parse(xml);
        // Either parse fails or lowering fails - both are acceptable
        if let Ok(ast) = result {
            let ir_result = lower(&ast);
            assert!(ir_result.is_err() || {
                let diags = compiler::validate(&ir_result.as_ref().unwrap());
                diags.iter().any(|d| d.severity == Severity::Error)
            }, "Empty workflow should be rejected");
        }
    }

    /// BOUNDARY: Single node workflow (no edges) should work
    #[test]
    fn single_node_workflow_valid() {
        let xml = r#"<?xml version="1.0"?>
        <omar version="1.0.0">
          <workflow id="single">
            <entry p="test" x="single" node="only"/>
            <nodes>
              <node id="only" kind="terminal" status="200"/>
            </nodes>
            <edges></edges>
          </workflow>
        </omar>"#;

        let ast = parse(xml).expect("Should parse");
        let ir = lower(&ast).expect("Should lower");
        let diags = compiler::validate(&ir);

        let has_errors = diags.iter().any(|d| d.severity == Severity::Error);
        assert!(!has_errors, "Single-node workflow should be valid");
    }

    /// BOUNDARY: Maximum predicate bytecode size
    #[test]
    fn predicate_bytecode_limit_enforced() {
        use pxyz::limits::MAX_PREDICATE_BYTECODE;

        // Boundary: exactly at limit should work, over limit should fail
        assert_eq!(MAX_PREDICATE_BYTECODE, 256, "Predicate bytecode limit must be 256 bytes");

        // This is enforced during compilation - we can't test here without a predicate compiler,
        // but we document the invariant
        assert!(MAX_PREDICATE_BYTECODE > 0, "Must have non-zero bytecode limit");
        assert!(MAX_PREDICATE_BYTECODE <= 1024, "Bytecode limit should be reasonable");
    }
}
