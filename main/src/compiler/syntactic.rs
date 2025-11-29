//! Syntactic Constraints - Structure Validation
//!
//! These checks verify the graph structure is valid:
//! - All references resolve
//! - No duplicates
//! - Required elements present
//!
//! Errors here indicate malformed input, not business logic issues.

use crate::{Diagnostic, Location, Severity};
use crate::compiler::ir::*;
use std::collections::HashSet;

/// Run all syntactic checks
pub fn check(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    diags.extend(check_edge_targets_exist(ir));
    diags.extend(check_entry_points_exist(ir));
    diags.extend(check_predicate_refs_exist(ir));
    diags.extend(check_no_duplicate_node_ids(ir));
    diags.extend(check_has_entry_points(ir));
    diags.extend(check_no_duplicate_entries(ir));
    diags.extend(check_edge_sources_exist(ir));
    
    diags
}

/// SYN001: Edge targets must reference existing nodes
fn check_edge_targets_exist(ir: &GraphIR) -> Vec<Diagnostic> {
    let node_ids: HashSet<NodeId> = ir.nodes.iter().map(|n| n.id).collect();
    let mut diags = Vec::new();
    
    for edge in &ir.edges {
        if !node_ids.contains(&edge.target) {
            diags.push(Diagnostic {
                severity: Severity::Error,
                code: "SYN001".into(),
                message: format!(
                    "Edge {} targets non-existent node {}",
                    edge.id, edge.target
                ),
                hint: Some("Check that target node is defined in the workflow".into()),
                location: Some(Location {
                    edge_id: Some(edge.id.to_string()),
                    ..Default::default()
                }),
            });
        }
    }
    
    diags
}

/// SYN002: Entry points must reference existing nodes
fn check_entry_points_exist(ir: &GraphIR) -> Vec<Diagnostic> {
    let node_ids: HashSet<NodeId> = ir.nodes.iter().map(|n| n.id).collect();
    let mut diags = Vec::new();
    
    for entry in &ir.entries {
        if !node_ids.contains(&entry.node_id) {
            diags.push(Diagnostic {
                severity: Severity::Error,
                code: "SYN002".into(),
                message: format!(
                    "Entry point ({}, {}) references non-existent node {}",
                    entry.p, entry.x, entry.node_id
                ),
                hint: Some("Check that entry node is defined".into()),
                location: None,
            });
        }
    }
    
    diags
}

/// SYN003: Edge predicate references must exist
fn check_predicate_refs_exist(ir: &GraphIR) -> Vec<Diagnostic> {
    let pred_ids: HashSet<PredicateId> = ir.predicates.iter().map(|p| p.id).collect();
    let mut diags = Vec::new();
    
    for edge in &ir.edges {
        // Predicate ID 0 = always true (implicit), no need to check
        if edge.predicate_id > 0 && !pred_ids.contains(&edge.predicate_id) {
            diags.push(Diagnostic {
                severity: Severity::Error,
                code: "SYN003".into(),
                message: format!(
                    "Edge {} references non-existent predicate {}",
                    edge.id, edge.predicate_id
                ),
                hint: Some("Check predicate definition".into()),
                location: Some(Location {
                    edge_id: Some(edge.id.to_string()),
                    ..Default::default()
                }),
            });
        }
    }
    
    // Also check node auth predicates
    for node in &ir.nodes {
        if let Some(pred_id) = node.auth_predicate {
            if !pred_ids.contains(&pred_id) {
                diags.push(Diagnostic {
                    severity: Severity::Error,
                    code: "SYN003".into(),
                    message: format!(
                        "Node '{}' references non-existent auth predicate {}",
                        node.name, pred_id
                    ),
                    hint: Some("Check predicate definition".into()),
                    location: Some(Location {
                        node_id: Some(node.name.clone()),
                        ..Default::default()
                    }),
                });
            }
        }
    }
    
    diags
}

/// SYN004: No duplicate node IDs
fn check_no_duplicate_node_ids(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut seen_ids: HashSet<NodeId> = HashSet::new();
    let mut seen_names: HashSet<&str> = HashSet::new();
    let mut diags = Vec::new();
    
    for node in &ir.nodes {
        if !seen_ids.insert(node.id) {
            diags.push(Diagnostic {
                severity: Severity::Error,
                code: "SYN004".into(),
                message: format!("Duplicate node ID: {}", node.id),
                hint: None,
                location: Some(Location {
                    node_id: Some(node.name.clone()),
                    ..Default::default()
                }),
            });
        }
        
        if !seen_names.insert(&node.name) {
            diags.push(Diagnostic {
                severity: Severity::Error,
                code: "SYN004".into(),
                message: format!("Duplicate node name: {}", node.name),
                hint: Some("Node names must be unique within a workflow".into()),
                location: Some(Location {
                    node_id: Some(node.name.clone()),
                    ..Default::default()
                }),
            });
        }
    }
    
    diags
}

