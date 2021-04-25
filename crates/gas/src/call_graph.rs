use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::{FuncIndex, ProgramError};

#[derive(Debug)]
pub(crate) struct CallGraph {
    functions: HashSet<FuncIndex>,
    roots: HashSet<FuncIndex>,
    in_calls: HashMap<FuncIndex, HashSet<FuncIndex>>,
    out_calls: HashMap<FuncIndex, HashSet<FuncIndex>>,
}

impl CallGraph {
    pub(crate) fn new(functions: Vec<FuncIndex>) -> Self {
        let functions = HashSet::from_iter(functions);
        let roots = functions.clone();

        Self {
            functions,
            roots,
            out_calls: HashMap::new(),
            in_calls: HashMap::new(),
        }
    }

    pub(crate) fn add_call(&mut self, source: FuncIndex, dest: FuncIndex) {
        debug_assert!(source != dest);
        debug_assert!(self.functions.contains(&source));
        debug_assert!(self.functions.contains(&dest));

        self.roots.remove(&source);

        let entry = self.out_calls.entry(source).or_insert_with(HashSet::new);
        entry.insert(dest);

        let entry = self.in_calls.entry(dest).or_insert_with(HashSet::new);
        entry.insert(source);
    }

    #[must_use]
    pub(crate) fn assert_no_recursive_calls(&self) -> Result<(), ProgramError> {
        let mut visited = HashSet::new();

        let mut functions = self.functions.iter().copied().collect::<Vec<FuncIndex>>();

        // we sort `functions` in order to make the unit-tests execution deterministic
        functions.sort();

        for func in functions.iter() {
            let mut call_stack = Vec::new();

            self.traverse(*func, &mut visited, &mut call_stack)?;
        }

        Ok(())
    }

    pub(crate) fn topological_sort(&self) -> Vec<FuncIndex> {
        let mut result = Vec::new();
        let mut out_calls = self.out_calls.clone();

        let mut roots = self.roots.iter().copied().collect::<Vec<FuncIndex>>();

        while let Some(root) = roots.pop() {
            result.push(root);

            if let Some(callers) = self.in_calls.get(&root) {
                for caller in callers.iter() {
                    let callees = out_calls.get_mut(&caller).unwrap();

                    debug_assert!(callees.contains(&root));

                    callees.remove(&root);

                    if callees.is_empty() {
                        roots.push(*caller);
                    }
                }
            }
        }

        debug_assert_eq!(self.functions.len(), result.len());

        result
    }

    fn traverse(
        &self,
        caller: FuncIndex,
        visited: &mut HashSet<FuncIndex>,
        call_stack: &mut Vec<FuncIndex>,
    ) -> Result<(), ProgramError> {
        if visited.contains(&caller) {
            return Ok(());
        }

        if call_stack.contains(&caller) {
            call_stack.push(caller);

            let cycle = std::mem::take(call_stack);

            return Err(ProgramError::RecursiveCall(cycle));
        }

        call_stack.push(caller);

        if let Some(callees) = self.out_calls.get(&caller) {
            for callee in callees.iter() {
                self.traverse(*callee, visited, call_stack)?;
            }
        }

        visited.insert(caller);

        Ok(())
    }
}
