//! various algorithms on graphs

use crate::{
    graph::{Graph, Handle},
    unweighted::Unweighted,
    HasEdge,
};

impl<V, E> Graph<V, E>
where
    Graph<V, E>: HasEdge,
{
    /// Returns whether the given graph is connected.
    ///
    /// ## Running Time
    /// This algorithm has a running time of `O(n + m)` where `n` is the number of vertices and `m` is the number of edges
    pub fn is_connected(&self) -> bool {
        if self.vertices.is_empty() {
            return true;
        }

        let mut zhk = vec![None; self.vertices.len()];
        let mut current_zhk = 1usize; // can be NonZeroUsize
        let mut zhk_connections = vec![];

        for i in 0..self.vertices.len() {
            if zhk[i].is_none() {
                let mut stack = vec![i];
                let mut connections = vec![];

                while let Some(top) = stack.pop() {
                    zhk[top] = Some(current_zhk);

                    for Handle(neighbor) in self.connected_neighbors(Handle(top)) {
                        match zhk[neighbor] {
                            None => stack.push(neighbor),
                            Some(z) if z == current_zhk => {}
                            Some(z) => connections.push(z),
                        }
                    }
                }
                eprintln!("{current_zhk} {connections:?}");

                zhk_connections.push(connections);
                current_zhk += 1;
            }
        }

        let mut zhk_graph = (1..current_zhk).collect::<Unweighted<_>>();
        zhk_graph.construct_edges_from(|&a, &b| {
            zhk_connections[b - 1].iter().any(|&elem| elem == a)
                || zhk_connections[a - 1].iter().any(|&elem| elem == b)
        });

        let mut seen = vec![false; zhk_graph.size()];
        let mut stack = vec![0];
        while let Some(top) = stack.pop() {
            seen[top] = true;
            for Handle(neighbor) in zhk_graph.connected_neighbors(Handle(top)) {
                if !seen[neighbor] {
                    stack.push(neighbor)
                }
            }
        }

        seen.iter().all(|&seen| seen)
    }
}

#[cfg(test)]
mod tests {
    use crate::{unweighted::Unweighted, weighted::Weighted};

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
        // crate::tests::dump(&graph);

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

    #[test]
    fn connected1() {
        let mut graph: Unweighted<_> = ('a'..='c').collect();
        graph.construct_edges_from(|&from, &to| matches!((from, to), ('b', 'a') | ('c', 'a')));
        assert!(graph.is_connected());
    }
}
