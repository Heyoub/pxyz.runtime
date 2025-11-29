//! Binary Emission (graph.bin)

use crate::graph::ir::*;
use crate::core::binary::*;
use crate::CompileError;
use sha2::{Sha256, Digest};

/// Emit graph.bin
pub fn emit(ir: &GraphIR, source_xml: &str) -> Result<Vec<u8>, CompileError> {
    let mut buffer = vec![0u8; HEADER_SIZE];
    
    // Emit nodes (16 bytes each)
    let nodes_offset = buffer.len() as u32;
    for node in &ir.nodes {
        buffer.extend_from_slice(&node.id.to_le_bytes());        // 4: id
        buffer.push(node.kind as u8);                            // 1: kind
        buffer.push(node.flags);                                 // 1: flags
        buffer.extend_from_slice(&node.op_code.to_le_bytes());   // 2: op_code
        buffer.extend_from_slice(&node.data_offset.to_le_bytes()); // 4: data
        buffer.extend_from_slice(&node.edge_start.to_le_bytes()); // 2: edge_start
        buffer.extend_from_slice(&node.edge_count.to_le_bytes()); // 2: edge_count
    }
    
    // Emit edges (12 bytes each)
    let edges_offset = buffer.len() as u32;
    for edge in &ir.edges {
        buffer.extend_from_slice(&edge.target.to_le_bytes());    // 4: target
        buffer.extend_from_slice(&(edge.predicate_id as u32).to_le_bytes()); // 4: predicate
        buffer.extend_from_slice(&edge.weight.to_le_bytes());    // 2: weight
        buffer.extend_from_slice(&edge.flags.to_le_bytes());     // 2: flags
    }
    
    // Emit predicates (2-byte length + bytecode)
    let predicates_offset = buffer.len() as u32;
    for pred in &ir.predicates {
        buffer.extend_from_slice(&(pred.bytecode.len() as u16).to_le_bytes());
        buffer.extend_from_slice(&pred.bytecode);
    }
    
    // Emit string pool
    let strings_offset = buffer.len() as u32;
    buffer.extend_from_slice(&ir.strings.data);
    
    // Emit entry points (8 bytes each)
    let entries_offset = buffer.len() as u32;
    for entry in &ir.entries {
        buffer.extend_from_slice(&entry.px_hash.to_le_bytes()); // 4: px_hash
        buffer.extend_from_slice(&entry.node_id.to_le_bytes()); // 4: node_id
    }
    
    // Compute source hash
    let mut hasher = Sha256::new();
    hasher.update(source_xml.as_bytes());
    let source_hash = hasher.finalize();
    
    // Fill header
    let h = &mut buffer[..HEADER_SIZE];
    h[header_offsets::MAGIC..header_offsets::MAGIC+4].copy_from_slice(&MAGIC.to_le_bytes());
    h[header_offsets::VERSION_MAJOR..header_offsets::VERSION_MAJOR+2].copy_from_slice(&VERSION_MAJOR.to_le_bytes());
    h[header_offsets::VERSION_MINOR..header_offsets::VERSION_MINOR+2].copy_from_slice(&VERSION_MINOR.to_le_bytes());
    h[header_offsets::NODE_COUNT..header_offsets::NODE_COUNT+4].copy_from_slice(&(ir.nodes.len() as u32).to_le_bytes());
    h[header_offsets::EDGE_COUNT..header_offsets::EDGE_COUNT+4].copy_from_slice(&(ir.edges.len() as u32).to_le_bytes());
    h[header_offsets::PREDICATE_COUNT..header_offsets::PREDICATE_COUNT+4].copy_from_slice(&(ir.predicates.len() as u32).to_le_bytes());
    h[header_offsets::STRING_POOL_SIZE..header_offsets::STRING_POOL_SIZE+4].copy_from_slice(&(ir.strings.data.len() as u32).to_le_bytes());
    h[header_offsets::ENTRY_COUNT..header_offsets::ENTRY_COUNT+4].copy_from_slice(&(ir.entries.len() as u32).to_le_bytes());
    h[header_offsets::SOURCE_HASH..header_offsets::SOURCE_HASH+32].copy_from_slice(&source_hash);
    h[header_offsets::NODES_OFFSET..header_offsets::NODES_OFFSET+4].copy_from_slice(&nodes_offset.to_le_bytes());
    h[header_offsets::EDGES_OFFSET..header_offsets::EDGES_OFFSET+4].copy_from_slice(&edges_offset.to_le_bytes());
    h[header_offsets::PREDICATES_OFFSET..header_offsets::PREDICATES_OFFSET+4].copy_from_slice(&predicates_offset.to_le_bytes());
    h[header_offsets::STRINGS_OFFSET..header_offsets::STRINGS_OFFSET+4].copy_from_slice(&strings_offset.to_le_bytes());
    h[header_offsets::ENTRIES_OFFSET..header_offsets::ENTRIES_OFFSET+4].copy_from_slice(&entries_offset.to_le_bytes());
    
    Ok(buffer)
}

