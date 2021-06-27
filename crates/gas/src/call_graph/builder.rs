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
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            inner: GraphBuilder::new(),
        }
    }

    /// Add a new `call target` (i.e a function)
    pub fn add_target(&mut self, label: L) {
        let _ = self.inner.get_or_create_mut(label);
    }

    /// Add a new existing `call` between function `source` to function `dest`.
    ///
    /// Adding a `call` signifies that there is at least one possible execution path of function `source`
    /// that will involve calling to function `dest`.
    ///
    /// There can be only one drawn edge in the underlying graph between any two nodes.
    /// So if there are multiple execution paths that contain more different `call`-ing between `source` to `dest`,
    /// only one edge will exist under the `CallGraph`.
    pub fn add_call(&mut self, origin: L, target: L) {
        self.inner.add_edge(origin, target);
    }

    /// Finalize building the `CallGraph`
    pub fn build(self) -> CallGraph<L> {
        let graph = self.inner.build();

        CallGraph { inner: graph }
    }
}
