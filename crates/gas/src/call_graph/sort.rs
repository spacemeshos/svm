use std::collections::HashSet;

use crate::{FuncIndex, Program, ProgramError};

use super::{CallGraph, Node, NodeRef, Value};

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
    call_graph.nodes().iter().any(|node| {
        let isolated = node.is_source() && node.is_sink();

        (!isolated)
    })
}

fn derive_cycle<T>(call_graph: &CallGraph<T>, return_cycles: bool) -> Option<Vec<T>>
where
    T: Value,
{
    if !return_cycles {
        return None;
    }

    let mut candidates = call_graph
        .nodes()
        .iter()
        .filter(|node| node.has_incoming() && node.has_outgoing())
        .cloned()
        .collect::<Vec<_>>();

    candidates.sort_by_key(|node| node.value());

    let first = candidates.first().unwrap();

    let mut cycle = Vec::new();
    let mut visited = HashSet::new();

    let found = find_cycle(&first, &mut cycle, &mut visited);

    debug_assert!(found);

    Some(cycle)
}

fn find_cycle<T>(current: &NodeRef<T>, cycle: &mut Vec<T>, visited: &mut HashSet<T>) -> bool
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
        let found = find_cycle(next, cycle, visited);

        if found {
            return true;
        }
    }

    let value_ = cycle.pop().unwrap();
    debug_assert_eq!(value_, value);

    visited.remove(&value);

    false
}
