use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::vec::IntoIter;

use super::interning;

use svm_types::Type;

use lazy_static::lazy_static;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct svm_resource_t {
    pub type_id: usize,

    pub count: i32,
}

#[allow(non_camel_case_types)]
pub struct svm_resource_iter_t {
    iter: IntoIter<svm_resource_t>,
}

impl svm_resource_iter_t {
    pub fn new(iter: IntoIter<svm_resource_t>) -> Self {
        Self { iter }
    }
}

impl Iterator for svm_resource_iter_t {
    type Item = svm_resource_t;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

lazy_static! {
    static ref STATS: Mutex<HashMap<usize, i32>> = Mutex::new(HashMap::new());
}

#[must_use]
pub fn acquire() -> MutexGuard<'static, HashMap<usize, i32>> {
    STATS.lock().unwrap()
}

#[allow(dead_code)]
pub fn release(_guard: MutexGuard<'static, HashMap<usize, i32>>) {
    //
}

pub fn take_snapshot() -> svm_resource_iter_t {
    let stats = acquire();

    let resources: Vec<_> = stats
        .iter()
        .map(|(type_id, count)| svm_resource_t {
            type_id: *type_id,
            count: *count,
        })
        .collect();

    let iter = resources.into_iter();

    svm_resource_iter_t::new(iter)
}

pub fn increment_live(ty: Type) {
    let ty = interning::interned_type(ty);

    increment_live_1(ty)
}

pub fn increment_live_1(ty: usize) {
    let mut stats = acquire();

    let entry = stats.entry(ty).or_insert(0);
    *entry += 1;
}

pub fn decrement_live(ty: Type) {
    let ty = interning::interned_type(ty);

    decrement_live_1(ty);
}

pub fn decrement_live_1(ty: usize) {
    let mut stats = acquire();

    let entry = stats.entry(ty).or_insert(0);
    *entry -= 1;

    if *entry == 0 {
        stats.remove(&ty);
    }
}

pub fn live_count(ty: Type) -> i32 {
    let ty = interning::interned_type(ty);

    live_count_1(ty)
}

pub fn live_count_1(ty: usize) -> i32 {
    let stats = acquire();

    match stats.get(&ty) {
        None => 0,
        Some(count) => *count,
    }
}

pub fn total_live() -> i32 {
    let stats = acquire();

    stats.iter().map(|(_ty, count)| count).sum()
}
