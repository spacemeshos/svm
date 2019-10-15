use crate::block::{BlockOffsets, IfBlockOffsets};
use crate::error::Error;
use crate::function::{FuncBody, FuncIndex};
use crate::gas::Gas;
use crate::program::Program;

use std::collections::HashMap;
use std::iter::Iterator;

use parity_wasm::elements::{Instruction, Instructions};

pub(crate) struct ProgramState {
    /// Current call-stack derived from program static-analysis
    call_stack: Vec<FuncIndex>,

    /// Cache of already estimated functions' gas
    func_gas_cache: HashMap<FuncIndex, Gas>,
}

impl ProgramState {
    fn new() -> Self {
        Self {
            call_stack: Vec::new(),
            func_gas_cache: HashMap::new(),
        }
    }

    fn record_func_call_start(&mut self, func_idx: FuncIndex) -> Result<(), Error> {
        if self.call_stack.contains(&func_idx) {
            // The function `func_idx` is already in the call-stack.
            // We've a recursive call and return an error
            //
            // Here is an example:
            //
            // ```wasm
            // (module
            //   (func $function_0 (param i32 i32) (result i32)
            //     ...
            //     ...
            //
            //     ;; here our call-stack contains the current function, `$function_0`
            //     ...
            //     ...
            //
            //     ;; next instruction is a recursive call to `$function_0`
            //     ;; since we already have `$function_0` in the static-analysis call-stack will error
            //     call $function_0
            //     ....
            //   ))
            // ```

            let mut recursive_chain = self.call_stack.clone();
            recursive_chain.push(func_idx);

            Err(Error::RecursiveCall(recursive_chain))
        } else {
            self.call_stack.push(func_idx);
            Ok(())
        }
    }

    fn record_func_call_end(&mut self, func_idx: FuncIndex) -> Result<(), Error> {
        assert_eq!(func_idx, self.call_stack.pop().unwrap());
        Ok(())
    }

    fn contains_gas_estimation_for(&self, func_idx: FuncIndex) -> bool {
        // This can happen with the following scenario:
        // ```wasm
        // (module
        //   (func $function_0 (param i32 i32) (result i32)
        //     ...
        //     ...
        //
        //     call $function_1
        //
        //     ...
        //   )
        //
        //  (func $function_1 (result i32)
        //    ...))
        // ```
        //
        // Now, if we call `estimate_function_gas` for `$function_0` piror calling `function_1`,
        // when we reach the `call $function_1` instruction, we'll procceed to `estimate_function_gas` with `function_1`.
        //
        // Later, we'll invoke `estimate_function_gas` with `function_1` from `estimate_program_gas`.
        // This invocation will return the already computed function gas for `function_1` and return.
        self.func_gas_cache.contains_key(&func_idx)
    }

    fn gas_estimation_for(&self, func_idx: FuncIndex) -> Gas {
        match self.func_gas_cache.get(&func_idx) {
            Some(gas) => gas.clone(),
            None => panic!("expects gas estimation for function: `{}`", func_idx.0),
        }
    }

    fn set_gas_estimation(&mut self, func_idx: FuncIndex, gas: Gas) {
        self.func_gas_cache.insert(func_idx, gas);
    }
}

#[derive(Debug, Clone)]
struct BlockState {
    pub cursor: usize,
    pub gas: Gas,
    eof: usize,
}

impl BlockState {
    fn new(block_offsets: BlockOffsets) -> Self {
        Self {
            gas: Gas::Fixed(0),
            cursor: block_offsets.0,
            eof: block_offsets.1,
        }
    }

    fn is_eof(&self) -> bool {
        self.cursor > self.eof
    }

    fn advance_cursor(&mut self) {
        self.cursor += 1;
    }

    fn back_cursor(&mut self) {
        assert!(self.cursor > 0);

        self.cursor -= 1;
    }

    fn inc_gas(&mut self) {
        self.gas = self.gas * Gas::Fixed(1);
    }
}

pub fn estimate_program_gas(program: &Program) -> Result<HashMap<FuncIndex, Gas>, Error> {
    let mut state = ProgramState::new();

    for func_idx in program.functions_ids().iter() {
        estimate_function_gas(*func_idx, program, &mut state)?;
    }

    Ok(state.func_gas_cache)
}

