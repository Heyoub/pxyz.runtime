//! Pragmatic Constraints - Business Rule Validation
//!
//! These checks enforce safety and business policies:
//! - LLM outputs can't directly trigger irreversible actions
//! - Irreversible actions require human confirmation
//! - Quarantined data can't escape
//!
//! These are the "you probably don't want to do that" checks.

use crate::{Diagnostic, Location, Severity, NodeKind, ActorKind, ConfirmationStatus, SideEffects};
use crate::compiler::ir::*;
use std::collections::{HashSet, VecDeque, HashMap};

/// Run all pragmatic checks
pub fn check(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    diags.extend(check_llm_to_irreversible(ir));
    diags.extend(check_write_error_branches(ir));
    diags.extend(check_irreversible_human_path(ir));
    diags.extend(check_irreversible_confirmed(ir));
    diags.extend(check_quarantined_escape(ir));
    
    diags
}

/// PRAG001: LLM → Irreversible paths must have validation gate
///
/// If an LLM node can reach an irreversible action without passing through
/// an auth or transform node, that's dangerous - the LLM could trigger
/// email sends, webhooks, etc. without human oversight.
fn check_llm_to_irreversible(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    let llm_nodes: Vec<&GNode> = ir.nodes.iter()
        .filter(|n| n.is_llm())
        .collect();
    
    let irreversible_nodes: Vec<&GNode> = ir.nodes.iter()
        .filter(|n| n.is_irreversible())
        .collect();
    
    for llm_node in &llm_nodes {
        for irrev_node in &irreversible_nodes {
            if let Some(path) = find_path(ir, llm_node.id, irrev_node.id) {
                // Check if there's a gate (auth or transform) in between
                let has_gate = path.iter().skip(1).any(|&node_id| {
                    ir.nodes
                        .iter()
                        .find(|n| n.id == node_id)
                        .map(|n| {
                            n.kind == NodeKind::Auth || 
                            n.kind == NodeKind::Transform ||
                            n.actor_kind == ActorKind::Human
                        })
                        .unwrap_or(false)
                });
                
                if !has_gate {
                    let path_names: Vec<_> = path.iter()
                        .filter_map(|&id| ir.nodes.iter().find(|n| n.id == id).map(|n| n.name.as_str()))
                        .collect();
                    
                    diags.push(Diagnostic {
                        severity: Severity::Error,
                        code: "PRAG001".into(),
                        message: format!(
                            "LLM '{}' can reach irreversible '{}' without validation",
                            llm_node.name, irrev_node.name
                        ),
                        hint: Some(format!(
                            "Path: {}. Add auth/transform node before irreversible action.",
                            path_names.join(" → ")
                        )),
                        location: None,
                    });
                }
            }
        }
    }
    
    diags
}

/// PRAG002: Write operations should have error branches
///
/// If a write/irreversible operation fails, there should be a way to handle it.
fn check_write_error_branches(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    for node in &ir.nodes {
        if !node.is_write() {
            continue;
        }
        
        // Check if any outgoing edge handles errors
        let edges = ir.get_edges_from(node.id);
        let has_error_handling = edges.iter().any(|e| {
            e.is_fallback() || e.is_error_edge() ||
            ir.nodes
                .iter()
                .find(|n| n.id == e.target)
                .map(|n| n.kind == NodeKind::Error)
                .unwrap_or(false)
        });
        
        if !has_error_handling && !edges.is_empty() {
            diags.push(Diagnostic {
                severity: Severity::Warn,
                code: "PRAG002".into(),
                message: format!(
                    "Write node '{}' has no error handling",
                    node.name
                ),
                hint: Some("Add edge with <when><fail/></when> to error node".into()),
                location: Some(Location {
                    node_id: Some(node.name.clone()),
                    ..Default::default()
                }),
            });
        }
    }
    
    diags
}

