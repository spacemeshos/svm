use crate::{GasMode, SectionKind, SectionLike};

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
    /// Creates a new Section
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

    /// Creates a new `Section` holding executable code and of `Fixed Gas`
    pub fn new_fixed(code: Vec<u8>, svm_version: u32) -> Self {
        Self::new(
            CodeKind::Wasm,
            code,
            EXEC_FLAGS,
            GasMode::Fixed,
            svm_version,
        )
    }

    /// Returns the constant integer denoting that a `Template` is executable
    pub const fn exec_flags() -> u64 {
        EXEC_FLAGS
    }

    /// Returns the bytecode kind being used (only `Wasm` for now)
    pub fn kind(&self) -> CodeKind {
        self.kind
    }

    /// Returns the execution flags
    pub fn flags(&self) -> u64 {
        self.flags
    }

    /// Returns the `GasMode`
    pub fn gas_mode(&self) -> GasMode {
        self.gas_mode
    }

    /// Returns the target `SVM Version` the `Template` expects to run at
    pub fn svm_version(&self) -> u32 {
        self.svm_version
    }

    /// Returns whether the `Template` can execute or not.
    ///
    /// If it's NOT, then it should be used by other Templates as a dependency
    pub fn is_exec(&self) -> bool {
        self.flags & EXEC_FLAGS != 0
    }

    /// Is the `GasMode` being used is of `Fixed Gas`
    pub fn is_fixed_gas(&self) -> bool {
        matches!(self.gas_mode, GasMode::Fixed)
    }

    /// Returns the code of the `Template` (a Blob of bytes)
    pub fn code(&self) -> &[u8] {
        &self.code
    }
}

impl SectionLike for CodeSection {
    const KIND: SectionKind = SectionKind::Code;
}

/// Represent a ByteCode kind.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum CodeKind {
    /// WebAssembly Byte Code
    Wasm,
}
