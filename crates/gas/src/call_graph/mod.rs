use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::{FuncIndex, ProgramError};

mod builder;
mod node;
mod sort;

pub use builder::CallGraphBuilder;
pub use node::{Node, NodeRef};

pub struct CallGraph<T = FuncIndex> {
    nodes: HashMap<T, NodeRef<T>>,
}

pub trait Value: Copy + Eq + Ord + Copy + Clone + Hash + Debug + Display + 'static {}

impl<T: Copy + Eq + Ord + Copy + Clone + Hash + Debug + Display + 'static> Value for T {}

impl<T> CallGraph<T>
where
    T: Value,
{
    #[must_use]
    pub fn find_cycles(&self, return_cycles: bool) -> Result<(), ProgramError<T>> {
        let result = sort::try_topological_sort::<T>(self, return_cycles);

        result.map(|_cycle| ())
    }

    pub fn nodes(&self) -> Vec<NodeRef<T>> {
        self.nodes.values().cloned().collect()
    }

    pub fn remove_edge(&self, (source, dest): (T, T)) {
        let source = self.nodes.get(&source).unwrap();
        let dest = self.nodes.get(&dest).unwrap();

        source.as_mut().remove_out_edge(dest);
        dest.as_mut().remove_in_edge(source);
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<T> Debug for CallGraph<T>
where
    T: Value,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.nodes() {
            node.fmt(f);
        }

        Ok(())
    }
}
#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    macro_rules! graph {
        ( $( $source:expr => $dest:expr ),* ) => {{
            let mut builder = CallGraphBuilder::<&str>::new();

            $( builder.add_call($source, $dest); )*

            builder.build()
        }};
    }

    macro_rules! assert_cycles {
        ($graph:expr, $expected:expr) => {
            let return_cycles = true;

            let actual = $graph.find_cycles(return_cycles).unwrap_err();

            let err = ProgramError::CallCycle(Some($expected));

            assert_eq!(actual, err);
        };
    }

    macro_rules! assert_no_cycles {
        ($g:expr) => {
            let return_cycles = true;

            assert!($g.find_cycles(return_cycles).is_ok());
        };
    }

    #[test]
    fn call_graph_empty() {
        let g = graph! {};

        assert_no_cycles!(g);
    }

    #[test]
    fn call_graph_cycle_size_2() {
        let g = graph! {
            "a" => "b",
            "b" => "a"
        };

        assert_cycles!(g, vec!["a", "b", "a"]);
    }

    #[test]
    fn call_graph_cycle_size_3() {
        let g = graph! {
            "a" => "b",
            "b" => "c",
            "c" => "a"
        };

        assert_cycles!(g, vec!["a", "b", "c", "a"]);
    }

    #[test]
    fn call_graph_two_subgraphs_no_cycles() {
        let g = graph! {
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
        let g = graph! {
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
