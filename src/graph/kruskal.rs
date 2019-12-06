use super::weighted_graph::WeightedGraph;
use std::collections::HashMap;

impl WeightedGraph {
    /// Implementation of [Kruskal's
    /// algorithm](https://en.m.wikipedia.org/wiki/Kruskal%27s_algorithm) finding a minimum
    /// spanning tree (MST) in a undirected weighted graph.
    ///
    /// ```
    /// let mut graph = algorithms::graph::WeightedGraph::new();
    ///
    /// graph.add_edge(0, 1, 7);
    /// graph.add_edge(0, 2, 8);
    /// graph.add_edge(1, 2, 3);
    /// graph.add_edge(1, 3, 6);
    /// graph.add_edge(2, 3, 4);
    /// graph.add_edge(2, 4, 3);
    /// graph.add_edge(3, 4, 2);
    /// graph.add_edge(3, 5, 5);
    /// graph.add_edge(4, 5, 2);
    ///
    /// let mst = graph.mst_kruskal();
    ///
    /// let mut expected = std::collections::HashSet::new();
    /// expected.insert((0, 1, 7));
    /// expected.insert((3, 4, 2));
    /// expected.insert((4, 5, 2));
    /// expected.insert((1, 2, 3));
    /// expected.insert((2, 4, 3));
    ///
    /// assert_eq!(mst.len_edges(), expected.len());
    ///
    /// for edge in mst.as_tuples() {
    ///     assert!(expected.contains(&edge));
    /// }
    /// ```
    pub fn mst_kruskal(&self) -> WeightedGraph {
        // The algorithm is simple enough: Create an empty new Graph. Go through each edge
        // (incrementally sorted by weight) and add it to our new graph iff the new edge does not make
        // it so that the new graph contains cycles.
        //
        // The only tricky part, is to check wether adding a graph, will mean there are now a cycle
        // in the graph. The best way to think about it for me, is to imagine we divide the graph
        // up into sets. So we start out with nothing connected, so each node is its own set. As we
        // add connect nodes they form a larger set, and so we need something to represent that
        // set, and make sure that no matter what node in the set we look at, we can get the
        // "reprensentation" of that set.
        //
        // To do this, we have a HashMap<i32, i32> "reps" with initially no values, but as we add
        // edges, it starts forming a chain where we can, with `find_rep`, recursively walk the
        // chain to find the reprenstation.
        // With this, we can just connect the two reps that we found, by adding them to the reps
        // HashMap.

        // Our return value
        let mut mst = WeightedGraph::new();

        // Holds reprenstations of each set.
        let mut reps = HashMap::new();

        // we need to walk through each so tuples are easier to deal with
        let mut edges: Vec<(i32, i32, usize)> = self.as_tuples();

        edges.sort_unstable_by(|e1, e2| e1.2.cmp(&e2.2)); // sort by weight

        for (u, v, w) in edges {
            let u_rep = find_rep(&reps, &u); // rep for the set that u belongs to
            let v_rep = find_rep(&reps, &v); // rep for the set that v belongs to

            // If they do not belong to the same set...
            if u_rep != v_rep {
                reps.insert(u_rep, v_rep); // join the sets
                mst.add_edge(u, v, w); // add them to our MST
            }
        }

        mst
    }
}

fn find_rep<'a>(reps: &'a HashMap<i32, i32>, u: &'a i32) -> i32 {
    match reps.get(u) {
        None => *u,                   // No rep for u, so we found the rep, u
        Some(v) => find_rep(reps, v), // Some other value is the rep, so try again.
    }
}
