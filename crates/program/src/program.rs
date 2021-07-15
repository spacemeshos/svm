use indexmap::IndexMap;
use parity_wasm::elements::{CodeSection, Module};

use crate::{
    validate_no_floats, Exports, FuncIndex, Function, Imports, Instruction, ProgramError,
    ProgramVisitor,
};

/// A fully parsed and validated smWasm program.
///
/// smWasm is the language in which Spacemesh smart contracts are written in.
/// It's a proper subset of WebAssembly, also known as Wasm, similarly to
/// Ethereum's [*ewasm*](https://github.com/ewasm/design).
///
/// smWasm follows the official WebAssembly specification, but imposes some
/// other restrictions as well:
///
/// * No floating-point operations.
/// * No more than [`std::u16::MAX`] functions. This includes functions that are
///   both imported and defined.
/// * It must
///   [export](https://webassembly.github.io/spec/core/syntax/modules.html#syntax-export)
///   several functions which are part of the SVM APIs.
///
/// The main use of [`Program`] is providing a simple smWasm validation tool.
/// Introspection capabilities into actual smWasm modules' contents are very
/// basic and limited in scope.
#[derive(Debug, Default)]
pub struct Program {
    imports: Imports,
    exports: Exports,
    functions: IndexMap<FuncIndex, Vec<Instruction>>,
}

impl Program {
    /// Reads a Wasm program and constructs a `Program` struct
    pub fn new(wasm_module: &[u8]) -> Result<Self, ProgramError> {
        let module = read_module(wasm_module)?;

        let code = read_code(&module)?;
        let imports = Imports::read(&module)?;
        let exports = Exports::read(&module)?;

        let mut program = Program::default();

        for (i, fn_body) in code.bodies().iter().enumerate() {
            let fn_index = i + imports.count();

            let fn_index = FuncIndex(fn_index as u32);
            let fn_code = fn_body.code().elements().to_vec();

            program.add_func(fn_index, fn_code);
        }

        program.set_imports(imports);
        program.set_exports(exports);

        validate_no_floats(&program)?;
        validate_exports(&module)?;
        if count_functions_in_program(&program) > u16::MAX as u64 {
            return Err(ProgramError::FunctionIndexTooLarge);
        }

        Ok(program)
    }

    /// Calls [`Program::new`] after parsing `wat_module` and transforming it
    /// from [WebAssembly Text
    /// Format](https://webassembly.github.io/spec/core/text/index.html) to
    /// [WebAssembly Binary
    /// Format](https://webassembly.github.io/spec/core/binary/index.html).
    pub fn from_wat(wat_module: &str) -> Result<Self, ProgramError> {
        wat::parse_str(wat_module)
            .map_err(|_| ProgramError::InvalidWasm)
            .and_then(|wasm| Program::new(&wasm))
    }

    /// The functions imports
    pub fn imports(&self) -> &Imports {
        &self.imports
    }

    /// Returns an immutable borrow to the [`Exports`] of `self`.
    pub fn exports(&self) -> &Exports {
        &self.exports
    }

    /// Replace the function [`Imports`] of `self`.
    pub fn set_imports(&mut self, imports: Imports) {
        self.imports = imports;
    }

    /// Replace the function [`Exports`] of `self`.
    pub fn set_exports(&mut self, exports: Exports) {
        self.exports = exports;
    }

    /// Returns whether function is an import function or not
    pub fn is_imported(&self, fn_index: FuncIndex) -> bool {
        (fn_index.0 as usize) < self.imports.count()
    }

    /// Returns whether function is an export function or not.
    pub fn is_exported(&self, func_name: &str) -> bool {
        self.exports.contains(func_name)
    }

    /// Adding a function with index` fn_index` and instructions `ops`
    pub fn add_func(&mut self, fn_index: FuncIndex, ops: Vec<Instruction>) {
        self.functions.insert(fn_index, ops);
    }

    /// Returns a `Function` with index `fn_index`
    pub fn get_func(&self, fn_index: FuncIndex) -> Function {
        let code = self.functions.get(&fn_index).unwrap();
        Function::new(fn_index, code)
    }

    /// Returns the indexes of the non-import functions
    pub fn func_indexes(&self) -> Vec<FuncIndex> {
        self.functions.keys().copied().collect()
    }
}

fn read_module(wasm: &[u8]) -> Result<Module, ProgramError> {
    let module = parity_wasm::deserialize_buffer(wasm);

    module.map_err(|_| ProgramError::InvalidWasm)
}

fn read_code(module: &Module) -> Result<CodeSection, ProgramError> {
    match module.code_section() {
        Some(code) => Ok(code.clone()),
        None => Err(ProgramError::MissingCodeSection),
    }
}

fn count_functions_in_program(program: &Program) -> u64 {
    #[derive(Debug, Default, Copy, Clone)]
    struct Counter(u64);

    impl ProgramVisitor for Counter {
        type Output = u64;
        type Error = ();

        fn on_func_end(
            &mut self,
            _fn_index: FuncIndex,
            _program: &Program,
        ) -> Result<(), Self::Error> {
            self.0 += 1;
            Ok(())
        }

        fn on_end(self, _program: &Program) -> Result<Self::Output, Self::Error> {
            Ok(self.0)
        }
    }

    Counter::default().visit(program).unwrap()
}

/// Checks whether `wasm_module` exports a well-defined `svm_alloc` function.
/// `svm_alloc` is required by SVM for all WASM code and must have a `I32 ->
/// I32` type signature.
fn validate_exports(wasm_module: &Module) -> Result<(), ProgramError> {
    use parity_wasm::elements::{ExportSection, FunctionSection, Type, TypeSection, ValueType};

    let empty_export_section = ExportSection::with_entries(vec![]);
    let empty_function_sig_section = FunctionSection::with_entries(vec![]);
    let empty_type_section = TypeSection::with_types(vec![]);

    let module_functions = wasm_module
        .function_section()
        .unwrap_or(&empty_function_sig_section)
        .entries();
    let module_types = wasm_module
        .type_section()
        .unwrap_or(&empty_type_section)
        .types();
    let module_exports = wasm_module
        .export_section()
        .unwrap_or(&empty_export_section)
        .entries();

    for entry in module_exports.iter() {
        if entry.field() != "svm_alloc" {
            continue;
        }

        let type_signature = {
            let function = if let parity_wasm::elements::Internal::Function(i) = entry.internal() {
                module_functions[*i as usize]
            } else {
                // We don't care about anything but functions right now.
                continue;
            };
            &module_types[function.type_ref() as usize]
        };

        #[allow(irrefutable_let_patterns)]
        if let Type::Function(f) = type_signature {
            if f.params() == &[ValueType::I32] && f.results() == &[ValueType::I32] {
                return Ok(());
            }
        }
    }

    Err(ProgramError::FunctionNotFound {
        func_name: "svm_alloc".to_string(),
    })
}
