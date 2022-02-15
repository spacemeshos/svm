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
                Instruction::GetLocal(0),
                Instruction::GetLocal(1)
            ],
            in_jumps: [],
            out_jumps: [],
            in_cont: [(0, "default")],
            out_cont: []
        }
    ]
}