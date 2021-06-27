#![allow(unused)]

use svm_gas::{
    BlockNum, FuncIndex, FuncPrice, Function, Graph, NodeData, NodeLabel, PriceResolver, Program,
    ProgramPricing, ProgramVisitor, CFG,
};

pub fn parse_wasm(wasm: &str) -> Program {
    let wasm = wat::parse_str(wasm).unwrap();

    svm_gas::read_program(&wasm).unwrap()
}

pub fn get_func(program: &Program, fn_index: u32) -> Function {
    let index = FuncIndex(fn_index);
    let func = program.get_func(index);
    let code = func.code();

    // Please uncomment this code when adding new tests in order to grab the instructions in flat WAT format
    // for (i, op) in code.iter().enumerate() {
    //     println!("{}: {:?}", i, op);
    // }

    Function::new(index, code)
}

pub fn price_program<R>(wasm: &str, resolver: R) -> FuncPrice
where
    R: PriceResolver,
{
    let program = parse_wasm(wasm);

    let mut func_price: Option<FuncPrice> = None;

    let mut pp = ProgramPricing::new(resolver);

    let func_price = pp.visit(&program).unwrap();

    func_price
}

#[macro_export]
macro_rules! cfg {
    (id: $id:expr, blocks: [ $( $block:expr ),* ]) => {{
        use svm_gas::CFG;

        let mut blocks = Vec::new();

        $( blocks.push($block); )*

        CFG { blocks }
    }};
}

#[macro_export]
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

        #[allow(unused)]
        use parity_wasm::elements::{Instruction, BlockType, ValueType};

        #[allow(unused)]
        use svm_gas::{Block, BlockNum, Op, Cont, ContKind, Jump, Edge};

        let mut ops: Vec<Op> = Vec::new();

        $(
            let op = Op::new(&$op, 0);
            ops.push(op);
        )*

        for (i, op) in ops.iter_mut().enumerate() {
            op.offset = $offset + i;
        }

        #[allow(unused)]
        let mut incoming_edges: IndexSet<Edge> = IndexSet::new();

        #[allow(unused)]
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

pub fn assert_graph_eq<L, D>(actual: &Graph<L, D>, expected: &Graph<L, D>)
where
    L: NodeLabel,
    D: NodeData + std::fmt::Display,
{
    let n = actual.node_count();
    let m = expected.node_count();

    if n != m {
        panic!("Not the same #Nodes. left = {}, right = {}", n, m);
    }

    for left in expected.nodes() {
        let label = left.label();
        let right = actual.try_get_node(label);

        if let Some(right) = right {
            let left = left.as_ref();
            let right = right.as_ref();

            if left.data() != right.data() {
                panic!(
                    "Nodes labeled `{}`: `data` mismatch.\n\nleft: {}\n\nright: {}",
                    label,
                    left.data(),
                    right.data()
                )
            }

            if left.outgoing() != right.outgoing() || left.incoming() != right.incoming() {
                panic!(
                    "Nodes labeled `{}`: `edges` mismatch.\n\nleft: {:?}\n\nright: {:?}",
                    label, left, right
                );
            }
        } else {
            panic!("Expected graph to have node labeled `{}`", label);
        }
    }
}

pub fn assert_cfg_eq<'f>(actual: &CFG<'f>, expected: &CFG<'f>) {
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

#[macro_export]
macro_rules! weighted_graph {
    (
        nodes: [
            $( (label: $block_num:expr, weight: $weight:expr) ),*
        ],
        edges: [
            $( $source:expr => $dest:expr ),*
        ]
    ) => {{
            use svm_gas::{WeightedGraph, GraphBuilder, BlockNum, NodeWeight};

            let mut builder = WeightedGraphBuilder::new();

            $(
                let label = BlockNum($block_num);

                builder.add_node(label);
            )*

            $(
                let source = BlockNum($source);
                let dest = BlockNum($dest);

                builder.add_edge(source, dest);
            )*

            let graph = builder.build();

            $(
                let label = BlockNum($block_num);
                let node = graph.get_node(label);
                let node_ref = node.as_ref();
                let node_data = node_ref.data();

                node_data.set_label(label);
                node_data.set_weight($weight);
            )*

            graph
        }};
    }
