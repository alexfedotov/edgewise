## edgewise

**edgewise** is a lightweight and ergonomic Rust 🦀 library for working with graphs.

It provides:
- A simple adjacency-list graph structure
- Random graph generation ((un)weighted, (un)directed)
- Breadth-First Search (BFS)
- Depth-First Search (DFS)
- Dijkstra’s shortest-path algorithm

### Example

```rust
use edgewise::{Graph, Weighted};

let g = Graph::new(vec![
    vec![(1, Weighted(4)), (2, Weighted(1))],
    vec![(3, Weighted(1))],
    vec![(1, Weighted(2))],
    vec![],
]);

let distances = g.dijkstra(0)?;
println!("{:?}", distances);
```

### Installation

```toml
[dependencies]
edgewise = "0.1"
```

### License

MIT or Apache-2.0