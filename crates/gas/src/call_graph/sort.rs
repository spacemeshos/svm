use std::collections::HashSet;

use crate::{FuncIndex, Program, ProgramError};

use super::{CallGraph, Node, NodeRef, Value};

/// This function implements a [Topological-Sort](https://en.wikipedia.org/wiki/Topological_sorting) over the given `CallGraph`.
/// The algorithm used is Kahn's algorithm which has a linear running time.
///
/// In case the are no cycles - a Topological sorted `Vec` of `T` (the type of a `Node` value) are returned.
/// Otherwise, an error stating there are cycles in the `CallGraph` are returned.
///
/// When `return_cycles = true` a specific cycles is returned. This is handy for debugging / testing purposes.
/// However, it should not be used in production for two reasons:
///
/// * It's a waste of CPU cycles. An invalid Wasm program should be discarded from the mem-pool at once.
/// * The current code for deriving a cycles is recursive (it's much easier to code a DFS search using recursions).
///   Malicious Wasm programs might contain very big cycles which has the potential to exhaust the Stack when recursing.
pub fn try_topological_sort<T>(
    call_graph: &CallGraph<T>,
    return_cycles: bool,
) -> Result<Vec<T>, ProgramError<T>>
where
    T: Value,
{
    let mut sorted = Vec::new();
    let mut queued = source_nodes(call_graph);

    while queued.is_empty() == false {
        let source = queued.pop().unwrap();

        sorted.push(source.value());

        for dest in source.outgoing() {
            let edge = (source.value(), dest.value());

            call_graph.remove_edge(edge);

            if dest.is_source() {
                queued.push(dest);
            }
        }
    }

    if has_edges(call_graph) {
        let cycle = derive_cycle(call_graph, return_cycles);

        Err(ProgramError::CallCycle(cycle))
    } else {
        Ok(sorted)
    }
}

fn source_nodes<T>(call_graph: &CallGraph<T>) -> Vec<NodeRef<T>>
where
    T: Value,
{
    call_graph
        .nodes()
        .iter()
        .filter(|node| node.is_source())
        .cloned()
        .collect()
}

fn has_edges<T>(call_graph: &CallGraph<T>) -> bool
where
    T: Value,
{
    call_graph
        .nodes()
        .iter()
        .any(|node| node.is_isolated() == false)
}

fn derive_cycle<T>(call_graph: &CallGraph<T>, return_cycles: bool) -> Option<Vec<T>>
where
    T: Value,
{
    // The `return_cycles` feature is optional.
    // In production we should have it turned-off.
    if !return_cycles {
        return None;
    }

    // Since, we know there is at least one cycle within the `CallGraph`.
    // Each cycle's node must have at least one outgoing and one incoming edges.
    let mut candidates = call_graph
        .nodes()
        .iter()
        .filter(|node| node.has_incoming() && node.has_outgoing())
        .cloned()
        .collect::<Vec<_>>();

    // We order so that the tests will be deterministic.
    // So we always pick the first node out of the sorted `candidates`
    candidates.sort_by_key(|node| node.value());

    let start = candidates.first().unwrap();
    let cycle = find_cycle(&start);

    Some(cycle)
}

fn find_cycle<T>(start: &NodeRef<T>) -> Vec<T>
where
    T: Value,
{
    let mut cycle = Vec::new();
    let mut visited = HashSet::new();

    let found = recur_find_cycle(start, &mut cycle, &mut visited);

    debug_assert!(found);

    cycle
}

fn recur_find_cycle<T>(current: &NodeRef<T>, cycle: &mut Vec<T>, visited: &mut HashSet<T>) -> bool
where
    T: Value,
{
    let value = current.value();

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
