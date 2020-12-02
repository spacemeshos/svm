use std::collections::HashMap;
use std::sync::{Condvar, Mutex, MutexGuard};
use std::vec::IntoIter;

use std::thread::ThreadId;

use super::interning;

use svm_types::Type;

use lazy_static::lazy_static;

lazy_static! {
    static ref STATS: Mutex<HashMap<usize, i32>> = Mutex::new(HashMap::new());
    static ref CURRENT_TEST_TOKEN: Mutex<Option<ThreadId>> = Mutex::new(None);
    static ref CURRENT_TEST_CVAR: Condvar = Condvar::new();
}

#[cfg(test)]
pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<usize, i32>>> {
    if is_tracking_on() {
        todo!()
    } else {
        None
    }
}

#[must_use]
#[cfg(not(test))]
pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<usize, i32>>> {
    let lock = STATS.lock().unwrap();

    Some(lock)
}

#[allow(dead_code)]
pub fn release_stats(_guard: Option<MutexGuard<'static, HashMap<usize, i32>>>) {
    //
}

pub fn set_tracking_on() {
    let mut lock = CURRENT_TEST_TOKEN.lock().unwrap();

    while lock.is_some() {
        lock = CURRENT_TEST_CVAR.wait(lock).unwrap();
    }

    assert!(lock.is_none());

    let token = std::thread::current().id();

    *lock = Some(token);
}

pub fn set_tracking_off() {
    let mut lock = CURRENT_TEST_TOKEN.lock().unwrap();
    let token = std::thread::current().id();

    assert_eq!(lock.as_ref(), Some(&token));

    *lock = None;
}

#[cfg(test)]
fn is_tracking_on() -> bool {
    let lock = CURRENT_TEST_TOKEN.lock().unwrap();

    if lock.is_none() {
        return false;
    }

    let held_token = lock.unwrap();
    let token = std::thread::current().id();

    held_token == token
}

#[allow(dead_code)]
#[cfg(not(test))]
#[inline]
fn is_tracking_on() -> bool {
    true
}

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

pub fn take_snapshot() -> svm_resource_iter_t {
    let stats = acquire_stats();

    assert!(stats.is_some());

    let resources: Vec<_> = stats
        .unwrap()
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
    let stats = acquire_stats();

    if let Some(mut stats) = stats {
        let entry = stats.entry(ty).or_insert(0);
        *entry += 1;
    }
}

pub fn decrement_live(ty: Type) {
    let ty = interning::interned_type(ty);

    decrement_live_1(ty);
}

pub fn decrement_live_1(ty: usize) {
    let stats = acquire_stats();

    if let Some(mut stats) = stats {
        let entry = stats.entry(ty).or_insert(0);
        *entry -= 1;

        if *entry == 0 {
            stats.remove(&ty);
        }
    }
}

pub fn live_count(ty: Type) -> i32 {
    let ty = interning::interned_type(ty);

    live_count_1(ty)
}

pub fn live_count_1(ty: usize) -> i32 {
    let stats = acquire_stats();

    if let Some(stats) = stats {
        match stats.get(&ty) {
            None => 0,
            Some(count) => *count,
        }
    } else {
        0
    }
}

pub fn total_live() -> i32 {
    let stats = acquire_stats();

    if let Some(stats) = stats {
        stats.iter().map(|(_ty, count)| count).sum()
    } else {
        0
    }
}
