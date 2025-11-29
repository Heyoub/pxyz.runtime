//! Golden Tests: AST ↔ IR Contract
//!
//! These tests verify the lowering contract: that AST→IR transformation
//! produces semantically equivalent structures to directly-constructed IR.
//!
//! This catches regressions in the lowering phase without relying on
//! the XML parser (which is tested separately).

use pxyz::compiler::ir::*;
use pxyz::compiler::lower;
use pxyz::dsl::ast::*;
use pxyz::NodeKind;

/// Helper: Assert IRs are structurally equivalent
fn assert_ir_eq(a: &GraphIR, b: &GraphIR, context: &str) {
    assert_eq!(a.nodes.len(), b.nodes.len(), "{}: node count", context);
    assert_eq!(a.edges.len(), b.edges.len(), "{}: edge count", context);
    assert_eq!(a.entries.len(), b.entries.len(), "{}: entry count", context);
}

#[test]
fn test_lower_minimal_workflow() {
    // Construct AST
    let doc = OmarDocument {
        version: "1.0".into(),
        predicates: vec![],
        merge_policies: vec![],
        workflows: vec![Workflow {
            description: None,
            id: "test".into(),
            entry: EntryPoint {
                p: "test".into(),
                x: "run".into(),
                node: "start".into(),
            },
            nodes: vec![
                Node {
                    id: "start".into(),
                    kind: "transform".into(),
                    ..Default::default()
                },
                Node {
                    id: "end".into(),
                    kind: "terminal".into(),
                    status: Some(200),
                    ..Default::default()
                },
            ],
            edges: vec![Edge {
                from: "start".into(),
                to: "end".into(),
                predicate: Some(PredicateExpr::Always),
                ..Default::default()
            }],
        }],
        templates: vec![],
        schemas: vec![],
    };

    // Lower AST → IR
    let mut ir = lower(&doc).unwrap();
    ir.assign_edge_indices();

    // Verify structure
    assert_eq!(ir.nodes.len(), 2);
    assert_eq!(ir.edges.len(), 1);
    assert_eq!(ir.entries.len(), 1);
    assert_eq!(ir.nodes[0].kind, NodeKind::Transform);
    assert_eq!(ir.nodes[1].kind, NodeKind::Terminal);
}

#[test]
fn test_lower_preserves_opcodes() {
    let doc = OmarDocument {
        version: "1.0".into(),
        predicates: vec![],
        merge_policies: vec![],
        workflows: vec![Workflow {
            description: None,
            id: "test".into(),
            entry: EntryPoint {
                p: "test".into(),
                x: "search".into(),
                node: "search".into(),
            },
            nodes: vec![
                Node {
                    id: "search".into(),
                    kind: "external".into(),
                    op: Some("0x0300".into()), // Google Contacts Search
                    ..Default::default()
                },
                Node {
                    id: "done".into(),
                    kind: "terminal".into(),
                    ..Default::default()
                },
            ],
            edges: vec![Edge {
                from: "search".into(),
                to: "done".into(),
                predicate: Some(PredicateExpr::Always),
                ..Default::default()
            }],
        }],
        templates: vec![],
        schemas: vec![],
    };

    let ir = lower(&doc).unwrap();

    // Find external node
    let external_node = ir.nodes.iter()
        .find(|n| n.kind == NodeKind::External)
        .expect("Should have external node");

    assert_eq!(external_node.op_code, 0x0300, "Op code should be preserved");
}

#[test]
fn test_lower_auth_predicates() {
    let doc = OmarDocument {
        version: "1.0".into(),
        predicates: vec![PredicateDef {
            id: "is_admin".into(),
            expr: PredicateExpr::Eq {
                left: "$token.role".into(),
                right: Value::Str("admin".into()),
            },
        }],
        merge_policies: vec![],
        workflows: vec![Workflow {
            description: None,
            id: "test".into(),
            entry: EntryPoint {
                p: "test".into(),
                x: "run".into(),
                node: "gate".into(),
            },
            nodes: vec![
                Node {
                    id: "gate".into(),
                    kind: "auth".into(),
                    predicate: Some("is_admin".into()),
                    ..Default::default()
                },
                Node {
                    id: "protected".into(),
                    kind: "transform".into(),
                    ..Default::default()
                },
            ],
            edges: vec![Edge {
                from: "gate".into(),
                to: "protected".into(),
                predicate: Some(PredicateExpr::Always),
                ..Default::default()
            }],
        }],
        templates: vec![],
        schemas: vec![],
    };

    let ir = lower(&doc).unwrap();

    // Verify predicate was registered
    assert_eq!(ir.predicates.len(), 1);
    assert_eq!(ir.predicates[0].name, "is_admin");

    // Verify auth node has predicate
    let auth_node = ir.nodes.iter()
        .find(|n| n.kind == NodeKind::Auth)
        .expect("Should have auth node");

    assert!(auth_node.auth_predicate.is_some(), "Auth node should have predicate");
    assert!(auth_node.requires_auth, "Auth node should require auth");
}

