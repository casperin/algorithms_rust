use super::directed_graph::DirectedGraph;
use std::collections::HashSet;

impl DirectedGraph {
    /// Returns a vector of each of the nodes, ordered by a breadth first search.
    ///
    /// ```
    /// let mut graph = algorithms::graph::DirectedGraph::new();
    ///
    /// graph.add_edge(0, 1); // these are directed edges
    /// graph.add_edge(0, 2);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 0);
    /// graph.add_edge(2, 3);
    /// graph.add_edge(3, 3);
    ///
    /// let v = graph.breadth_first_search_new(2);
    ///
    /// assert_eq!(v, vec![2, 0, 3, 1]);
    /// ```
    ///
    /// See [wikipedia](https://en.wikipedia.org/wiki/Breadth-first_search#Applications) for
    /// applications.
    pub fn breadth_first_search_new(&self, node: i32) -> Vec<i32> {
        let num_of_nodes = self.graph.len();

        let mut queue = Vec::with_capacity(num_of_nodes);
        queue.push(node);

        let mut visited = HashSet::with_capacity(num_of_nodes);
        visited.insert(node);

        let mut i = 0;

        while let Some(node) = queue.get(i) {
            i += 1;

            if let Some(connections) = self.graph.get(&node) {
                for c in connections {
                    if !visited.contains(c) {
                        visited.insert(*c);
                        queue.push(*c);
                    }
                }
            }
        }

        queue
    }
}
