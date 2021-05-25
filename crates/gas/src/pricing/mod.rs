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

mod cfg;
pub use cfg::build_weighted_graph;

mod func_price;
pub use func_price::FuncPrice;

pub struct ProgramPricing<'p, F, R>
where
    R: PriceResolver,
    F: FnOnce(FuncPrice),
{
    current_func: Option<FuncIndex>,

    builder: CallGraphBuilder<FuncIndex>,

    program: &'p Program,

    resolver: R,

    callback: F,
}

impl<'p, F, R> ProgramPricing<'p, F, R>
where
    R: PriceResolver,
    F: FnOnce(FuncPrice),
{
    pub fn new(program: &'p Program, resolver: R, callback: F) -> Self {
        Self {
            current_func: None,
            builder: CallGraphBuilder::new(),
            program,
            callback,
            resolver,
        }
    }

    pub fn current_func(&self) -> FuncIndex {
        self.current_func.unwrap()
    }

    pub fn add_target(&mut self, label: FuncIndex) {
        self.builder.add_target(label);
    }

    pub fn add_call(&mut self, op: &Op, origin: FuncIndex, target: FuncIndex) {
        debug_assert!(origin != target);

        self.builder.add_call(origin, target);
    }
}

impl<F, R> ProgramVisitor for ProgramPricing<'_, F, R>
where
    R: PriceResolver,
    F: FnOnce(FuncPrice),
{
    type Error = ProgramError;

    fn on_start(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_end(mut self) -> Result<(), Self::Error> {
        let mut call_graph = self.builder.build();

        let mut sorted = graph::topological_sort(&call_graph);

        let mut func_price = FuncPrice::new();
        let imports = Imports::new();

        while let Some(func_index) = sorted.pop() {
            let price = compute_func_price(&self.program, func_index, &self.resolver, &func_price);

            func_price.set(func_index, price);
        }

        (self.callback)(func_price);

        Ok(())
    }

    fn on_func_start(&mut self, fn_index: FuncIndex) -> Result<(), Self::Error> {
        self.current_func = Some(fn_index);

        self.add_target(fn_index);

        Ok(())
    }

    fn on_func_end(&mut self, func_index: FuncIndex) -> Result<(), Self::Error> {
        self.current_func = None;

        Ok(())
    }

    fn on_op(&mut self, op: &Op) -> Result<(), Self::Error> {
        if let Instruction::Call(target) = *op.raw() {
            let target = FuncIndex(target);

            if self.program.is_imported(target) == false {
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
