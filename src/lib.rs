use rand::{Rng, rngs::ThreadRng};
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weighted(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unweighted(pub ());

#[derive(Debug, PartialEq, PartialOrd)]
pub enum GraphError {
    OutOfBoundsNode {
        node: u32,
    },
    DistanveOverflow {
        node_from: u32,
        node_to: u32,
        current_distance: u32,
        edge_weight: u32,
    },
}

/// A graph is represented as an adjacency list, which is internally
/// modelled as a vector of vectors `Vec<Vec<(u32, W)>>`.
/// Each index corresponds to a node, and each inner `Vec` stores
/// edges of the form `(target_node, weight)`. The weight type `W`
/// is user-defined:
/// - use [`Weighted`] for weighted graphs,
/// - use [`Unweighted`] for unweighted graphs.
///
/// Note: node identifiers are always `u32`, and must be valid indices
/// into the adjacency list.
///
/// # Examples
///
/// ## Weighted directed graph
/// ```rust
/// use edgewise::{Graph, Weighted};
/// let weighted: Graph<Weighted> = Graph::new(vec![
///         vec![(1, Weighted(1)), (2, Weighted(3))], // edges from node 0
///         vec![(2, Weighted(0))],     // edges from node 1
///         vec![(0, Weighted(5))],     // edges from node 2
///     ]);
/// ```
///
/// ## Unweighted undirected graph
/// ```rust
/// use edgewise::{Graph, Unweighted};
/// let unweighted: Graph<Unweighted> = Graph::new(vec![
///         vec![(1, Unweighted(()))],  // edge 0 -> 1
///         vec![(0, Unweighted(()))],  // edge 1 -> 0
///     ]);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Graph<W> {
    graph: Vec<Vec<(u32, W)>>,
}

impl<W> Graph<W> {
    pub fn new(g: Vec<Vec<(u32, W)>>) -> Self {
        let n: usize = g.len();
        assert!(
            n <= u32::MAX as usize,
            "The number of nodes of the graph must fit in u32."
        );
        Self { graph: g }
    }

    // An iterator over the edges of the graph.
    pub fn edges(&self) -> impl Iterator<Item = (u32, u32, &W)> + '_ {
        self.graph.iter().enumerate().flat_map(|(u, v)| {
            let x = u as u32; //safe as the number of nodes is capped by u32 anyway
            v.iter().map(move |(y, w)| (x, *y, w))
        })
    }

    pub fn bfs(&self, starting_node: u32) -> Result<Vec<u32>, GraphError> {
        if (starting_node as usize) >= self.graph.len() {
            return Err(GraphError::OutOfBoundsNode {
                node: starting_node,
            });
        }
        let mut nodes_left_to_process: VecDeque<u32> = VecDeque::new();
        let mut nodes_visited_lookup: Vec<bool> = vec![false; self.graph.len()];
        let mut nodes_visited: Vec<u32> = Vec::new();
        nodes_left_to_process.push_back(starting_node);
        nodes_visited_lookup[starting_node as usize] = true;
        nodes_visited.push(starting_node);
        while let Some(node_to_process) = nodes_left_to_process.pop_front() {
            if let Some(neighbours_of_node) = self.graph.get(node_to_process as usize) {
                for &(n, _) in neighbours_of_node {
                    if !nodes_visited_lookup[n as usize] {
                        nodes_visited_lookup[n as usize] = true;
                        nodes_visited.push(n);
                        nodes_left_to_process.push_back(n);
                    }
                }
            }
        }
        Ok(nodes_visited)
    }

    pub fn dfs(&self, starting_node: u32) -> Result<Vec<u32>, GraphError> {
        if (starting_node as usize) >= self.graph.len() {
            return Err(GraphError::OutOfBoundsNode {
                node: starting_node,
            });
        }
        let mut nodes_left_to_process: VecDeque<u32> = VecDeque::new();
        let mut nodes_visited_lookup: Vec<bool> = vec![false; self.graph.len()];
        let mut nodes_visited: Vec<u32> = Vec::new();
        nodes_left_to_process.push_back(starting_node);
        nodes_visited_lookup[starting_node as usize] = true;
        nodes_visited.push(starting_node);
        while !nodes_left_to_process.is_empty() {
            let mut found_unvisited = false;
            if let Some(&node_to_process) = nodes_left_to_process.back()
                && let Some(neighbours_of_node) = self.graph.get(node_to_process as usize)
            {
                for &(n, _) in neighbours_of_node {
                    if !nodes_visited_lookup[n as usize] {
                        nodes_visited_lookup[n as usize] = true;
                        nodes_visited.push(n);
                        nodes_left_to_process.push_back(n);
                        found_unvisited = true;
                        break;
                    }
                }
                if !found_unvisited {
                    nodes_left_to_process.pop_back();
                }
            }
        }
        Ok(nodes_visited)
    }
}

