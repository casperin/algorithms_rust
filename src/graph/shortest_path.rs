use super::directed_graph::DirectedGraph;
use std::collections::{HashMap, HashSet};

impl DirectedGraph {
    /// Uses a modified version of BFS to find the shortest path (if it exists) between two nodes
    /// in an unweighted graph.
    ///
    /// In the example below, we build this graph, then ask for the shorted path between 0 and 7.
    ///
    /// ```txt
    ///  0 --> 1 --> 2
    ///  |
    ///  v
    ///  3 --> 4 --> 5
    ///  |    / \    |
    ///  v   |   |   |
    ///  7 <-'   v   |
    ///  ^______ 6 <-'
    ///
    /// ```
    ///
    /// ```
    /// use algorithms::graph::DirectedGraph;
    ///
    /// let mut g = DirectedGraph::new();
    ///
    /// g.add_edge(0, 1);
    /// g.add_edge(0, 3);
    /// g.add_edge(1, 2);
    /// g.add_edge(3, 4);
    /// g.add_edge(3, 7);
    /// g.add_edge(4, 5);
    /// g.add_edge(4, 6);
    /// g.add_edge(4, 7);
    /// g.add_edge(5, 6);
    /// g.add_edge(6, 7);
    ///
    /// let source = 0;
    /// let destination = 7;
    ///
    /// let path = g.shortest_path(source, destination);
    ///
    /// assert_eq!(path, Some(vec![0, 3, 7]));
    /// ```
    pub fn shortest_path(&self, src: i32, dst: i32) -> Option<Vec<i32>> {
        let predecessors = self.bfs(src, dst)?;

        let mut path: Vec<i32> = Vec::new(); // output

        // We need to traverse backwards from the destination to the source.
        let mut current_node = dst;
        path.push(current_node);

        while let Some(&p) = predecessors.get(&current_node) {
            path.push(p);
            current_node = p;
        }

        path.reverse();

        return Some(path);
    }

    fn bfs(&self, src: i32, dst: i32) -> Option<HashMap<i32, i32>> {
        // For each node, we need to keep a reference to its predecessor so we can find our way
        // back. Each node can of course have more than one predecessor, but since we are using
        // BFS, we know that the first one we find, will be the one with the shortest path.
        let mut predecessors: HashMap<i32, i32> = HashMap::new();

        // From here, it is a basic breadt first search, except we keep track of the predecessor
        // for each node, and once we find the node we are searching for (dst) we return some
        // predecessors and let caller work out the exact path.
        let mut queue: Vec<i32> = Vec::new();
        let mut visited: HashSet<i32> = HashSet::new();

        visited.insert(src);
        queue.push(src);

        let mut i = 0;

        while let Some(&node) = queue.get(i) {
            i += 1;

            let connections = match self.graph.get(&node) {
                Some(c) => c,
                None => continue,
            };

            for &c in connections {
                if visited.contains(&c) {
                    continue;
                }

                visited.insert(c);
                queue.push(c);
                predecessors.insert(c, node);

                if c == dst {
                    return Some(predecessors);
                }
            }
        }

        None
    }
}
