use std::fmt::{self, Debug};

use crate::{Graph, NodeLabel};

#[derive(Clone, PartialEq)]
pub enum GraphCycles<L>
where
    L: NodeLabel,
{
    NoCycles,

    HasCycles(Option<Vec<L>>),
}

impl<L> Debug for GraphCycles<L>
where
    L: NodeLabel,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCycles => write!(f, "no-cycles"),
            Self::HasCycles(None) => write!(f, "has-cycles"),
            Self::HasCycles(Some(ref cycle)) => {
                debug_assert!(cycle.len() > 0);

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
    use std::fmt::format;

    use parity_wasm::elements::Func;

    use super::*;

    use crate::{FuncIndex, GraphCycles};

    fn fmt_cycle(cycle: GraphCycles<FuncIndex>) -> String {
        format!("{:?}", cycle)
    }

    fn elem(n: u32) -> FuncIndex {
        FuncIndex(n)
    }

    #[test]
    fn graph_cycle_fmt() {
        assert_eq!(fmt_cycle(GraphCycles::NoCycles), "no-cycles");
        assert_eq!(fmt_cycle(GraphCycles::HasCycles(None)), "has-cycles");

        assert_eq!(
            fmt_cycle(GraphCycles::HasCycles(Some(vec![elem(1)]))),
            "has-cycles (for example: `1`)"
        );

        assert_eq!(
            fmt_cycle(GraphCycles::HasCycles(Some(vec![
                elem(1),
                elem(2),
                elem(5),
                elem(1),
            ]))),
            "has-cycles (for example: `1 -> 2 -> 5 -> 1`)"
        );
    }
}
