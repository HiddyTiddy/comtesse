//! A weighted Graph, containing vertices of type `V`

use crate::graph::{Graph, Handle};

#[derive(Clone, Copy)]
pub struct Connection<W>
where
    W: Copy,
{
    to: Handle,
    weight: W,
}

impl<W> Connection<W>
where
    W: Copy,
{
    pub fn weight(&self) -> W {
        self.weight
    }

    pub fn to(&self) -> Handle {
        self.to
    }
}

pub type Weighted<V, W> = Graph<V, Connection<W>>;

impl<V, W> Weighted<V, W>
where
    W: Copy,
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

    /// returns a list of neighbors of `vertex` in the graph
    pub fn neighbors(&self, vertex: Handle) -> &[Connection<W>] {
        let vertex = vertex.0;
        &self.edges[vertex]
    }
}
