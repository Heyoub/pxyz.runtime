//! Graph IR Types
//!
//! Intermediate representation for compiled workflows.
//! This is what the compiler operates on between parsing and emission.

use crate::{
    NodeKind, ActorKind, SideEffects, ConfirmationStatus,
    node_flags, edge_flags, is_llm_op, is_irreversible_op, is_write_op,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type NodeId = u32;
pub type EdgeId = u32;
pub type PredicateId = u16;

/// Graph intermediate representation
#[derive(Debug, Clone, Default)]
pub struct GraphIR {
    pub nodes: Vec<GNode>,
    pub edges: Vec<GEdge>,
    pub predicates: Vec<CompiledPredicate>,
    pub strings: StringPool,
    pub entries: Vec<GEntry>,
    pub workflow_ids: Vec<String>,
}

impl GraphIR {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn get_node(&self, id: NodeId) -> Option<&GNode> {
        self.nodes.iter().find(|n| n.id == id)
    }
    
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut GNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }
    
    pub fn get_node_by_name(&self, name: &str) -> Option<&GNode> {
        self.nodes.iter().find(|n| n.name == name)
    }
    
    pub fn get_edges_from(&self, node_id: NodeId) -> Vec<&GEdge> {
        self.edges.iter().filter(|e| e.from == node_id).collect()
    }
    
    pub fn get_edges_to(&self, node_id: NodeId) -> Vec<&GEdge> {
        self.edges.iter().filter(|e| e.target == node_id).collect()
    }
    
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    
    pub fn predicate_count(&self) -> usize {
        self.predicates.len()
    }
    
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Assign edge indices to nodes (IR invariant)
    ///
    /// MUST be called after edges are added and before validation.
    /// Sets `edge_start` and `edge_count` on each node.
    pub fn assign_edge_indices(&mut self) {
        // Sort edges by source node for efficient lookup
        self.edges.sort_by_key(|e| (e.from, e.id));

        // Collect edge assignments first (to avoid borrow conflict)
        let mut edge_assignments: Vec<(NodeId, u16, u16)> = Vec::new();
        let mut current_node: Option<NodeId> = None;
        let mut current_start: u16 = 0;
        let mut current_count: u16 = 0;

        for (i, edge) in self.edges.iter().enumerate() {
            if Some(edge.from) != current_node {
                // Record previous node's edges
                if let Some(node_id) = current_node {
                    edge_assignments.push((node_id, current_start, current_count));
                }

                // Start new node
                current_node = Some(edge.from);
                current_start = i as u16;
                current_count = 1;
            } else {
                current_count += 1;
            }
        }

        // Record last node
        if let Some(node_id) = current_node {
            edge_assignments.push((node_id, current_start, current_count));
        }

        // Apply edge assignments
        for (node_id, start, count) in edge_assignments {
            if let Some(node) = self.get_node_mut(node_id) {
                node.edge_start = start;
                node.edge_count = count;
            }
        }
    }

    /// Validate IR invariants (debug builds only)
    ///
    /// Call this after lowering to catch structural bugs early.
    #[cfg(debug_assertions)]
    pub fn assert_invariants(&self) {
        // All edge targets must exist
        for edge in &self.edges {
            assert!(
                self.nodes.iter().any(|n| n.id == edge.from),
                "Edge {} references non-existent source node {}",
                edge.id, edge.from
            );
            assert!(
                self.nodes.iter().any(|n| n.id == edge.target),
                "Edge {} references non-existent target node {}",
                edge.id, edge.target
            );
        }

        // All entry points must reference existing nodes
        for entry in &self.entries {
            assert!(
                self.nodes.iter().any(|n| n.id == entry.node_id),
                "Entry ({}, {}) references non-existent node {}",
                entry.p, entry.x, entry.node_id
            );
        }

        // No duplicate node IDs
        let mut seen = std::collections::HashSet::new();
        for node in &self.nodes {
            assert!(
                seen.insert(node.id),
                "Duplicate node ID: {}",
                node.id
            );
        }
    }
}

