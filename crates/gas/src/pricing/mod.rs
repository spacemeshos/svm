use crate::{FuncIndex, Gas, Program};

use super::call_graph::try_topological_sort;

pub fn func_price(program: &Program, func: FuncIndex) -> Gas {
    todo!()
    // let functions = try_topological_sort(call_graph, false).unwrap();
}
