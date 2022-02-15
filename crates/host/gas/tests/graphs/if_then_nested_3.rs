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
                Instruction::GetLocal(0)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(0, "default")],
            out_cont: [(2, "on-if-true"), (12, "on-if-false")]
        },
        block! {
            id: 2,
            offset: 2,
            ops: [
                Instruction::GetLocal(1)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(1, "on-if-true")],
            out_cont: [(3, "on-if-true"), (7, "on-if-false")]
        },
        block! {
            id: 3,
            offset: 4,
            ops: [
                Instruction::GetLocal(2)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(2, "on-if-true")],
            out_cont: [(4, "on-if-true"), (5, "on-if-false")]
        },
        block! {
            id: 4,
            offset: 6,
            ops: [
                Instruction::I32Const(8)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(3, "on-if-true")],
            out_cont: [(6, "after-then")]
        },
        block! {
            id: 5,
            offset: 8,
            ops: [
                Instruction::I32Const(9)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(3, "on-if-false")],
            out_cont: [(6, "after-else")]
        },
        block! {
            id: 6,
            offset: 0,
            ops: [],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(4, "after-then"), (5, "after-else")],
            out_cont: [(11, "after-then")]
        },
        block! {
            id: 7,
            offset: 11,
            ops: [
                Instruction::GetLocal(3)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(2, "on-if-false")],
            out_cont: [(8, "on-if-true"), (9, "on-if-false")]
        },
        block! {
            id: 8,
            offset: 13,
            ops: [
                Instruction::I32Const(10)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(7, "on-if-true")],
            out_cont: [(10, "after-then")]
        },
        block! {
            id: 9,
            offset: 15,
            ops: [
                Instruction::I32Const(11)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(7, "on-if-false")],
            out_cont: [(10, "after-else")]
        },
        block! {
            id: 10,
            offset: 0,
            ops: [],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(8, "after-then"), (9, "after-else")],
            out_cont: [(11, "after-else")]
        },
        block! {
            id: 11,
            offset: 0,
            ops: [],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(6, "after-then"), (10, "after-else")],
            out_cont: [(12, "after-then")]
        },
        block! {
            id: 12,
            offset: 0,
            ops: [],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(1, "on-if-false"), (11, "after-then")],
            out_cont: []
        }
    ]
}
