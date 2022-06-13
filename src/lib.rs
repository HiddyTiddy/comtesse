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

use std::{
    borrow::Cow,
    fmt::{Debug, Formatter},
};

pub struct Graph<V> {
    vertices: Vec<V>,
    edges: Vec<Vec<Handle>>,
}

mod graph_macro;

/// Handle to Vertices in the graoh
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Handle(usize);

// TODO: generic over V, E
// -> V: Vertex type
// -> E: Edge type
impl<V> Graph<V> {
    /// Constructs a new, empty `Graph<V>`
    pub fn new() -> Self {
        Graph {
            edges: vec![],
            vertices: vec![],
        }
    }

    /// Constructs a new, empty `Graph<V>` with capacity `size`
    ///
    /// The adjacency list will not reallocate if the number of vertices does not exceed `size`
    pub fn new_with_size(size: usize) -> Self {
        Graph {
            edges: Vec::with_capacity(size),
            vertices: Vec::with_capacity(size),
        }
    }

    /// Adds vertex with given `value` to graph. This returns a handle to the inserted element
    pub fn add_vertex(&mut self, value: V) -> Handle {
        let handle = self.vertices.len();
        self.vertices.push(value);
        self.edges.push(Vec::new());
        Handle(handle)
    }

    /// Connects two vertices, as given by `from` and `to`
    pub fn add_edge(&mut self, from: Handle, to: Handle) {
        let from = from.0;
        self.edges[from].push(to);
    }

    /// Returns the number of vertices in the graph.
    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.edges.iter().map(|elem| elem.len()).sum()
    }

    /// Constructs edges that satisfy the given `condition`
    pub fn construct_edges_from<F>(&mut self, condition: F)
    where
        F: Fn(&V, &V) -> bool,
    {
        for u in 0..self.vertices.len() {
            for v in 0..self.vertices.len() {
                if condition(&self.vertices[u], &self.vertices[v]) {
                    self.add_edge(Handle(u), Handle(v))
                }
            }
        }
    }

    /// Returns whether the edge starting at `from` and going to `to` exists in the graph
    pub fn edge_exists(&self, from: Handle, to: Handle) -> bool {
        let from = from.0;
        self.edges[from].iter().any(|&idx| idx == to)
    }
}

impl<V> Default for Graph<V> {
    fn default() -> Self {
        Self::new()
    }
}

fn make_safer(input: &str) -> Cow<'_, str> {
    if let Some(ok_until) = input.find(|ch| ch == '"') {
        let mut out = String::from(&input[..ok_until]);
        out.reserve(input.len() - ok_until);
        let rest = input[ok_until..].chars();
        for ch in rest {
            match ch {
                '"' => out.push_str(r#"\""#),
                _ => out.push(ch),
            }
        }
        Cow::Owned(out)
    } else {
        Cow::Borrowed(input)
    }
}

impl<V> Debug for Graph<V>
where
    V: Debug,
{
    /// formats to a graphviz (dot) compatible *thing*
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("digraph {\n")?;
        for vertex in &self.vertices {
            // TODO: vertex:? could inject stuff
            let vertex_str = format!("{vertex:?}");
            let vertex_str = make_safer(&vertex_str);
            f.write_fmt(format_args!("  \"{}\";\n", vertex_str))?;
        }

        for (from, edge) in self.edges.iter().enumerate() {
            let from = &self.vertices[from];
            let from = format!("{from:?}");
            let from = make_safer(&from);

            for &to in edge {
                let to = &self.vertices[to.0];
                let to = format!("{to:?}");
                let to = make_safer(&to);

                f.write_fmt(format_args!("  \"{from}\" -> \"{to}\";\n"))?;
            }
        }
        f.write_str("}")?;

        Ok(())
    }
}

impl<V> Graph<V>
where
    V: Eq,
{
    pub fn get_vertex(&self, vertex_value: V) -> Option<Handle> {
        self.vertices
            .iter()
            .enumerate()
            .find(|(_, vertex)| **vertex == vertex_value)
            .map(|(i, _)| Handle(i))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::Graph;

    fn dump<V>(graph: &Graph<V>)
    where
        V: std::fmt::Debug,
    {
        let graph = format!("{graph:?}");
        std::fs::File::create("dump.dot")
            .unwrap()
            .write_all(graph.as_bytes())
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

        dump(&graph);
        assert!(graph.edge_exists(two, six));
        assert!(!graph.edge_exists(two, seven));
    }
}
