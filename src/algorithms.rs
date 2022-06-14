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

#[cfg(test)]
mod tests {
    use crate::weighted::Weighted;

    #[test]
    fn is_connected() {
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

        assert!(graph.is_connected());

        graph.remove_edge(
            graph.get_vertex('a').unwrap(),
            graph.get_vertex('b').unwrap(),
        );
        graph.remove_edge(
            graph.get_vertex('a').unwrap(),
            graph.get_vertex('d').unwrap(),
        );

        assert!(!graph.is_connected());
    }
}
