use std::fmt::{self, Debug};

use crate::{Graph, NodeLabel};

/// Represents an optional `Cycle` in a `Graph`.
#[derive(Clone, PartialEq)]
pub enum GraphCycles<L>
where
    L: NodeLabel,
{
    /// Graph has no cycles (it's a DAG)
    NoCycles,

    /// Graph has at least one cycle.
    ///
    /// If its owned data is `None` - we can't give a concrete example of a cycle.
    /// Otherwise, it's a `Some(cycle)` and owns an example of a cycle.
    /// (there might be more cycles within the `Graph`).
    HasCycles(Vec<L>),
}

impl<L> Debug for GraphCycles<L>
where
    L: NodeLabel,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCycles => write!(f, "no-cycles"),
            Self::HasCycles(cycle) if cycle.is_empty() => write!(f, "has-cycles"),
            Self::HasCycles(cycle) => {
                let first = cycle.first().unwrap();
                let mut cycle_str = format!("{}", first);

                for label in cycle.iter().skip(1) {
                    cycle_str.push_str(&format!(" -> {}", label));
                }

                write!(f, "has-cycles (for example: `{}`)", cycle_str)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use parity_wasm::elements::Func;

    use std::fmt::format;

    use super::*;
    use crate::{FuncIndex, GraphCycles};

    fn fmt_cycle(cycle: GraphCycles<FuncIndex>) -> String {
        format!("{:?}", cycle)
    }

    #[test]
    fn graph_cycle_fmt() {
        assert_eq!(fmt_cycle(GraphCycles::NoCycles), "no-cycles");
        assert_eq!(fmt_cycle(GraphCycles::HasCycles(vec![])), "has-cycles");

        assert_eq!(
            fmt_cycle(GraphCycles::HasCycles(vec![FuncIndex(1)])),
            "has-cycles (for example: `1`)"
        );

        assert_eq!(
            fmt_cycle(GraphCycles::HasCycles(vec![
                FuncIndex(1),
                FuncIndex(2),
                FuncIndex(5),
                FuncIndex(1),
            ])),
            "has-cycles (for example: `1 -> 2 -> 5 -> 1`)"
        );
    }
}
