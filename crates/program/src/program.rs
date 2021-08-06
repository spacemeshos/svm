use indexmap::IndexMap;

use parity_wasm::elements::{self as pwasm, FunctionSection, ValueType};

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
    /// Reads a Wasm program and constructs a [`Program`] struct
    pub fn new(wasm_module: &[u8], validate_exports: bool) -> Result<Self, ProgramError> {
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
        if validate_exports {
            module_validate_exports(&module)?;
        }
        if count_functions_in_program(&program) > u16::MAX as u64 {
            return Err(ProgramError::FunctionIndexTooLarge);
        }

        Ok(program)
    }

    /// Calls [`Program::new`] after parsing `wat_module` and transforming it
    /// from [WebAssembly Text Format] to [WebAssembly Binary Format].
    ///
    /// [WebAssembly Text Format]: https://webassembly.github.io/spec/core/text/index.html
    /// [WebAssembly Binary Format]: https://webassembly.github.io/spec/core/binary/index.html
    pub fn from_wat(wat_module: &str, validate_exports: bool) -> Result<Self, ProgramError> {
        wat::parse_str(wat_module)
            .map_err(|_| ProgramError::InvalidWasm)
            .and_then(|wasm| Program::new(&wasm, validate_exports))
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

fn read_module(wasm: &[u8]) -> Result<pwasm::Module, ProgramError> {
    let module = parity_wasm::deserialize_buffer(wasm);

    module.map_err(|_| ProgramError::InvalidWasm)
}

fn read_code(module: &pwasm::Module) -> Result<pwasm::CodeSection, ProgramError> {
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

/// Checks whether `wasm_module` exports a well-defined `svm_alloc` and `svm_verify` functions.
/// Both are required by SVM to exist in each `Template`.

/// * `svm_alloc`  must have a `I32 -> I32` type signature.
///     - The input param is for the amount of bytes to allocate.
///     - The output is the `offset` pointing to the allocated memory first cell.  
///   
/// * `svm_verify` must have a `() -> I32` type signature.
///     - The input is `()` since it is passed using the `Verify Data` mechanism.
///     - The output is the `offset` pointing to the `returndata` first cell.
///
fn module_validate_exports(module: &pwasm::Module) -> Result<(), ProgramError> {
    use pwasm::{ExportSection, FunctionSection, TypeSection};

    let empty_function_section = FunctionSection::with_entries(vec![]);
    let empty_type_section = TypeSection::with_types(vec![]);
    let empty_export_section = ExportSection::with_entries(vec![]);

    let module_functions = module_functions(module, &empty_function_section);
    let module_types = module_types(module, &empty_type_section);
    let module_exports = module_exports(module, &empty_export_section);

    let mut seen_alloc = false;
    let mut seen_verify = false;

    for export in module_exports.iter() {
        match export.field() {
            "svm_alloc" => {
                seen_alloc = true;
                validate_export_alloc(export, module_functions, module_types)?;
            }
            "svm_verify" => {
                seen_verify = true;
                svm_verify_validate(export, &module_functions, &module_types)?;
            }
            _ => (),
        }
    }

    if !seen_alloc {
        return Err(ProgramError::FunctionNotFound("svm_alloc".to_string()));
    }

    if !seen_verify {
        return Err(ProgramError::FunctionNotFound("svm_verify".to_string()));
    }

    Ok(())
}

fn validate_export_alloc(
    export: &pwasm::ExportEntry,
    module_funcs: &[pwasm::Func],
    module_types: &[pwasm::Type],
) -> Result<(), ProgramError> {
    use pwasm::ValueType;

    validate_func_signature(
        "svm_alloc",
        export,
        module_funcs,
        module_types,
        &[ValueType::I32],
        &[ValueType::I32],
    )
}

fn svm_verify_validate(
    export: &pwasm::ExportEntry,
    module_funcs: &[pwasm::Func],
    module_types: &[pwasm::Type],
) -> Result<(), ProgramError> {
    use pwasm::ValueType;

    validate_func_signature(
        "svm_verify",
        export,
        module_funcs,
        module_types,
        &[],
        &[ValueType::I32],
    )
}

fn validate_func_signature(
    func_name: &str,
    export: &pwasm::ExportEntry,
    module_funcs: &[pwasm::Func],
    module_types: &[pwasm::Type],
    expected_params: &[pwasm::ValueType],
    expected_rets: &[pwasm::ValueType],
) -> Result<(), ProgramError> {
    let sig = export_func_signature(func_name, export, &module_funcs, &module_types)?;

    #[allow(irrefutable_let_patterns)]
    if let pwasm::Type::Function(f) = sig {
        if f.params() == expected_params && f.results() == expected_rets {
            Ok(())
        } else {
            Err(ProgramError::InvalidExportFunctionSignature(
                func_name.to_string(),
            ))
        }
    } else {
        unreachable!()
    }
}

fn export_func_signature<'p>(
    func_name: &str,
    entry: &'p pwasm::ExportEntry,
    module_functions: &'p [pwasm::Func],
    module_types: &'p [pwasm::Type],
) -> Result<&'p pwasm::Type, ProgramError> {
    if let pwasm::Internal::Function(i) = entry.internal() {
        let func = &module_functions[*i as usize];
        let sig = &module_types[func.type_ref() as usize];

        Ok(sig)
    } else {
        Err(ProgramError::InvalidExportKind)
    }
}

fn module_functions<'p>(
    module: &'p pwasm::Module,
    default: &'p pwasm::FunctionSection,
) -> &'p [pwasm::Func] {
    module.function_section().unwrap_or(default).entries()
}

fn module_types<'p>(
    module: &'p pwasm::Module,
    default: &'p pwasm::TypeSection,
) -> &'p [pwasm::Type] {
    module.type_section().unwrap_or(default).types()
}

fn module_exports<'p>(
    module: &'p pwasm::Module,
    default: &'p pwasm::ExportSection,
) -> &'p [pwasm::ExportEntry] {
    module.export_section().unwrap_or(default).entries()
}
