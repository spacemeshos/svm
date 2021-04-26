use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::{FuncIndex, ProgramError};

mod builder;
mod node;
mod sort;

pub use builder::CallGraphBuilder;
pub use node::Node;

#[derive(Debug)]
pub struct CallGraph<T = FuncIndex> {
    nodes: HashMap<T, Rc<Node<T>>>,
}

impl<T> CallGraph<T>
where
    T: Copy + PartialEq + Eq + Copy + Clone + Hash + 'static,
{
    #[must_use]
    pub fn assert_no_cycles(&self) -> Result<(), ProgramError> {
        let result = sort::try_topological_sort::<T, ProgramError>(self);

        result.map(|_| ())
    }
}