#[allow(private_bounds)]
impl<W: InsertEdge> Graph<W> {
    fn insert_edge(&mut self, rng: &mut ThreadRng, i: u32, j: u32, is_directed: bool) {
        W::insert_edge(self, rng, i, j, is_directed);
    }

    pub fn random_graph(num_nodes: u32, probability: f64, is_directed: bool) -> Self {
        let mut v: Vec<Vec<(u32, W)>> = Vec::new();
        v.resize_with(num_nodes as usize, Vec::new);
        let mut graph = Graph::new(v);
        let mut rng = rand::rng();
        for i in 0..num_nodes {
            let z = if is_directed { 0 } else { i + 1 };
            for j in z..num_nodes {
                let r: f64 = rng.random();
                if r < probability {
                    graph.insert_edge(&mut rng, i, j, is_directed);
                }
            }
        }
        graph
    }
}

impl Graph<Weighted> {
    pub fn dijkstra(&self, starting_node: u32) -> Result<Vec<Option<u32>>, GraphError> {
        if (starting_node as usize) >= self.graph.len() {
            return Err(GraphError::OutOfBoundsNode {
                node: starting_node,
            });
        }
        let mut nodes_distance: Vec<Option<u32>> = vec![None; self.graph.len()];
        let mut nodes_visited: Vec<bool> = vec![false; self.graph.len()];
        nodes_distance[starting_node as usize] = Some(0);
        while let Some((current_node, current_distance)) = (0..nodes_visited.len())
            .filter(|&i| !nodes_visited[i])
            .filter_map(|i| nodes_distance[i].map(|d| (i, d)))
            .min_by_key(|&(_, d)| d)
        {
            if let Some(neighbors) = self.graph.get(current_node) {
                for &(neighbor_node, neighbor_weight) in neighbors {
                    if let Some(new_distance) = current_distance.checked_add(neighbor_weight.0) {
                        if let Some(neighbor_distance) = nodes_distance[neighbor_node as usize] {
                            if new_distance < neighbor_distance {
                                nodes_distance[neighbor_node as usize] = Some(new_distance)
                            }
                        } else {
                            nodes_distance[neighbor_node as usize] = Some(new_distance)
                        }
                    } else {
                        // New distance for the current_node causes an overflow.
                        return Err(GraphError::DistanveOverflow {
                            node_from: current_node as u32,
                            node_to: neighbor_node,
                            current_distance,
                            edge_weight: neighbor_weight.0,
                        });
                    }
                }
            }
            nodes_visited[current_node] = true
        }
        Ok(nodes_distance)
    }
}

trait InsertEdge: Sized {
    fn insert_edge(g: &mut Graph<Self>, rng: &mut ThreadRng, i: u32, j: u32, is_directed: bool);
}

impl InsertEdge for Unweighted {
    fn insert_edge(
        g: &mut Graph<Unweighted>,
        _rng: &mut ThreadRng,
        i: u32,
        j: u32,
        is_directed: bool,
    ) {
        let u = g
            .graph
            .get_mut(i as usize)
            .unwrap_or_else(|| panic!("Node {i} is out of bounds"));
        u.push((j, Unweighted(())));
        if !is_directed {
            let v = g
                .graph
                .get_mut(j as usize)
                .unwrap_or_else(|| panic!("Node {j} is out of bounds"));
            v.push((i, Unweighted(())));
        }
    }
}

