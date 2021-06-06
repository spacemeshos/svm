use std::cell::{Ref, RefMut};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use crate::FuncIndex;

use super::{CallGraph, Node, NodeRef, Value};

/// This struct is used while building the `Call-Graph` on-the-fly as part of validation process of a Wasm program.
/// Once the building is done, calling `build` will output a `CallGraph` struct.
pub struct CallGraphBuilder<T = FuncIndex> {
    nodes: HashMap<T, NodeRef<T>>,
}

impl<T> CallGraphBuilder<T>
where
    T: Value,
{
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Add a new `call target` (i.e a function)
    pub fn add_target(&mut self, value: T) {
        let _ = self.get_or_create_mut(value);
    }

    /// Add a new existing `call` between function `source` to function `dest`.
    ///
    /// Adding a `call` signifies that there is at least one possible execution path of function `source`
    /// that will involve calling to function `dest`.
    ///
    /// There can be only one drawn edge in the underlying graph between any two nodes.
    /// So if there are multiple execution paths that contain more different `call`-ing between `source` to `dest`,
    /// only one edge will exist under the `CallGraph`.
    pub fn add_call(&mut self, source: T, dest: T) {
        debug_assert!(source != dest);

        self.add_outgoing(source, dest);
        self.add_incoming(source, dest);
    }

    /// Finalize building the `CallGraph`
    pub fn build(self) -> CallGraph<T> {
        CallGraph { nodes: self.nodes }
    }

    fn get_or_create_mut(&mut self, value: T) -> NodeRef<T> {
        let entry = self.nodes.entry(value);

        entry
            .or_insert_with(|| {
                let node = Node::new(value);

                NodeRef::new(node)
            })
            .clone()
    }

    fn add_outgoing(&mut self, source: T, dest: T) {
        let dest = self.get_or_create_mut(dest);
        let source = self.get_or_create_mut(source);

        let mut source = source.as_mut();
        source.add_out_edge(dest);
    }

    fn add_incoming(&mut self, source: T, dest: T) {
        let source = self.get_or_create_mut(source);
        let dest = self.get_or_create_mut(dest);

        let mut dest = dest.as_mut();
        dest.add_in_edge(source);
    }
}
