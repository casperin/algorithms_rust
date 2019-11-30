use super::directed_graph::DirectedGraph;
use std::collections::{HashMap, HashSet};

impl DirectedGraph {
    /// Returns a vector of each of the nodes, ordered by a depth first search.
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
    /// let v = graph.depth_first_search_new(2);
    ///
    /// assert_eq!(v, vec![2, 0, 1, 3]);
    /// ```
    ///
    /// See [wikipedia](https://en.wikipedia.org/wiki/Depth-first_search#Applications) for
    /// applications.
    pub fn depth_first_search_new(&self, node: i32) -> Vec<i32> {
        let num_of_nodes = self.graph.len();
        let mut queue = Vec::with_capacity(num_of_nodes);
        let mut visited = HashSet::with_capacity(num_of_nodes);

        add_to_queue(&self.graph, &mut queue, &mut visited, node);

        queue
    }
}

// Recursive helper function. Adds the current node and its connections to the queue.
fn add_to_queue(
    graph: &HashMap<i32, Vec<i32>>,
    queue: &mut Vec<i32>,
    visited: &mut HashSet<i32>,
    node: i32,
) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);
    queue.push(node);

    if let Some(connections) = graph.get(&node) {
        for c in connections {
            add_to_queue(&graph, queue, visited, *c);
        }
    }
}