//! Binary Format Constants

/// Magic number: "PXYZ" in little-endian
pub const MAGIC: u32 = 0x504E5958;

/// Format version
pub const VERSION_MAJOR: u16 = 1;
pub const VERSION_MINOR: u16 = 0;

/// Header size in bytes
pub const HEADER_SIZE: usize = 0x60; // 96 bytes

/// Node entry size
pub const NODE_ENTRY_SIZE: usize = 16;

/// Edge entry size
pub const EDGE_ENTRY_SIZE: usize = 12;

/// Entry point size
pub const ENTRY_SIZE: usize = 8;

/// Header field offsets
pub mod header_offsets {
    pub const MAGIC: usize = 0x00;
    pub const VERSION_MAJOR: usize = 0x04;
    pub const VERSION_MINOR: usize = 0x06;
    pub const NODE_COUNT: usize = 0x08;
    pub const EDGE_COUNT: usize = 0x0C;
    pub const PREDICATE_COUNT: usize = 0x10;
    pub const STRING_POOL_SIZE: usize = 0x14;
    pub const ENTRY_COUNT: usize = 0x18;
    pub const SCHEMA_COUNT: usize = 0x1C;
    pub const SOURCE_HASH: usize = 0x20;
    pub const NODES_OFFSET: usize = 0x40;
    pub const EDGES_OFFSET: usize = 0x44;
    pub const PREDICATES_OFFSET: usize = 0x48;
    pub const STRINGS_OFFSET: usize = 0x4C;
    pub const ENTRIES_OFFSET: usize = 0x50;
    pub const SCHEMAS_OFFSET: usize = 0x54;
}

/// Node entry field offsets
pub mod node_offsets {
    pub const ID: usize = 0x00;
    pub const KIND: usize = 0x04;
    pub const FLAGS: usize = 0x05;
    pub const OP_CODE: usize = 0x06;
    pub const DATA_OFFSET: usize = 0x08;
    pub const EDGE_START: usize = 0x0C;
    pub const EDGE_COUNT: usize = 0x0E;
}

/// Edge entry field offsets
pub mod edge_offsets {
    pub const TARGET_NODE: usize = 0x00;
    pub const PREDICATE_ID: usize = 0x04;
    pub const WEIGHT: usize = 0x08;
    pub const FLAGS: usize = 0x0A;
}

/// Entry point field offsets
pub mod entry_offsets {
    pub const PX_HASH: usize = 0x00;
    pub const NODE_ID: usize = 0x04;
}

/// FNV-1a hash for (P, X) coordinate lookup
pub fn hash_px(p: &str, x: &str) -> u32 {
    const FNV_PRIME: u32 = 16777619;
    const FNV_OFFSET: u32 = 2166136261;
    
    let mut hash = FNV_OFFSET;
    
    for byte in p.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    
    hash ^= 0xFF; // Separator
    hash = hash.wrapping_mul(FNV_PRIME);
    
    for byte in x.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_px() {
        let h1 = hash_px("contact", "search");
        let h2 = hash_px("contact", "search");
        let h3 = hash_px("contact", "create");
        
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }
}

