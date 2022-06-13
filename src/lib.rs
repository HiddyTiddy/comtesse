use std::fmt::{Debug, Formatter};

pub struct Graph<V> {
    vertices: Vec<V>,
    edges: Vec<Vec<Handle>>,
}

type Handle = usize;

impl<V> Graph<V> {
    pub fn new() -> Self {
        Graph {
            edges: vec![],
            vertices: vec![],
        }
    }

    pub fn new_with_size(size: usize) -> Self {
        Graph {
            edges: Vec::with_capacity(size),
            vertices: Vec::with_capacity(size),
        }
    }

    pub fn add_vertex(&mut self, value: V) -> Handle {
        let handle = self.vertices.len();
        self.vertices.push(value);
        self.edges.push(Vec::new());
        handle
    }

    pub fn add_edge(&mut self, from: Handle, to: Handle) {
        self.edges[from].push(to);
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edges.iter().map(|elem| elem.len()).sum()
    }
}

impl<V> Default for Graph<V> {
    fn default() -> Self {
        Self::new()
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
            f.write_fmt(format_args!("  \"{:?}\";\n", vertex))?;
        }

        for (from, edge) in self.edges.iter().enumerate() {
            let from = &self.vertices[from];
            for &to in edge {
                let to = &self.vertices[to];
                f.write_fmt(format_args!("  \"{from:?}\" -> \"{to:?}\";\n"))?;
            }
        }
        f.write_str("}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::Graph;

    fn dump<V>(graph: Graph<V>)
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

        dump(graph);
    }
}