/// PRAG003: Irreversible actions require human in path
///
/// Every path to an irreversible action must pass through a node
/// that requires human interaction (actor="human").
fn check_irreversible_human_path(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    for node in &ir.nodes {
        if !node.is_irreversible() {
            continue;
        }
        
        // Check all paths from entry points to this node
        let mut has_any_human_path = false;
        let mut all_paths_checked = false;
        
        for entry in &ir.entries {
            if let Some(path) = find_path(ir, entry.node_id, node.id) {
                all_paths_checked = true;
                
                let path_has_human = path.iter().any(|&nid| {
                    ir.nodes
                        .iter()
                        .find(|n| n.id == nid)
                        .map(|n| n.actor_kind == ActorKind::Human)
                        .unwrap_or(false)
                });
                
                if path_has_human {
                    has_any_human_path = true;
                    break;
                }
            }
        }
        
        // Only error if we found paths but none had humans
        if all_paths_checked && !has_any_human_path {
            diags.push(Diagnostic {
                severity: Severity::Error,
                code: "PRAG003".into(),
                message: format!(
                    "Irreversible node '{}' has no human in any path from entry",
                    node.name
                ),
                hint: Some("Add a node with actor=\"human\" before irreversible action".into()),
                location: Some(Location {
                    node_id: Some(node.name.clone()),
                    ..Default::default()
                }),
            });
        }
    }
    
    diags
}

/// PRAG004: Irreversible actions require confirmed inputs
///
/// If suggested (unconfirmed) data can reach an irreversible action
/// without confirmation, that's dangerous.
fn check_irreversible_confirmed(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    let suggested_nodes: Vec<&GNode> = ir.nodes.iter()
        .filter(|n| n.confirmation_status == ConfirmationStatus::Suggested)
        .collect();
    
    let irreversible_nodes: Vec<&GNode> = ir.nodes.iter()
        .filter(|n| n.is_irreversible())
        .collect();
    
    for suggested in &suggested_nodes {
        for irrev in &irreversible_nodes {
            if let Some(path) = find_path(ir, suggested.id, irrev.id) {
                // Check if there's a confirmation point in the path
                let has_confirmation = path.iter().skip(1).any(|&nid| {
                    ir.nodes
                        .iter()
                        .find(|n| n.id == nid)
                        .map(|n| {
                            n.actor_kind == ActorKind::Human ||
                            n.confirmation_status == ConfirmationStatus::Confirmed
                        })
                        .unwrap_or(false)
                });
                
                if !has_confirmation {
                    diags.push(Diagnostic {
                        severity: Severity::Error,
                        code: "PRAG004".into(),
                        message: format!(
                            "Suggested data from '{}' reaches irreversible '{}' without confirmation",
                            suggested.name, irrev.name
                        ),
                        hint: Some("Add human confirmation before irreversible action".into()),
                        location: None,
                    });
                }
            }
        }
    }
    
    diags
}

/// PRAG005: Quarantined data cannot escape to external operations
///
/// Data marked as quarantined (flagged for review) should not flow
/// directly to external operations that have side effects.
fn check_quarantined_escape(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    
    for node in &ir.nodes {
        if node.confirmation_status != ConfirmationStatus::Quarantined {
            continue;
        }
        
        // Check direct successors
        for edge in ir.get_edges_from(node.id) {
            if let Some(target) = ir.nodes.iter().find(|n| n.id == edge.target) {
                // Quarantined data can't go directly to external nodes with side effects
                if target.kind == NodeKind::External && target.side_effects != SideEffects::None {
                    diags.push(Diagnostic {
                        severity: Severity::Error,
                        code: "PRAG005".into(),
                        message: format!(
                            "Quarantined node '{}' flows directly to external '{}'",
                            node.name, target.name
                        ),
                        hint: Some("Quarantined data must pass through review before external use".into()),
                        location: None,
                    });
                }
                
                // Also can't go to irreversible actions
                if target.is_irreversible() {
                    diags.push(Diagnostic {
                        severity: Severity::Error,
                        code: "PRAG005".into(),
                        message: format!(
                            "Quarantined node '{}' flows to irreversible '{}'",
                            node.name, target.name
                        ),
                        hint: Some("Quarantined data must be reviewed before irreversible actions".into()),
                        location: None,
                    });
                }
            }
        }
    }
    
    diags
}

