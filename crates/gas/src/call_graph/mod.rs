use svm_program::{FuncIndex, ProgramError};

use std::ops::{Deref, DerefMut};

use crate::{Graph, GraphCycles, NodeLabel};

mod builder;
pub use builder::CallGraphBuilder;

/// A `CallGraph` is a `Graph` where its nodes represent functions and its edges represent for possible `call`s
/// between functions as appears in the original code.
use super::graph;

/// A `CallGraph` is a `Graph` where the `Node` are `Program` functions
/// and each `call` between a function `origin` to another `target` function results
/// in an `Edge` in the derived `CallGraph`
pub struct CallGraph<L>
where
    L: NodeLabel,
{
    inner: Graph<L, ()>,
}

impl<L> CallGraph<L>
where
    L: NodeLabel,
{
    /// Looks for cycles in the `CallGraph`.
    ///
    /// If there are no cycles - the graph is a DAG and we return `Ok())`
    /// Otherwise, an error is returned signaling there is at least one cycle within the `CallGraph`
    #[must_use]
    pub fn find_cycles(&self, return_cycles: bool) -> GraphCycles<L> {
        let result = graph::try_topological_sort::<L, (), CallGraph<L>>(self, return_cycles);

        match result {
            Ok(_sorted) => GraphCycles::NoCycles,
            Err(cycle) => cycle,
        }
    }
}

impl<L> Deref for CallGraph<L>
where
    L: NodeLabel,
{
    type Target = Graph<L, ()>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    macro_rules! call_graph {
        ( $( $source:expr => $dest:expr ),* ) => {{
            let mut builder = CallGraphBuilder::<&str>::new();

            $( builder.add_call($source, $dest); )*

            builder.build()
        }};
    }

    macro_rules! assert_cycles {
        ($graph:expr, $expected:expr) => {
            let return_cycles = true;

            let actual = $graph.find_cycles(return_cycles);

            let expected = GraphCycles::HasCycles($expected);

            assert_eq!(actual, expected);
        };
    }

    macro_rules! assert_no_cycles {
        ($g:expr) => {
            assert!(matches!($g.find_cycles(false), GraphCycles::NoCycles));
        };
    }

    #[test]
    fn call_graph_empty() {
        let g = call_graph! {};

        assert_no_cycles!(g);
    }

    #[test]
    fn call_graph_cycle_size_2() {
        let g = call_graph! {
            "a" => "b",
            "b" => "a"
        };

        assert_cycles!(g, vec!["a", "b", "a"]);
    }

    #[test]
    fn call_graph_cycle_size_3() {
        let g = call_graph! {
            "a" => "b",
            "b" => "c",
            "c" => "a",
            "c" => "d",
            "c" => "e"
        };

        assert_cycles!(g, vec!["a", "b", "c", "a"]);
    }

    #[test]
    fn call_graph_two_subgraphs_no_cycles() {
        let g = call_graph! {
            "a" => "b",
            "b" => "c",
            "c" => "d",

            "e" => "f",
            "f" => "g"
        };

        assert_no_cycles!(g);
    }

    #[test]
    fn call_graph_two_subgraphs_one_cycle() {
        let g = call_graph! {
            "a" => "b",
            "b" => "c",
            "c" => "d",

            "e" => "f",
            "f" => "g",
            "g" => "e"
        };

        assert_cycles!(g, vec!["e", "f", "g", "e"]);
    }
}
