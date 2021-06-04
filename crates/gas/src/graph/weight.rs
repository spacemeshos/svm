use std::cell::Cell;
use std::fmt::{self, Debug, Display};

use crate::{Graph, Node, NodeData, NodeLabel, NodeRef};

pub type WeightedGraph<L> = Graph<L, NodeWeight<L>>;

/// Given an input `Weighted Graph` which is assumed to be a DAG (has no cycles).
///
/// Usually, a `weighted graph` is implemented where each `Edge` has `weight`.
/// We use `WeightedGraph` where each `Node`'s data has an associated `weight`.
///
/// This function computes a path where the sum of its `Node`s weights is the maximum.
/// The result is the `maximum weight path` and an example yielding the maximum total weight.
pub fn compute_max_weight_path<L>(graph: &WeightedGraph<L>, start: L, end: L) -> WeightedPath<L>
where
    L: NodeLabel,
{
    init_start(graph, start);

    prune_unreachable_nodes(graph, start);

    let start_node = graph.get_node(start);

    let mut queued = vec![start_node];
    let mut touched_end = false;

    while queued.is_empty() == false {
        let source = queued.pop().unwrap();

        if source.label() == end {
            touched_end = true;
        }

        for dest in source.outgoing() {
            relax(graph, &dest, &source);

            let edge = (source.label(), dest.label());
            graph.remove_edge(edge);

            if dest.is_source() {
                queued.push(dest);
            }
        }
    }

    assert!(touched_end);

    construct_path(graph, start, end)
}

fn prune_unreachable_nodes<L>(graph: &WeightedGraph<L>, start: L)
where
    L: NodeLabel,
{
    graph
        .source_nodes()
        .iter()
        .filter(|node| node.label() != start)
        .for_each(|source| {
            graph.remove_node(source);
        });
}

fn construct_path<L>(graph: &WeightedGraph<L>, start: L, end: L) -> WeightedPath<L>
where
    L: NodeLabel,
{
    let mut label = end;
    let mut path = Vec::new();

    while label != start {
        let weight = node_weight_by_label(graph, label);
        path.push((label, weight));

        // println!("path node: (label = `{}`, weight = {})", label, weight);

        label = node_dep_by_label(graph, label);
    }

    let w = node_weight_by_label(graph, start);
    path.push((start, w));

    path.reverse();

    let total = node_total_weight_by_label(graph, end);

    WeightedPath { path, total }
}

fn init_start<L>(graph: &WeightedGraph<L>, start: L)
where
    L: NodeLabel,
{
    let start = graph.get_node(start);
    let start_ref = start.as_ref();
    let start_data = start_ref.data();

    start_data.mark_start();
}

fn relax<L>(
    graph: &WeightedGraph<L>,
    node: &NodeRef<L, NodeWeight<L>>,
    dep_node: &NodeRef<L, NodeWeight<L>>,
) where
    L: NodeLabel,
{
    let dep_weight = node_total_weight_by_ref(&dep_node);

    let node_ref = node.as_ref();
    let node_data = node_ref.data();

    node_data.relax(dep_node.label(), dep_weight);
}

fn node_weight_by_label<L>(graph: &WeightedGraph<L>, label: L) -> usize
where
    L: NodeLabel,
{
    let node = graph.get_node(label);

    node_weight_by_ref(&node)
}

fn node_total_weight_by_label<L>(graph: &WeightedGraph<L>, label: L) -> usize
where
    L: NodeLabel,
{
    let node = graph.get_node(label);

    node_total_weight_by_ref(&node)
}

fn node_weight_by_ref<L>(node: &NodeRef<L, NodeWeight<L>>) -> usize
where
    L: NodeLabel,
{
    let node_ref = node.as_ref();
    let data = node_ref.data();

    data.weight()
}

fn node_total_weight_by_ref<L>(node: &NodeRef<L, NodeWeight<L>>) -> usize
where
    L: NodeLabel,
{
    let node_ref = node.as_ref();
    let data = node_ref.data();

    data.total_weight()
}

fn node_dep_by_label<L>(graph: &WeightedGraph<L>, label: L) -> L
where
    L: NodeLabel,
{
    let node = graph.get_node(label);
    let node_ref = node.as_ref();

    let node_data = node_ref.data();
    node_data.dep_label()
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeWeight<L>
where
    L: NodeLabel,
{
    label: Cell<Option<L>>,

    weight: Cell<Option<usize>>,

    dep_label: Cell<Option<L>>,

    dep_weight: Cell<Option<usize>>,
}

impl<L> Default for NodeWeight<L>
where
    L: NodeLabel,
{
    fn default() -> Self {
        Self {
            label: Cell::new(None),
            weight: Cell::new(None),
            dep_label: Cell::new(None),
            dep_weight: Cell::new(None),
        }
    }
}

impl<L> fmt::Display for NodeWeight<L>
where
    L: NodeLabel,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label();
        let weight = match self.try_weight() {
            Some(weight) => format!("{}", weight),
            None => "N/A".to_string(),
        };

        match self.try_dep_label() {
            None => {
                write!(
                    f,
                    "Node `{}` data: self-weight: {}, depends-on: N/A",
                    label, weight
                )
            }
            Some(dep_label) => {
                let dep_weight = self.dep_weight();

                write!(
                    f,
                    "Node `{}` data: self-weight: {}, depends-on: (label: {}, weight: {})",
                    label, weight, dep_label, dep_weight
                )
            }
        }
    }
}

