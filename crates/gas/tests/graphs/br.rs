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
                Instruction::Block(BlockType::Value(ValueType::I32)),
                Instruction::Nop,
                Instruction::I32Const(10)
            ],
            in_jumps: [],
            out_jumps: [3],
            in_cont: [(0, "default")],
            out_cont: []
        },
        block! {
            id: 2,
            offset: 4,
            ops: [
                Instruction::I32Const(20)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [],
            out_cont: [(3, "default")]
        },
        block! {
            id: 3,
            offset: 6,
            ops: [
                Instruction::I32Const(30)
            ],
            in_jumps: [1],
            out_jumps: [],
            in_cont: [(2, "default")],
            out_cont: []
        }
    ]
}
