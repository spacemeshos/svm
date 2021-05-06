#![allow(unused)]

use svm_gas::{build_func_cfg, read_program, BlockNum, FuncIndex, Function, Program};

fn parse_wasm(wasm: &str) -> Program {
    let wasm = wat::parse_str(wasm).unwrap();

    read_program(&wasm).unwrap()
}

fn get_func(program: &Program, fn_index: u16) -> Function {
    let func = FuncIndex(fn_index);
    let body = program.get_func_body(func);
    let instructions = body.instructions();

    dbg!(&instructions);

    Function::new(func, instructions)
}

macro_rules! cfg {
    (id: $id:expr, blocks: [ $( $block:expr ),* ]) => {{
        use svm_gas::CFG;

        let mut blocks = Vec::new();

        $( blocks.push($block); )*

        CFG { blocks }
    }};
}

macro_rules! block {
    (
      id: $id:expr,
      offset: $offset:expr,
      ops: [ $( $op:expr ),* ],
      in_jumps: [ $( $a:expr ),* ],
      out_jumps: [ $( $b:expr ),* ],
      in_cont: [ $( $c:expr ),* ],
      out_cont: [ $( $d:expr ),* ]
    ) => {{
        use indexmap::IndexSet;
        use parity_wasm::elements::{Instruction, BlockType};

        use svm_gas::{Block, BlockNum, Op, Cont, Jump, Edge};

        let mut ops: Vec<Op> = Vec::new();

        $(
            let op = Op::new(&$op, 0, false);
            ops.push(op);
        )*

        for (i, op) in ops.iter_mut().enumerate() {
            op.offset = $offset + i;
        }

        if let Some(last) = ops.last_mut() {
            last.is_last = true;
        }

        let mut incoming_edges: IndexSet<Edge> = IndexSet::new();
        let mut outgoing_edges: IndexSet<Edge> = IndexSet::new();

        let me = BlockNum($id);

        $(
            let jump = Jump { origin: BlockNum($a), target: me };
            let edge = Edge::Jump(jump);

            incoming_edges.insert(edge);
        )*

        $(
            let jump = Jump { origin: me, target: BlockNum($b) };
            let edge = Edge::Jump(jump);

            outgoing_edges.insert(edge);
        )*

        $(
            let cont = Cont { origin: BlockNum($c), target: me };
            let edge = Edge::Cont(cont);

            incoming_edges.insert(edge);
        )*

        $(
            let cont = Cont { origin: me, target: BlockNum($d) };
            let edge = Edge::Cont(cont);

            outgoing_edges.insert(edge);
        )*

        Block {
            num: me,
            ops,
            incoming_edges,
            outgoing_edges
        }
    }};
}

#[test]
fn build_cfg_empty() {
    let expected = cfg! {
        id: 0,
        blocks: [
            block! {
                id: 0,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: [1]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [Instruction::End],
                in_jumps: [],
                out_jumps: [],
                in_cont: [0],
                out_cont: []
            }
        ]
    };

    let wasm = r#" 
        (module
            (func $f0))
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);
    let actual = build_func_cfg(&func);

    assert_eq!(actual, expected);
}

#[test]
fn build_cfg_implicit_block() {
    let expected = cfg! {
        id: 0,
        blocks: [
            block! {
                id: 0,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: [1]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    Instruction::GetLocal(0),
                    Instruction::GetLocal(1),
                    Instruction::End
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [0],
                out_cont: []
            }
        ]
    };

    let wasm = r#" 
        (module
            (func $f0
                (local.get 0)
                (local.get 1)
            )
        )
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);
    let actual = build_func_cfg(&func);

    assert_eq!(actual, expected);
}

#[test]
fn build_cfg_explicit_block() {
    let expected = cfg! {
        id: 0,
        blocks: [
            block! {
                id: 0,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: [1]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    Instruction::Block(BlockType::NoResult),
                    Instruction::End,
                    Instruction::End
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [0],
                out_cont: []
            }
        ]
    };

    let wasm = r#" 
        (module
            (func $f0
                (block)))
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);
    let actual = build_func_cfg(&func);

    assert_eq!(actual, expected);
}

#[test]
fn build_cfg_nested_empty_blocks() {
    let expected = cfg! {
        id: 0,
        blocks: [
            block! {
                id: 0,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: [1]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    Instruction::Block(BlockType::NoResult),
                    Instruction::Block(BlockType::NoResult),
                    Instruction::Block(BlockType::NoResult),
                    Instruction::Block(BlockType::NoResult),
                    Instruction::End,
                    Instruction::End,
                    Instruction::End,
                    Instruction::End,
                    Instruction::End
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [0],
                out_cont: []
            }
        ]
    };

    let wasm = r#" 
        (module
            (func $f0
                (block
                    (block
                        (block 
                            (block)
                        )
                    )
                )
            )
        )
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);

    let actual = build_func_cfg(&func);

    assert_eq!(actual, expected);
}

#[test]
fn build_cfg_with_return_op() {
    let expected = cfg! {
        id: 0,
        blocks: [
            block! {
                id: 0,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: [1]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    // Instruction::Return
                ],
                in_jumps: [],
                out_jumps: [3],
                in_cont: [0],
                out_cont: []
            },
            block! {
                id: 2,
                offset: 2,
                ops: [
                    Instruction::End
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: [3]
            },
            block! {
                id: 3,
                offset: 3,
                ops: [],
                in_jumps: [1],
                out_jumps: [],
                in_cont: [2],
                out_cont: []
            }
        ]
    };

    let wasm = r#" 
        (module
            (func $f0
                return
            )
        )
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);

    let actual = build_func_cfg(&func);

    assert_eq!(actual, expected);
}
