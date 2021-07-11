use wasmer::WasmTypeList;

use std::marker::PhantomData;

pub struct Function<'a, Args, Rets>
where
    Args: WasmTypeList,
    Rets: WasmTypeList,
{
    func: &'a wasmer::Function,

    name: &'a str,

    phantom: PhantomData<(Args, Rets)>,
}

impl<'a, Args, Rets> Function<'a, Args, Rets>
where
    Args: WasmTypeList,
    Rets: WasmTypeList,
{
    pub fn new(func: &'a wasmer::Function, name: &'a str) -> Self {
        Self {
            func,
            name,
            phantom: PhantomData,
        }
    }

    pub fn wasmer_func(&self) -> &wasmer::Function {
        &self.func
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
