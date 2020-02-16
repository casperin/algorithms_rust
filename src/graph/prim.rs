/*
use super::weighted_graph::WeightedGraph;
use std::collections::{HashMap, HashSet};

impl WeightedGraph {
    pub fn mst_prim(&self) -> WeightedGraph {
        // Our return value
        let mut mst = WeightedGraph::new();

        // Holds representations of each set.
        let mut reps = HashMap::new();

        // Hold vertices with their weight
        let mut keys = HashMap::new();
        let (starting_point, edges) = self.graph.iter().first().unwrap();

        // keys.insert(v, (None, 0));
        edges.for_each(|(u, w)| {
            keys.insert(u, (Some((starting_point, w)), w));
        });

        loop {
            // find smallest edge and get its
        }

        mst
    }
}
*/
