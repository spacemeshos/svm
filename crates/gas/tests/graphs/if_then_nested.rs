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
                Instruction::GetLocal(1)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(0, "default")],
            out_cont: [(2, "on-if-true"), (5, "on-if-false")]
        },
        block! {
            id: 2,
            offset: 2,
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
            offset: 5,
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
}
