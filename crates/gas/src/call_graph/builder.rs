use crate::{CallGraph, FuncIndex, GraphBuilder, NodeLabel};

/// This struct is used while building the `Call-Graph` on-the-fly as part of validation process of a Wasm program.
/// Once the build is done, calling `build` will output a `CallGraph` struct.
pub struct CallGraphBuilder<L>
where
    L: NodeLabel,
{
    inner: GraphBuilder<L, ()>,
}

impl<L> CallGraphBuilder<L>
where
    L: NodeLabel,
{
    pub fn new() -> Self {
        Self {
            inner: GraphBuilder::new(),
        }
    }

    pub fn add_target(&mut self, label: L) {
        let _ = self.inner.get_or_create_mut(label);
    }

    pub fn add_call(&mut self, origin: L, target: L) {
        self.inner.add_edge(origin, target);
    }

    pub fn build(self) -> CallGraph<L> {
        let graph = self.inner.build();

        CallGraph { inner: graph }
    }
}
