//! # Graph Layout Optimization
//!
//! Optimal graph layout minimizes data movement energy by placing
//! frequently-communicating nodes adjacent in memory.
//!
//! ## Principles
//!
//! 1. **Temporal Locality**: If X follows Y, place them together
//! 2. **Spatial Locality**: Cluster related operations
//! 3. **Linear Preference**: DAGs favor linear layout (minimal jumps)
//! 4. **Cache Line Packing**: Node + edges fit in 64 bytes
//!
//! ## Algorithm
//!
//! 1. Build communication graph (edge = data dependency)
//! 2. Weight edges by frequency (hot paths weigh more)
//! 3. Topological sort with locality-aware tie-breaking
//! 4. Pack into cache-aligned blocks

use std::collections::{HashMap, HashSet, VecDeque};

/// A node in the layout graph
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: u32,
    /// Outgoing edges (target_id, weight/frequency)
    pub edges: Vec<(u32, f64)>,
    /// Number of incoming edges (for topo sort)
    pub in_degree: usize,
    /// Assigned memory position (after layout)
    pub position: Option<usize>,
}

/// Graph layout optimizer
#[derive(Debug)]
pub struct LayoutOptimizer {
    nodes: HashMap<u32, LayoutNode>,
    entry_points: Vec<u32>,
}

impl LayoutOptimizer {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            entry_points: Vec::new(),
        }
    }

    /// Add a node to the layout graph
    pub fn add_node(&mut self, id: u32) {
        self.nodes.entry(id).or_insert_with(|| LayoutNode {
            id,
            edges: Vec::new(),
            in_degree: 0,
            position: None,
        });
    }

    /// Add an edge with optional weight (default 1.0)
    pub fn add_edge(&mut self, from: u32, to: u32, weight: f64) {
        self.add_node(from);
        self.add_node(to);

        self.nodes.get_mut(&from).unwrap().edges.push((to, weight));
        self.nodes.get_mut(&to).unwrap().in_degree += 1;
    }

    /// Mark entry points
    pub fn add_entry_point(&mut self, id: u32) {
        self.add_node(id);
        self.entry_points.push(id);
    }

    /// Compute optimal layout using locality-aware topological sort
    ///
    /// Returns: Ordered list of node IDs (memory order)
    pub fn compute_layout(&mut self) -> Vec<u32> {
        // Kahn's algorithm with locality-aware tie-breaking
        let mut in_degree: HashMap<u32, usize> = self
            .nodes
            .iter()
            .map(|(&id, node)| (id, node.in_degree))
            .collect();

        let mut result = Vec::with_capacity(self.nodes.len());
        let mut available: VecDeque<u32> = VecDeque::new();

        // Start with entry points (they have in_degree 0)
        for &entry in &self.entry_points {
            if in_degree.get(&entry) == Some(&0) {
                available.push_back(entry);
            }
        }

        // Add any other nodes with in_degree 0
        for (&id, &deg) in &in_degree {
            if deg == 0 && !self.entry_points.contains(&id) {
                available.push_back(id);
            }
        }

        let mut last_processed: Option<u32> = None;

        while !available.is_empty() {
            // Locality-aware selection:
            // Prefer nodes that are successors of the last processed node
            let next = if let Some(last) = last_processed {
                // Find successor of last node that's available
                let last_node = self.nodes.get(&last);
                let successor = last_node.and_then(|node| {
                    node.edges
                        .iter()
                        .find(|(target, _)| available.contains(target))
                        .map(|(target, _)| *target)
                });

                if let Some(succ) = successor {
                    // Remove from available
                    let idx = available.iter().position(|&x| x == succ).unwrap();
                    available.remove(idx).unwrap()
                } else {
                    // No successor available, pick highest weighted
                    self.pick_highest_weight(&mut available)
                }
            } else {
                // First node: pick from entry points or highest weight
                available.pop_front().unwrap()
            };

            result.push(next);
            last_processed = Some(next);

            // Update in-degrees
            if let Some(node) = self.nodes.get(&next) {
                for (target, _) in &node.edges {
                    if let Some(deg) = in_degree.get_mut(target) {
                        *deg -= 1;
                        if *deg == 0 {
                            available.push_back(*target);
                        }
                    }
                }
            }
        }

        // Assign positions
        for (pos, &id) in result.iter().enumerate() {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.position = Some(pos);
            }
        }

        result
    }

    /// Pick node with highest total outgoing edge weight
    fn pick_highest_weight(&self, available: &mut VecDeque<u32>) -> u32 {
        let mut best_idx = 0;
        let mut best_weight = 0.0;

        for (idx, &id) in available.iter().enumerate() {
            let weight: f64 = self
                .nodes
                .get(&id)
                .map(|n| n.edges.iter().map(|(_, w)| w).sum())
                .unwrap_or(0.0);

            if weight > best_weight {
                best_weight = weight;
                best_idx = idx;
            }
        }

        available.remove(best_idx).unwrap()
    }

    /// Compute locality score for a layout
    ///
    /// Higher score = better locality (more edges go to adjacent nodes)
    pub fn locality_score(&self, layout: &[u32]) -> f64 {
        let position: HashMap<u32, usize> = layout
            .iter()
            .enumerate()
            .map(|(pos, &id)| (id, pos))
            .collect();

        let mut total_weight = 0.0;
        let mut locality_weight = 0.0;

        for node in self.nodes.values() {
            let from_pos = position.get(&node.id).copied().unwrap_or(0);

            for (to, weight) in &node.edges {
                let to_pos = position.get(to).copied().unwrap_or(0);
                let distance = (to_pos as isize - from_pos as isize).abs() as usize;

                total_weight += weight;

                // Adjacent = full locality credit
                // Further = diminishing returns
                if distance <= 1 {
                    locality_weight += weight;
                } else if distance <= 4 {
                    // Same cache line (4 nodes × 16 bytes)
                    locality_weight += weight * 0.75;
                } else if distance <= 16 {
                    // Same L1 block
                    locality_weight += weight * 0.25;
                }
                // Else: no locality credit
            }
        }

        if total_weight > 0.0 {
            locality_weight / total_weight
        } else {
            1.0
        }
    }

    /// Estimate energy for traversal with this layout
    pub fn estimate_energy(&self, layout: &[u32]) -> u64 {
        let position: HashMap<u32, usize> = layout
            .iter()
            .enumerate()
            .map(|(pos, &id)| (id, pos))
            .collect();

        let mut total_energy = 0u64;

        for node in self.nodes.values() {
            let from_pos = position.get(&node.id).copied().unwrap_or(0);

            for (to, _) in &node.edges {
                let to_pos = position.get(to).copied().unwrap_or(0);
                let distance = (to_pos as isize - from_pos as isize).abs() as usize;

                // Energy model: further = more cache misses
                let edge_energy = if distance <= 1 {
                    5  // L1 hit
                } else if distance <= 4 {
                    10  // Same cache line, but not sequential
                } else if distance <= 16 {
                    50  // L1 miss, L2 hit
                } else if distance <= 64 {
                    100  // L2 miss, L3 hit
                } else {
                    500  // L3 miss, DRAM
                };

                total_energy += edge_energy;
            }
        }

        total_energy
    }
}

