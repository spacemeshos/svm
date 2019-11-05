use std::fmt;

#[derive(Copy, Clone)]
pub(crate) struct BlockOffsets(pub usize, pub usize);

impl fmt::Debug for BlockOffsets {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, " offsets (inclusive): {} - {}", self.0, self.1)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct IfBlockOffsets {
    pub true_offsets: BlockOffsets,
    pub else_offsets: Option<BlockOffsets>,
}

impl fmt::Debug for IfBlockOffsets {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "If-Statement offsets:\n")?;
        write!(f, "     True-Block: {:?}\n", self.true_offsets)?;

        if self.else_offsets.is_some() {
            write!(f, "     Else-Block: {:?}\n", self.else_offsets.unwrap())?;
        }

        Ok(())
    }
}
