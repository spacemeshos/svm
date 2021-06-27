use indexmap::IndexMap;

use std::fmt::{self, Debug};
use std::ops::Deref;

mod builder;
mod cycle;
mod data;
mod label;
mod node;
mod sort;
mod weight;

pub use builder::GraphBuilder;
pub use cycle::GraphCycles;
pub use data::NodeData;
pub use label::NodeLabel;
pub use node::{Node, NodeRef};
pub use sort::{topological_sort, try_topological_sort};
pub use weight::{compute_max_weight_path, NodeWeight, WeightedGraph, WeightedPath};

pub struct Graph<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    nodes: IndexMap<L, NodeRef<L, D>>,
}

impl<L, D> Graph<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    /// Returns references to the `Graph`'s nodes
    pub fn nodes(&self) -> Vec<NodeRef<L, D>> {
        self.nodes.values().cloned().collect()
    }

    /// Removes a `Node` from the `Graph`
    pub fn remove_node(&self, node: &NodeRef<L, D>) {
        for neighbor in node.outgoing() {
            let edge = (node.label(), neighbor.label());

            self.remove_edge(edge);
        }
    }

    /// Removes an `Edge` from the `Graph`
    pub fn remove_edge(&self, (source, dest): (L, L)) {
        let source = self.nodes.get(&source).unwrap();
        let dest = self.nodes.get(&dest).unwrap();

        source.as_mut().remove_out_edge(dest);
        dest.as_mut().remove_in_edge(source);
    }

    /// Returns the number of `Node`s in the `Graph`
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    //// Returns references to `Nodes` which have no incoming edges
    pub fn source_nodes(&self) -> Vec<NodeRef<L, D>> {
        self.nodes()
            .iter()
            .filter(|node| node.is_source())
            .cloned()
            .collect()
    }

    /// Returns a reference to a `Node` given its `label` identifier
    ///
    /// # Panics
    ///  
    /// Panics if there is node `Node` labeled with `label`
    ///
    pub fn get_node(&self, label: L) -> NodeRef<L, D> {
        self.try_get_node(label).unwrap()
    }

    /// Tries to return a reference to a `Node` given its `label` identifier
    pub fn try_get_node(&self, label: L) -> Option<NodeRef<L, D>> {
        let node_ref = self.nodes.get(&label);

        node_ref.cloned()
    }
}

impl<L, D> Debug for Graph<L, D>
where
    L: NodeLabel,
    D: NodeData,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.nodes() {
            node.fmt(f);
        }

        Ok(())
    }
}
