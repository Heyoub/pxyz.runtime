//! AST to IR Lowering
//!
//! Transforms the parsed XML AST into the graph IR used for
//! validation and binary emission.

use crate::{
    NodeKind, ActorKind, ConfirmationStatus, SideEffects, CompileError,
    node_flags, edge_flags, hash_px,
    is_irreversible_op, is_write_op,
};
use crate::dsl::ast::*;
use crate::compiler::ir::*;
use std::collections::HashMap;

/// Lower AST to IR
pub fn lower(doc: &OmarDocument) -> Result<GraphIR, CompileError> {
    let mut lowerer = Lowerer::new();
    lowerer.lower(doc)
}

struct Lowerer {
    ir: GraphIR,
    node_id_counter: NodeId,
    edge_id_counter: EdgeId,
    predicate_id_counter: PredicateId,
    
    // Maps for resolving references
    // (workflow_id, node_name) -> global node id
    node_map: HashMap<(String, String), NodeId>,
    // predicate_name -> predicate id
    predicate_map: HashMap<String, PredicateId>,
}

impl Lowerer {
    fn new() -> Self {
        Self {
            ir: GraphIR::new(),
            node_id_counter: 0,
            edge_id_counter: 0,
            predicate_id_counter: 1, // 0 is reserved for "always true"
            node_map: HashMap::new(),
            predicate_map: HashMap::new(),
        }
    }
    
    fn lower(mut self, doc: &OmarDocument) -> Result<GraphIR, CompileError> {
        // First pass: Register all named predicates
        self.register_predicates(&doc.predicates)?;
        
        // Second pass: Process all workflows
        for workflow in &doc.workflows {
            self.lower_workflow(workflow)?;
        }
        
        // Assign edge indices to nodes
        self.assign_edge_indices();
        
        Ok(self.ir)
    }
    
    fn register_predicates(&mut self, predicates: &[PredicateDef]) -> Result<(), CompileError> {
        for pred_def in predicates {
            if self.predicate_map.contains_key(&pred_def.id) {
                return Err(CompileError::Lower(format!(
                    "Duplicate predicate definition: {}",
                    pred_def.id
                )));
            }
            
            self.predicate_map.insert(pred_def.id.clone(), self.predicate_id_counter);
            self.ir.predicates.push(CompiledPredicate::new(
                self.predicate_id_counter,
                pred_def.id.clone(),
            ));
            self.predicate_id_counter += 1;
        }
        Ok(())
    }
    
    fn lower_workflow(&mut self, workflow: &Workflow) -> Result<(), CompileError> {
        self.ir.workflow_ids.push(workflow.id.clone());
        
        // Create nodes
        for node in &workflow.nodes {
            self.lower_node(workflow, node)?;
        }
        
        // Create edges
        for edge in &workflow.edges {
            self.lower_edge(workflow, edge)?;
        }
        
        // Create entry point
        self.lower_entry(workflow)?;
        
        Ok(())
    }
    
