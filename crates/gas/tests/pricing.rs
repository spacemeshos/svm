#![allow(unused)]

mod helpers;
use helpers::assert_graph_eq;

use svm_gas::build_weighted_graph;
use svm_gas::{
    BlockNum, FuncIndex, FuncPrice, GraphBuilder, Imports, NodeWeight, Op, PriceResolver,
};

type WeightedGraphBuilder = GraphBuilder<BlockNum, NodeWeight<BlockNum>>;

struct TestResolver {
    op_price: usize,
}

impl TestResolver {
    pub fn new(num: usize) -> Self {
        Self { op_price: num }
    }
}

impl PriceResolver for TestResolver {
    fn op_price(&self, _op: &Op) -> usize {
        self.op_price
    }

    fn import_price(&self, import: (&str, &str)) -> usize {
        match import {
            ("env", "foo") => 100,
            ("env", "bar") => 50,
            _ => unreachable!(),
        }
    }
}

macro_rules! func_price {
    ( $( (func: $fn_index:expr, price: $price:expr) ),*  ) => {{
        let mut func_price = FuncPrice::new();

        $(
            func_price.set(FuncIndex($fn_index), $price);
        )*

        func_price
    }};
}

#[test]
fn func_cfg_to_weighted_graph_1() {
    let cfg = include!("graphs/empty.rs");
    let resolver = TestResolver::new(1);
    let func_price = FuncPrice::new();
    let imports = Imports::new();

    let expected = weighted_graph! {
        nodes: [
            (label: 0, weight: 0),
            (label: 1, weight: 0)
        ],
        edges: [0 => 1]
    };

    let actual = build_weighted_graph(&cfg, &resolver, &imports, &func_price);

    assert_graph_eq(&expected, &actual);
}

#[test]
fn func_cfg_to_weighted_graph_2() {
    let cfg = include!("graphs/br.rs");
    let resolver = TestResolver::new(1);
    let func_price = FuncPrice::new();
    let imports = Imports::new();

    let expected = weighted_graph! {
        nodes: [
            (label: 0, weight: 0),
            (label: 1, weight: 3),
            (label: 2, weight: 1),
            (label: 3, weight: 1)
        ],
        edges: [
            0 => 1,
            1 => 3,
            2 => 3
        ]
    };

    let actual = build_weighted_graph(&cfg, &resolver, &imports, &func_price);

    assert_graph_eq(&expected, &actual);
}

#[test]
fn func_cfg_to_weighted_graph_3() {
    let cfg = include!("graphs/call.rs");

    // Node #1 Op-Price
    // ================
    //  1. Nop       => price = 1
    //  2. Call(10)  => price = 100
    //  2. Call(20)  => price = 50
    //  3. Nop       => price = 1
    //  4. Call(30)  => price = 20

    let resolver = TestResolver::new(1);

    let mut imports = Imports::with_capacity(3);
    imports.insert("env", "foo", FuncIndex(10));
    imports.insert("env", "bar", FuncIndex(20));

    let mut func_price = FuncPrice::with_capacity(3);
    func_price.set(FuncIndex(30), 20);

    let expected = weighted_graph! {
        nodes: [
            (label: 0, weight: 0),
            (label: 1, weight: 1 + 100 + 50 + 1 + 20)
        ],
        edges: [0 => 1]
    };

    let actual = build_weighted_graph(&cfg, &resolver, &imports, &func_price);

    assert_graph_eq(&expected, &actual);
}

#[test]
fn program_pricing_1() {
    let wasm = r#"
          (module
            ;; function #0
            (func $foo (import "env" "foo") (param i32) (result i32))

            ;; function #1
            (func $func1 (result i32)
                (nop)          ;; price = 1
                (call $foo)    ;; price = 100 (import)
                (nop))         ;; price = 1

            ;; function #2
            (func $func2 (result i32)
                (nop)          ;; price = 1
                (call $func1)  ;; price = 102 (1 + 100 + 1)
                (nop)          ;; price = 1
                (nop))         ;; price = 1
        )
        "#;

    let resolver = TestResolver::new(1);
    let actual = helpers::price_program(wasm, resolver);

    let expected = func_price! {
        (func: 1, price: 1 + 100 + 1),
        (func: 2, price: 1 + 102 + 1 + 1)
    };

    assert_eq!(expected, actual);
}
