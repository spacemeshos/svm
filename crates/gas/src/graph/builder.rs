use indexmap::IndexMap;

use svm_program::FuncIndex;

use crate::{Graph, Node, NodeData, NodeLabel, NodeRef};

/// Builders a `Graph`
///
/// * `L` - The letter `L` stands for `Label`. Each `Node`'s has a unique label of type `L`.
/// * `D` - The letter `D` stands for `Data`. Each `Node`'s owns data of type `D`.
pub struct GraphBuilder<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    nodes: IndexMap<L, NodeRef<L, D>>,
}

impl<L, D> GraphBuilder<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    /// Creates a new `GraphBuilder`
    pub fn new() -> Self {
        Self {
            nodes: IndexMap::new(),
        }
    }

    /// Adds a empty `Node` labeled with input parameter `label`
    pub fn add_node(&mut self, label: L) {
        let _ = self.get_or_create_mut(label);
    }

    /// Adds an `Edge` between `Node`s labeled with input parameters `source` and `dest` respectively.
    /// If there is no `Node` with a given label, it creates a new one.
    pub fn add_edge(&mut self, source: L, dest: L) {
        debug_assert!(source != dest);

        self.add_outgoing(source, dest);
        self.add_incoming(source, dest);
    }

    /// Finished the building process, and outputs a `Graph`
    pub fn build(self) -> Graph<L, D> {
        Graph { nodes: self.nodes }
    }

    /// Returns a reference to the `Node` labeled with input `label`.
    ///
    /// If there is no `Node` with labeled with `label`,
    /// it first creates it and then returns a reference to it.
    pub fn get_or_create_mut(&mut self, label: L) -> NodeRef<L, D> {
        let entry = self.nodes.entry(label);

        entry
            .or_insert_with(|| {
                let node = Node::new(label);

                NodeRef::new(node)
            })
            .clone()
    }

    fn add_outgoing(&mut self, source: L, dest: L) {
        let dest = self.get_or_create_mut(dest);
        let source = self.get_or_create_mut(source);

        let mut source = source.as_mut();
        source.add_out_edge(dest);
    }

    fn add_incoming(&mut self, source: L, dest: L) {
        let source = self.get_or_create_mut(source);
        let dest = self.get_or_create_mut(dest);

        let mut dest = dest.as_mut();
        dest.add_in_edge(source);
    }
}