impl InsertEdge for Weighted {
    fn insert_edge(
        g: &mut Graph<Weighted>,
        rng: &mut ThreadRng,
        i: u32,
        j: u32,
        is_directed: bool,
    ) {
        let w: u32 = rng.random_range(1..=10);
        let u = g
            .graph
            .get_mut(i as usize)
            .unwrap_or_else(|| panic!("Node {i} is out of bounds"));
        u.push((j, Weighted(w)));
        if !is_directed {
            let v = g
                .graph
                .get_mut(j as usize)
                .unwrap_or_else(|| panic!("Node {j} is out of bounds"));
            v.push((i, Weighted(w)));
        }
    }
}

impl fmt::Display for Graph<Unweighted> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i = self.edges();
        for (x, y, _) in i {
            writeln!(f, "{x}->{y}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Graph<Weighted> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i = self.edges();
        for (x, y, w) in i {
            writeln!(f, "{x}-({})->{y}", w.0)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static TEST_GRAPH_UNWEIGHTED: Lazy<Graph<Unweighted>> = Lazy::new(|| {
        Graph::new(vec![
            vec![
                (1, Unweighted(())),
                (2, Unweighted(())),
                (5, Unweighted(())),
            ], // 0
            vec![(0, Unweighted(())), (5, Unweighted(()))], // 1
            vec![(0, Unweighted(()))],                      // 2
            vec![(4, Unweighted(()))],                      // 3
            vec![(3, Unweighted(()))],                      // 4
            vec![(0, Unweighted(()))],                      // 5
        ])
    });

    static TEST_GRAPH_WEIGHTED: Lazy<Graph<Weighted>> = Lazy::new(|| {
        Graph::new(vec![
            vec![(1, Weighted(4)), (2, Weighted(1))], // 0
            vec![(3, Weighted(1)), (4, Weighted(7))], // 1
            vec![(1, Weighted(2)), (3, Weighted(5)), (5, Weighted(8))], // 2
            vec![(6, Weighted(3))],                   // 3
            vec![(6, Weighted(2)), (7, Weighted(3))], // 4
            vec![(4, Weighted(2)), (8, Weighted(6))], // 5
            vec![(9, Weighted(4))],                   // 6
            vec![(6, Weighted(3)), (9, Weighted(2))], // 7
            vec![(7, Weighted(1)), (9, Weighted(8))], // 8
            vec![(5, Weighted(1))],                   // 9
            // island
            vec![(11, Weighted(3))],  // 10
            vec![(12, Weighted(4))],  // 11
            vec![(13, Weighted(2))],  // 12
            vec![(10, Weighted(10))], // 13
            vec![(12, Weighted(1))],  // 14
        ])
    });

    #[test]
    fn random_gen_weights_gr_zero() {
        let g: Graph<Weighted> = Graph::random_graph(10, 0.5, true);
        for (_, _, w) in g.edges() {
            assert!(*w > Weighted(0))
        }
    }

    #[test]
    fn random_gen_undirected_weight_simmetry() {
        let g: Graph<Weighted> = Graph::random_graph(10, 0.5, false);
        let edges: Vec<_> = g.edges().collect();
        for &(u, v, w) in &edges {
            if let Some(&(_, _, w1)) = edges.iter().find(|&&(u1, v1, _)| u1 == v && v1 == u) {
                assert!(
                    w == w1,
                    "Symmetric edges {u}-({})->{v} and {v}-({})->{u} have non-symmetric weights.",
                    w.0,
                    w1.0
                )
            } else {
                panic!(
                    "Unable to find symmetric edge {v}-(w)->{u} for {u}-(w)->{v} in an undirected graph."
                );
            }
        }
    }

    #[test]
    fn basic_bfs_test() {
        let mut bfs_result_start_from_0 = TEST_GRAPH_UNWEIGHTED
            .bfs(0)
            .expect("bfs(0) resulted in an error unexpectedly");
        bfs_result_start_from_0.sort();
        let bfs_expected_result_start_from_0 = vec![0, 1, 2, 5];
        assert_eq!(bfs_result_start_from_0, bfs_expected_result_start_from_0);
        let mut bfs_result_start_from_4 = TEST_GRAPH_UNWEIGHTED
            .bfs(4)
            .expect("bfs(4) resulted in an error unexpectedly");
        bfs_result_start_from_4.sort();
        let bfs_expected_result_start_from_4 = vec![3, 4];
        assert_eq!(bfs_result_start_from_4, bfs_expected_result_start_from_4);
        let bfs_result_start_from_6 = TEST_GRAPH_UNWEIGHTED.bfs(6);
        assert!(matches!(
            bfs_result_start_from_6,
            Err(GraphError::OutOfBoundsNode { node: 6 })
        ));
    }

    #[test]
    fn basic_dfs_test() {
        let mut dfs_result_start_from_0 = TEST_GRAPH_UNWEIGHTED
            .dfs(0)
            .expect("dfs(0) resulted in an error unexpectedly");
        dfs_result_start_from_0.sort();
        let dfs_expected_result_start_from_0 = vec![0, 1, 2, 5];
        assert_eq!(dfs_result_start_from_0, dfs_expected_result_start_from_0);
        let mut dfs_result_start_from_4 = TEST_GRAPH_UNWEIGHTED
            .dfs(4)
            .expect("dfs(4) resulted in an error unexpectedly");
        dfs_result_start_from_4.sort();
        let dfs_expected_result_start_from_4 = vec![3, 4];
        assert_eq!(dfs_result_start_from_4, dfs_expected_result_start_from_4);
        let dfs_result_start_from_6 = TEST_GRAPH_UNWEIGHTED.dfs(6);
        assert!(matches!(
            dfs_result_start_from_6,
            Err(GraphError::OutOfBoundsNode { node: 6 })
        ));
    }

    #[test]
    fn bfs_dfs_equal_test() {
        let g: Graph<Weighted> = Graph::random_graph(20, 0.5, true);
        let mut bfs_result = g.bfs(0).expect("bfs(0) returned None unexpectedly");
        bfs_result.sort();
        let mut dfs_result = g.dfs(0).expect("bfs(0) returned None unexpectedly");
        dfs_result.sort();
        assert_eq!(bfs_result, dfs_result);
    }

    #[test]
    fn basic_dijkstra_test() {
        let dijkstra_result_start_from_0 = TEST_GRAPH_WEIGHTED
            .dijkstra(0)
            .expect("dijkstra(0) returned None unexpectedly");
        let dijkstra_expected_result_start_from_0 = [
            Some(0),  // 0
            Some(3),  // 1
            Some(1),  // 2
            Some(4),  // 3
            Some(10), // 4
            Some(9),  // 5
            Some(7),  // 6
            Some(13), // 7
            Some(15), // 8
            Some(11), // 9
            None,     // 10
            None,     // 11
            None,     // 12
            None,     // 13
            None,     // 14
        ];
        assert_eq!(
            dijkstra_result_start_from_0,
            dijkstra_expected_result_start_from_0
        );
        let dijkstra_result_start_from_10 = TEST_GRAPH_WEIGHTED
            .dijkstra(10)
            .expect("dijkstra(10) returned None unexpectedly");
        let dijkstra_expected_result_start_from_10 = [
            None,    // 0
            None,    // 1
            None,    // 2
            None,    // 3
            None,    // 4
            None,    // 5
            None,    // 6
            None,    // 7
            None,    // 8
            None,    // 9
            Some(0), // 10
            Some(3), // 11
            Some(7), // 12
            Some(9), // 13
            None,    // 14
        ];
        assert_eq!(
            dijkstra_result_start_from_10,
            dijkstra_expected_result_start_from_10
        );
        let dijkstra_result_start_from_15 = TEST_GRAPH_WEIGHTED.dijkstra(15);
        assert!(matches!(
            dijkstra_result_start_from_15,
            Err(GraphError::OutOfBoundsNode { node: 15 })
        ));
    }
}
