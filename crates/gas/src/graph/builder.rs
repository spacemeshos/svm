use indexmap::IndexMap;

use crate::{FuncIndex, Graph, Node, NodeData, NodeLabel, NodeRef};

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
    pub fn new() -> Self {
        Self {
            nodes: IndexMap::new(),
        }
    }

    pub fn add_node(&mut self, label: L) {
        let _ = self.get_or_create_mut(label);
    }

    pub fn add_edge(&mut self, source: L, dest: L) {
        debug_assert!(source != dest);

        self.add_outgoing(source, dest);
        self.add_incoming(source, dest);
    }

    pub fn build(self) -> Graph<L, D> {
        Graph { nodes: self.nodes }
    }

    pub fn get_or_create_mut(&mut self, value: L) -> NodeRef<L, D> {
        let entry = self.nodes.entry(value);

        entry
            .or_insert_with(|| {
                let node = Node::new(value);

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
