//! # Cache-Aware Memory Layout
//!
//! The graph.bin format is designed for cache efficiency.
//! This module provides tools to verify and optimize cache behavior.
//!
//! ## Cache Line Alignment
//!
//! Modern CPUs use 64-byte cache lines. PXYZ structures are designed
//! to fit within cache line boundaries:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                     Cache Line (64 bytes)                        │
//! ├──────────────────┬──────────────┬──────────────┬────────────────┤
//! │ Node (16 bytes)  │ Edge (12 B)  │ Edge (12 B)  │ Edge (12 B)    │
//! └──────────────────┴──────────────┴──────────────┴────────────────┘
//!                    │ 12 bytes padding for alignment │
//! ```
//!
//! ## Locality Principles
//!
//! 1. A node and its outgoing edges should be in the same cache line
//! 2. Frequently traversed paths should be contiguous in memory
//! 3. Predicates for an edge should be near the edge data

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

/// Cache line size on most modern CPUs
pub const CACHE_LINE_SIZE: usize = 64;

/// L1 cache typical size (32-64 KB)
pub const L1_SIZE: usize = 32 * 1024;

/// L2 cache typical size (256 KB - 1 MB)
pub const L2_SIZE: usize = 256 * 1024;

/// L3 cache typical size (varies widely)
pub const L3_SIZE: usize = 8 * 1024 * 1024;

/// Memory tier for cache-aware allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTier {
    /// Hot data: fits in L1 (< 32KB)
    Hot,
    /// Warm data: fits in L2 (< 256KB)
    Warm,
    /// Cold data: fits in L3 (< 8MB)
    Cold,
    /// External: requires DRAM access
    External,
}

impl MemoryTier {
    /// Determine tier from data size
    pub fn from_size(bytes: usize) -> Self {
        if bytes <= L1_SIZE {
            MemoryTier::Hot
        } else if bytes <= L2_SIZE {
            MemoryTier::Warm
        } else if bytes <= L3_SIZE {
            MemoryTier::Cold
        } else {
            MemoryTier::External
        }
    }

    /// Approximate access latency in cycles
    pub fn latency_cycles(&self) -> u32 {
        match self {
            MemoryTier::Hot => 4,       // L1: ~4 cycles
            MemoryTier::Warm => 12,     // L2: ~12 cycles
            MemoryTier::Cold => 40,     // L3: ~40 cycles
            MemoryTier::External => 200, // DRAM: ~200 cycles
        }
    }

    /// Energy cost multiplier relative to register access
    pub fn energy_multiplier(&self) -> u32 {
        match self {
            MemoryTier::Hot => 5,
            MemoryTier::Warm => 100,
            MemoryTier::Cold => 500,
            MemoryTier::External => 6400,
        }
    }
}

/// Cache-line aligned buffer
#[derive(Debug)]
pub struct CacheAlignedBuffer {
    ptr: NonNull<u8>,
    len: usize,
    capacity: usize,
}

impl CacheAlignedBuffer {
    /// Allocate a cache-line aligned buffer
    pub fn new(capacity: usize) -> Self {
        // Round up to cache line boundary
        let aligned_capacity = (capacity + CACHE_LINE_SIZE - 1) & !(CACHE_LINE_SIZE - 1);

        let layout = Layout::from_size_align(aligned_capacity, CACHE_LINE_SIZE)
            .expect("Invalid layout");

        let ptr = unsafe {
            let raw = alloc(layout);
            if raw.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            NonNull::new_unchecked(raw)
        };

        Self {
            ptr,
            len: 0,
            capacity: aligned_capacity,
        }
    }

    /// Get a slice of the buffer contents
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Get a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }

    /// Write data to buffer
    pub fn write(&mut self, data: &[u8]) -> Result<(), CacheError> {
        if self.len + data.len() > self.capacity {
            return Err(CacheError::BufferFull);
        }

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.ptr.as_ptr().add(self.len),
                data.len(),
            );
        }
        self.len += data.len();
        Ok(())
    }

    /// Number of cache lines used
    pub fn cache_lines_used(&self) -> usize {
        (self.len + CACHE_LINE_SIZE - 1) / CACHE_LINE_SIZE
    }

    /// Check if a range fits in a single cache line
    pub fn fits_in_cache_line(&self, offset: usize, len: usize) -> bool {
        let start_line = offset / CACHE_LINE_SIZE;
        let end_line = (offset + len - 1) / CACHE_LINE_SIZE;
        start_line == end_line
    }

    /// Memory tier based on current usage
    pub fn memory_tier(&self) -> MemoryTier {
        MemoryTier::from_size(self.len)
    }
}