/// Find a path from source to target using BFS
/// Returns the path as a vector of node IDs, or None if no path exists
fn find_path(ir: &GraphIR, source: NodeId, target: NodeId) -> Option<Vec<NodeId>> {
    if source == target {
        return Some(vec![source]);
    }
    
    let mut visited: HashMap<NodeId, Option<NodeId>> = HashMap::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(source);
    visited.insert(source, None);
    
    while let Some(current) = queue.pop_front() {
        for edge in ir.edges.iter().filter(|e| e.from == current) {
            if visited.contains_key(&edge.target) {
                continue;
            }
            
            visited.insert(edge.target, Some(current));
            
            if edge.target == target {
                // Reconstruct path
                let mut path = vec![target];
                let mut node = target;
                while let Some(Some(prev)) = visited.get(&node) {
                    path.push(*prev);
                    node = *prev;
                }
                path.reverse();
                return Some(path);
            }
            
            queue.push_back(edge.target);
        }
    }
    
    None
}

/// Find all paths from source to target (for comprehensive checking)
#[allow(dead_code)]
fn find_all_paths(ir: &GraphIR, source: NodeId, target: NodeId) -> Vec<Vec<NodeId>> {
    let mut paths = Vec::new();
    let mut current_path = vec![source];
    let mut visited = HashSet::new();
    
    dfs_all_paths(ir, source, target, &mut current_path, &mut visited, &mut paths);
    
    paths
}

