use crate::block::{BlockOffsets, IfBlockOffsets};
use crate::cursor::Cursor;
use crate::error::Error;
use crate::function::FuncIndex;
use crate::gas::Gas;
use crate::program::Program;

use std::collections::HashMap;

use parity_wasm::elements::Instruction;

struct ProgramState {
    call_stack: Vec<FuncIndex>,

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
        // Now, if we call `` for `$function_0` piror calling `function_1`,
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
    cursor: Cursor,
    gas: Gas,
    eof: usize,
}

impl BlockState {
    fn new(block_offsets: BlockOffsets) -> Self {
        Self {
            gas: Gas::Fixed(0),
            cursor: Cursor::new(block_offsets.0),
            eof: block_offsets.1,
        }
    }

    fn is_eof(&self) -> bool {
        self.cursor.get() > self.eof
    }

    fn advance_cursor(&mut self) {
        self.cursor.next();
    }

    fn inc_gas(&mut self) {
        self.gas *= Gas::Fixed(1);
    }
}

/// Receives a wasm program reprsented as a vector of parsed wasm instructions.
/// On success returns for each function the `Gas` it requires.
/// Otherwise, returns an `crate::error::Error`
pub fn estimate_program_gas(program: &Program) -> Result<HashMap<FuncIndex, Gas>, Error> {
    // dbg!(program);

    let mut program_state = ProgramState::new();

    // we sort `functions_ids`, this is important in order to maintain determinsitic execution of unit-tests
    let mut functions_ids: Vec<FuncIndex> = program.functions_ids().clone();
    functions_ids.sort();

    for func_idx in functions_ids.drain(..) {
        estimate_function_gas(func_idx, program, &mut program_state)?;
    }

    Ok(program_state.func_gas_cache)
}

fn estimate_function_gas(
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

    log::info!("estimating gas for function {:?}", func_idx);

    program_state.record_func_call_start(func_idx)?;
    let func_gas = do_function_gas_estimation(program, program_state, func_idx)?;

    log::info!("estimated gas for function {:?}: {:?}", func_idx, func_gas);

    program_state.set_gas_estimation(func_idx, func_gas);
    program_state.record_func_call_end(func_idx)?;

    Ok(func_gas)
}

fn estimate_vmcall(_func_idx: FuncIndex, _program: &Program) -> Result<Gas, Error> {
    unimplemented!()
}

fn do_function_gas_estimation(
    program: &Program,
    program_state: &mut ProgramState,
    func_idx: FuncIndex,
) -> Result<Gas, Error> {
    let func_body = program.get_function_body(func_idx).to_vec();
    let block_offsets = BlockOffsets(0, func_body.len() - 1);

    let (eof_offset, func_gas) =
        estimate_block(func_idx, program, program_state, &func_body, block_offsets)?;

    assert_eq!(func_body.len(), eof_offset);

    Ok(func_gas)
}

