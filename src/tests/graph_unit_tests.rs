use crate::*;
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