fn dfs_all_paths(
    ir: &GraphIR,
    current: NodeId,
    target: NodeId,
    path: &mut Vec<NodeId>,
    visited: &mut HashSet<NodeId>,
    paths: &mut Vec<Vec<NodeId>>,
) {
    if current == target {
        paths.push(path.clone());
        return;
    }
    
    visited.insert(current);
    
    for edge in ir.edges.iter().filter(|e| e.from == current) {
        if !visited.contains(&edge.target) {
            path.push(edge.target);
            dfs_all_paths(ir, edge.target, target, path, visited, paths);
            path.pop();
        }
    }
    
    visited.remove(&current);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_llm_op;
    
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
    fn test_prag001_llm_to_irreversible_direct() {
        let mut ir = GraphIR::new();
        
        // LLM node
        let mut llm = GNode::new(0, "llm".into(), NodeKind::External);
        llm.op_code = 0x0800; // LLM_COMPLETE
        assert!(llm.is_llm());
        ir.nodes.push(llm);
        
        // Irreversible node
        let mut irrev = GNode::new(1, "send_email".into(), NodeKind::External);
        irrev.op_code = 0x0340; // EMAIL_SEND
        irrev.side_effects = SideEffects::Irreversible;
        ir.nodes.push(irrev);
        
        // Direct edge LLM -> irreversible
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "PRAG001"));
    }
    
    #[test]
    fn test_prag001_llm_with_gate_passes() {
        let mut ir = GraphIR::new();
        
        // LLM node
        let mut llm = GNode::new(0, "llm".into(), NodeKind::External);
        llm.op_code = 0x0800;
        ir.nodes.push(llm);
        
        // Auth gate
        let mut auth = GNode::new(1, "confirm".into(), NodeKind::Auth);
        auth.op_code = 1; // Has predicate
        ir.nodes.push(auth);
        
        // Irreversible node
        let mut irrev = GNode::new(2, "send_email".into(), NodeKind::External);
        irrev.op_code = 0x0340;
        irrev.side_effects = SideEffects::Irreversible;
        ir.nodes.push(irrev);
        
        // LLM -> Auth -> irreversible
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.edges.push(GEdge::new(1, 1, 2));
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        let diags = check(&ir);
        
        assert!(!diags.iter().any(|d| d.code == "PRAG001"));
    }
    
    #[test]
    fn test_prag002_write_without_error_handling() {
        let mut ir = GraphIR::new();
        
        let mut write_node = GNode::new(0, "write".into(), NodeKind::External);
        write_node.op_code = 0x0100; // Entity create
        write_node.side_effects = SideEffects::Write;
        write_node.edge_start = 0;
        write_node.edge_count = 1;
        ir.nodes.push(write_node);
        
        ir.nodes.push(GNode::new(1, "next".into(), NodeKind::Transform));
        
        // Only success path, no error handling
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "PRAG002"));
    }
    
    #[test]
    fn test_prag003_irreversible_without_human() {
        let mut ir = GraphIR::new();
        
        // Start (agent)
        ir.nodes.push(GNode::new(0, "start".into(), NodeKind::Transform));
        
        // Irreversible (agent)
        let mut irrev = GNode::new(1, "send".into(), NodeKind::External);
        irrev.op_code = 0x0340;
        irrev.side_effects = SideEffects::Irreversible;
        ir.nodes.push(irrev);
        
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "PRAG003"));
    }
    
    #[test]
    fn test_prag003_with_human_passes() {
        let mut ir = GraphIR::new();
        
        // Human confirmation node
        let mut human = GNode::new(0, "confirm".into(), NodeKind::Auth);
        human.actor_kind = ActorKind::Human;
        human.op_code = 1;
        ir.nodes.push(human);
        
        // Irreversible
        let mut irrev = GNode::new(1, "send".into(), NodeKind::External);
        irrev.op_code = 0x0340;
        irrev.side_effects = SideEffects::Irreversible;
        ir.nodes.push(irrev);
        
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        let diags = check(&ir);
        
        assert!(!diags.iter().any(|d| d.code == "PRAG003"));
    }
    
    #[test]
    fn test_prag004_suggested_to_irreversible() {
        let mut ir = GraphIR::new();
        
        // Suggested data
        let mut suggested = GNode::new(0, "llm_output".into(), NodeKind::Transform);
        suggested.confirmation_status = ConfirmationStatus::Suggested;
        ir.nodes.push(suggested);
        
        // Irreversible
        let mut irrev = GNode::new(1, "send".into(), NodeKind::External);
        irrev.op_code = 0x0340;
        irrev.side_effects = SideEffects::Irreversible;
        ir.nodes.push(irrev);
        
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "PRAG004"));
    }
    
    #[test]
    fn test_prag005_quarantined_escape() {
        let mut ir = GraphIR::new();
        
        // Quarantined data
        let mut quarantined = GNode::new(0, "flagged".into(), NodeKind::Transform);
        quarantined.confirmation_status = ConfirmationStatus::Quarantined;
        ir.nodes.push(quarantined);
        
        // External with side effects
        let mut external = GNode::new(1, "api".into(), NodeKind::External);
        external.op_code = 0x0401; // HTTP POST
        external.side_effects = SideEffects::Write;
        ir.nodes.push(external);
        
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        let diags = check(&ir);
        
        assert!(diags.iter().any(|d| d.code == "PRAG005"));
    }
    
    #[test]
    fn test_find_path() {
        let mut ir = GraphIR::new();
        
        ir.nodes.push(GNode::new(0, "a".into(), NodeKind::Transform));
        ir.nodes.push(GNode::new(1, "b".into(), NodeKind::Transform));
        ir.nodes.push(GNode::new(2, "c".into(), NodeKind::Transform));
        
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.edges.push(GEdge::new(1, 1, 2));
        
        let path = find_path(&ir, 0, 2);
        assert_eq!(path, Some(vec![0, 1, 2]));
        
        let no_path = find_path(&ir, 2, 0);
        assert_eq!(no_path, None);
    }
}