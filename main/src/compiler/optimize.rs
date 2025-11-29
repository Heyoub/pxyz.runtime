//! Graph Optimization Passes
//!
//! Optimizations run after lowering, before emission:
//! - Dead code elimination (unreachable nodes)
//! - Predicate deduplication (merge identical bytecode)
//! - Edge ordering (by weight for faster matching)

use crate::compiler::ir::*;
use std::collections::{HashSet, VecDeque};

/// Run all optimization passes
pub fn optimize(ir: GraphIR) -> GraphIR {
    let ir = remove_unreachable_nodes(ir);
    let ir = deduplicate_predicates(ir);
    let ir = optimize_edge_order(ir);
    ir
}

/// Remove nodes not reachable from any entry point
pub fn remove_unreachable_nodes(mut ir: GraphIR) -> GraphIR {
    let reachable = compute_reachable(&ir);
    
    if reachable.len() == ir.nodes.len() {
        // All nodes reachable, nothing to do
        return ir;
    }
    
    // Build old_id -> new_id mapping
    let mut id_remap: std::collections::HashMap<NodeId, NodeId> = std::collections::HashMap::new();
    let mut new_id: NodeId = 0;
    
    for node in &ir.nodes {
        if reachable.contains(&node.id) {
            id_remap.insert(node.id, new_id);
            new_id += 1;
        }
    }
    
    // Filter and renumber nodes
    ir.nodes.retain(|n| reachable.contains(&n.id));
    for node in &mut ir.nodes {
        node.id = id_remap[&node.id];
    }
    
    // Filter and renumber edges
    ir.edges.retain(|e| reachable.contains(&e.from) && reachable.contains(&e.target));
    for edge in &mut ir.edges {
        edge.from = id_remap[&edge.from];
        edge.target = id_remap[&edge.target];
    }
    
    // Update entry points
    for entry in &mut ir.entries {
        entry.node_id = id_remap[&entry.node_id];
    }
    
    // Reassign edge indices
    reassign_edge_indices(&mut ir);
    
    ir
}

/// Compute set of reachable node IDs from entry points
fn compute_reachable(ir: &GraphIR) -> HashSet<NodeId> {
    let mut reachable = HashSet::new();
    let mut queue = VecDeque::new();
    
    // Start from all entry points
    for entry in &ir.entries {
        queue.push_back(entry.node_id);
    }
    
    // BFS
    while let Some(node_id) = queue.pop_front() {
        if reachable.insert(node_id) {
            // Add targets of outgoing edges
            for edge in &ir.edges {
                if edge.from == node_id && !reachable.contains(&edge.target) {
                    queue.push_back(edge.target);
                }
            }
        }
    }
    
    reachable
}

/// Merge predicates with identical bytecode
pub fn deduplicate_predicates(mut ir: GraphIR) -> GraphIR {
    use std::collections::HashMap;
    
    // Map bytecode -> canonical predicate ID
    let mut bytecode_to_id: HashMap<Vec<u8>, PredicateId> = HashMap::new();
    // Map old ID -> new ID (for duplicates)
    let mut id_remap: HashMap<PredicateId, PredicateId> = HashMap::new();
    
    // Find duplicates
    for pred in &ir.predicates {
        // Skip empty bytecode (not yet compiled)
        if pred.bytecode.is_empty() {
            continue;
        }
        
        if let Some(&canonical_id) = bytecode_to_id.get(&pred.bytecode) {
            // This is a duplicate
            id_remap.insert(pred.id, canonical_id);
        } else {
            // This is canonical
            bytecode_to_id.insert(pred.bytecode.clone(), pred.id);
        }
    }
    
    if id_remap.is_empty() {
        // No duplicates found
        return ir;
    }
    
    // Remap edge predicate IDs
    for edge in &mut ir.edges {
        if let Some(&new_id) = id_remap.get(&edge.predicate_id) {
            edge.predicate_id = new_id;
        }
    }
    
    // Remap node auth predicates
    for node in &mut ir.nodes {
        if let Some(pred_id) = node.auth_predicate {
            if let Some(&new_id) = id_remap.get(&pred_id) {
                node.auth_predicate = Some(new_id);
            }
        }
    }
    
    // Remove duplicate predicates
    let keep_ids: HashSet<PredicateId> = bytecode_to_id.values().copied().collect();
    ir.predicates.retain(|p| {
        // Keep if canonical or if not yet compiled
        keep_ids.contains(&p.id) || p.bytecode.is_empty()
    });
    
    ir
}

