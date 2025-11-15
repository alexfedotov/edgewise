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

    pub fn random_graph(
        &self,
        _num_nodes: u32,
        _probability: f64,
        _is_directed: Option<bool>,
        _is_weighted: Option<bool>,
    ) -> Self {
        Self::new(vec![])
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
    #[test]
    fn stub() {
        assert!(true);
    }
}