    fn lower_node(&mut self, workflow: &Workflow, node: &Node) -> Result<(), CompileError> {
        let kind: NodeKind = node.kind.parse()
            .map_err(|e: String| CompileError::Lower(e))?;
        
        let mut gnode = GNode::new(self.node_id_counter, node.id.clone(), kind);
        
        // Parse op code for external nodes
        if let Some(ref op_str) = node.op {
            gnode.op_code = parse_op_code(op_str)?;
            
            // Determine side effects from op code
            if is_irreversible_op(gnode.op_code) {
                gnode.side_effects = SideEffects::Irreversible;
                gnode.set_flag(node_flags::IRREVERSIBLE);
                gnode.set_flag(node_flags::HAS_SIDE_EFFECTS);
            } else if is_write_op(gnode.op_code) {
                gnode.side_effects = SideEffects::Write;
                gnode.set_flag(node_flags::HAS_SIDE_EFFECTS);
            }
        }
        
        // Handle auth predicate
        if let Some(ref pred_name) = node.predicate {
            if let Some(&pred_id) = self.predicate_map.get(pred_name) {
                gnode.auth_predicate = Some(pred_id);
                gnode.requires_auth = true;
                gnode.set_flag(node_flags::REQUIRES_AUTH);
                
                // For auth nodes, store predicate ID in op_code field
                if kind == NodeKind::Auth {
                    gnode.op_code = pred_id as u16;
                }
            } else {
                return Err(CompileError::Lower(format!(
                    "Node '{}' references unknown predicate: {}",
                    node.id, pred_name
                )));
            }
        }
        
        // Handle actor kind
        if let Some(ref actor) = node.actor {
            gnode.actor_kind = actor.parse().unwrap_or(ActorKind::Agent);
            if gnode.actor_kind == ActorKind::Human {
                gnode.set_flag(node_flags::REQUIRES_HUMAN);
            }
        }
        
        // Handle confirmation status
        if let Some(ref conf) = node.confirmation {
            gnode.confirmation_status = conf.parse().unwrap_or(ConfirmationStatus::Confirmed);
        }
        
        // Store optional metadata
        gnode.template = node.template.clone();
        gnode.selector = node.selector.clone();
        gnode.status = node.status;
        gnode.message = node.message.clone();
        
        // Handle async flag
        if node.async_node {
            gnode.set_flag(node_flags::ASYNC);
        }
        
        // Handle cacheable flag
        if node.cacheable {
            gnode.set_flag(node_flags::CACHEABLE);
        }
        
        // Intern node name in string pool
        gnode.data_offset = self.ir.strings.intern(&node.id);
        
        // Register in node map
        self.node_map.insert(
            (workflow.id.clone(), node.id.clone()),
            self.node_id_counter,
        );
        
        self.ir.nodes.push(gnode);
        self.node_id_counter += 1;
        
        Ok(())
    }
    
    fn lower_edge(&mut self, workflow: &Workflow, edge: &Edge) -> Result<(), CompileError> {
        // Resolve source node
        let from_id = self.node_map
            .get(&(workflow.id.clone(), edge.from.clone()))
            .copied()
            .ok_or_else(|| CompileError::Lower(format!(
                "Edge references unknown source node: {} (workflow: {})",
                edge.from, workflow.id
            )))?;
        
        // Resolve target node
        let to_id = self.node_map
            .get(&(workflow.id.clone(), edge.to.clone()))
            .copied()
            .ok_or_else(|| CompileError::Lower(format!(
                "Edge references unknown target node: {} (workflow: {})",
                edge.to, workflow.id
            )))?;
        
        let mut gedge = GEdge::new(self.edge_id_counter, from_id, to_id);
        
        // Handle predicate
        gedge.predicate_id = self.resolve_edge_predicate(edge)?;
        
        // Handle flags
        if edge.parallel {
            gedge.set_flag(edge_flags::PARALLEL);
        }
        if edge.fallback {
            gedge.set_flag(edge_flags::FALLBACK);
        }
        
        // Check if target is error node
        if let Some(target_node) = self.ir.get_node(to_id) {
            if target_node.kind == NodeKind::Error {
                gedge.set_flag(edge_flags::ERROR_EDGE);
            }
        }
        
        // Set weight
        gedge.weight = edge.weight.unwrap_or(0);
        
        self.ir.edges.push(gedge);
        self.edge_id_counter += 1;
        
        Ok(())
    }
    
    fn resolve_edge_predicate(&mut self, edge: &Edge) -> Result<PredicateId, CompileError> {
        // Check for named predicate reference
        if let Some(ref pred_ref) = edge.predicate_ref {
            return self.predicate_map
                .get(pred_ref)
                .copied()
                .ok_or_else(|| CompileError::Lower(format!(
                    "Edge references unknown predicate: {}",
                    pred_ref
                )));
        }
        
        // Check for inline predicate
        if let Some(ref pred_expr) = edge.predicate {
            return self.lower_inline_predicate(pred_expr);
        }
        
        // No predicate = always true (id 0)
        Ok(0)
    }
    
