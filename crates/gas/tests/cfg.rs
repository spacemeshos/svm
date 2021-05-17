#![allow(unused)]

use svm_gas::CFG;
use svm_gas::{build_func_cfg, read_program, BlockNum, FuncIndex, Function, Program};

fn parse_wasm(wasm: &str) -> Program {
    let wasm = wat::parse_str(wasm).unwrap();

    read_program(&wasm).unwrap()
}

fn get_func(program: &Program, fn_index: u16) -> Function {
    let index = FuncIndex(fn_index);
    let func = program.get_func(index);
    let code = func.code();

    Function::new(index, code)
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
      in_cont: [ $( ($c1:expr, $c2:expr) ),* ],
      out_cont: [ $( ($d1:expr, $d2:expr) ),* ]
    ) => {{
        use indexmap::IndexSet;
        use parity_wasm::elements::{Instruction, BlockType, ValueType};

        use svm_gas::{Block, BlockNum, Op, Cont, ContKind, Jump, Edge};

        let mut ops: Vec<Op> = Vec::new();

        $(
            let op = Op::new(&$op, 0);
            ops.push(op);
        )*

        for (i, op) in ops.iter_mut().enumerate() {
            op.offset = $offset + i;
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
            let cont = Cont { origin: BlockNum($c1), target: me, kind: $c2.into() };
            let edge = Edge::Cont(cont);

            incoming_edges.insert(edge);
        )*

        $(
            let cont = Cont { origin: me, target: BlockNum($d1), kind: $d2.into() };
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

fn assert_cfg_eq<'f>(actual: &CFG<'f>, expected: &CFG<'f>) {
    let n = actual.blocks().len();
    let m = expected.blocks().len();

    if n != m {
        panic!("Not the same #Blocks. left = {}, right = {}", n, m);
    }

    for i in 0..n {
        let block_num = BlockNum(i);

        let left = actual.get_block(block_num);
        let right = expected.get_block(block_num);

        if left.start_offset() != right.start_offset() {
            panic!(
                "Block {} `offset` mismatch.\n\nleft: {:?}\n\nright: {:?}",
                i, left, right
            );
        }

        if left.ops() != right.ops() {
            panic!(
                "Block {} `instructions` mismatch.\n\nleft: {:?}\n\nright: {:?}",
                i, left, right
            );
        }

        if left.outgoing_edges() != right.outgoing_edges()
            || left.incoming_edges() != right.incoming_edges()
        {
            panic!(
                "Block {} `edges` mismatch.\n\nleft: {:?}\n\nright: {:?}",
                i, left, right
            );
        }
    }
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
                out_cont: [(1, "default")]
            },
            block! {
                id: 1,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(0, "default")],
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

    assert_cfg_eq(&actual, &expected);
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
                out_cont: [(1, "default")]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    Instruction::GetLocal(0),
                    Instruction::GetLocal(1)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(0, "default")],
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

    assert_cfg_eq(&actual, &expected);
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
                out_cont: [(1, "default")]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    Instruction::Block(BlockType::NoResult)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(0, "default")],
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

    assert_cfg_eq(&actual, &expected);
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
                out_cont: [(1, "default")]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    Instruction::Block(BlockType::NoResult),
                    Instruction::Block(BlockType::NoResult),
                    Instruction::Block(BlockType::NoResult),
                    Instruction::Block(BlockType::NoResult)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(0, "default")],
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

    assert_cfg_eq(&actual, &expected);
}

#[test]
fn build_cfg_with_return() {
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
                out_cont: [(1, "default")]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [],
                in_jumps: [],
                out_jumps: [3],
                in_cont: [(0, "default")],
                out_cont: []
            },
            block! {
                id: 2,
                offset: 2,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: [(3, "default")]
            },
            block! {
                id: 3,
                offset: 3,
                ops: [],
                in_jumps: [1],
                out_jumps: [],
                in_cont: [(2, "default")],
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

    assert_cfg_eq(&actual, &expected);
}

#[test]
fn build_cfg_with_unreachable() {
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
                out_cont: [(1, "default")]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [],
                in_jumps: [],
                out_jumps: [3],
                in_cont: [(0, "default")],
                out_cont: []
            },
            block! {
                id: 2,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: [(3, "default")]
            },
            block! {
                id: 3,
                offset: 3,
                ops: [],
                in_jumps: [1],
                out_jumps: [],
                in_cont: [(2, "default")],
                out_cont: []
            }
        ]
    };

    let wasm = r#"
        (module
            (func $f0
                unreachable
            )
        )
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);

    let actual = build_func_cfg(&func);

    assert_cfg_eq(&actual, &expected);
}

