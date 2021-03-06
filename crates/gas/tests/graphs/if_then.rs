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
            out_cont: [(2, "on-if-true"), (3, "on-if-false")]
        },
        block! {
            id: 2,
            offset: 2,
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
}
