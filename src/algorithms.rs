use crate::{graph::Graph, HasEdge};

impl<V, E> Graph<V, E>
where
    Graph<V, E>: HasEdge,
{
    /// Returns whether the given graph is connected
    pub fn is_connected(&self) -> bool {
        todo!()
    }
}