#[test]
fn build_cfg_with_if_then() {
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
                out_cont: [(1, "default")]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    Instruction::GetLocal(0)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(0, "default")],
                out_cont: [(2, "on-if-true"), (3, "on-if-false")]
            },
            block! {
                id: 2,
                offset: 3,
                ops: [
                    Instruction::Nop
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(1, "on-if-true")],
                out_cont: [(3, "after-then")]
            },
            block! {
                id: 3,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(2, "after-then"), (1, "on-if-false")],
                out_cont: []
            }
        ]
    };

    let wasm = r#"
        (module
            (func $f0
                (if (local.get 0)
                (then (nop)))
            )
        )
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);

    let actual = build_func_cfg(&func);

    assert_cfg_eq(&actual, &expected);
}

#[test]
fn build_cfg_with_if_then_else() {
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
                out_cont: [(1, "default")]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    Instruction::GetLocal(0)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(0, "default")],
                out_cont: [(2, "on-if-true"), (3, "on-if-false")]
            },
            block! {
                id: 2,
                offset: 3,
                ops: [
                    Instruction::I32Const(7)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(1, "on-if-true")],
                out_cont: [(4, "after-then")]
            },
            block! {
                id: 3,
                offset: 5,
                ops: [
                    Instruction::I32Const(8)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(1, "on-if-false")],
                out_cont: [(4, "after-else")]
            },
            block! {
                id: 4,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(2, "after-then"), (3, "after-else")],
                out_cont: []
            }
        ]
    };

    let wasm = r#"
        (module
            (func $f0
                (if (result i32) (local.get 0)
                (then (i32.const 7)) 
                (else (i32.const 8)))
            )
        )
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);

    let actual = build_func_cfg(&func);

    assert_cfg_eq(&actual, &expected);
}

#[test]
fn build_cfg_with_if_then_nested() {
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
                out_cont: [(1, "default")]
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    Instruction::GetLocal(1)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(0, "default")],
                out_cont: [(2, "on-if-true"), (5, "on-if-false")]
            },
            block! {
                id: 2,
                offset: 3,
                ops: [
                    Instruction::I32Const(2),
                    Instruction::GetLocal(3)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(1, "on-if-true")],
                out_cont: [(3, "on-if-true"), (4, "on-if-false")]
            },
            block! {
                id: 3,
                offset: 6,
                ops: [
                    Instruction::I32Const(4)
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(2, "on-if-true")],
                out_cont: [(4, "after-then")]
            },
            block! {
                id: 4,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(2, "on-if-false"), (3, "after-then")],
                out_cont: [(5, "after-then")]
            },
            block! {
                id: 5,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [(1, "on-if-false"), (4, "after-then")],
                out_cont: []
            }
        ]
    };

    let wasm = r#"
        (module
            (func $f0
                (if (result i32) (local.get 1) 
                    (then 
                        (i32.const 2) 
                        (if (local.get 3) 
                        (then 
                            (i32.const 4)
                        ))
                    )
                )
            )
        )
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);

    let actual = build_func_cfg(&func);

    assert_cfg_eq(&actual, &expected);
}

#[test]
fn build_cfg_with_if_then_nested_2() {
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
                out_cont: []
            },
            block! {
                id: 1,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: []
            },
            block! {
                id: 2,
                offset: 2,
                ops: [
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: []
            },
            block! {
                id: 3,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: []
            },
            block! {
                id: 4,
                offset: 0,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: []
            }
        ]
    };

    let wasm = r#"
        (module
            (func $f0
                (if (result i32) (local.get 0)
                (then
                    (if (result i32) (local.get 1)
                    (then 
                        (i32.const 8)
                    )
                    (else
                        (i32.const 9)
                    ))
                )
                (else
                    (if (result i32) (local.get 2)
                    (then 
                        (i32.const 10)
                    )
                    (else
                        (i32.const 11))
                    ))
                )
            )
        )
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);

    let actual = build_func_cfg(&func);

    dbg!(&actual);

    // assert_cfg_eq(&actual, &expected);
}

#[test]
fn build_cfg_with_if_then_nested_3() {
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
                out_cont: []
            },
            block! {
                id: 1,
                offset: 1,
                ops: [
                    //
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: []
            },
            block! {
                id: 2,
                offset: 2,
                ops: [
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: []
            },
            block! {
                id: 3,
                offset: 4,
                ops: [
                ],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: []
            },
            block! {
                id: 4,
                offset: 7,
                ops: [],
                in_jumps: [],
                out_jumps: [],
                in_cont: [],
                out_cont: []
            }
        ]
    };

    let wasm = r#"
        (module
            (func $f0
                (if (result i32) (local.get 0)
                (then
                    (if (result i32) (local.get 1)
                    (then
                        (if (result i32) (local.get 2)
                        (then
                            (i32.const 8)
                        )
                        (else
                            (i32.const 9)
                        ))
                    )
                    (else
                        (if (result i32) (local.get 3)
                        (then
                            (i32.const 10)
                        )
                        (else
                            (i32.const 11))
                        )
                    ))
                ))
            )
        )
    "#;

    let program = parse_wasm(&wasm);
    let func = get_func(&program, 0);

    let actual = build_func_cfg(&func);

    dbg!(&actual);

    // assert_cfg_eq(&actual, &expected);
}
