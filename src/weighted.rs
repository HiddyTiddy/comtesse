//! A weighted Graph, containing vertices of type `V`

use crate::graph::{Graph, Handle};

pub struct Connection<W> {
    to: Handle,
    weight: W,
}

pub type Weighted<V, W> = Graph<V, Connection<W>>;
