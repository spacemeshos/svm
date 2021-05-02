use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::{FuncIndex, ProgramError};

mod builder;
// mod sort;

mod node;

pub use builder::CallGraphBuilder;
pub use node::Node;

#[derive(Debug)]
pub struct CallGraph<T = FuncIndex> {
    // nodes: HashMap<T, Rc<RefCell<Node<T>>>>,
    phantom: PhantomData<T>,
}

impl<T> CallGraph<T>
where
    T: Copy + PartialEq + Eq + Copy + Clone + Hash + 'static,
{
    #[must_use]
    pub fn assert_no_cycles(&self) -> Result<(), ProgramError> {
        todo!()
        // let result = sort::try_topological_sort::<T, ProgramError>(self);

        // result.map(|_| ())
    }
}