fn estimate_function_gas<'a>(
    func_idx: FuncIndex,
    program: &Program,
    program_state: &mut ProgramState,
) -> Result<Gas, Error> {
    if program.is_imported(func_idx) {
        return estimate_vmcall(func_idx, program);
    }

    if program_state.contains_gas_estimation_for(func_idx) {
        let func_gas = program_state.gas_estimation_for(func_idx);
        return Ok(func_gas.clone());
    }

    program_state.record_func_call_start(func_idx)?;

    let func_gas = do_function_gas_estimation(program, program_state, func_idx)?;
    program_state.set_gas_estimation(func_idx, func_gas);

    program_state.record_func_call_end(func_idx)?;

    Ok(func_gas)
}

fn estimate_vmcall(func_idx: FuncIndex, program: &Program) -> Result<Gas, Error> {
    unimplemented!()
}

fn do_function_gas_estimation(
    program: &Program,
    program_state: &mut ProgramState,
    func_idx: FuncIndex,
) -> Result<Gas, Error> {
    let func_body = program.get_function_body(func_idx).to_vec();
    let block_offsets = BlockOffsets(0, func_body.len() - 1);

    let (cont_offset, func_gas) =
        estimate_block(program, program_state, &func_body, block_offsets)?;

    // TODO:
    // asserting we've went through all the instructions of the function `func_idx`

    Ok(func_gas)
}

fn estimate_block<'a>(
    program: &Program,
    program_state: &mut ProgramState,
    func_body: &Vec<Instruction>,
    block_offsets: BlockOffsets,
) -> Result<(usize, Gas), Error> {
    let mut block_state = BlockState::new(block_offsets);

    while !(block_state.is_eof()) {
        let op = func_body.get(block_state.cursor).unwrap();

        match op {
            Instruction::End => {
                // End of the current block, `block_state.cursor` now points to the continuation of the parent block
                break;
            }
            Instruction::Block(_) => {
                block_state.advance_cursor();

                let inner_offsets = BlockOffsets(block_state.cursor, func_body.len());
                let (cont_offset, inner_gas) =
                    estimate_block(program, program_state, func_body, inner_offsets)?;

                block_state.cursor = cont_offset;
                block_state.gas = block_state.gas * inner_gas;
            }
            Instruction::Call(func_idx) => {
                let called_func_gas =
                    estimate_function_gas(FuncIndex(*func_idx), program, program_state)?;

                block_state.advance_cursor();
                block_state.gas *= called_func_gas;
            }
            Instruction::If(_) => {
                let (_, if_offsets) =
                    find_if_stmt_boundaries(program, program_state, func_body, &mut block_state)?;

                let (_, true_gas) =
                    estimate_block(program, program_state, func_body, if_offsets.true_offsets)?;

                let else_gas = if if_offsets.else_offsets.is_some() {
                    let (_, gas) = estimate_block(
                        program,
                        program_state,
                        func_body,
                        if_offsets.else_offsets.unwrap(),
                    )?;

                    gas
                } else {
                    Gas::Fixed(0)
                };

                let if_gas = true_gas + else_gas;
                block_state.gas *= if_gas;
            }
            Instruction::Loop(_) => return Err(Error::LoopNotAllowed),
            Instruction::Br(_) => return Err(Error::BrNotAllowed),
            Instruction::BrIf(_) => return Err(Error::BrIfNotAllowed),
            Instruction::BrTable(_) => return Err(Error::BrTableNotAllowed),
            Instruction::CallIndirect(..) => return Err(Error::CallIndirectNotAllowed),
            Instruction::Nop => block_state.advance_cursor(),
            (_) => {
                block_state.inc_gas();
                block_state.advance_cursor();
            }
        };
    }

    Ok((block_state.cursor, block_state.gas))
}

fn find_if_stmt_boundaries<'a>(
    program: &Program,
    program_state: &mut ProgramState,
    func_body: &Vec<Instruction>,
    block_state: &mut BlockState,
) -> Result<(usize, IfBlockOffsets), Error> {
    let (true_offsets, has_else) = find_if_stmt_true_block(func_body, block_state)?;

    let else_offsets = if has_else {
        Some(find_if_stmt_else_block(func_body, block_state)?)
    } else {
        None
    };

    let offsets = IfBlockOffsets {
        true_offsets,
        else_offsets,
    };

    Ok((0, offsets))
}

