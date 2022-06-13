# Comtesse

Utility crate to handle common tasks that require graphs

warning! this crate is not yet complete and the API is yet to change.

```rs
use graph::Graph;

let mut graph = Graph::new();
// insert the numbers 1 to 10 as vertices
for i in 1..=10 {
    graph.add_vertex(i);
}
assert_eq!(graph.size(), 10);

// construct a graph satisfying the following condition
// there exists an edge (u, v) if the condition holds
graph.construct_edges_from(|&u, &v| u != v && (u + v) % 10 == 0);
```
