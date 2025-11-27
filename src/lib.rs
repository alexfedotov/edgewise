use rand::{Rng, rngs::ThreadRng};
use std::collections::VecDeque;
use std::fmt;

/// A graph represented as a vector of vectors, which
/// models an adjacency list.
///
/// The index of each element in the outer vector corresponds to a node.
/// Each inner vector contains tuples `(n, w)` representing outgoing edges:
/// `n` is the target node, and `w` is the edge weight. To model an
/// unweighted graph, use the unit type `()` as the weight.
///
/// # Examples
/// ```rust
///
/// use edgewise::Graph;
///
/// // Weighted directed graph
/// let weighted: Graph<u32> = Graph::new(vec![
///         vec![(1, 1), (2, 3)], // edges from node 0
///         vec![(2, 0)],         // edges from node 1
///         vec![(0, 5)],         // edges from node 2
///     ]);
///
/// // Unweighted undirected graph
/// let unweighted: Graph<()> = Graph::new(vec![
///         vec![(1, ())],        // edge 0 -> 1
///         vec![(0, ())],        // edge 1 -> 0
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

    pub fn bfs(&self, starting_node: u32) -> Vec<u32> {
        if (starting_node as usize) >= self.graph.len() {
            panic!("Node {starting_node} does not exist in the graph.")
        }
        let mut nodes_left_to_process: VecDeque<u32> = VecDeque::new();
        let mut nodes_visited_lookup: Vec<bool> = vec![false; self.graph.len()];
        let mut nodes_visited: Vec<u32> = Vec::new();
        nodes_left_to_process.push_back(starting_node);
        nodes_visited_lookup[starting_node as usize] = true;
        nodes_visited.push(starting_node);
        while !nodes_left_to_process.is_empty() {
            if let Some(node_to_process) = nodes_left_to_process.pop_front() {
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
        }
        nodes_visited
    }

    pub fn dfs(&self, starting_node: u32) -> Vec<u32> {
        if (starting_node as usize) >= self.graph.len() {
            panic!("Node {starting_node} does not exist in the graph.")
        }
        let mut nodes_left_to_process: VecDeque<u32> = VecDeque::new();
        let mut nodes_visited_lookup: Vec<bool> = vec![false; self.graph.len()];
        let mut nodes_visited: Vec<u32> = Vec::new();
        nodes_left_to_process.push_back(starting_node);
        nodes_visited_lookup[starting_node as usize] = true;
        nodes_visited.push(starting_node);
        while !nodes_left_to_process.is_empty() {
            let mut found_unvisited = false;
            if let Some(&node_to_process) = nodes_left_to_process.back() {
                if let Some(neighbours_of_node) = self.graph.get(node_to_process as usize) {
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
        }
        nodes_visited
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

impl Graph<u32> {
    pub fn dijkstra(&self, starting_node: u32) -> Vec<Option<u32>> {
        if (starting_node as usize) >= self.graph.len() {
            panic!("Node {starting_node} does not exist in the graph.")
        }
        let mut nodes_distance: Vec<Option<u32>> = vec![None; self.graph.len()];
        let mut nodes_visited: Vec<bool> = vec![false; self.graph.len()];
        nodes_distance[starting_node as usize] = Some(0);
        loop {
            if let Some((current_node, current_distance)) = (0..nodes_visited.len())
                .filter(|&i| !nodes_visited[i])
                .filter_map(|i| nodes_distance[i].map(|d| (i, d)))
                .min_by_key(|&(_, d)| d)
            {
                if let Some(neighbors) = self.graph.get(current_node as usize) {
                    for &(neighbor_node, neighbor_weight) in neighbors {
                        if let Some(new_distance) = current_distance.checked_add(neighbor_weight) {
                            if let Some(neighbor_distance) = nodes_distance[neighbor_node as usize]
                            {
                                if new_distance < neighbor_distance {
                                    nodes_distance[neighbor_node as usize] = Some(new_distance)
                                }
                            } else {
                                nodes_distance[neighbor_node as usize] = Some(new_distance)
                            }
                        } else {
                            panic!(
                                "Computation of new distance for node {current_node} causes u32 overflow."
                            )
                        }
                    }
                }
                nodes_visited[current_node as usize] = true
            } else {
                break;
            }
        }
        nodes_distance
    }
}

trait InsertEdge: Sized {
    fn insert_edge(g: &mut Graph<Self>, rng: &mut ThreadRng, i: u32, j: u32, is_directed: bool);
}

impl InsertEdge for () {
    fn insert_edge(g: &mut Graph<()>, _rng: &mut ThreadRng, i: u32, j: u32, is_directed: bool) {
        let u = g
            .graph
            .get_mut(i as usize)
            .expect(&format!("Node {i} is out of bounds"));
        u.push((j, ()));
        if !is_directed {
            let v = g
                .graph
                .get_mut(j as usize)
                .expect(&format!("Node {j} is out of bounds"));
            v.push((i, ()));
        }
    }
}

impl InsertEdge for u32 {
    fn insert_edge(g: &mut Graph<u32>, rng: &mut ThreadRng, i: u32, j: u32, is_directed: bool) {
        let w: u32 = rng.random_range(1..=10);
        let u = g
            .graph
            .get_mut(i as usize)
            .expect(&format!("Node {i} is out of bounds"));
        u.push((j, w));
        if !is_directed {
            let v = g
                .graph
                .get_mut(j as usize)
                .expect(&format!("Node {j} is out of bounds"));
            v.push((i, w));
        }
    }
}

impl fmt::Display for Graph<()> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i = self.edges();
        for (x, y, _) in i {
            writeln!(f, "{x}->{y}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Graph<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i = self.edges();
        for (x, y, w) in i {
            writeln!(f, "{x}-({w})->{y}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static TEST_GRAPH: Lazy<Graph<()>> = Lazy::new(|| {
        Graph::new(vec![
            vec![(1, ()), (2, ()), (5, ())],
            vec![(0, ()), (5, ())],
            vec![(0, ())],
            vec![(4, ())],
            vec![(3, ())],
            vec![(0, ())],
        ])
    });

    #[test]
    fn random_gen_weights_gr_zero() {
        let g: Graph<u32> = Graph::random_graph(10, 0.5, true);
        for (_, _, w) in g.edges() {
            assert!(*w > 0)
        }
    }

    #[test]
    fn random_gen_undirected_weigh_simmetry() {
        let g: Graph<u32> = Graph::random_graph(10, 0.5, false);
        let edges: Vec<_> = g.edges().collect();
        for &(u, v, w) in &edges {
            if let Some(&(_, _, w1)) = edges.iter().find(|&&(u1, v1, _)| u1 == v && v1 == u) {
                assert!(
                    w == w1,
                    "Symmetric edges {u}-({w})->{v} and {v}-({w1})->{u} have non-symmetric weights."
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
        let mut bfs_result_start_from_0 = TEST_GRAPH.bfs(0).clone();
        bfs_result_start_from_0.sort();
        let bfs_expected_result_start_from_0 = vec![0, 1, 2, 5];
        assert_eq!(bfs_result_start_from_0, bfs_expected_result_start_from_0);
        let mut bfs_result_start_from_4 = TEST_GRAPH.bfs(4).clone();
        bfs_result_start_from_4.sort();
        let bfs_expected_result_start_from_4 = vec![3, 4];
        assert_eq!(bfs_result_start_from_4, bfs_expected_result_start_from_4);
    }

    #[test]
    fn basic_dfs_test() {
        let mut dfs_result_start_from_0 = TEST_GRAPH.dfs(0).clone();
        dfs_result_start_from_0.sort();
        let dfs_expected_result_start_from_0 = vec![0, 1, 2, 5];
        assert_eq!(dfs_result_start_from_0, dfs_expected_result_start_from_0);
        let mut dfs_result_start_from_4 = TEST_GRAPH.dfs(4).clone();
        dfs_result_start_from_4.sort();
        let dfs_expected_result_start_from_4 = vec![3, 4];
        assert_eq!(dfs_result_start_from_4, dfs_expected_result_start_from_4);
    }

    #[test]
    fn bfs_dfs_equal_test() {
        let g: Graph<u32> = Graph::random_graph(20, 0.5, true);
        let mut bfs_result = g.bfs(0);
        bfs_result.sort();
        let mut dfs_result = g.dfs(0);
        dfs_result.sort();
        assert_eq!(bfs_result, dfs_result);
    }
}