impl<L> NodeWeight<L>
where
    L: NodeLabel,
{
    pub fn set_label(&self, label: L) {
        self.label.set((Some(label)));
    }

    pub fn set_weight(&self, weight: usize) {
        self.weight.set(Some(weight));
    }

    pub fn mark_start(&self) {
        self.dep_weight.set(Some(0));
    }

    pub fn relax(&self, dep_label: L, dep_weight: usize) {
        match self.try_dep_weight() {
            Some(cur_depw) => {
                if cur_depw < dep_weight {
                    self.dep_label.set(Some(dep_label));
                    self.dep_weight.set(Some(dep_weight));
                }
            }
            None => {
                self.dep_label.set(Some(dep_label));
                self.dep_weight.set(Some(dep_weight));
            }
        }
    }

    pub fn total_weight(&self) -> usize {
        let dep_weight = self.try_dep_weight().unwrap();

        dep_weight + self.weight()
    }

    fn label(&self) -> L {
        match self.label.get() {
            Some(label) => label,
            None => unreachable!("No `label` configured"),
        }
    }

    fn weight(&self) -> usize {
        match self.try_weight() {
            Some(weight) => weight,
            None => panic!("Node `{}` is missing `weight`", self.label()),
        }
    }

    fn dep_label(&self) -> L {
        match self.try_dep_label() {
            Some(label) => label,
            None => panic!("Node `{}` is missing `dependant`-node label", self.label()),
        }
    }

    fn dep_weight(&self) -> usize {
        match self.try_dep_weight() {
            Some(weight) => weight,
            None => panic!("Node `{}` is missing `dependant`-node weight", self.label()),
        }
    }

    fn try_weight(&self) -> Option<usize> {
        self.weight.get()
    }

    fn try_dep_label(&self) -> Option<L> {
        self.dep_label.get()
    }

    fn try_dep_weight(&self) -> Option<usize> {
        self.dep_weight.get()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeightedPath<L>
where
    L: NodeLabel,
{
    path: Vec<(L, usize)>,

    total: usize,
}

impl<L> WeightedPath<L>
where
    L: NodeLabel,
{
    pub fn total(&self) -> usize {
        self.total
    }

    pub fn path(&self) -> &[(L, usize)] {
        &self.path
    }

    pub fn iter(&self) -> std::slice::Iter<(L, usize)> {
        self.path.iter()
    }
}

impl<L> Default for WeightedPath<L>
where
    L: NodeLabel,
{
    fn default() -> Self {
        Self {
            path: Vec::new(),
            total: 0,
        }
    }
}

fn assert_node_weight_ty<L>()
where
    L: NodeLabel,
{
    fn assert_ty<T: NodeData>() {}

    assert_ty::<NodeWeight<L>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Graph, GraphBuilder};

    macro_rules! graph {
        (
            nodes: [
                $( ($label:expr, $weight:expr) ),*
            ],
            edges: [
                $( $source:expr => $dest:expr ),*
            ]
        ) =>
        {{
            let mut builder = GraphBuilder::<_, NodeWeight<&'static str>>::new();

            $(
                {
                    let node = builder.get_or_create_mut($label);
                    let node_ref = node.as_ref();

                    let node_data: &NodeWeight<&'static str> = node_ref.data();

                    node_data.set_label($label);
                    node_data.set_weight($weight);
                }
            )*

            $( builder.add_edge($source, $dest); )*

            builder.build()
        }};
    }

    #[test]
    fn max_weight_path_1() {
        let g = graph! {
            nodes: [("a", 10)],
            edges: []
        };

        let path = compute_max_weight_path(&g, "a", "a");

        assert_eq!(
            path,
            WeightedPath {
                path: vec![("a", 10)],
                total: 10
            }
        );
    }

    #[test]
    fn max_weight_path_2() {
        let g = graph! {
            nodes: [("a", 10), ("b", 20)],
            edges: ["a" => "b"]
        };

        let path = compute_max_weight_path(&g, "a", "b");

        assert_eq!(
            path,
            WeightedPath {
                path: vec![("a", 10), ("b", 20)],
                total: 30
            }
        );
    }

    fn max_weight_path_3() {
        let g = graph! {
            nodes: [("a", 10), ("b", 20), ("c", 5)],
            edges: [
                "a" => "b",
                "b" => "c",
                "a" => "c"
            ]
        };

        let path = compute_max_weight_path(&g, "a", "c");

        assert_eq!(
            path,
            WeightedPath {
                path: vec![("a", 10), ("b", 20), ("c", 5)],
                total: 35
            }
        );
    }

    #[test]
    fn max_weight_path_4() {
        let g = graph! {
            nodes: [("a", 10), ("b", 20), ("c", 5), ("d", 6)],
            edges: [
                "a" => "b",
                "a" => "c",
                "b" => "d",
                "c" => "d"
            ]
        };

        let path = compute_max_weight_path(&g, "a", "d");

        assert_eq!(
            path,
            WeightedPath {
                path: vec![("a", 10), ("b", 20), ("d", 6)],
                total: 36
            }
        );
    }

    #[test]
    fn max_weight_path_5() {
        let g = graph! {
            nodes: [("a", 10), ("b", 20), ("c", 5), ("d", 6), ("e", 1000)],
            edges: [
                "a" => "b",
                "a" => "c",
                "b" => "d",
                "c" => "d",
                "e" => "d"
            ]
        };

        let path = compute_max_weight_path(&g, "a", "d");

        assert_eq!(
            path,
            WeightedPath {
                path: vec![("a", 10), ("b", 20), ("d", 6)],
                total: 36
            }
        );
    }
}