    fn lower_inline_predicate(&mut self, expr: &PredicateExpr) -> Result<PredicateId, CompileError> {
        match expr {
            PredicateExpr::Always => Ok(0), // Special case: always true
            PredicateExpr::Fail => {
                // Create a "fail" predicate
                let id = self.predicate_id_counter;
                self.ir.predicates.push(CompiledPredicate::new(
                    id,
                    format!("_fail_{}", id),
                ));
                self.predicate_id_counter += 1;
                Ok(id)
            }
            _ => {
                // Create inline predicate (bytecode compiled later)
                let id = self.predicate_id_counter;
                self.ir.predicates.push(CompiledPredicate::new(
                    id,
                    format!("_inline_{}", id),
                ));
                self.predicate_id_counter += 1;
                Ok(id)
            }
        }
    }
    
    fn lower_entry(&mut self, workflow: &Workflow) -> Result<(), CompileError> {
        let entry_node_id = self.node_map
            .get(&(workflow.id.clone(), workflow.entry.node.clone()))
            .copied()
            .ok_or_else(|| CompileError::Lower(format!(
                "Entry references unknown node: {} (workflow: {})",
                workflow.entry.node, workflow.id
            )))?;
        
        self.ir.entries.push(GEntry::new(
            workflow.entry.p.clone(),
            workflow.entry.x.clone(),
            entry_node_id,
        ));
        
        Ok(())
    }
    
    fn assign_edge_indices(&mut self) {
        // Sort edges by source node for efficient lookup
        self.ir.edges.sort_by_key(|e| (e.from, e.id));
        
        // Collect edge assignments first (to avoid borrow conflict)
        let mut edge_assignments: Vec<(NodeId, u16, u16)> = Vec::new();
        let mut current_node: Option<NodeId> = None;
        let mut current_start: u16 = 0;
        let mut current_count: u16 = 0;

        for (i, edge) in self.ir.edges.iter().enumerate() {
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
            if let Some(node) = self.ir.get_node_mut(node_id) {
                node.edge_start = start;
                node.edge_count = count;
            }
        }
    }
}

/// Parse op code from string (supports "0x0300" or "768")
fn parse_op_code(s: &str) -> Result<u16, CompileError> {
    let s = s.trim();
    
    if s.starts_with("0x") || s.starts_with("0X") {
        u16::from_str_radix(&s[2..], 16)
            .map_err(|_| CompileError::Lower(format!("Invalid hex op code: {}", s)))
    } else {
        s.parse()
            .map_err(|_| CompileError::Lower(format!("Invalid op code: {}", s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_minimal_doc() -> OmarDocument {
        OmarDocument {
            version: "1.0".into(),
            predicates: vec![],
            workflows: vec![Workflow {
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
        }
    }
    
    #[test]
    fn test_lower_minimal() {
        let doc = make_minimal_doc();
        let ir = lower(&doc).unwrap();
        
        assert_eq!(ir.nodes.len(), 2);
        assert_eq!(ir.edges.len(), 1);
        assert_eq!(ir.entries.len(), 1);
    }
    
    #[test]
    fn test_lower_preserves_node_kinds() {
        let doc = make_minimal_doc();
        let ir = lower(&doc).unwrap();
        
        assert_eq!(ir.nodes[0].kind, NodeKind::Transform);
        assert_eq!(ir.nodes[1].kind, NodeKind::Terminal);
    }
    
    #[test]
    fn test_lower_entry_hash() {
        let doc = make_minimal_doc();
        let ir = lower(&doc).unwrap();
        
        let expected_hash = hash_px("test", "run");
        assert_eq!(ir.entries[0].px_hash, expected_hash);
    }
    
    #[test]
    fn test_parse_op_code_hex() {
        assert_eq!(parse_op_code("0x0300").unwrap(), 0x0300);
        assert_eq!(parse_op_code("0X0300").unwrap(), 0x0300);
    }
    
    #[test]
    fn test_parse_op_code_decimal() {
        assert_eq!(parse_op_code("768").unwrap(), 768);
    }
    
    #[test]
    fn test_lower_unknown_node_error() {
        let mut doc = make_minimal_doc();
        doc.workflows[0].edges[0].to = "nonexistent".into();
        
        let result = lower(&doc);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_lower_unknown_predicate_error() {
        let mut doc = make_minimal_doc();
        doc.workflows[0].nodes[0].predicate = Some("unknown_pred".into());
        
        let result = lower(&doc);
        assert!(result.is_err());
    }
}