use std::collections::HashMap;

pub struct WeightedGraph {
    pub graph: HashMap<i32, Vec<(i32, usize)>>,
}

impl WeightedGraph {
    pub fn new() -> Self {
        WeightedGraph {
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
        self.graph.get_mut(&u).map(|edges| edges.push((v, weight)));
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn len_edges(&self) -> usize {
        self.graph.values().map(|c| c.len()).sum()
    }

    pub fn as_tuples(&self) -> Vec<(i32, i32, usize)> {
        let mut edges = Vec::new();

        for &k in self.graph.keys() {
            for (v, w) in &self.graph[&k] {
                edges.push((k, *v, *w));
            }
        }

        edges
    }
}

#[cfg(test)]
mod tests {
    use super::WeightedGraph;

    #[test]
    fn basic_graph_structure() {
        let mut graph = WeightedGraph::new();

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
