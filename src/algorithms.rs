//! various algorithms on graphs

use std::collections::VecDeque;

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

    /// Finds the shortest path between `start` and `end`.
    ///
    /// ```
    /// let mut graph: comtesse::unweighted::Unweighted<_> = ('a'..='h').collect();
    /// graph.construct_edges_from(|&u, &v| {
    ///     matches!(
    ///         (u, v),
    ///         ('f', 'd')
    ///             | ('d', 'h')
    ///             | ('c', 'g')
    ///             | ('c', 'a')
    ///             | ('b', 'f')
    ///             | ('b', 'e')
    ///             | ('a', 'b')
    ///             | ('e', 'h')
    ///             | ('d', 'g')
    ///             | ('d', 'e')
    ///             | ('e', 'c')
    ///     )
    /// });
    ///
    /// let a = graph.get_vertex('a').unwrap();
    /// let d = graph.get_vertex('d').unwrap();
    ///
    /// let path_ad = graph.shortest_path_unweighted(a, d);
    /// assert_eq!(
    ///     path_ad,
    ///     ['a', 'b', 'f', 'd']
    ///         .iter()
    ///         .map(|&i| graph.get_vertex(i))
    ///         .collect(),
    /// );
    /// ```
    ///
    /// # Running Time
    /// This algorithm has a running time of `O(n + m)` where `n` is the number of vertices and `m` is the number of edges
    pub fn shortest_path_unweighted(&self, start: Handle, end: Handle) -> Option<Vec<Handle>> {
        let start = start.0;
        let end = end.0;

        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut seen = vec![None; self.size()];

        while let Some(front) = queue.pop_front() {
            if front == end {
                break;
            }

            for Handle(neighbor) in self.connected_neighbors(Handle(front)) {
                if seen[neighbor].is_none() {
                    seen[neighbor] = Some(front);
                    queue.push_back(neighbor);
                }
            }
        }

        let mut path = vec![];
        let mut cur = end;
        while cur != start {
            path.push(Handle(cur));
            cur = seen[cur]?;
        }
        path.push(Handle(start));

        Some(path.iter().rev().copied().collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::{Graph, Handle},
        unweighted::Unweighted,
        weighted::Weighted,
    };

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

    fn assert_one_of_paths<V, E>(
        graph: &Graph<V, E>,
        actual: Option<Vec<Handle>>,
        expected: &[&[V]],
    ) where
        V: Eq + Clone,
    {
        if expected.is_empty() {
            return;
        }

        let actual = actual.unwrap();

        for &expected in expected {
            let expected = expected
                .iter()
                .map(|i| graph.get_vertex(i.clone()).unwrap())
                .collect::<Vec<_>>();
            if actual == expected {
                return;
            }
        }

        panic!("no path matched with {actual:?}");
    }

    fn make_graph() -> Unweighted<char> {
        let mut graph: Unweighted<_> = ('a'..='h').collect();
        graph.construct_edges_from(|&u, &v| {
            matches!(
                (u, v),
                ('f', 'd')
                    | ('d', 'h')
                    | ('c', 'g')
                    | ('c', 'a')
                    | ('b', 'f')
                    | ('b', 'e')
                    | ('a', 'b')
                    | ('e', 'h')
                    | ('d', 'g')
                    | ('d', 'e')
                    | ('e', 'c')
            )
        });
        graph
    }

    #[test]
    fn shortest_path() {
        let graph = make_graph();

        let a = graph.get_vertex('a').unwrap();
        let g = graph.get_vertex('g').unwrap();
        let d = graph.get_vertex('d').unwrap();

        let path_ag = graph.shortest_path_unweighted(a, g);
        assert_one_of_paths(
            &graph,
            path_ag,
            &[&['a', 'b', 'e', 'c', 'g'], &['a', 'b', 'f', 'd', 'g']],
        );

        let path_ad = graph.shortest_path_unweighted(a, d);
        assert_eq!(
            path_ad,
            ['a', 'b', 'f', 'd']
                .iter()
                .map(|&i| graph.get_vertex(i))
                .collect(),
        );
    }
}