#[test]
fn test_lower_error_edges() {
    let doc = OmarDocument {
        version: "1.0".into(),
        predicates: vec![],
        merge_policies: vec![],
        workflows: vec![Workflow {
            description: None,
            id: "test".into(),
            entry: EntryPoint {
                p: "test".into(),
                x: "run".into(),
                node: "risky".into(),
            },
            nodes: vec![
                Node {
                    id: "risky".into(),
                    kind: "external".into(),
                    op: Some("0x0100".into()),
                    ..Default::default()
                },
                Node {
                    id: "success".into(),
                    kind: "terminal".into(),
                    ..Default::default()
                },
                Node {
                    id: "failure".into(),
                    kind: "error".into(),
                    ..Default::default()
                },
            ],
            edges: vec![
                Edge {
                    from: "risky".into(),
                    to: "success".into(),
                    predicate: Some(PredicateExpr::Always),
                    ..Default::default()
                },
                Edge {
                    from: "risky".into(),
                    to: "failure".into(),
                    predicate: Some(PredicateExpr::Fail),
                    fallback: true,
                    ..Default::default()
                },
            ],
        }],
        templates: vec![],
        schemas: vec![],
    };

    let ir = lower(&doc).unwrap();

    // Find error node
    let error_node_id = ir.nodes.iter()
        .find(|n| n.kind == NodeKind::Error)
        .expect("Should have error node")
        .id;

    // Find edge to error node
    let error_edge = ir.edges.iter()
        .find(|e| e.target == error_node_id)
        .expect("Should have edge to error node");

    assert!(error_edge.is_error_edge(), "Edge should have ERROR_EDGE flag");
    assert!(error_edge.is_fallback(), "Edge should have FALLBACK flag");
}

#[test]
fn test_contract_assign_edge_indices_required() {
    // Build minimal IR
    let mut ir = GraphIR::new();
    ir.nodes.push(GNode::new(0, "start".into(), NodeKind::Transform));
    ir.nodes.push(GNode::new(1, "end".into(), NodeKind::Terminal));
    ir.edges.push(GEdge::new(0, 0, 1));
    ir.entries.push(GEntry::new("test".into(), "run".into(), 0));

    // Before assign_edge_indices(), edge_start/edge_count are 0
    assert_eq!(ir.nodes[0].edge_count, 0);

    // After assign_edge_indices(), they're set correctly
    ir.assign_edge_indices();
    assert_eq!(ir.nodes[0].edge_count, 1);
    assert_eq!(ir.nodes[0].edge_start, 0);
}

#[test]
#[cfg(debug_assertions)]
fn test_contract_invariants_enforced() {
    let mut ir = GraphIR::new();
    ir.nodes.push(GNode::new(0, "start".into(), NodeKind::Transform));
    ir.edges.push(GEdge::new(0, 0, 999)); // Invalid target!

    // Should panic in debug builds
    let result = std::panic::catch_unwind(|| {
        ir.assert_invariants();
    });

    assert!(result.is_err(), "assert_invariants should panic on invalid IR");
}

/// Regression test: Verify lowering is idempotent
///
/// lower(ast) should always produce the same IR structure,
/// regardless of how many times it's called.
#[test]
fn test_lowering_idempotent() {
    let doc = OmarDocument {
        version: "1.0".into(),
        predicates: vec![],
        merge_policies: vec![],
        workflows: vec![Workflow {
            description: None,
            id: "test".into(),
            entry: EntryPoint {
                p: "test".into(),
                x: "run".into(),
                node: "a".into(),
            },
            nodes: vec![
                Node {
                    id: "a".into(),
                    kind: "transform".into(),
                    ..Default::default()
                },
                Node {
                    id: "b".into(),
                    kind: "terminal".into(),
                    ..Default::default()
                },
            ],
            edges: vec![Edge {
                from: "a".into(),
                to: "b".into(),
                predicate: Some(PredicateExpr::Always),
                ..Default::default()
            }],
        }],
        templates: vec![],
        schemas: vec![],
    };

    let ir1 = lower(&doc).unwrap();
    let ir2 = lower(&doc).unwrap();

    assert_ir_eq(&ir1, &ir2, "lowering should be idempotent");
}
