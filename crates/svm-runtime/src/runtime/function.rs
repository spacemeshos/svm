use std::marker::PhantomData;

use wasmer::WasmTypeList;

pub struct Function<'a, Args, Rets>
where
    Args: WasmTypeList,
    Rets: WasmTypeList,
{
    func: &'a wasmer::Function,

    name: String,

    phantom: PhantomData<(Args, Rets)>,
}

impl<'a, Args, Rets> Function<'a, Args, Rets>
where
    Args: WasmTypeList,
    Rets: WasmTypeList,
{
    pub fn new(func: &'a wasmer::Function, name: &str) -> Self {
        Self {
            func,
            name: name.to_string(),
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
