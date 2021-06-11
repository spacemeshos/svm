use svm_layout::SymbolicVar;

#[derive(Clone)]
pub struct SchemaSection {
    vars: Vec<SymbolicVar>,
}

impl SchemaSection {
    pub fn var_count(&self) -> usize {
        self.vars.len()
    }

    pub fn push_var(&mut self, var: SymbolicVar) {
        self.vars.push(var)
    }

    pub fn vars(&self) -> &[SymbolicVar] {
        &self.vars
    }
}
