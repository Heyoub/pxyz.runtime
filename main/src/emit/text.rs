//! Human-readable text output

use crate::compiler::ir::*;
use crate::compiler::bytecode::disassemble;

pub fn format(ir: &GraphIR) -> String {
    let mut out = String::new();
    
    out.push_str("╔══════════════════════════════════════════╗\n");
    out.push_str("║           PXYZ GRAPH DUMP                ║\n");
    out.push_str("╚══════════════════════════════════════════╝\n\n");
    
    // Stats
    out.push_str(&format!("Nodes: {}  Edges: {}  Predicates: {}  Entries: {}\n\n",
        ir.nodes.len(), ir.edges.len(), ir.predicates.len(), ir.entries.len()));
    
    // Entry points
    out.push_str("ENTRY POINTS:\n");
    out.push_str("─────────────\n");
    for entry in &ir.entries {
        let node_name = ir.nodes.iter()
            .find(|n| n.id == entry.node_id)
            .map(|n| n.name.as_str())
            .unwrap_or("?");
        out.push_str(&format!("  ({}, {}) → {} [hash: 0x{:08x}]\n",
            entry.p, entry.x, node_name, entry.px_hash));
    }
    out.push('\n');
    
    // Nodes
    out.push_str("NODES:\n");
    out.push_str("──────\n");
    for node in &ir.nodes {
        out.push_str(&format!("  [{}] {} ({:?})\n", node.id, node.name, node.kind));
        if node.op_code != 0 {
            out.push_str(&format!("       op: 0x{:04x}\n", node.op_code));
        }
        if node.edge_count > 0 {
            out.push_str(&format!("       edges: {}..{}\n", 
                node.edge_start, node.edge_start + node.edge_count));
        }
        if node.flags != 0 {
            out.push_str(&format!("       flags: 0b{:08b}\n", node.flags));
        }
    }
    out.push('\n');
    
    // Edges
    out.push_str("EDGES:\n");
    out.push_str("──────\n");
    for edge in &ir.edges {
        let from_name = ir.nodes.iter()
            .find(|n| n.id == edge.from)
            .map(|n| n.name.as_str())
            .unwrap_or("?");
        let to_name = ir.nodes.iter()
            .find(|n| n.id == edge.target)
            .map(|n| n.name.as_str())
            .unwrap_or("?");
        let pred_name = if edge.predicate_id == 0 {
            "always".to_string()
        } else {
            ir.predicates.iter()
                .find(|p| p.id == edge.predicate_id)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| format!("#{}", edge.predicate_id))
        };
        out.push_str(&format!("  {} → {} [{}]\n", from_name, to_name, pred_name));
    }
    out.push('\n');
    
    // Predicates
    if !ir.predicates.is_empty() {
        out.push_str("PREDICATES:\n");
        out.push_str("───────────\n");
        for pred in &ir.predicates {
            out.push_str(&format!("  [{}] {} ({} bytes)\n", pred.id, pred.name, pred.bytecode.len()));
            if !pred.bytecode.is_empty() {
                let disasm = disassemble(&pred.bytecode, &ir.strings);
                for line in disasm.lines() {
                    out.push_str(&format!("       {}\n", line));
                }
            }
        }
        out.push('\n');
    }
    
    // String pool
    if !ir.strings.data.is_empty() {
        out.push_str(&format!("STRING POOL ({} bytes, {} strings):\n", 
            ir.strings.data.len(), ir.strings.count()));
        out.push_str("────────────────────\n");
        for (i, (offset, s)) in ir.strings.iter().enumerate().take(20) {
            out.push_str(&format!("  @{}: \"{}\"\n", offset, s));
        }
        if ir.strings.count() > 20 {
            out.push_str(&format!("  ... and {} more\n", ir.strings.count() - 20));
        }
    }
    
    out
}