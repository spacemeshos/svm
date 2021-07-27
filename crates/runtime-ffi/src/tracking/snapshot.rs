use std::collections::HashMap;
use std::sync::{Condvar, Mutex, MutexGuard};
use std::thread::ThreadId;
use std::vec::IntoIter;

use super::interning;

use svm_types::Type;

use lazy_static::lazy_static;

lazy_static! {
    /// Stores for each interned type its number of live instances.
    /// In case a value is negative, it signals there is probably a bug in the code.
    /// (There must not be more de-allocations than allocations for a type).
    static ref STATS: Mutex<HashMap<usize, i32>> = Mutex::new(HashMap::new());

    /// By default when a tests are running, their resources tracking is off.
    /// It means that we can run in parallel tests that have no resources tracking.
    ///
    /// For the rest of the tests that do care about resources tracking,
    /// these must coordinate together to ensure the tracking logic is safe.
    /// It means that at any given point only one test with tracking on can have access and modify
    //// the state of allocated resources. (i.e modify `STATS` above).
    ///
    /// The best practice for tests that would like to track resources is to call
    /// `set_tracking_on` right in their beginning and `set_tracking_off` at their end.
    /// The way `set_tracking_on` works is by storing a unique token under `CURRENT_TEST_TOKEN`.
    /// That token denotes the currently running test. Here we take advantage of the fact that
    /// each thread runs only a single test at a time and so we use the `current thread id` as our token.
    ///
    /// When two tests (or more) want to turn tracking on, a parallel call `set_tracking_on` will arbitrarily
    /// let exactly one of these tests to acquire a lock on `CURRENT_TEST_TOKEN` and store its origin thread id (the so-called `token`).
    ///
    /// The other test (associated with a different thread) will await for the `CURRENT_TEST_TOKEN` value to be clear again.
    /// For that, it will use a condition-variable named `CURRENT_TEST_CVAR`.
    ///
    /// Once the running test will complete, it'll call `set_tracking_off`, and notify the `CURRENT_TEST_CVAR` subscribers.
    /// These threads will wake-up and again, only one thread will gain control and modify the `CURRENT_TEST_TOKEN` with a new value and so on.
    static ref CURRENT_TEST_TOKEN: Mutex<Option<ThreadId>> = Mutex::new(None);

    static ref CURRENT_TEST_CVAR: Condvar = Condvar::new();
}

#[cfg(test)]
pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<usize, i32>>> {
    if is_tracking_on() {
        let lock = STATS.lock().unwrap();

        Some(lock)
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

/// Turns on resources tracking
#[allow(dead_code)]
pub fn set_tracking_on() {
    let mut lock = CURRENT_TEST_TOKEN.lock().unwrap();

    while lock.is_some() {
        // There is another test (belonging to a different thread)
        // that is running with tracking on.
        // We'll register the condition-variable `CURRENT_TEST_CVAR` to be notified
        // when we can get a new chance to proceed.
        lock = CURRENT_TEST_CVAR.wait(lock).unwrap();
    }

    assert!(lock.is_none());

    let token = std::thread::current().id();

    // Each tracking test needs to start tracking for a blank slate.
    clear();

    // Mark that current thread is the new owner of resources tracking.
    *lock = Some(token);
}

fn clear() {
    let mut stats = STATS.lock().unwrap();
    *stats = HashMap::new();
}

/// Turns off resources tracking
#[allow(dead_code)]
pub fn set_tracking_off() {
    let mut lock = CURRENT_TEST_TOKEN.lock().unwrap();
    let token = std::thread::current().id();

    assert_eq!(lock.as_ref(), Some(&token));

    // We've finished running our test
    // It's time to let others get their chance to use resources tracking.
    *lock = None;
    CURRENT_TEST_CVAR.notify_all();
}

/// Resources whether resources tracking is on (test mode only).
#[cfg(test)]
fn is_tracking_on() -> bool {
    let lock = CURRENT_TEST_TOKEN.lock().unwrap();

    if lock.is_none() {
        return false;
    }

    let held_token = lock.unwrap();
    let token = std::thread::current().id();

    // Other tests that don't care about resources tracking
    // will have a token different than currently held one and hence
    // `held_token == token` will be `false`
    //
    // Tests that do want to track resources need first to call `set_tracking_on`
    // as explained above (under the `lazy_static!` section).
    // Calling `set_tracking_on` right at start guarantees that only a single test with tracking on enabled
    // can make a progress at any given time and we get `held_token == token` as `true`
    held_token == token
}

/// Resources tracking is always on when running in a non-test mode.
#[allow(dead_code)]
#[inline]
#[cfg(not(test))]
fn is_tracking_on() -> bool {
    true
}

/// Represents a manually-allocated resource.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct svm_resource_t {
    /// Type interned value
    pub type_id: usize,

    /// `#resources` of that type
    pub count: i32,
}

/// Iterator over the `svm_resource_t`.
#[allow(non_camel_case_types)]
pub struct svm_resource_iter_t {
    iter: IntoIter<svm_resource_t>,
}

impl svm_resource_iter_t {
    /// New instance, wraps the input iterator.
    pub fn new(iter: IntoIter<svm_resource_t>) -> Self {
        Self { iter }
    }

    /// Returns `HashMap`, where each entry maps between a `Type` to its #resources.
    pub fn prettify(self) -> HashMap<Option<Type>, i32> {
        let mut map = HashMap::new();

        for resource in self {
            let ty = interning::interned_type_rev(resource.type_id);
            let count = resource.count;

            map.insert(ty, count);
        }

        map
    }
}

impl Iterator for svm_resource_iter_t {
    type Item = svm_resource_t;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Takes a snapshot of the manually allocated resources
/// and returns an iterator over it.
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

/// Increments the number of manually-allocated instances of type `Type`.
pub fn increment_live(ty: Type) {
    let ty = interning::interned_type(ty);

    increment_live_1(ty)
}

/// Increments the number of manually-allocated instances of type (given as an interned value).
pub fn increment_live_1(ty: usize) {
    let stats = acquire_stats();

    if let Some(mut stats) = stats {
        // when tracking is on

        let entry = stats.entry(ty).or_insert(0);
        *entry += 1;
    }
}

/// Decrements the number of manually-allocated instances of type `Type`.
pub fn decrement_live(ty: Type) {
    let ty = interning::interned_type(ty);

    decrement_live_1(ty);
}

/// Decrements the number of manually-allocated instances of type (given as an interned value).
pub fn decrement_live_1(ty: usize) {
    let stats = acquire_stats();

    if let Some(mut stats) = stats {
        // when tracking is on

        let entry = stats.entry(ty).or_insert(0);
        *entry -= 1;

        if *entry == 0 {
            stats.remove(&ty);
        }
    }
}

/// The number of manually-allocated instances of type `Type`.
pub fn live_count(ty: Type) -> i32 {
    let ty = interning::interned_type(ty);

    live_count_1(ty)
}

/// The number of manually-allocated instances of type `Type` (given as an interned value).
pub fn live_count_1(ty: usize) -> i32 {
    let stats = acquire_stats();

    if let Some(stats) = stats {
        match stats.get(&ty) {
            None => 0,
            Some(count) => *count,
        }
    } else {
        // when tracking is off
        0
    }
}

/// The total number of manually-allocated resources.
pub fn total_live() -> i32 {
    let stats = acquire_stats();

    if let Some(stats) = stats {
        stats.iter().map(|(_ty, count)| count).sum()
    } else {
        // when tracking is off
        0
    }
}
