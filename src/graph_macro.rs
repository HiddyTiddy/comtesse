// // #[test]
// fn _graph_macro() {
//     let _graph: Graph<&str> = crate::graph! {
//         {"hello"} -> {"bye"};
//         {"hello"} -> {"no"} as A;
//         {"bye"} -> {"yes"} as B;
//     };

//     let _manually_constructed = {
//         let mut manually_constructed = Graph::new_with_size(4);
//         let unnamed_hello = manually_constructed.add_vertex("hello");
//         let unnamed_bye = manually_constructed.add_vertex("bye");
//         let named_a = manually_constructed.add_vertex("A");
//         let named_b = manually_constructed.add_vertex("B");

//         manually_constructed.add_edge(unnamed_hello, unnamed_bye);
//         manually_constructed.add_edge(unnamed_hello, named_a);
//         manually_constructed.add_edge(unnamed_bye, named_b);
//         manually_constructed
//     };
// }

// TODO
macro_rules! graph_inner {
    ($graph:ident; {$from:expr} -> {$to:expr}; $($rest:tt)*) => {{
        let from = $graph.add_vertex($from);
        let to = $graph.add_vertex($to);

        $graph.add_edge(from, to);

        graph_inner!($graph; $($rest)*);
    }};
    ($graph:ident;) => {};
}

#[macro_export]
macro_rules! graph {
    () => {crate::Graph::new()};
    ($($t:tt)*) => {{
        let mut graph = crate::Graph::new();

        graph_inner!(graph; $($t)*);

        graph

    }};
}

fn _for_cargo_expand() {
    let _g = graph!({"hi"} -> {"bye"};);
}
