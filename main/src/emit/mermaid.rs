//! Mermaid Diagram Generation

use crate::compiler::ir::*;
use crate::NodeKind;

pub fn format(ir: &GraphIR) -> String {
    let mut out = String::new();
    
    out.push_str("flowchart TD\n");
    
    // Entry points subgraph
    if !ir.entries.is_empty() {
        out.push_str("    subgraph Entries\n");
        for (i, entry) in ir.entries.iter().enumerate() {
            out.push_str(&format!("        entry{}([\"({}, {})\"])\n", i, entry.p, entry.x));
        }
        out.push_str("    end\n\n");
    }
    
    // Nodes with different shapes based on kind
    for node in &ir.nodes {
        let shape = match node.kind {
            NodeKind::Transform => format!("{}[{}]", node.name, node.name),
            NodeKind::External => format!("{}{{{{{}}}}} ", node.name, node.name),
            NodeKind::Render => format!("{}([{}])", node.name, node.name),
            NodeKind::Signal => format!("{}>{}]", node.name, node.name),
            NodeKind::Auth => format!("{}{{{{{}}}}} ", node.name, node.name),
            NodeKind::Terminal => format!("{}((({}))", node.name, node.name),
            NodeKind::Error => format!("{}[/{}\\]", node.name, node.name),
        };
        out.push_str(&format!("    {}\n", shape));
    }
    out.push('\n');
    
    // Entry connections
    for (i, entry) in ir.entries.iter().enumerate() {
        if let Some(node) = ir.nodes.iter().find(|n| n.id == entry.node_id) {
            out.push_str(&format!("    entry{} --> {}\n", i, node.name));
        }
    }
    
    // Edges with different arrow styles
    for edge in &ir.edges {
        let from = ir.nodes.iter().find(|n| n.id == edge.from).map(|n| &n.name);
        let to = ir.nodes.iter().find(|n| n.id == edge.target).map(|n| &n.name);
        
        if let (Some(f), Some(t)) = (from, to) {
            let arrow = if edge.is_fallback() {
                "-.->"
            } else if edge.is_error_edge() {
                "==>"
            } else {
                "-->"
            };
            
            let label = if edge.predicate_id > 0 {
                ir.predicates.iter()
                    .find(|p| p.id == edge.predicate_id)
                    .map(|p| format!("|{}|", p.name))
                    .unwrap_or_default()
            } else {
                String::new()
            };
            
            out.push_str(&format!("    {} {}{} {}\n", f, arrow, label, t));
        }
    }
    out.push('\n');
    
    // Styling based on node kind
    out.push_str("    %% Styling\n");
    for node in &ir.nodes {
        let color = match node.kind {
            NodeKind::Transform => "#e3f2fd",
            NodeKind::External => "#fff3e0",
            NodeKind::Render => "#e8f5e9",
            NodeKind::Signal => "#fce4ec",
            NodeKind::Auth => "#f3e5f5",
            NodeKind::Terminal => "#e0e0e0",
            NodeKind::Error => "#ffebee",
        };
        out.push_str(&format!("    style {} fill:{}\n", node.name, color));
    }
    
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_minimal() {
        let mut ir = GraphIR::new();
        ir.nodes.push(GNode::new(0, "start".into(), NodeKind::Transform));
        ir.nodes.push(GNode::new(1, "end".into(), NodeKind::Terminal));
        ir.edges.push(GEdge::new(0, 0, 1));
        ir.entries.push(GEntry::new("test".into(), "run".into(), 0));
        
        let mermaid = format(&ir);
        
        assert!(mermaid.contains("flowchart TD"));
        assert!(mermaid.contains("start"));
        assert!(mermaid.contains("end"));
        assert!(mermaid.contains("-->"));
    }
}