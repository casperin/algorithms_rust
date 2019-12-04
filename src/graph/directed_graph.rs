use std::collections::HashMap;

/// The basic structure of a directed graph. All it contains, is map with each node as a key, and
/// the value a vector containing its connections.
pub struct DirectedGraph {
    pub graph: HashMap<i32, Vec<i32>>,
}

impl DirectedGraph {
    pub fn new() -> Self {
        DirectedGraph {
            graph: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, u: i32, v: i32) {
        if self.graph.get(&u).is_none() {
            self.graph.insert(u, Vec::new());
        }
        if self.graph.get(&v).is_none() {
            self.graph.insert(v, Vec::new());
        }
        self.graph.get_mut(&u).map(|edges| edges.push(v));
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn len_edges(&self) -> usize {
        self.graph.values().map(|c| c.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::DirectedGraph;

    #[test]
    fn basic_graph_structure() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3);

        assert_eq!(graph.len(), 4);
        assert_eq!(graph.len_edges(), 6);
    }
}
