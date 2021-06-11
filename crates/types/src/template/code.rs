#[derive(Clone)]
pub struct CodeSection {
    kind: ByteCodeKind,

    code: Vec<u8>,
}

impl CodeSection {
    pub fn kind(&self) -> ByteCodeKind {
        self.kind
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn into_code(self) -> Vec<u8> {
        self.code
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ByteCodeKind {
    Wasm,
}
