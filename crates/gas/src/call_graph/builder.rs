use std::cell::{Ref, RefMut};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use crate::FuncIndex;

use super::{CallGraph, Node, NodeRef, Value};

pub struct CallGraphBuilder<T = FuncIndex> {
    nodes: HashMap<T, NodeRef<T>>,
}

impl<T> CallGraphBuilder<T>
where
    T: Value,
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_target(&mut self, value: T) {
        let _ = self.get_or_create_mut(value);
    }

    pub fn add_call(&mut self, source: T, dest: T) {
        debug_assert!(source != dest);

        self.add_outgoing(source, dest);
        self.add_incoming(source, dest);
    }

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