fn find_if_stmt_true_block(
    func_body: &Vec<Instruction>,
    block_state: &mut BlockState,
) -> Result<(BlockOffsets, bool), Error> {
    let op = func_body.get(block_state.cursor).unwrap();

    match op {
        Instruction::If(_) => block_state.advance_cursor(),
        _ => panic!("expects block to be an if-statement block"),
    };

    let (true_start, mut true_end): (usize, usize) = (block_state.cursor, block_state.cursor);

    let mut block_depth = 1;
    let mut found_else = false;

    while !(block_state.is_eof()) {
        let op = func_body.get(block_state.cursor).unwrap();

        match op {
            Instruction::Loop(_) => return Err(Error::LoopNotAllowed),
            Instruction::Block(_) | Instruction::If(_) => block_depth += 1,
            Instruction::Else if block_depth == 1 => {
                // the if-statement has an `else block`
                true_end = block_state.cursor;
                found_else = true;
                break;
            }
            Instruction::End => {
                block_depth -= 1;

                if (block_depth == 0) {
                    // the if-statement has no `else block`
                    true_end = block_state.cursor;
                    break;
                }
            }
            _ => (),
        }

        block_state.advance_cursor();
    }

    if block_state.is_eof() {
        panic!("invalid if-statement");
    }

    let offsets = BlockOffsets(true_start, true_end - 1);
    Ok((offsets, found_else))
}