/// Graph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNode {
    pub id: NodeId,
    pub name: String,
    pub kind: NodeKind,
    pub op_code: u16,
    pub data_offset: u32,
    pub edge_start: u16,
    pub edge_count: u16,
    pub flags: u8,
    
    // Metadata for validation (not serialized to binary)
    #[serde(skip)]
    pub side_effects: SideEffects,
    #[serde(skip)]
    pub requires_auth: bool,
    #[serde(skip)]
    pub auth_predicate: Option<PredicateId>,
    #[serde(skip)]
    pub actor_kind: ActorKind,
    #[serde(skip)]
    pub confirmation_status: ConfirmationStatus,
    #[serde(skip)]
    pub template: Option<String>,
    #[serde(skip)]
    pub selector: Option<String>,
    #[serde(skip)]
    pub status: Option<u16>,
    #[serde(skip)]
    pub message: Option<String>,
}

impl GNode {
    pub fn new(id: NodeId, name: String, kind: NodeKind) -> Self {
        Self {
            id,
            name,
            kind,
            op_code: 0,
            data_offset: 0,
            edge_start: 0,
            edge_count: 0,
            flags: 0,
            side_effects: SideEffects::None,
            requires_auth: false,
            auth_predicate: None,
            actor_kind: ActorKind::Agent,
            confirmation_status: ConfirmationStatus::Confirmed,
            template: None,
            selector: None,
            status: None,
            message: None,
        }
    }
    
    /// Check if this node calls an LLM
    pub fn is_llm(&self) -> bool {
        is_llm_op(self.op_code)
    }
    
    /// Check if this node performs an irreversible action
    pub fn is_irreversible(&self) -> bool {
        self.side_effects == SideEffects::Irreversible || is_irreversible_op(self.op_code)
    }
    
    /// Check if this node writes data
    pub fn is_write(&self) -> bool {
        self.side_effects == SideEffects::Write || 
        self.side_effects == SideEffects::Irreversible ||
        is_write_op(self.op_code)
    }
    
    /// Check if this node requires human interaction
    pub fn requires_human(&self) -> bool {
        self.actor_kind == ActorKind::Human || (self.flags & node_flags::REQUIRES_HUMAN) != 0
    }
    
    /// Check if this node has authentication requirements
    pub fn has_auth(&self) -> bool {
        self.requires_auth || (self.flags & node_flags::REQUIRES_AUTH) != 0
    }
    
    /// Set a flag
    pub fn set_flag(&mut self, flag: u8) {
        self.flags |= flag;
    }
    
    /// Check a flag
    pub fn has_flag(&self, flag: u8) -> bool {
        (self.flags & flag) != 0
    }
}

/// Graph edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GEdge {
    pub id: EdgeId,
    pub from: NodeId,
    pub target: NodeId,
    pub predicate_id: PredicateId,
    pub weight: u16,
    pub flags: u16,
}

impl GEdge {
    pub fn new(id: EdgeId, from: NodeId, target: NodeId) -> Self {
        Self {
            id,
            from,
            target,
            predicate_id: 0,
            weight: 0,
            flags: 0,
        }
    }
    
    /// Check if this edge can be traversed in parallel
    pub fn is_parallel(&self) -> bool {
        (self.flags & edge_flags::PARALLEL) != 0
    }
    
    /// Check if this is a fallback edge (taken on failure)
    pub fn is_fallback(&self) -> bool {
        (self.flags & edge_flags::FALLBACK) != 0
    }
    
    /// Check if this edge leads to an error handler
    pub fn is_error_edge(&self) -> bool {
        (self.flags & edge_flags::ERROR_EDGE) != 0
    }
    
    /// Set a flag
    pub fn set_flag(&mut self, flag: u16) {
        self.flags |= flag;
    }
    
    /// Check if predicate is "always true" (id 0)
    pub fn is_unconditional(&self) -> bool {
        self.predicate_id == 0
    }
}

