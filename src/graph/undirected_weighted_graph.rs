use std::collections::HashMap;

pub struct UndirectedWeightedGraph {
    pub graph: HashMap<i32, Vec<(i32, usize)>>,
}

impl UndirectedWeightedGraph {
    pub fn new() -> Self {
        UndirectedWeightedGraph {
            graph: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, u: i32, v: i32, weight: usize) {
        if self.graph.get(&u).is_none() {
            self.graph.insert(u, Vec::new());
        }
        if self.graph.get(&v).is_none() {
            self.graph.insert(v, Vec::new());
        }
        // We add the graph in both directions
        self.graph.get_mut(&u).map(|edges| edges.push((v, weight)));
        self.graph.get_mut(&v).map(|edges| edges.push((u, weight)));
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn len_edges(&self) -> usize {
        // Every edge is there twice
        let sum: usize = self.graph.values().map(|c| c.len()).sum();
        sum / 2
    }
}

#[cfg(test)]
mod tests {
    use super::UndirectedWeightedGraph;

    #[test]
    fn basic_graph_structure() {
        let mut graph = UndirectedWeightedGraph::new();

        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 0, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 3, 1);

        assert_eq!(graph.len(), 4);
        assert_eq!(graph.len_edges(), 6);
    }
}
