//! A weighted Graph, containing vertices of type `V`

use std::fmt::Write;

use crate::{
    graph::{Graph, Handle},
    make_safer, DumpGraphviz,
};

/// A Connection between two vertices, also called 'Edge'.
/// The type `W` can be any numeric type (any type implementing num_traits::Num) and should be efficiently Copy-able
///
/// This struct is not intended to be constructed manually.
#[derive(Clone, Copy)]
pub struct Connection<W>
where
    W: num_traits::Num + Copy,
{
    pub to: Handle,
    pub weight: W,
}

pub type Weighted<V, W> = Graph<V, Connection<W>>;

impl<V, W> Weighted<V, W>
where
    W: num_traits::Num + Copy,
{
    /// Connects two vertices, as given by `from` and `to` with an edge of weight `weight`
    pub fn add_edge(&mut self, from: Handle, to: Handle, weight: W) {
        let from = from.0;
        self.edges[from].push(Connection { to, weight });
    }

    /// Constructs edges that satisfy the given `condition`.
    ///
    /// the condition function should return `None` if the condition is not met
    /// and `Some(weight)` if the condition is met
    pub fn construct_edges_from<F>(&mut self, condition: F)
    where
        F: Fn(&V, &V) -> Option<W>,
    {
        for u in 0..self.vertices.len() {
            for v in 0..self.vertices.len() {
                if let Some(weight) = condition(&self.vertices[u], &self.vertices[v]) {
                    self.add_edge(Handle(u), Handle(v), weight)
                }
            }
        }
    }

    /// Returns whether the edge starting at `from` and going to `to` exists in the graph
    pub fn edge_exists(&self, from: Handle, to: Handle) -> bool {
        let from = from.0;
        self.edges[from]
            .iter()
            .any(|&Connection { to: idx, .. }| idx == to)
    }

    pub fn get_edge(&self, from: Handle, to: Handle) -> Option<W> {
        let from = from.0;
        self.edges[from]
            .iter()
            .find(|&Connection { to: idx, .. }| *idx == to)
            .map(|Connection { weight, .. }| *weight)
    }

    /// returns a list of neighbors of `vertex` in the graph
    pub fn neighbors(&self, vertex: Handle) -> &[Connection<W>] {
        let vertex = vertex.0;
        &self.edges[vertex]
    }
}

impl<V, W> DumpGraphviz for Weighted<V, W>
where
    V: std::fmt::Debug,
    W: std::fmt::Debug + num_traits::Num + Copy,
{
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

            for to in edge {
                let (to, weight) = (&self.vertices[to.to.0], to.weight);
                let to = format!("{to:?}");
                let to = make_safer(&to);

                writeln!(output, "  \"{from}\" -> \"{to}\" [label=\"{weight:?}\"];\n")?;
            }
        }
        writeln!(output, "}}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Weighted;

    #[test]
    fn construct_weighted() {
        let mut graph: Weighted<_, f32> = ('a'..='f').collect();
        graph.construct_edges_from(|&from, &to| match (from, to) {
            ('a', 'b') => Some(9.0),
            ('a', 'd') => Some(8.0),
            ('b', 'c') => Some(1.0),
            ('b', 'e') => Some(3.0),
            ('c', 'e') => Some(1.0),
            ('c', 'd') => Some(5.0),
            ('d', 'f') => Some(8.0),
            ('e', 'f') => Some(6.0),
            _ => None,
        });

        let a = graph.get_vertex('a').expect("'a' is in V");
        let b = graph.get_vertex('b').expect("'b' is in V");

        let weight = graph.get_edge(a, b).expect("'a' -> 'b' exists");
        assert!((weight - 9.0).abs() < 0.1);

        crate::tests::dump(&graph);
    }
}