/// Entry point mapping (P, X) → node
#[derive(Debug, Clone)]
pub struct GEntry {
    pub px_hash: u32,
    pub node_id: NodeId,
    pub p: String,
    pub x: String,
}

impl GEntry {
    pub fn new(p: String, x: String, node_id: NodeId) -> Self {
        Self {
            px_hash: crate::hash_px(&p, &x),
            node_id,
            p,
            x,
        }
    }
}

/// Compiled predicate with bytecode
#[derive(Debug, Clone)]
pub struct CompiledPredicate {
    pub id: PredicateId,
    pub name: String,
    pub bytecode: Vec<u8>,
}

impl CompiledPredicate {
    pub fn new(id: PredicateId, name: String) -> Self {
        Self {
            id,
            name,
            bytecode: Vec::new(),
        }
    }
    
    pub fn with_bytecode(id: PredicateId, name: String, bytecode: Vec<u8>) -> Self {
        Self { id, name, bytecode }
    }
    
    /// Check if bytecode has been compiled
    pub fn is_compiled(&self) -> bool {
        !self.bytecode.is_empty()
    }
    
    /// Get bytecode length
    pub fn bytecode_len(&self) -> usize {
        self.bytecode.len()
    }
}

/// String pool for binary emission
/// 
/// Strings are interned with null terminators for C-style access from WASM.
#[derive(Debug, Clone, Default)]
pub struct StringPool {
    pub data: Vec<u8>,
    pub offsets: HashMap<String, u32>,
}

impl StringPool {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Intern a string, returning its offset in the pool
    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(&offset) = self.offsets.get(s) {
            return offset;
        }
        
        let offset = self.data.len() as u32;
        self.data.extend_from_slice(s.as_bytes());
        self.data.push(0); // null terminator
        self.offsets.insert(s.to_string(), offset);
        offset
    }
    
    /// Get string at offset (reads until null terminator)
    pub fn get(&self, offset: u32) -> Option<&str> {
        let start = offset as usize;
        if start >= self.data.len() {
            return None;
        }
        
        let end = self.data[start..]
            .iter()
            .position(|&b| b == 0)
            .map(|i| start + i)?;
        
        std::str::from_utf8(&self.data[start..end]).ok()
    }
    
    /// Get total size of string pool
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    /// Get number of unique strings
    pub fn count(&self) -> usize {
        self.offsets.len()
    }
    
    /// Iterate over all strings with their offsets
    pub fn iter(&self) -> impl Iterator<Item = (u32, &str)> {
        self.offsets.iter().map(|(s, &offset)| {
            (offset, s.as_str())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_pool_intern() {
        let mut pool = StringPool::new();
        
        let offset1 = pool.intern("hello");
        let offset2 = pool.intern("world");
        let offset3 = pool.intern("hello"); // duplicate
        
        assert_eq!(offset1, offset3); // same string, same offset
        assert_ne!(offset1, offset2);
        assert_eq!(pool.get(offset1), Some("hello"));
        assert_eq!(pool.get(offset2), Some("world"));
    }
    
    #[test]
    fn test_node_flags() {
        let mut node = GNode::new(0, "test".into(), NodeKind::External);
        
        assert!(!node.has_flag(node_flags::IRREVERSIBLE));
        node.set_flag(node_flags::IRREVERSIBLE);
        assert!(node.has_flag(node_flags::IRREVERSIBLE));
    }
    
    #[test]
    fn test_edge_flags() {
        let mut edge = GEdge::new(0, 0, 1);
        
        assert!(!edge.is_error_edge());
        edge.set_flag(edge_flags::ERROR_EDGE);
        assert!(edge.is_error_edge());
    }
    
    #[test]
    fn test_gentry_hash() {
        let entry = GEntry::new("contact".into(), "search".into(), 0);
        assert_eq!(entry.px_hash, crate::hash_px("contact", "search"));
    }
}