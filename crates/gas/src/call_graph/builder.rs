use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

use crate::FuncIndex;

use super::{CallGraph, Node};

#[derive(Debug)]
pub struct CallGraphBuilder<T = FuncIndex> {
    nodes: HashMap<T, Rc<Node<T>>>,
}

impl<T> CallGraphBuilder<T>
where
    T: PartialEq + Eq + Copy + Clone + Hash + 'static,
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

    fn get_or_create_mut(&mut self, value: T) -> &mut Rc<Node<T>> {
        let entry = self.nodes.entry(value);

        entry.or_insert(Rc::new(Node::new(value)))
    }

    fn add_outgoing(&mut self, source: T, dest: T) {
        let dest = self.get_or_create_mut(dest);
        let dest = Rc::clone(dest);

        let source = self.get_or_create_mut(source);

        Rc::make_mut(source).add_out_edge(dest);
    }

    fn add_incoming(&mut self, source: T, dest: T) {
        let source = self.get_or_create_mut(source);
        let source = Rc::clone(source);

        let dest = self.get_or_create_mut(dest);

        Rc::make_mut(dest).add_in_edge(source);
    }
}