/// Sort edges by weight (higher weight = higher priority = first)
pub fn optimize_edge_order(mut ir: GraphIR) -> GraphIR {
    // Sort by (from_node, -weight, id)
    // Higher weight edges come first within a node
    ir.edges.sort_by(|a, b| {
        a.from
            .cmp(&b.from)
            .then(b.weight.cmp(&a.weight)) // Note: reversed for descending
            .then(a.id.cmp(&b.id))
    });
    
    // Reassign edge indices
    reassign_edge_indices(&mut ir);
    
    ir
}

/// Reassign edge_start/edge_count for all nodes after edge reordering
fn reassign_edge_indices(ir: &mut GraphIR) {
    // Reset all nodes
    for node in &mut ir.nodes {
        node.edge_start = 0;
        node.edge_count = 0;
    }
    
    // Collect edge assignments first (to avoid borrow conflict)
    let mut edge_assignments: Vec<(NodeId, u16, u16)> = Vec::new();
    let mut current_node: Option<NodeId> = None;
    let mut current_start: u16 = 0;
    let mut current_count: u16 = 0;

    for (i, edge) in ir.edges.iter().enumerate() {
        if Some(edge.from) != current_node {
            if let Some(node_id) = current_node {
                edge_assignments.push((node_id, current_start, current_count));
            }
            current_node = Some(edge.from);
            current_start = i as u16;
            current_count = 1;
        } else {
            current_count += 1;
        }
    }

    if let Some(node_id) = current_node {
        edge_assignments.push((node_id, current_start, current_count));
    }

    // Apply edge assignments
    for (node_id, start, count) in edge_assignments {
        if let Some(node) = ir.get_node_mut(node_id) {
            node.edge_start = start;
            node.edge_count = count;
        }
    }
}

/// Compute optimization statistics
pub fn compute_stats(before: &GraphIR, after: &GraphIR) -> OptimizationStats {
    OptimizationStats {
        nodes_before: before.nodes.len(),
        nodes_after: after.nodes.len(),
        nodes_removed: before.nodes.len().saturating_sub(after.nodes.len()),
        edges_before: before.edges.len(),
        edges_after: after.edges.len(),
        edges_removed: before.edges.len().saturating_sub(after.edges.len()),
        predicates_before: before.predicates.len(),
        predicates_after: after.predicates.len(),
        predicates_deduplicated: before.predicates.len().saturating_sub(after.predicates.len()),
    }
}

#[derive(Debug, Clone)]
pub struct OptimizationStats {
    pub nodes_before: usize,
    pub nodes_after: usize,
    pub nodes_removed: usize,
    pub edges_before: usize,
    pub edges_after: usize,
    pub edges_removed: usize,
    pub predicates_before: usize,
    pub predicates_after: usize,
    pub predicates_deduplicated: usize,
}

