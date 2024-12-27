use std::hash::Hash;

use super::bidirectional_map::BiMap;

pub struct Graph<T> {
    nodes: Vec<T>,
    edges: Vec<Edge>,
    map: BiMap<usize, T>,
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        let map = BiMap::new();
        let nodes = Vec::new();
        let edges = Vec::new();
        Self { nodes, edges, map }
    }

    pub fn add_node(&mut self, node: T) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node.clone());
        self.map.insert(id, node);
        id
    }

    pub fn add_node_with_edge(&mut self, node: T, other: T) {
        let from_id = self.add_node(node);
        let to_id = self.add_node(other);
        self.edges.push(Edge::new(from_id, to_id));
    }
}

pub struct Edge {
    forward: usize,
    backward: usize,
}

impl Edge {
    pub fn new(backward: usize, forward: usize) -> Self {
        Self { forward, backward }
    }
}
