//! Semantic Constraints - Logic Validation
//!
//! These checks verify the graph makes logical sense:
//! - Node types have required attributes
//! - Graph structure is coherent
//! - No cycles (DAG enforcement)
//!
//! Errors here indicate logic issues, not syntax problems.

use crate::{Diagnostic, Location, Severity, NodeKind};
use crate::compiler::ir::*;
use std::collections::{HashSet, VecDeque, HashMap};

/// Run all semantic checks
pub fn check(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    diags.extend(check_auth_nodes_have_predicates(ir));
    diags.extend(check_external_nodes_have_opcodes(ir));
    diags.extend(check_terminal_nodes_no_outgoing(ir));
    diags.extend(check_no_cycles(ir));
    diags.extend(check_all_nodes_reachable(ir));
    diags.extend(check_error_nodes_have_incoming(ir));
    diags.extend(check_render_nodes_have_templates(ir));
    
    diags
}

/// SEM001: Auth nodes must have predicates
fn check_auth_nodes_have_predicates(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    for node in &ir.nodes {
        if node.kind == NodeKind::Auth {
            // Auth nodes store predicate ID in op_code field
            let has_predicate = node.op_code > 0 || node.auth_predicate.is_some();
            
            if !has_predicate {
                diags.push(Diagnostic {
                    severity: Severity::Error,
                    code: "SEM001".into(),
                    message: format!("Auth node '{}' has no predicate", node.name),
                    hint: Some("Add <require predicate=\"...\"/> to auth node".into()),
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

/// SEM002: External nodes must have op codes
fn check_external_nodes_have_opcodes(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    for node in &ir.nodes {
        if node.kind == NodeKind::External && node.op_code == 0 {
            diags.push(Diagnostic {
                severity: Severity::Error,
                code: "SEM002".into(),
                message: format!("External node '{}' has no op code", node.name),
                hint: Some("Add op=\"0x0300\" or similar to external node".into()),
                location: Some(Location {
                    node_id: Some(node.name.clone()),
                    ..Default::default()
                }),
            });
        }
    }
    
    diags
}

/// SEM003: Terminal nodes shouldn't have outgoing edges
///
/// NOTE: This check is robust - it computes outgoing edges dynamically
/// instead of relying on cached `edge_count`, which might not be set yet.
fn check_terminal_nodes_no_outgoing(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    for node in &ir.nodes {
        if node.kind != NodeKind::Terminal {
            continue;
        }

        // Compute outgoing edges (don't trust cached edge_count)
        let outgoing_edges: Vec<_> = ir.edges.iter()
            .filter(|e| e.from == node.id)
            .collect();

        if !outgoing_edges.is_empty() {
            diags.push(Diagnostic {
                severity: Severity::Warn,
                code: "SEM003".into(),
                message: format!(
                    "Terminal node '{}' has {} outgoing edge(s)",
                    node.name, outgoing_edges.len()
                ),
                hint: Some("Terminal nodes end traversal, edges will never be taken".into()),
                location: Some(Location {
                    node_id: Some(node.name.clone()),
                    ..Default::default()
                }),
            });
        }
    }

    diags
}

/// SEM004: No cycles in the graph (must be DAG)
fn check_no_cycles(ir: &GraphIR) -> Vec<Diagnostic> {
    if let Some(cycle) = detect_cycle(ir) {
        vec![Diagnostic {
            severity: Severity::Error,
            code: "SEM004".into(),
            message: format!("Cycle detected: {}", cycle.join(" → ")),
            hint: Some("Workflows must be directed acyclic graphs (DAG)".into()),
            location: None,
        }]
    } else {
        vec![]
    }
}

/// Detect cycle using DFS, returns path if found
fn detect_cycle(ir: &GraphIR) -> Option<Vec<String>> {
    // Build adjacency list
    let mut adj: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    for edge in &ir.edges {
        adj.entry(edge.from).or_default().push(edge.target);
    }
    
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut path = Vec::new();
    
    for node in &ir.nodes {
        if !visited.contains(&node.id) {
            if let Some(cycle) = dfs_cycle(
                node.id,
                ir,
                &adj,
                &mut visited,
                &mut rec_stack,
                &mut path,
            ) {
                return Some(cycle);
            }
        }
    }
    
    None
}

fn dfs_cycle(
    node_id: NodeId,
    ir: &GraphIR,
    adj: &HashMap<NodeId, Vec<NodeId>>,
    visited: &mut HashSet<NodeId>,
    rec_stack: &mut HashSet<NodeId>,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    visited.insert(node_id);
    rec_stack.insert(node_id);
    
    let name = ir.nodes
        .iter()
        .find(|n| n.id == node_id)
        .map(|n| n.name.clone())
        .unwrap_or_else(|| node_id.to_string());
    path.push(name);
    
    if let Some(neighbors) = adj.get(&node_id) {
        for &next in neighbors {
            if !visited.contains(&next) {
                if let Some(cycle) = dfs_cycle(next, ir, adj, visited, rec_stack, path) {
                    return Some(cycle);
                }
            } else if rec_stack.contains(&next) {
                // Found cycle - add the repeated node to complete the cycle
                let next_name = ir.nodes
                    .iter()
                    .find(|n| n.id == next)
                    .map(|n| n.name.clone())
                    .unwrap_or_else(|| next.to_string());
                path.push(next_name);
                return Some(path.clone());
            }
        }
    }
    
    path.pop();
    rec_stack.remove(&node_id);
    None
}

/// SEM005: All nodes should be reachable from an entry point
fn check_all_nodes_reachable(ir: &GraphIR) -> Vec<Diagnostic> {
    let reachable = compute_reachable(ir);
    let mut diags = Vec::new();
    
    for node in &ir.nodes {
        if !reachable.contains(&node.id) {
            diags.push(Diagnostic {
                severity: Severity::Warn,
                code: "SEM005".into(),
                message: format!("Node '{}' is unreachable from any entry point", node.name),
                hint: Some("Remove dead node or add edge to it".into()),
                location: Some(Location {
                    node_id: Some(node.name.clone()),
                    ..Default::default()
                }),
            });
        }
    }
    
    diags
}

fn compute_reachable(ir: &GraphIR) -> HashSet<NodeId> {
    let mut reachable = HashSet::new();
    let mut queue = VecDeque::new();
    
    for entry in &ir.entries {
        queue.push_back(entry.node_id);
    }
    
    while let Some(node_id) = queue.pop_front() {
        if reachable.insert(node_id) {
            for edge in &ir.edges {
                if edge.from == node_id && !reachable.contains(&edge.target) {
                    queue.push_back(edge.target);
                }
            }
        }
    }
    
    reachable
}

/// SEM006: Error nodes should have incoming edges
fn check_error_nodes_have_incoming(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    // Build set of nodes that have incoming edges
    let has_incoming: HashSet<NodeId> = ir.edges.iter().map(|e| e.target).collect();
    
    for node in &ir.nodes {
        if node.kind == NodeKind::Error && !has_incoming.contains(&node.id) {
            // Also check if it's an entry point
            let is_entry = ir.entries.iter().any(|e| e.node_id == node.id);
            
            if !is_entry {
                diags.push(Diagnostic {
                    severity: Severity::Warn,
                    code: "SEM006".into(),
                    message: format!("Error node '{}' has no incoming edges", node.name),
                    hint: Some("Add error handling edges pointing to this node".into()),
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

/// SEM007: Render nodes should have templates
fn check_render_nodes_have_templates(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    for node in &ir.nodes {
        if node.kind == NodeKind::Render && node.template.is_none() {
            diags.push(Diagnostic {
                severity: Severity::Warn,
                code: "SEM007".into(),
                message: format!("Render node '{}' has no template", node.name),
                hint: Some("Add <template ref=\"...\"/> or inline template".into()),
                location: Some(Location {
                    node_id: Some(node.name.clone()),
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
        
        let errors: Vec<_> = diags.iter().filter(|d| d.severity == Severity::Error).collect();
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }
    
    #[test]
    fn test_sem001_auth_without_predicate() {
        let mut ir = make_valid_ir();
        ir.nodes.push(GNode::new(2, "auth".into(), NodeKind::Auth));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SEM001"));
    }
    
    #[test]
    fn test_sem001_auth_with_predicate_passes() {
        let mut ir = make_valid_ir();
        let mut auth_node = GNode::new(2, "auth".into(), NodeKind::Auth);
        auth_node.op_code = 1; // Predicate ID stored here
        ir.nodes.push(auth_node);
        
        let diags = check(&ir);
        
        assert!(!diags.iter().any(|d| d.code == "SEM001"));
    }
    
    #[test]
    fn test_sem002_external_without_opcode() {
        let mut ir = make_valid_ir();
        ir.nodes.push(GNode::new(2, "external".into(), NodeKind::External));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SEM002"));
    }
    
    #[test]
    fn test_sem002_external_with_opcode_passes() {
        let mut ir = make_valid_ir();
        let mut ext_node = GNode::new(2, "external".into(), NodeKind::External);
        ext_node.op_code = 0x0300;
        ir.nodes.push(ext_node);
        
        let diags = check(&ir);
        
        assert!(!diags.iter().any(|d| d.code == "SEM002"));
    }
    
    #[test]
    fn test_sem003_terminal_with_outgoing() {
        let mut ir = make_valid_ir();

        // Create an actual edge from the terminal node (node 1)
        // This simulates a workflow error where terminal has outgoing edges
        ir.edges.push(GEdge::new(99, 1, 0)); // Terminal (1) -> Start (0)

        let diags = check(&ir);

        assert!(diags.iter().any(|d| d.code == "SEM003"),
            "Terminal node with outgoing edge should trigger SEM003");
    }
    
    #[test]
    fn test_sem004_cycle_detected() {
        let mut ir = GraphIR::new();
        
        ir.nodes.push(GNode::new(0, "a".into(), NodeKind::Transform));
        ir.nodes.push(GNode::new(1, "b".into(), NodeKind::Transform));
        ir.nodes.push(GNode::new(2, "c".into(), NodeKind::Transform));
        
        // Create cycle: a -> b -> c -> a
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.edges.push(GEdge::new(1, 1, 2));
        ir.edges.push(GEdge::new(2, 2, 0)); // Back edge creates cycle
        
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SEM004"));
    }
    
    #[test]
    fn test_sem005_unreachable_node() {
        let mut ir = make_valid_ir();
        ir.nodes.push(GNode::new(2, "orphan".into(), NodeKind::Transform));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SEM005"));
    }
    
    #[test]
    fn test_sem006_error_node_no_incoming() {
        let mut ir = make_valid_ir();
        ir.nodes.push(GNode::new(2, "error".into(), NodeKind::Error));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SEM006"));
    }
    
    #[test]
    fn test_sem007_render_without_template() {
        let mut ir = make_valid_ir();
        ir.nodes.push(GNode::new(2, "render".into(), NodeKind::Render));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "SEM007"));
    }
}