fn find_if_stmt_else_block(
    func_body: &Vec<Instruction>,
    block_state: &mut BlockState,
) -> Result<BlockOffsets, Error> {
    let op = func_body.get(block_state.cursor).unwrap();
    match op {
        Instruction::Else => block_state.advance_cursor(),
        _ => panic!("expects block to start with `else` block"),
    };

    let else_start = block_state.cursor;
    let mut else_end = 0;

    let mut block_depth = 1;

    while !(block_state.is_eof()) {
        let op = func_body.get(block_state.cursor).unwrap();
        match op {
            Instruction::Block(_) | Instruction::If(_) => {
                block_depth += 1;
            }
            Instruction::End => {
                block_depth -= 1;

                if (block_depth == 0) {
                    else_end = block_state.cursor - 1;
                    break;
                }
            }
            Instruction::Loop(_) => return Err(Error::LoopNotAllowed),
            _ => block_state.advance_cursor(),
        }
    }

    if block_state.is_eof() {
        panic!("invalid if-statement");
    }

    assert!(else_end >= else_start);

    Ok(BlockOffsets(else_start, else_end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    macro_rules! estimate_gas {
        ($code:expr) => {{
            use crate::code_reader::read_program;

            let wasm = wabt::wat2wasm($code).unwrap();
            let program = read_program(&wasm);
            dbg!(&program);

            estimate_program_gas(&program)
        }};
    }

    #[test]
    fn function_gas_nop_function() {
        let code = r#"
          (module
            (func $func0
                (nop))

            (func $func1
                (block (nop)))

            (func $func2
                (block (block (nop)))))
        "#;

        let res = estimate_gas!(code);
        assert_eq!(
            hashmap! {
                FuncIndex(0) => Gas::Fixed(0),
                FuncIndex(1) => Gas::Fixed(0),
                FuncIndex(2) => Gas::Fixed(0)
            },
            res.unwrap()
        );
    }

    #[test]
    fn function_gas_constant_function() {
        let code = r#"
          (module
            (func $func0 (result i32)
                i32.const 10
                drop
                i32.const 20))
        "#;

        let res = estimate_gas!(code);
        assert_eq!(hashmap! {FuncIndex(0) => Gas::Fixed(3)}, res.unwrap());
    }

    #[test]
    fn function_gas_loop_not_allowed() {
        let code = r#"
          (module
            (func $func0
                (loop (nop))))
        "#;

        let res = estimate_gas!(code);
        assert_eq!(Err(Error::LoopNotAllowed), res);
    }

    #[test]
    fn function_gas_direct_recursive_call_not_allowed() {
        let code = r#"
          (module
            (func $func0
                (call $func0)))
        "#;

        let res = estimate_gas!(code);
        assert_eq!(
            Err(Error::RecursiveCall(vec![FuncIndex(0), FuncIndex(0)])),
            res
        );
    }

    #[test]
    fn function_gas_indirct_recursive_call_not_allowed() {
        let code = r#"
          (module
            (func $func0
                (call $func1))

            (func $func1
                (call $func2))

            (func $func2
                (call $func0)))
        "#;

        let res = estimate_gas!(code);
        assert_eq!(
            Err(Error::RecursiveCall(vec![
                FuncIndex(0),
                FuncIndex(1),
                FuncIndex(2),
                FuncIndex(0),
            ])),
            res
        );
    }

    #[test]
    fn function_gas_call_indirect_not_allowed() {
        let code = r#"
          (module
            (type $proc (func))

            (table funcref
                (elem
                    $func0))

            (func $func0 (type $proc)
                (nop))

            (func $func1
                (call_indirect (type $proc) (i32.const 0))))
        "#;

        let res = estimate_gas!(code);
        assert_eq!(Err(Error::CallIndirectNotAllowed), res);
    }

    #[test]
    fn function_gas_br_not_allowed() {
        let code = r#"
          (module
            (func $func0
                (br 0))

            (func $func1
                (block (br 0))))
        "#;

        let res = estimate_gas!(code);
        assert_eq!(Err(Error::BrNotAllowed), res);
    }

    #[test]
    #[ignore]
    fn function_gas_br_if_not_allowed() {
        let code = r#"
          (module
            (func $func0
                (block (br_if 0 1) (i32.const 0))))
        "#;

        let res = estimate_gas!(code);
        assert_eq!(Err(Error::BrIfNotAllowed), res);
    }

    #[test]
    fn function_gas_if_stmt_without_else() {
        let code = r#"
          (module
            (func $func0
                (i32.const 0)
                (drop)

                ;; here we have gas cost = fixed(2)

                (if (i32.const 1)
                    ;; if-condition costs fixed(1) gas

                    (then
                        ;; block gas cost = fixed(4)
                        (i32.const 2)
                        (i32.const 3)
                        (i32.add)
                        (drop)
                    ))))

                ;; total function `func0` gas:
                ;; * before if-stmt: fixed(2)
                ;; * if-condition: fixed(1)
                ;; * true-block: range(0, 4)
                ;;
                ;; total function `func0` gas:
                ;; fixed(2) * fixed(1) * range(0, 4) = fixed(3) * range(0, 4) = range(3, 7)
        "#;

        let res = estimate_gas!(code);
        assert_eq!(
            hashmap! {
                FuncIndex(0) => Gas::Range { min: 3, max: 7 }
            },
            res.unwrap()
        );
    }

    #[test]
    fn function_gas_if_stmt_without_else_nested() {
        let code = r#"
          (module
            (func $func0
                (i32.const 0)
                (drop)

                ;; here we have gas cost = fixed(2)

                (if (i32.const 1)
                    ;; if-condition costs fixed(1) gas
                    (then
                        ;; block gas cost = fixed(4)
                        (i32.const 2)
                        (i32.const 3)
                        (i32.add)
                        (drop)

                        (if (i32.const 4)
                            ;; if-condition costs fixed(1) gas
                            (then
                                ;; block gas = fixed(6)
                                (i32.const 5)
                                (i32.const 6)
                                (i32.add)
                                (i32.const 7)
                                (i32.add)
                                (drop)))
                    ))))

                ;; total function `func0` gas:
                ;; * before if-stmt: fixed(2)
                ;; * if-condition: fixed(1)
                ;; * if-statement:
                ;;      * preamble: fixed(4)
                ;;      * inner if-statement:
                ;;          * if-condition: fixed(1)
                ;;          * true-block: fixed(6)
                ;;      inner-if statement total: fixed(1) * range(0, 6) = range(1, 7)
                ;;
                ;;   if-statement total: fixed(1) + (fixed(4) * range(1, 7)) = fixed(1) + range(5, 11) = range(1, 12)
                ;;
                ;; total function `func0` gas:
                ;; fixed(2) * range(1, 12) = range(3, 14)
        "#;

        let res = estimate_gas!(code);
        assert_eq!(
            hashmap! {
                FuncIndex(0) => Gas::Range { min: 3, max: 14 }
            },
            res.unwrap()
        );
    }

    #[test]
    fn function_gas_if_stmt_with_else() {
        let code = r#"
          (module
            (func $func0
                (i32.const 0)            ;; 0
                (drop)                   ;; 1

                ;; here we have gas cost = fixed(2)

                (if (i32.const 1)        ;; 2 + 3

                    ;; if-condition costs fixed(1) gas

                    (then
                        ;; block gas cost = fixed(4)
                        (i32.const 2)    ;; 4
                        (i32.const 3)    ;; 5
                        (i32.add)        ;; 6
                        (drop)           ;; 7
                    )
                    (else                ;; 8
                        ;; block gas cost = fixed(0)

                        (nop)            ;; 9
                    ))))

                ;; total function `func0` gas:
                ;; * before if-stmt: fixed(2)
                ;; * if-condition: fixed(1)
                ;; * true-block: range(0, 4)
                ;; * else-block: fixed(0)
                ;;
                ;; total function `func0` gas:
                ;; fixed(2) * fixed(1) * range(0, 4) = fixed(3) * range(0, 4) = range(3, 7)
        "#;

        let res = estimate_gas!(code);
        assert_eq!(
            hashmap! {
                FuncIndex(0) => Gas::Range { min: 3, max: 7 }
            },
            res.unwrap()
        );
    }
}
