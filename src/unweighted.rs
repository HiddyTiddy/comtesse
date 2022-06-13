//! An unweighted graph, containing elements of type `V`

use std::fmt::{Debug, Write};

use crate::{
    graph::{Graph, Handle},
    make_safer, DumpGraphviz,
};

pub type Unweighted<V> = Graph<V, Handle>;

impl<V> Unweighted<V> {
    /// Connects two vertices, as given by `from` and `to`
    pub fn add_edge(&mut self, from: Handle, to: Handle) {
        let from = from.0;
        self.edges[from].push(to);
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

    /// returns a list of neighbors of `vertex` in the graph
    pub fn neighbors(&self, vertex: Handle) -> &[Handle] {
        let vertex = vertex.0;
        &self.edges[vertex]
    }
}

impl<V: Debug> DumpGraphviz for Unweighted<V> {
    fn dump(&self, output: &mut dyn Write) -> Result<(), std::fmt::Error> {
        writeln!(output, "digraph {{")?;
        for vertex in &self.vertices {
            // TODO: vertex:? could inject stuff
            let vertex_str = format!("{vertex:?}");
            let vertex_str = make_safer(&vertex_str);
            writeln!(output, "  \"{}\";", vertex_str)?;
        }

        for (from, edge) in self.edges.iter().enumerate() {
            let from = &self.vertices[from];
            let from = format!("{from:?}");
            let from = make_safer(&from);

            for &to in edge {
                let to = &self.vertices[to.0];
                let to = format!("{to:?}");
                let to = make_safer(&to);

                writeln!(output, "  \"{from}\" -> \"{to}\";\n")?;
            }
        }
        writeln!(output, "}}")?;

        Ok(())
    }
}