impl Drop for CacheAlignedBuffer {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.capacity, CACHE_LINE_SIZE)
            .expect("Invalid layout");
        unsafe {
            dealloc(self.ptr.as_ptr(), layout);
        }
    }
}

// Safety: The buffer owns its memory exclusively
unsafe impl Send for CacheAlignedBuffer {}
unsafe impl Sync for CacheAlignedBuffer {}

#[derive(Debug)]
pub enum CacheError {
    BufferFull,
    Misaligned,
}

/// Analyze cache behavior of a graph layout
#[derive(Debug, Default)]
pub struct CacheAnalysis {
    /// Total bytes
    pub total_bytes: usize,
    /// Cache lines used
    pub cache_lines: usize,
    /// Nodes that fit with their edges in one cache line
    pub well_packed_nodes: usize,
    /// Nodes that span multiple cache lines
    pub poorly_packed_nodes: usize,
    /// Estimated L1 hit rate
    pub estimated_l1_hit_rate: f64,
    /// Memory tier for entire graph
    pub tier: MemoryTier,
}

impl CacheAnalysis {
    /// Analyze a graph.bin buffer
    pub fn analyze(buffer: &[u8]) -> Self {
        let total_bytes = buffer.len();
        let cache_lines = (total_bytes + CACHE_LINE_SIZE - 1) / CACHE_LINE_SIZE;
        let tier = MemoryTier::from_size(total_bytes);

        // Estimate hit rate based on tier
        let estimated_l1_hit_rate = match tier {
            MemoryTier::Hot => 0.95,
            MemoryTier::Warm => 0.80,
            MemoryTier::Cold => 0.60,
            MemoryTier::External => 0.30,
        };

        Self {
            total_bytes,
            cache_lines,
            well_packed_nodes: 0, // Would need graph parsing to compute
            poorly_packed_nodes: 0,
            estimated_l1_hit_rate,
            tier,
        }
    }

    /// Estimated energy for full traversal
    pub fn estimated_energy(&self) -> u64 {
        let base_cost = self.cache_lines as u64;
        let tier_multiplier = self.tier.energy_multiplier() as u64;
        base_cost * tier_multiplier
    }
}

/// Node-edge packing for cache efficiency
///
/// Optimal packing: 1 node (16 bytes) + up to 4 edges (48 bytes) = 64 bytes
#[repr(C, align(64))]
pub struct CachePackedNodeUnit {
    /// Node data (16 bytes)
    pub node: [u8; 16],
    /// Up to 4 edges (12 bytes each)
    pub edges: [[u8; 12]; 4],
}

static_assertions::const_assert_eq!(std::mem::size_of::<CachePackedNodeUnit>(), 64);

impl CachePackedNodeUnit {
    /// Create empty unit
    pub const fn empty() -> Self {
        Self {
            node: [0u8; 16],
            edges: [[0u8; 12]; 4],
        }
    }

    /// Check if this unit uses exactly one cache line
    pub const fn is_cache_aligned() -> bool {
        std::mem::size_of::<Self>() == CACHE_LINE_SIZE
            && std::mem::align_of::<Self>() == CACHE_LINE_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_aligned_buffer() {
        let mut buf = CacheAlignedBuffer::new(128);

        // Check alignment
        assert_eq!(buf.ptr.as_ptr() as usize % CACHE_LINE_SIZE, 0);

        // Write some data
        buf.write(&[1, 2, 3, 4]).unwrap();
        assert_eq!(buf.as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_memory_tier() {
        assert_eq!(MemoryTier::from_size(1000), MemoryTier::Hot);
        assert_eq!(MemoryTier::from_size(50_000), MemoryTier::Warm);
        assert_eq!(MemoryTier::from_size(1_000_000), MemoryTier::Cold);
        assert_eq!(MemoryTier::from_size(100_000_000), MemoryTier::External);
    }

    #[test]
    fn test_packed_node_unit() {
        assert!(CachePackedNodeUnit::is_cache_aligned());
    }
}
