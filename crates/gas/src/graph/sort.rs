use svm_program::{Program, ProgramError};

use std::collections::HashSet;

use crate::{Graph, GraphCycles, Node, NodeData, NodeLabel, NodeRef};

/// This file implements a [Topological-Sort](https://en.wikipedia.org/wiki/Topological_sorting) over the given `Graph`.
/// The algorithm used is Kahn's algorithm which has a linear running time.
///
/// In case the are no cycles - a Topological sorted `Vec` of `L` (the type implementing trait `NodeLabel`) are returned.
/// Otherwise, an error stating there are cycles in the `Graph` are returned.
///
/// When `return_cycles = true` a specific cycles is returned. This is handy for debugging / testing purposes.
/// However, it should not be used in production for two reasons:
///
/// * It's a waste of CPU cycles. An invalid Wasm program should be discarded from the mem-pool at once.
/// * The current code for deriving a cycles is recursive (it's much easier to code a DFS search using recursions).
///   Malicious Wasm programs might contain very big cycles which has the potential to exhaust the Stack when recursing.

pub fn topological_sort<L, D, G>(graph: &G) -> Vec<L>
where
    G: std::ops::Deref<Target = Graph<L, D>>,
    L: NodeLabel,
    D: NodeData,
{
    try_topological_sort::<L, D, G>(graph, false).unwrap()
}

pub fn try_topological_sort<L, D, G>(
    graph: &G,
    return_cycles: bool,
) -> Result<Vec<L>, GraphCycles<L>>
where
    G: std::ops::Deref<Target = Graph<L, D>>,
    L: NodeLabel,
    D: NodeData,
{
    let mut sorted = Vec::new();
    let mut queued = graph.source_nodes();

    while queued.is_empty() == false {
        let source = queued.pop().unwrap();

        sorted.push(source.label());

        for dest in source.outgoing() {
            let edge = (source.label(), dest.label());

            graph.remove_edge(edge);

            if dest.is_source() {
                queued.push(dest);
            }
        }
    }

    if has_edges(graph) {
        let cycle = derive_cycle(graph, return_cycles);

        let cycle = GraphCycles::HasCycles(cycle.unwrap_or_default());

        Err(cycle)
    } else {
        Ok(sorted)
    }
}

fn has_edges<L, D, G>(graph: &G) -> bool
where
    G: std::ops::Deref<Target = Graph<L, D>>,
    L: NodeLabel,
    D: NodeData,
{
    graph.nodes().iter().any(|node| node.is_isolated() == false)
}

fn derive_cycle<L, D, G>(graph: &G, return_cycles: bool) -> Option<Vec<L>>
where
    G: std::ops::Deref<Target = Graph<L, D>>,
    L: NodeLabel,
    D: NodeData,
{
    // The `return_cycles` feature is optional.
    // In production we should have it turned-off.
    if !return_cycles {
        return None;
    }

    // Since, we know there is at least one cycle within the `Graph`.
    // Each cycle's node must have at least one outgoing and one incoming edges.
    let mut candidates = graph
        .nodes()
        .iter()
        .filter(|node| node.has_incoming() && node.has_outgoing())
        .cloned()
        .collect::<Vec<_>>();

    // We order so that the tests will be deterministic.
    // So we always pick the first node out of the sorted `candidates`
    candidates.sort_by_key(|node| node.label());

    let start = candidates.first().unwrap();
    let cycle = find_cycle(&start);

    Some(cycle)
}

fn find_cycle<L, D>(start: &NodeRef<L, D>) -> Vec<L>
where
    L: NodeLabel,
    D: NodeData,
{
    let mut cycle = Vec::new();
    let mut visited = HashSet::new();

    let found = recur_find_cycle(start, &mut cycle, &mut visited);

    debug_assert!(found);

    cycle
}

fn recur_find_cycle<L, D>(
    current: &NodeRef<L, D>,
    cycle: &mut Vec<L>,
    visited: &mut HashSet<L>,
) -> bool
where
    L: NodeLabel,
    D: NodeData,
{
    let value = current.label();

    if visited.contains(&value) {
        cycle.push(value);
        return true;
    }

    cycle.push(value);
    visited.insert(value);

    for next in current.outgoing().iter() {
        let found = recur_find_cycle(next, cycle, visited);

        if found {
            return true;
        }
    }

    let value_ = cycle.pop().unwrap();
    debug_assert_eq!(value_, value);

    visited.remove(&value);

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{CallGraph, CallGraphBuilder, NodeLabel};

    macro_rules! graph {
        (
            nodes: [
                $( $node:expr ),*
            ],
            edges: [
                $( $source:expr => $dest:expr ),*
            ]
        ) =>
        {{
            let mut builder = CallGraphBuilder::<&'static str>::new();

            $( builder.add_target($node); )*
            $( builder.add_call($source, $dest); )*

            builder.build()
        }};
    }

    fn sort(g: CallGraph<&'static str>) -> Vec<&'static str> {
        topological_sort::<_, _, CallGraph<_>>(&g)
    }

    #[test]
    fn topological_sort_empty() {
        let g = graph! { nodes: [], edges: [] };

        let sorted = sort(g);

        assert!(sorted.is_empty());
    }

    #[test]
    fn topological_sort_one_node() {
        let mut g = graph! { nodes: ["a"], edges: [] };

        let sorted = sort(g);

        assert_eq!(sorted, vec!["a"]);
    }

    #[test]
    fn topological_sort_test_1() {
        let mut g = graph! {
            nodes: ["a", "b", "c"],
            edges: [
                "a" => "b",
                "b" => "c"
            ]
        };

        let sorted = sort(g);

        assert_eq!(sorted, vec!["a", "b", "c"]);
    }

    #[test]
    fn topological_sort_test_2() {
        let mut g = graph! {
            nodes: ["a", "b", "c", "d"],
            edges: [
                "a" => "b",
                "b" => "c",
                "a" => "d",
                "c" => "d"
            ]
        };

        let sorted = sort(g);

        assert_eq!(sorted, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn topological_sort_test_3() {
        let mut g = graph! {
            nodes: ["a", "b", "c", "d"],
            edges: [
                "a" => "b"
            ]
        };

        let sorted = sort(g);

        assert_eq!(sorted, vec!["d", "c", "a", "b"]);
    }
}
