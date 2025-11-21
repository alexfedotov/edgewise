use rand::{Rng, rngs::ThreadRng};
use std::fmt;

/// A graph represented as a vector of vectors, which
/// itself models an adjacency list.
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

    pub fn bfs(&self) -> Vec<u32> {
        // Create an empty queue
        // Create an empty set visited
        // Add start to the queue
        // Add start to visited
        // While the queue is not empty:
        //     Remove the front element and call it u
        //     For each neighbor v of u in the graph:
        //         If v is not in visited:
        //             Add v to visited
        //             Enqueue v
        // Return visited (these are all reachable nodes)
        vec![42]
    }
}

#[allow(private_bounds)]
impl<W: InsertEdge> Graph<W> {
    fn insert_edge(&mut self, rng: &mut ThreadRng, i: u32, j: u32, is_directed: bool) -> &mut Self {
        W::insert_edge(self, rng, i, j, is_directed)
    }

    pub fn random_graph(
        num_nodes: u32,
        probability: f64,
        is_directed: bool,
        _is_weighted: bool,
    ) -> Self {
        let mut v: Vec<Vec<(u32, W)>> = Vec::new();
        v.resize_with(num_nodes as usize, Vec::new);
        let mut graph = Graph::new(v);
        let mut rng = rand::thread_rng();
        for i in 0..num_nodes {
            let z = if is_directed { 0 } else { i + 1 };
            for j in z..num_nodes {
                let r: f64 = rng.r#gen();
                if r < probability {
                    graph.insert_edge(&mut rng, i, j, is_directed);
                }
            }
        }
        graph
    }
}

trait InsertEdge: Sized {
    fn insert_edge<'a>(
        g: &'a mut Graph<Self>,
        rng: &mut ThreadRng,
        i: u32,
        j: u32,
        is_directed: bool,
    ) -> &'a mut Graph<Self>;
}

impl InsertEdge for () {
    fn insert_edge<'a>(
        g: &'a mut Graph<()>,
        _rng: &mut ThreadRng,
        i: u32,
        j: u32,
        is_directed: bool,
    ) -> &'a mut Graph<()> {
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
        g
    }
}

impl InsertEdge for u32 {
    fn insert_edge<'a>(
        g: &'a mut Graph<u32>,
        rng: &mut ThreadRng,
        i: u32,
        j: u32,
        is_directed: bool,
    ) -> &'a mut Graph<u32> {
        let w: u32 = rng.gen_range(1..=10);
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
        g
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

    #[test]
    fn random_gen_weights_gr_zero() {
        let g: Graph<u32> = Graph::random_graph(10, 0.5, true, true);
        for (_, _, w) in g.edges() {
            assert!(*w > 0)
        }
    }

    #[test]
    fn random_gen_undirected_weigh_simmetry() {
        let g: Graph<u32> = Graph::random_graph(10, 0.5, false, true);
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
}
