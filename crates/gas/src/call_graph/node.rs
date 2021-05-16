use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use super::Value;

/// This struct implements a Node for the `CallGraph`.
///
/// Each `Node` has:
///
/// * `value`    - It's a value associated with it, and is assumed to be unique across the Graph.
/// * `incoming` - References to other `Nodes` that have *incoming* connections to `self`.
/// * `outgoing` - References to other `Nodes` that have *outgoing* connections to `self`.
pub struct Node<T> {
    value: T,
    incoming: HashSet<NodeRef<T>>,
    outgoing: HashSet<NodeRef<T>>,
}

impl<T> Node<T>
where
    T: Value,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            incoming: HashSet::new(),
            outgoing: HashSet::new(),
        }
    }

    /// Returns the value associated with the `Node`
    pub fn value(&self) -> T {
        self.value
    }

    /// Returns the a `Vec` of references to `Node`(s) with incoming edges to self
    pub fn incoming(&self) -> Vec<NodeRef<T>> {
        self.incoming.iter().cloned().collect()
    }

    /// Whether the current node has incoming edges
    pub fn has_incoming(&self) -> bool {
        self.incoming.len() > 0
    }

    /// Returns the a `Vec` of references to outgoing `Node`(s) from `self`
    pub fn outgoing(&self) -> Vec<NodeRef<T>> {
        self.outgoing.iter().cloned().collect()
    }

    /// Whether the current node has outgoing edges
    pub fn has_outgoing(&self) -> bool {
        self.outgoing.len() > 0
    }

    /// Adds an outgoing edge to `dest` (i.e: `self` -> `dest`)
    pub fn add_out_edge(&mut self, dest: NodeRef<T>) {
        self.outgoing.insert(dest);
    }

    /// Adds an incoming edge from `dest` (i.e: `dest` -> `self`)
    pub fn add_in_edge(&mut self, source: NodeRef<T>) {
        self.incoming.insert(source);
    }

    /// Removes an incoming edge from `dest` (i.e: `dest` -> `self`)
    pub fn remove_in_edge(&mut self, source: &NodeRef<T>) {
        self.incoming.remove(source);
    }

    /// Removes an outgoing edge to `dest` (i.e: `self` -> `dest`)
    pub fn remove_out_edge(&mut self, dest: &NodeRef<T>) {
        self.outgoing.remove(dest);
    }

    /// Whether is a `sink` node. (i.e: has no outgoing edges)
    pub fn is_sink(&self) -> bool {
        self.outgoing.is_empty()
    }

    /// Whether is a `source` node. (i.e: has no incoming edges)
    pub fn is_source(&self) -> bool {
        self.incoming.is_empty()
    }

    /// Whether is an `isolated` node. (i.e: has neither incoming nor outgoing edges)
    pub fn is_isolated(&self) -> bool {
        self.is_sink() && self.is_source()
    }
}

impl<T> Hash for Node<T>
where
    T: Value,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value().hash(state);
    }
}

impl<T> PartialEq for Node<T>
where
    T: Value,
{
    fn eq(&self, other: &Self) -> bool {
        let value = self.value();
        let other = other.value();

        value.eq(&other)
    }
}

impl<T> Eq for Node<T> where T: Value {}

/// The `NodeRef` wrapper struct adds readability to the code
/// (less cognitive load instead of seeing `Rc<RefCell<Node<T>>>` all over the place).
#[repr(transparent)]
pub struct NodeRef<T> {
    inner: Rc<RefCell<Node<T>>>,
}

impl<T> Debug for Node<T>
where
    T: Value,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value();

        write!(f, "{} [outgoing]\n", value)?;

        if self.outgoing().is_empty() {
            write!(f, "\tno outgoing\n")?;
        } else {
            for node in self.outgoing() {
                write!(f, "\t{} -> {}\n", value, node.value())?;
            }
        }

        write!(f, "{} [incoming]\n", value)?;

        if self.incoming().is_empty() {
            write!(f, "\tno incoming\n")?;
        } else {
            for node in self.incoming() {
                write!(f, "\t{} -> {}\n", node.value(), value)?;
            }
        }

        Ok(())
    }
}

impl<T> Debug for NodeRef<T>
where
    T: Value,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node: Ref<Node<T>> = self.as_ref();

        <Ref<Node<T>> as Debug>::fmt(&node, f)
    }
}

impl<T> Clone for NodeRef<T>
where
    T: Value,
{
    fn clone(&self) -> Self {
        let inner = Rc::clone(&self.inner);

        Self { inner }
    }
}

impl<T> NodeRef<T>
where
    T: Value,
{
    pub fn new(node: Node<T>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(node)),
        }
    }

    pub fn as_ref(&self) -> Ref<Node<T>> {
        self.inner.borrow()
    }

    pub fn as_mut(&self) -> RefMut<Node<T>> {
        self.inner.borrow_mut()
    }

    pub fn value(&self) -> T {
        self.as_ref().value()
    }

    pub fn incoming(&self) -> Vec<NodeRef<T>> {
        self.as_ref().incoming()
    }

    pub fn has_incoming(&self) -> bool {
        self.as_ref().has_incoming()
    }

    pub fn outgoing(&self) -> Vec<NodeRef<T>> {
        self.as_ref().outgoing()
    }

    pub fn has_outgoing(&self) -> bool {
        self.as_ref().has_outgoing()
    }

    pub fn is_sink(&self) -> bool {
        self.as_ref().is_sink()
    }

    pub fn is_source(&self) -> bool {
        self.as_ref().is_source()
    }

    pub fn is_isolated(&self) -> bool {
        self.as_ref().is_isolated()
    }
}

impl<T> Hash for NodeRef<T>
where
    T: Value,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value().hash(state);
    }
}

impl<T> PartialEq for NodeRef<T>
where
    T: Value,
{
    fn eq(&self, other: &Self) -> bool {
        let value = self.value();
        let other = other.value();

        value.eq(&other)
    }
}

impl<T> Eq for NodeRef<T> where T: Value {}
