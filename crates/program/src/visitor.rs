use crate::{FuncIndex, FuncIterator, Op, Program};

/// A trait to be implemented for adding logic while visiting a `Program`
pub trait ProgramVisitor: Sized {
    /// Type to return after `visit` completes
    type Output;

    /// Type for `Error`(s) returned while visiting the `Program`
    type Error;

    /// An Entry point to visiting a `Program`
    fn visit(mut self, program: &Program) -> Result<Self::Output, Self::Error> {
        let func_index = program.func_indexes();
        self.on_start(program)?;

        for &fn_index in func_index.iter() {
            self.on_func_start(fn_index, program)?;

            let func = program.get_func(fn_index);
            let iter = FuncIterator::new(&func);

            for op in iter {
                self.on_op(&op, program)?;
            }

            self.on_func_end(fn_index, program)?;
        }

        self.on_end(program)
    }

    /// An hook to be called when about to start visiting `Program`
    fn on_start(&mut self, _program: &Program) -> Result<(), Self::Error> {
        Ok(())
    }

    /// An hook to be called just when about to finish visiting `Program`
    fn on_end(self, program: &Program) -> Result<Self::Output, Self::Error>;

    /// An hook to be called when about to start visiting function `fn_index`
    fn on_func_start(
        &mut self,
        _fn_index: FuncIndex,
        _program: &Program,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// An hook to be called when about to finish visiting function `fn_index`
    fn on_func_end(&mut self, _fn_index: FuncIndex, _program: &Program) -> Result<(), Self::Error> {
        Ok(())
    }

    /// An hook to be called when visiting instruction `op`
    fn on_op(&mut self, _op: &Op, _program: &Program) -> Result<(), Self::Error> {
        Ok(())
    }
}
