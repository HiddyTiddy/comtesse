//! An unweighted graph, containing elements of type `V`

use std::fmt::{Debug, Write};

use crate::{
    graph::{Graph, Handle},
    make_safer,
    weighted::Weighted,
    DumpGraphviz, HasEdge,
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

    /// returns a list of neighbors of `vertex` in the graph
    pub fn neighbors(&self, vertex: Handle) -> &[Handle] {
        let vertex = vertex.0;
        &self.edges[vertex]
    }

    /// Removes the edge going from `from` to `to`.
    ///
    /// ## Panics
    ///
    /// Panics if edge does not exist
    pub fn remove_edge(&mut self, from: Handle, to: Handle) {
        let to = self.edges[from.0]
            .iter()
            .enumerate()
            .find(|(_, &idx)| idx == to)
            .map(|(to, _)| to);
        let to = if let Some(to) = to {
            to
        } else {
            panic!("edge does not exist");
        };
        self.edges[from.0].swap_remove(to);
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

impl<V> HasEdge for Unweighted<V> {
    /// Returns whether the edge starting at `from` and going to `to` exists in the graph
    fn has_edge(&self, from: Handle, to: Handle) -> bool {
        let from = from.0;
        self.edges[from].iter().any(|&idx| idx == to)
    }

    fn connected_neighbors<'a>(&'a self, vertex: Handle) -> Box<dyn Iterator<Item = Handle> + 'a> {
        let vertex = vertex.0;
        Box::new(self.edges[vertex].iter().copied())
    }
}

impl<V, W> From<Weighted<V, W>> for Unweighted<V>
where
    W: Copy + num_traits::Num,
{
    /// creates a new graph based on `weighted`, ignoring all nonzero weights
    fn from(weighted: Weighted<V, W>) -> Self {
        let mut edges = Vec::with_capacity(weighted.vertices.len());
        for neighbors in weighted.edges {
            edges.push(
                neighbors
                    .iter()
                    .filter(|&elem| !W::is_zero(&elem.weight))
                    .map(|elem| elem.to)
                    .collect(),
            )
        }
        Unweighted {
            vertices: weighted.vertices,
            edges,
        }
    }
}

#[test]
fn from_weighted() {
    let mut graph: Weighted<_, f32> = ('a'..='f').collect();
    graph.construct_edges_from(|&from, &to| match (from, to) {
        ('a', 'b') => Some(9.0),
        ('a', 'd') => Some(8.0),
        ('b', 'c') => Some(0.0),
        ('b', 'e') => Some(3.0),
        ('c', 'e') => Some(1.0),
        ('c', 'd') => Some(5.0),
        ('d', 'f') => Some(8.0),
        ('e', 'f') => Some(6.0),
        _ => None,
    });

    let graph: Unweighted<_> = graph.into();
    assert!(graph.has_edge(
        graph.get_vertex('a').unwrap(),
        graph.get_vertex('d').unwrap()
    ));
    assert!(!graph.has_edge(
        graph.get_vertex('b').unwrap(),
        graph.get_vertex('c').unwrap()
    ));
}
