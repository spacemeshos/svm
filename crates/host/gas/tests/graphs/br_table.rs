cfg! {
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
            ops: [
                Instruction::Block(BlockType::NoResult),
                Instruction::Block(BlockType::NoResult),
                Instruction::GetLocal(0)
            ],
            in_jumps: [],
            out_jumps: [4, 6],
            in_cont: [(0, "default")],
            out_cont: []
        },
        block! {
            id: 2,
            offset: 4,
            ops: [
                Instruction::I32Const(21)
            ],
            in_jumps: [],
            out_jumps: [7],
            in_cont: [],
            out_cont: []
        },
        block! {
            id: 3,
            offset: 6,
            ops: [
                Instruction::I32Const(30)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [],
            out_cont: [(4, "default")]
        },
        block! {
            id: 4,
            offset: 8,
            ops: [
                Instruction::I32Const(20)
            ],
            in_jumps: [1],
            out_jumps: [7],
            in_cont: [(3, "default")],
            out_cont: []
        },
        block! {
            id: 5,
            offset: 10,
            ops: [
                Instruction::I32Const(40)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [],
            out_cont: [(6, "default")]
        },
        block! {
            id: 6,
            offset: 12,
            ops: [
                Instruction::I32Const(22)
            ],
            in_jumps: [1],
            out_jumps: [],
            in_cont: [(5, "default")],
            out_cont: [(7, "default")]
        },
        block! {
            id: 7,
            offset: 0,
            ops: [],
            in_jumps: [2, 4],
            out_jumps: [],
            in_cont: [(6, "default")],
            out_cont: []
        }
    ]
}
