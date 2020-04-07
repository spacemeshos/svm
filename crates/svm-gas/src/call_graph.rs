use crate::{error::ProgramError, function::FuncIndex};

use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

#[derive(Debug)]
pub(crate) struct CallGraph {
    all_funcs: HashSet<FuncIndex>,
    root_funcs: HashSet<FuncIndex>,
    in_calls: HashMap<FuncIndex, HashSet<FuncIndex>>,
    out_calls: HashMap<FuncIndex, HashSet<FuncIndex>>,
}

impl CallGraph {
    pub(crate) fn new(funcs_ids: Vec<FuncIndex>) -> Self {
        let all_funcs = HashSet::from_iter(funcs_ids);
        let root_funcs = all_funcs.clone();

        Self {
            all_funcs,
            root_funcs,
            out_calls: HashMap::new(),
            in_calls: HashMap::new(),
        }
    }

    pub(crate) fn add_call(&mut self, from: FuncIndex, to: FuncIndex) {
        assert!(from != to);
        assert!(self.all_funcs.contains(&from));
        assert!(self.all_funcs.contains(&to));

        self.root_funcs.remove(&from);

        let entry = self.out_calls.entry(from).or_insert_with(HashSet::new);
        entry.insert(to);

        let entry = self.in_calls.entry(to).or_insert_with(HashSet::new);
        entry.insert(from);
    }

    #[must_use]
    pub(crate) fn ensure_no_recursive_calls(&self) -> Result<(), ProgramError> {
        let mut visited = HashSet::new();

        let mut all_funcs = self.all_funcs.iter().copied().collect::<Vec<FuncIndex>>();

        // we sort `all_funcs` in order to make the unit-tests execution determinstic
        all_funcs.sort();

        for func_idx in all_funcs.iter() {
            let mut path = Vec::new();
            self.internal_graph_traverse(*func_idx, &mut visited, &mut path)?;
        }

        Ok(())
    }

    pub(crate) fn topological_sort(&self) -> Vec<FuncIndex> {
        let mut res = Vec::new();
        let mut out_calls = self.out_calls.clone();

        let mut roots_funcs = self.root_funcs.iter().copied().collect::<Vec<FuncIndex>>();

        while let Some(root) = roots_funcs.pop() {
            res.push(root);

            if let Some(callers) = self.in_calls.get(&root) {
                for caller in callers.iter() {
                    let caller_callees = out_calls.get_mut(&caller).unwrap();

                    assert!(caller_callees.contains(&root));

                    caller_callees.remove(&root);

                    if caller_callees.is_empty() {
                        roots_funcs.push(*caller);
                    }
                }
            }
        }

        assert_eq!(self.all_funcs.len(), res.len());

        res
    }

    fn internal_graph_traverse(
        &self,
        caller: FuncIndex,
        visited: &mut HashSet<FuncIndex>,
        path: &mut Vec<FuncIndex>,
    ) -> Result<(), ProgramError> {
        if visited.contains(&caller) {
            return Ok(());
        }

        if path.contains(&caller) {
            path.push(caller);

            return Err(ProgramError::RecursiveCall(path.to_vec()));
        }

        path.push(caller);

        if let Some(callees) = self.out_calls.get(&caller) {
            for callee in callees.iter() {
                self.internal_graph_traverse(*callee, visited, path)?;
            }
        }

        visited.insert(caller);

        Ok(())
    }
}