/// SYN005: At least one entry point required
fn check_has_entry_points(ir: &GraphIR) -> Vec<Diagnostic> {
    if ir.entries.is_empty() {
        vec![Diagnostic {
            severity: Severity::Error,
            code: "SYN005".into(),
            message: "No entry points defined".into(),
            hint: Some("Add <entry p=\"...\" x=\"...\" node=\"...\"/> to workflow".into()),
            location: None,
        }]
    } else {
        vec![]
    }
}

/// SYN006: No duplicate entry points (same P, X)
fn check_no_duplicate_entries(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut seen: HashSet<u32> = HashSet::new();
    let mut diags = Vec::new();
    
    for entry in &ir.entries {
        if !seen.insert(entry.px_hash) {
            diags.push(Diagnostic {
                severity: Severity::Error,
                code: "SYN006".into(),
                message: format!(
                    "Duplicate entry point: ({}, {})",
                    entry.p, entry.x
                ),
                hint: Some("Each (P, X) combination can only have one entry".into()),
                location: None,
            });
        }
    }
    
    diags
}

/// SYN007: Edge sources must reference existing nodes
fn check_edge_sources_exist(ir: &GraphIR) -> Vec<Diagnostic> {
    let node_ids: HashSet<NodeId> = ir.nodes.iter().map(|n| n.id).collect();
    let mut diags = Vec::new();
    
    for edge in &ir.edges {
        if !node_ids.contains(&edge.from) {
            diags.push(Diagnostic {
                severity: Severity::Error,
                code: "SYN007".into(),
                message: format!(
                    "Edge {} originates from non-existent node {}",
                    edge.id, edge.from
                ),
                hint: Some("Check that source node is defined".into()),
                location: Some(Location {
                    edge_id: Some(edge.id.to_string()),
                    ..Default::default()
                }),
            });
        }
    }
    
    diags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeKind;
    
    fn make_valid_ir() -> GraphIR {
        let mut ir = GraphIR::new();
        
        ir.nodes.push(GNode::new(0, "start".into(), NodeKind::Transform));
        ir.nodes.push(GNode::new(1, "end".into(), NodeKind::Terminal));
        
        ir.edges.push(GEdge::new(0, 0, 1));
        
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        ir
    }
    
    #[test]
    fn test_valid_ir_passes() {
        let ir = make_valid_ir();
        let diags = check(&ir);
        
        assert!(diags.is_empty(), "Expected no errors, got: {:?}", diags);
    }
    
    #[test]
    fn test_syn001_edge_target_missing() {
        let mut ir = make_valid_ir();
        ir.edges.push(GEdge::new(1, 0, 99)); // Target 99 doesn't exist
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SYN001"));
    }
    
    #[test]
    fn test_syn002_entry_node_missing() {
        let mut ir = make_valid_ir();
        ir.entries.push(GEntry::new("other".into(), "action".into(), 99));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SYN002"));
    }
    
    #[test]
    fn test_syn003_predicate_missing() {
        let mut ir = make_valid_ir();
        ir.edges[0].predicate_id = 999; // Doesn't exist
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SYN003"));
    }
    
    #[test]
    fn test_syn004_duplicate_node() {
        let mut ir = make_valid_ir();
        ir.nodes.push(GNode::new(0, "duplicate".into(), NodeKind::Transform));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SYN004"));
    }
    
    #[test]
    fn test_syn005_no_entries() {
        let mut ir = make_valid_ir();
        ir.entries.clear();
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SYN005"));
    }
    
    #[test]
    fn test_syn006_duplicate_entry() {
        let mut ir = make_valid_ir();
        ir.entries.push(GEntry::new("test".into(), "run".into(), 1)); // Same P, X
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SYN006"));
    }
    
    #[test]
    fn test_syn007_edge_source_missing() {
        let mut ir = make_valid_ir();
        ir.edges.push(GEdge::new(1, 99, 1)); // Source 99 doesn't exist
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SYN007"));
    }
}