impl OptimizationStats {
    pub fn has_changes(&self) -> bool {
        self.nodes_removed > 0 || self.edges_removed > 0 || self.predicates_deduplicated > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeKind;
    
    fn make_test_ir() -> GraphIR {
        let mut ir = GraphIR::new();
        
        // Node 0: entry
        ir.nodes.push(GNode::new(0, "start".into(), NodeKind::Transform));
        // Node 1: reachable
        ir.nodes.push(GNode::new(1, "middle".into(), NodeKind::Transform));
        // Node 2: terminal
        ir.nodes.push(GNode::new(2, "end".into(), NodeKind::Terminal));
        // Node 3: unreachable
        ir.nodes.push(GNode::new(3, "dead".into(), NodeKind::Transform));
        
        // Edges: 0 -> 1 -> 2
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.edges.push(GEdge::new(1, 1, 2));
        // Orphan edge from dead node
        ir.edges.push(GEdge::new(2, 3, 2));
        
        // Entry point at node 0
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        ir
    }
    
    #[test]
    fn test_remove_unreachable() {
        let ir = make_test_ir();
        assert_eq!(ir.nodes.len(), 4);
        
        let ir = remove_unreachable_nodes(ir);
        
        // Node 3 should be removed
        assert_eq!(ir.nodes.len(), 3);
        assert!(ir.nodes.iter().all(|n| n.name != "dead"));
    }
    
    #[test]
    fn test_deduplicate_predicates() {
        let mut ir = GraphIR::new();
        
        // Two predicates with identical bytecode
        ir.predicates.push(CompiledPredicate::with_bytecode(
            1,
            "pred1".into(),
            vec![0x01, 0x00, 0x00, 0x00, 0x01, 0xFF],
        ));
        ir.predicates.push(CompiledPredicate::with_bytecode(
            2,
            "pred2".into(),
            vec![0x01, 0x00, 0x00, 0x00, 0x01, 0xFF], // Same bytecode
        ));
        // One different predicate
        ir.predicates.push(CompiledPredicate::with_bytecode(
            3,
            "pred3".into(),
            vec![0x01, 0x00, 0x00, 0x00, 0x00, 0xFF], // Different
        ));
        
        // Edges using the predicates
        ir.edges.push(GEdge { predicate_id: 1, ..GEdge::new(0, 0, 1) });
        ir.edges.push(GEdge { predicate_id: 2, ..GEdge::new(1, 0, 1) });
        ir.edges.push(GEdge { predicate_id: 3, ..GEdge::new(2, 0, 1) });
        
        let ir = deduplicate_predicates(ir);
        
        // Should have 2 predicates (one duplicate removed)
        assert_eq!(ir.predicates.len(), 2);
        
        // Edge that used pred2 should now use pred1
        assert!(ir.edges.iter().filter(|e| e.predicate_id == 1).count() >= 1);
    }
    
    #[test]
    fn test_edge_ordering() {
        let mut ir = GraphIR::new();
        
        ir.nodes.push(GNode::new(0, "start".into(), NodeKind::Transform));
        ir.nodes.push(GNode::new(1, "a".into(), NodeKind::Transform));
        ir.nodes.push(GNode::new(2, "b".into(), NodeKind::Transform));
        
        // Edges from node 0 with different weights
        ir.edges.push(GEdge { weight: 10, ..GEdge::new(0, 0, 1) });
        ir.edges.push(GEdge { weight: 50, ..GEdge::new(1, 0, 2) }); // Higher weight
        
        let ir = optimize_edge_order(ir);
        
        // Higher weight edge should come first
        let first_edge = &ir.edges[0];
        assert_eq!(first_edge.weight, 50);
    }
    
    #[test]
    fn test_compute_stats() {
        let before = make_test_ir();
        let after = remove_unreachable_nodes(before.clone());
        
        let stats = compute_stats(&before, &after);
        
        assert_eq!(stats.nodes_before, 4);
        assert_eq!(stats.nodes_after, 3);
        assert_eq!(stats.nodes_removed, 1);
        assert!(stats.has_changes());
    }
    
    #[test]
    fn test_full_optimize() {
        let ir = make_test_ir();
        let ir = optimize(ir);
        
        // Should have removed unreachable node
        assert_eq!(ir.nodes.len(), 3);
        
        // Should have removed orphan edge
        assert_eq!(ir.edges.len(), 2);
    }
}