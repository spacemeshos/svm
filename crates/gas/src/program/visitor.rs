use crate::{FuncIndex, FuncIterator, Function, Op, Program};

pub trait ProgramVisitor: Sized {
    type Output;

    type Error;

    /// An Entry point to visiting a `Program`
    fn visit(mut self, program: &Program) -> Result<Self::Output, Self::Error> {
        visit_program(program, self)
    }

    /// An hook to be called just when about to start
    fn on_start(&mut self, program: &Program) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_end(self, program: &Program) -> Result<Self::Output, Self::Error>;

    fn on_func_start(&mut self, fn_index: FuncIndex, program: &Program) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_func_end(&mut self, fn_index: FuncIndex, program: &Program) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_op(&mut self, op: &Op, program: &Program) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn visit_program<V: ProgramVisitor>(
    program: &Program,
    mut visitor: V,
) -> Result<V::Output, V::Error> {
    let func_index = program.func_indexes();

    visitor.on_start(program)?;

    for &fn_index in func_index.iter() {
        visitor.on_func_start(fn_index, program)?;

        let func = program.get_func(fn_index);
        let iter = FuncIterator::new(&func);

        for op in iter {
            visitor.on_op(&op, program)?;
        }

        visitor.on_func_end(fn_index, program)?;
    }

    visitor.on_end(program)
}
