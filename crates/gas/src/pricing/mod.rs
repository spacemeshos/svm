use parity_wasm::elements::Instruction;

use crate::{CallGraph, Gas, Program};

mod depth;
pub use depth::Depth;

mod function;
pub use function::{FuncIterator, Function};

mod block;
pub use block::{Block, BlockBuilder, BlockNum, BlockRef};

mod jump;
pub use jump::{Jump, UnresolvedJump, WasmJump};

mod cfg;
pub use cfg::{build_func_cfg, CFGBuilder, Cont, Edge, Op, CFG};

pub fn price_for(program: &Program) -> Gas {
    todo!()
    // for index in program.functions() {
    //     let body = program.get_func_body(index);
    //     let func = Function::new(index, body.instructions());

    //     let cfg = build_cfg(func);
    // }

    // Gas::Fixed(0)
}
