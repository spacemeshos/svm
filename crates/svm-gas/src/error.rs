use crate::function::FuncIndex;

#[derive(Debug, PartialEq)]
pub enum Error {
    CallIndirectNotAllowed,
    LoopNotAllowed,
    BrNotAllowed,
    BrIfNotAllowed,
    BrTableNotAllowed,
    RecursiveCall(Vec<FuncIndex>),
}
