# Comtesse

Utility crate to handle common tasks that require graphs

```rs
use comtesse::{unweighted::Unweighted, HasEdge};

let mut graph = Unweighted::new();
// insert the numbers 1 to 10 as vertices
for i in 1..=10 {
    graph.add_vertex(i);
}
assert_eq!(graph.size(), 10);

// construct a graph satisfying the following condition
// there exists an edge (u, v) if the condition holds
graph.construct_edges_from(|&u, &v| u != v && (u + v) % 10 == 0);

// (1, 9) should be an edge, since (1 + 9) % 10 == 0
assert!(graph.has_edge(
    graph.get_vertex(1).unwrap(),
    graph.get_vertex(9).unwrap()
));
```

## Algorithms

### Shortest Path in Unweighted Graphs

```rs
let mut graph: comtesse::unweighted::Unweighted<_> = ('a'..='f').collect();
graph.construct_edges_from(|&u, &v| {
    matches!(
        (u, v),
        ('f', 'd')
            | ('c', 'a')
            | ('b', 'f')
            | ('b', 'e')
            | ('a', 'b')
            | ('d', 'e')
            | ('e', 'c')
    )
});

let a = graph.get_vertex('a').unwrap();
let d = graph.get_vertex('d').unwrap();

let path_ad = graph.shortest_path_unweighted(a, d);
assert_eq!(
    path_ad,
    ['a', 'b', 'f', 'd']
        .iter()
        .map(|&i| graph.get_vertex(i))
        .collect(),
);
```
