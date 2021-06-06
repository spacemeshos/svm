use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use indexmap::IndexSet;

use super::{NodeData, NodeLabel};

/// This struct implements a Node for the `CallGraph`.
///
/// Each `Node` has:
///
/// * `value`    - It's a value associated with it, and is assumed to be unique across the Graph.
/// * `incoming` - References to other `Nodes` that have *incoming* connections to `self`.
/// * `outgoing` - References to other `Nodes` that have *outgoing* connections to `self`.
pub struct Node<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    label: L,
    data: D,
    incoming: IndexSet<NodeRef<L, D>>,
    outgoing: IndexSet<NodeRef<L, D>>,
}

impl<L, D> Node<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    // Creates a new `Node` labeled as `label`
    pub fn new(label: L) -> Self {
        Self {
            label,
            data: D::default(),
            incoming: IndexSet::new(),
            outgoing: IndexSet::new(),
        }
    }

    /// Returns the `label` associated with the `Node`
    pub fn label(&self) -> L {
        self.label
    }

    /// Returns the `data` associated with the `Node`
    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn set_data(&mut self, data: D) {
        self.data = data;
    }

    /// Returns the a `Vec` of references to `Node`(s) with incoming edges to `self`
    pub fn incoming(&self) -> Vec<NodeRef<L, D>> {
        self.incoming.iter().cloned().collect()
    }

    /// Whether the current node has incoming edges
    pub fn has_incoming(&self) -> bool {
        self.incoming.len() > 0
    }

    /// Returns the a `Vec` of references to outgoing `Node`(s) from `self`
    pub fn outgoing(&self) -> Vec<NodeRef<L, D>> {
        self.outgoing.iter().cloned().collect()
    }

    /// Whether the current node has outgoing edges
    pub fn has_outgoing(&self) -> bool {
        self.outgoing.len() > 0
    }

    /// Adds an outgoing edge to `dest` (i.e: `self` -> `dest`)
    pub fn add_out_edge(&mut self, dest: NodeRef<L, D>) {
        self.outgoing.insert(dest);
    }

    /// Adds an incoming edge from `dest` (i.e: `dest` -> `self`)
    pub fn add_in_edge(&mut self, source: NodeRef<L, D>) {
        self.incoming.insert(source);
    }

    /// Removes an incoming edge from `dest` (i.e: `dest` -> `self`)
    pub fn remove_in_edge(&mut self, source: &NodeRef<L, D>) {
        self.incoming.remove(source);
    }

    /// Removes an outgoing edge to `dest` (i.e: `self` -> `dest`)
    pub fn remove_out_edge(&mut self, dest: &NodeRef<L, D>) {
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

impl<L, D> Hash for Node<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label().hash(state);
    }
}

impl<L, D> PartialEq for Node<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    fn eq(&self, other: &Self) -> bool {
        let value = self.label();
        let other = other.label();

        value.eq(&other)
    }
}

impl<L, D> Eq for Node<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
}

/// The `NodeRef` wrapper struct adds readability to the code
/// (less cognitive load instead of seeing `Rc<RefCell<Node<T>>>` all over the place).
#[repr(transparent)]
pub struct NodeRef<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    inner: Rc<RefCell<Node<L, D>>>,
}

impl<L, D> Debug for Node<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.label();

        write!(f, "{} [outgoing]\n", value)?;

        if self.outgoing().is_empty() {
            write!(f, "\tno outgoing\n")?;
        } else {
            for node in self.outgoing() {
                write!(f, "\t{} -> {}\n", value, node.label())?;
            }
        }

        write!(f, "{} [incoming]\n", value)?;

        if self.incoming().is_empty() {
            write!(f, "\tno incoming\n")?;
        } else {
            for node in self.incoming() {
                write!(f, "\t{} -> {}\n", node.label(), value)?;
            }
        }

        Ok(())
    }
}

impl<L, D> Debug for NodeRef<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node: Ref<Node<L, D>> = self.as_ref();

        <Ref<Node<L, D>> as Debug>::fmt(&node, f)
    }
}

impl<L, D> Clone for NodeRef<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    fn clone(&self) -> Self {
        let inner = Rc::clone(&self.inner);

        Self { inner }
    }
}

impl<L, D> NodeRef<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    pub fn new(node: Node<L, D>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(node)),
        }
    }

    pub fn as_ref(&self) -> Ref<Node<L, D>> {
        self.inner.borrow()
    }

    pub fn as_mut(&self) -> RefMut<Node<L, D>> {
        self.inner.borrow_mut()
    }

    pub fn label(&self) -> L {
        self.as_ref().label()
    }

    pub fn incoming(&self) -> Vec<NodeRef<L, D>> {
        self.as_ref().incoming()
    }

    pub fn has_incoming(&self) -> bool {
        self.as_ref().has_incoming()
    }

    pub fn outgoing(&self) -> Vec<NodeRef<L, D>> {
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

impl<L, D> Hash for NodeRef<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label().hash(state);
    }
}

impl<L, D> PartialEq for NodeRef<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    fn eq(&self, other: &Self) -> bool {
        let value = self.label();
        let other = other.label();

        value.eq(&other)
    }
}

impl<L, D> Eq for NodeRef<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
}
