use std::hash::Hash;
use std::collections::HashSet;

use super::CallGraph;

pub fn try_topological_sort<T, E>(call_graph: &CallGraph<T>) -> Result<Vec<T>, E>
where
    T: Copy + PartialEq + Eq + Copy + Clone + Hash + 'static,
 {
     todo!()
    // let mut sorted: Vec<T> = Vec::new();

    // let mut sources = call_graph.source_nodes();

    // while sources.is_empty() == false {
    //     let source = sources.pop();

    // }

    // Ok(sorted)
}