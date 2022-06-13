use std::iter::repeat_with;

pub struct Graph<V, E> {
    pub(crate) vertices: Vec<V>,
    pub(crate) edges: Vec<Vec<E>>,
}

/// Handle to Vertices in the graph
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Handle(pub(crate) usize);

impl<V, E> Graph<V, E> {
    /// Constructs a new, empty `Graph<V>`
    pub fn new() -> Self {
        Graph {
            vertices: vec![],
            edges: vec![],
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

    /// Returns the number of vertices in the graph.
    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.edges.iter().map(|elem| elem.len()).sum()
    }
}

impl<V, E> FromIterator<V> for Graph<V, E> {
    /// creates a new graph, taking the vertices from the iterator
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let vertices: Vec<V> = iter.into_iter().collect();
        let size = vertices.len();
        Graph {
            vertices,
            edges: repeat_with(Vec::new).take(size).collect(),
        }
    }
}

impl<V, E> Default for Graph<V, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V, E> Graph<V, E>
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