impl Default for LayoutOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Layout quality metrics
#[derive(Debug, Clone)]
pub struct LayoutMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub locality_score: f64,
    pub estimated_energy: u64,
    pub cache_lines_used: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_layout() {
        let mut opt = LayoutOptimizer::new();

        // Linear chain: A -> B -> C -> D
        opt.add_entry_point(1);
        opt.add_edge(1, 2, 1.0);
        opt.add_edge(2, 3, 1.0);
        opt.add_edge(3, 4, 1.0);

        let layout = opt.compute_layout();
        assert_eq!(layout, vec![1, 2, 3, 4]);

        // Perfect locality
        let score = opt.locality_score(&layout);
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_diamond_layout() {
        let mut opt = LayoutOptimizer::new();

        // Diamond: A -> B, A -> C, B -> D, C -> D
        opt.add_entry_point(1);
        opt.add_edge(1, 2, 1.0);
        opt.add_edge(1, 3, 1.0);
        opt.add_edge(2, 4, 1.0);
        opt.add_edge(3, 4, 1.0);

        let layout = opt.compute_layout();
        assert_eq!(layout.len(), 4);
        assert_eq!(layout[0], 1);  // Entry first
        assert_eq!(layout[3], 4);  // Sink last

        // Score should be good but not perfect
        let score = opt.locality_score(&layout);
        assert!(score > 0.5);
    }
}
