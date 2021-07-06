use crate::{GasMode, Section, SectionKind};

const EXEC_FLAGS: u64 = 0x01;

/// Contains the `Template` Code along other properties
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSection {
    kind: CodeKind,

    svm_version: u32,

    code: Vec<u8>,

    flags: u64,

    gas_mode: GasMode,
}

impl CodeSection {
    pub fn new(
        kind: CodeKind,
        code: Vec<u8>,
        flags: u64,
        gas_mode: GasMode,
        svm_version: u32,
    ) -> Self {
        Self {
            kind,
            code,
            flags,
            gas_mode,
            svm_version,
        }
    }

    pub const fn exec_flags() -> u64 {
        EXEC_FLAGS
    }

    pub fn new_fixed(code: Vec<u8>, svm_version: u32) -> Self {
        Self::new(
            CodeKind::Wasm,
            code,
            EXEC_FLAGS,
            GasMode::Fixed,
            svm_version,
        )
    }

    pub fn kind(&self) -> CodeKind {
        self.kind
    }

    pub fn flags(&self) -> u64 {
        self.flags
    }

    pub fn gas_mode(&self) -> GasMode {
        self.gas_mode
    }

    pub fn svm_version(&self) -> u32 {
        self.svm_version
    }

    pub fn is_exec(&self) -> bool {
        self.flags & EXEC_FLAGS != 0
    }

    pub fn is_fixed_gas(&self) -> bool {
        matches!(self.gas_mode, GasMode::Fixed)
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }
}

impl Section for CodeSection {
    const KIND: SectionKind = SectionKind::Code;
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum CodeKind {
    Wasm,
}
