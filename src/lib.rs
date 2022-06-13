//! # Graph Utils
//!
//! Utility crate to handle common tasks that require graphs
//!
//! ```
//! use comtesse::Graph;
//!
//! let mut graph = Graph::new();
//! // insert the numbers 1 to 10 as vertices
//! for i in 1..=10 {
//!     graph.add_vertex(i);
//! }
//! assert_eq!(graph.size(), 10);
//!
//! // construct a graph satisfying the following condition
//! // there exists an edge (u, v) if the condition holds
//! graph.construct_edges_from(|&u, &v| u != v && (u + v) % 10 == 0);
//! ```

use std::fmt::Write;

pub mod graph;
pub mod unweighted;

trait DumpGraphviz {
    fn dump(&self, output: &mut dyn Write) -> Result<(), std::fmt::Error>;
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, io::Write};

    use crate::{graph::Graph, DumpGraphviz};

    fn dump<V, E>(graph: &Graph<V, E>)
    where
        V: std::fmt::Debug,
        Graph<V, E>: DumpGraphviz,
    {
        let mut graph_str = String::new();
        graph.dump(&mut graph_str);
        std::fs::File::create("dump.dot")
            .unwrap()
            .write_all(graph_str.as_bytes())
            .unwrap();
        std::process::Command::new("dot")
            .args(["-Tpng", "dump.dot", "-o", "dump.png"])
            .spawn()
            .expect("failed to run dot");
    }

    #[test]
    fn test_size() {
        let mut graph = Graph::new();
        let a = graph.add_vertex(1);
        let b = graph.add_vertex(2);
        let c = graph.add_vertex(3);
        let d = graph.add_vertex(4);

        graph.add_edge(a, b);
        graph.add_edge(c, a);
        graph.add_edge(d, c);

        assert_eq!(4, graph.size());
        assert_eq!(3, graph.num_edges());
    }

    #[test]
    fn construct() {
        let mut graph = Graph::new();
        for i in 1..=10 {
            graph.add_vertex(i);
        }

        graph.construct_edges_from(|&from, &to| to != from && to % from == 0);

        let two = graph.get_vertex(2).expect("2 is in 1..=10");
        let six = graph.get_vertex(6).expect("6 is in 1..=10");
        let seven = graph.get_vertex(7).expect("7 is in 1..=10");

        assert!(graph.edge_exists(two, six));
        assert!(!graph.edge_exists(two, seven));
    }

    #[test]
    fn dfs() {
        let graph = {
            let mut graph = Graph::new();
            for i in 2..=11 {
                graph.add_vertex(i);
            }
            assert_eq!(graph.size(), 10);

            graph.construct_edges_from(|&u, &v| u != v && u % v == 1);
            graph
        };

        let mut stack = vec![graph.get_vertex(8).expect("8 is in 2..=11")];
        let dest = graph.get_vertex(4).expect("4 is in 2..=11");

        let mut length = 0;
        let mut seen = HashSet::new();
        while let Some(top) = stack.pop() {
            length += 1;
            if top == dest {
                break;
            }

            for neighbor in graph.neighbors(top) {
                if !seen.contains(neighbor) {
                    stack.push(*neighbor);
                    seen.insert(neighbor);
                }
            }
        }

        assert!(length > 4);

        dump(&graph);
    }
}
