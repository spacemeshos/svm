use std::ops::Index;

use indexmap::IndexMap;
use parity_wasm::elements::{Func, Instruction};

use crate::{build_func_cfg, graph};
use crate::{
    BlockNum, CFGBuilder, CallGraphBuilder, FuncIndex, Function, Imports, Op, Program,
    ProgramVisitor,
};

type ProgramError = crate::ProgramError<FuncIndex>;

mod resolver;
pub use resolver::PriceResolver;

pub mod resolvers;

mod cfg;
pub use cfg::build_weighted_graph;

mod func_price;
pub use func_price::FuncPrice;

pub struct ProgramPricing<R>
where
    R: PriceResolver,
{
    current_func: Option<FuncIndex>,

    builder: CallGraphBuilder<FuncIndex>,

    resolver: R,
}

impl<R> ProgramPricing<R>
where
    R: PriceResolver,
{
    pub fn new(resolver: R) -> Self {
        Self {
            current_func: None,
            builder: CallGraphBuilder::new(),
            resolver,
        }
    }

    pub fn run(
        self,
        program: &Program,
    ) -> Result<<Self as ProgramVisitor>::Output, <Self as ProgramVisitor>::Error> {
        self.visit(program)
    }

    fn current_func(&self) -> FuncIndex {
        self.current_func.unwrap()
    }

    fn add_target(&mut self, label: FuncIndex) {
        self.builder.add_target(label);
    }

    fn add_call(&mut self, op: &Op, origin: FuncIndex, target: FuncIndex) {
        debug_assert!(origin != target);

        self.builder.add_call(origin, target);
    }
}

impl<R> ProgramVisitor for ProgramPricing<R>
where
    R: PriceResolver,
{
    type Error = ProgramError;

    type Output = FuncPrice;

    fn on_start(&mut self, _program: &Program) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_end(mut self, program: &Program) -> Result<Self::Output, Self::Error> {
        let mut call_graph = self.builder.build();
        let mut sorted = graph::topological_sort(&call_graph);

        let mut func_price = FuncPrice::new();
        let imports = Imports::new();

        while let Some(fn_index) = sorted.pop() {
            let price = compute_func_price(program, fn_index, &self.resolver, &func_price);

            func_price.set(fn_index, price);
        }

        Ok(func_price)
    }

    fn on_func_start(
        &mut self,
        fn_index: FuncIndex,
        _program: &Program,
    ) -> Result<(), Self::Error> {
        self.current_func = Some(fn_index);

        self.add_target(fn_index);

        Ok(())
    }

    fn on_func_end(&mut self, fn_index: FuncIndex, _program: &Program) -> Result<(), Self::Error> {
        self.current_func = None;

        Ok(())
    }

    fn on_op(&mut self, op: &Op, program: &Program) -> Result<(), Self::Error> {
        if let Instruction::Call(target) = *op.raw() {
            let target = FuncIndex(target);

            if program.is_imported(target) == false {
                let origin = self.current_func();

                self.add_call(op, origin, target);
            }
        }

        Ok(())
    }
}

fn compute_func_price<R>(
    program: &Program,
    func_index: FuncIndex,
    resolver: &R,
    func_price: &FuncPrice,
) -> usize
where
    R: PriceResolver,
{
    let func = program.get_func(func_index);
    let imports = program.imports();

    let func_cfg = build_func_cfg(&func);
    let func_graph = build_weighted_graph(&func_cfg, resolver, &imports, func_price);

    let start = func_cfg.start();
    let end = func_cfg.end();

    let path = graph::compute_max_weight_path(&func_graph, start, end);

    path.total()
}
