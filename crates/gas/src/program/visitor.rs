use crate::{FuncIndex, FuncIterator, Function, Op, Program};

pub trait ProgramVisitor: Sized {
    type Error;

    fn visit(mut self, program: &Program) -> Result<(), Self::Error> {
        visit_program(program, self)
    }

    fn on_start(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_end(mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_func_start(&mut self, func_index: FuncIndex) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_func_end(&mut self, func_index: FuncIndex) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_op(&mut self, op: &Op) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn visit_program<V: ProgramVisitor>(program: &Program, mut visitor: V) -> Result<(), V::Error> {
    let func_index = program.func_indexes();

    visitor.on_start()?;

    for &fn_index in func_index.iter() {
        visitor.on_func_start(fn_index)?;

        let func = program.get_func(fn_index);
        let iter = FuncIterator::new(&func);

        for op in iter {
            visitor.on_op(&op)?;
        }

        visitor.on_func_end(fn_index)?;
    }

    visitor.on_end()?;

    Ok(())
}