/// estimated the current pointed to block and returns its gas and the offset for the instruction following it
/// (a.k.a continuation offset)
fn estimate_block(
    func_idx: FuncIndex,
    program: &Program,
    program_state: &mut ProgramState,
    func_body: &Vec<Instruction>,
    block_offsets: BlockOffsets,
) -> Result<(usize, Gas), Error> {
    log::debug!(
        "[{:?}] about to estimate gas for block [{}, {}] ...",
        func_idx,
        block_offsets.0,
        block_offsets.1,
    );

    let mut block_state = BlockState::new(block_offsets);

    while !(block_state.is_eof()) {
        let op = func_body.get(block_state.cursor.get()).unwrap();

        match op {
            Instruction::End => {
                // End of the current block, `block_state.cursor` now points to the continuation of the parent block
                block_state.advance_cursor();
                break;
            }
            Instruction::Block(_) => {
                block_state.advance_cursor();

                let inner_offsets = BlockOffsets(block_state.cursor.get(), block_offsets.1);
                let (cont_offset, inner_gas) =
                    estimate_block(func_idx, program, program_state, func_body, inner_offsets)?;

                block_state.cursor.set(cont_offset);
                block_state.gas *= inner_gas;
            }
            Instruction::Call(called_func) => {
                let called_func_gas =
                    estimate_function_gas(FuncIndex(*called_func), program, program_state)?;

                block_state.advance_cursor();
                block_state.gas *= called_func_gas;
            }
            Instruction::If(_) => {
                let (cont_offset, if_offsets) =
                    find_if_stmt_boundaries(func_idx, func_body, &mut block_state)?;

                let (_, true_gas) = estimate_block(
                    func_idx,
                    program,
                    program_state,
                    func_body,
                    if_offsets.true_offsets,
                )?;

                let else_gas = if if_offsets.else_offsets.is_some() {
                    let (_, gas) = estimate_block(
                        func_idx,
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
                block_state.cursor.set(cont_offset);
            }
            Instruction::Loop(_) => return Err(Error::LoopNotAllowed),
            Instruction::Br(_) => return Err(Error::BrNotAllowed),
            Instruction::BrIf(_) => return Err(Error::BrIfNotAllowed),
            Instruction::BrTable(_) => return Err(Error::BrTableNotAllowed),
            Instruction::CallIndirect(..) => return Err(Error::CallIndirectNotAllowed),
            Instruction::Nop => block_state.advance_cursor(),
            _ => {
                block_state.inc_gas();
                block_state.advance_cursor();
            }
        };
    }

    log::debug!(
        "[{:?}] estimated gas for block [{}, {}]: {:?}",
        func_idx,
        block_offsets.0,
        block_offsets.1,
        block_state.gas
    );

    Ok((block_state.cursor.get(), block_state.gas))
}

/// returns the if-statement `true-block` **inclusive** offsets and the continuation offset
/// (the offset following the if-statement)
fn find_if_stmt_boundaries(
    func_idx: FuncIndex,
    func_body: &Vec<Instruction>,
    block_state: &mut BlockState,
) -> Result<(usize, IfBlockOffsets), Error> {
    log::debug!(
        "[{:?}] seeking if-stmt boundaries starting from offset={}...",
        func_idx,
        block_state.cursor.get()
    );

    let (true_cont_offset, true_offsets, has_else) =
        find_if_stmt_true_block(func_body, block_state)?;

    log::debug!(
        "[{:?}] found if-stmt true-block at [{}, {}]",
        func_idx,
        true_offsets.0,
        true_offsets.1
    );

    let (if_cont_offset, else_offsets) = if has_else {
        block_state.cursor.set(true_cont_offset);
        block_state.cursor.prev();
        // now `block_state.cursor` points to the `else` instruction

        let (else_cont_offset, offsets) = find_if_stmt_else_block(func_body, block_state)?;

        log::debug!(
            "[{:?}] found if-stmt else-block at [{}, {}]",
            func_idx,
            offsets.0,
            offsets.1
        );

        (else_cont_offset, Some(offsets))
    } else {
        (true_cont_offset, None)
    };

    let offsets = IfBlockOffsets {
        true_offsets,
        else_offsets,
    };

    Ok((if_cont_offset, offsets))
}

/// returns the if-statement `true-block` **inclusive** offsets and whether it has an `else-block`
fn find_if_stmt_true_block(
    func_body: &Vec<Instruction>,
    block_state: &mut BlockState,
) -> Result<(usize, BlockOffsets, bool), Error> {
    let op = func_body.get(block_state.cursor.get()).unwrap();

    match op {
        Instruction::If(_) => block_state.advance_cursor(),
        _ => panic!("expects block to be an if-statement block"),
    };

    let (true_start, mut true_end): (usize, usize) =
        (block_state.cursor.get(), block_state.cursor.get());

    let mut block_depth = 1;
    let mut found_else = false;

    while !(block_state.is_eof()) {
        let op = func_body.get(block_state.cursor.get()).unwrap();

        match op {
            Instruction::Loop(_) => return Err(Error::LoopNotAllowed),
            Instruction::Block(_) | Instruction::If(_) => block_depth += 1,
            Instruction::Else if block_depth == 1 => {
                // the if-statement has an `else block`
                block_state.cursor.prev();
                true_end = block_state.cursor.get();
                found_else = true;
                break;
            }
            Instruction::End => {
                block_depth -= 1;

                if block_depth == 0 {
                    // the if-statement has no `else block`
                    block_state.cursor.prev();
                    true_end = block_state.cursor.get();
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

    block_state.cursor.next();
    block_state.cursor.next();
    let cont_offset = block_state.cursor.get();

    let true_offsets = BlockOffsets(true_start, true_end);
    Ok((cont_offset, true_offsets, found_else))
}

/// returns the if-statement `else-block` **inclusive** offsets
fn find_if_stmt_else_block(
    func_body: &Vec<Instruction>,
    block_state: &mut BlockState,
) -> Result<(usize, BlockOffsets), Error> {
    let op = func_body.get(block_state.cursor.get()).unwrap();
    match op {
        Instruction::Else => block_state.advance_cursor(),
        _ => panic!("expects block to start with `else` block"),
    };

    block_state.advance_cursor();
    let else_start = block_state.cursor.get();
    let mut else_end = 0;

    let mut block_depth = 1;

    while !(block_state.is_eof()) {
        let op = func_body.get(block_state.cursor.get()).unwrap();

        match op {
            Instruction::Block(_) | Instruction::If(_) => {
                block_depth += 1;
            }
            Instruction::End => {
                block_depth -= 1;

                if block_depth == 0 {
                    block_state.cursor.prev();
                    else_end = block_state.cursor.get();
                    break;
                }
            }
            Instruction::Loop(_) => return Err(Error::LoopNotAllowed),
            _ => (),
        }

        block_state.advance_cursor();
    }

    if block_state.is_eof() {
        panic!("invalid if-statement");
    }

    assert!(else_end >= else_start);

    let else_offsets = BlockOffsets(else_start, else_end);

    block_state.cursor.next();
    block_state.cursor.next();
    let cont_offset = block_state.cursor.get();

    Ok((cont_offset, else_offsets))
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
    fn function_gas_indirect_recursive_call_not_allowed() {
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
    fn function_gas_br_if_not_allowed() {
        let code = r#"
          (module
            (func $func0 (result i32)
                (block (result i32) (br_if 0 (i32.const 0) (i32.const 0)))))
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
                    )
                    (else
                        ;; block gas cost = fixed(2)
                        (i32.const 0)
                        (drop)
                    ))))

                ;; total function `func0` gas:
                ;; * before if-stmt: fixed(2)
                ;; * if-condition: fixed(1)
                ;; * if-stmt true-block: fixed(4)
                ;; * if-stmt else-block: fixed(2)
                ;;
                ;; if-stmt total gas: range(2, 4)
                ;;
                ;; total function `func0` gas:
                ;; fixed(2) * fixed(1) * range(2, 4) = fixed(3) * range(2, 4) = range(5, 7)
        "#;

        let res = estimate_gas!(code);
        assert_eq!(
            hashmap! {
                FuncIndex(0) => Gas::Range { min: 5, max: 7 }
            },
            res.unwrap()
        );
    }

    #[test]
    fn function_gas_if_stmt_with_else_nested() {
        env_logger::init();

        let code = r#"
          (module
            (func $func0
                (i32.const 0)                                       ;; 0
                (drop)                                              ;; 1

                ;; here we have gas cost = fixed(2)

                (if (i32.const 1)                                   ;; 2 + 3
                    ;; if-condition costs fixed(1) gas

                    (then
                        ;; block gas cost = fixed(4)
                        (i32.const 2)                               ;; 4
                        (i32.const 3)                               ;; 5
                        (i32.add)                                   ;; 6
                        (drop)                                      ;; 7
                    )
                    (else                                           ;; 8
                        (i32.const 4)                               ;; 9
                        (drop)                                      ;; 10

                        (if (i32.const 5)                           ;; 11 + 12
                            ;; if-condition costs fixed(1) gas

                            (then
                                ;; block gas cost = fixed(2)
                                (i32.const 6)                       ;; 13
                                (drop)                              ;; 14
                            )
                            (else                                   ;; 15
                                ;; block gas cost = fixed(6)
                                (i32.const 7)                       ;; 16
                                (i32.const 8)                       ;; 17
                                (i32.const 9)                       ;; 18
                                (i32.add)                           ;; 19
                                (i32.add)                           ;; 20
                                (drop)                              ;; 21
                            ))))))                                  ;; 22 + 23 + 24


                ;; total function `func0` gas:
                ;; * before if-stmt: fixed(2)
                ;; * if-condition: fixed(1)
                ;; * if-stmt true-block: fixed(4)
                ;; * if-stmt else-block:
                ;;      * preamble: fixed(2)
                ;;      * inner if-condition: fixed(1)
                ;;      * inner if-stmt true-block: fixed(2)
                ;;      * inner if-stmt else-block: fixed(6)
                ;;          => inner-if stmt gas = fixed(1) * (fixed(2) + fixed(6)) = fixed(1) * range(2, 6) = range(3, 7)
                ;;      => if-stmt else-block total gas = fixed(2) * range(3, 7) = range(5, 9)
                ;;  => if-stmt total gas = fixed(1) * (fixed(4) + range(5, 9)) = fixed(1) * range(4, 9) = range(5, 10)
                ;;
                ;; total function `func0` gas:
                ;; fixed(2) * range(5, 10) = range(7, 12)
        "#;

        let _res = estimate_gas!(code);
        // assert_eq!(
        //     hashmap! {
        //         FuncIndex(0) => Gas::Range { min: 7, max: 12 }
        //     },
        //     res.unwrap()
        // );
    }
}
