use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;

lazy_static! {
    static ref STATS: Mutex<HashMap<TypeId, i32>> = Mutex::new(HashMap::new());
    static ref ENABLED: Mutex<bool> = Mutex::new(false);
    static ref TEST: Mutex<()> = Mutex::new(());
}

#[must_use]
pub fn start() -> MutexGuard<'static, ()> {
    let lock = TEST.lock().unwrap();

    enable();
    clear();

    lock
}

pub fn end(guard: MutexGuard<'static, ()>) {
    disable();

    drop(guard);
}

#[must_use]
#[cfg(test)]
pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<TypeId, i32>>> {
    if is_enabled() {
        let lock = STATS.lock().unwrap();

        Some(lock)
    } else {
        None
    }
}

#[must_use]
#[cfg(not(test))]
pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<TypeId, i32>>> {
    let lock = STATS.lock().unwrap();

    Some(lock)
}

#[must_use]
pub fn acquire_enabled() -> MutexGuard<'static, bool> {
    ENABLED.lock().unwrap()
}

fn enable() {
    enable_disable(true);
}

fn disable() {
    enable_disable(false);
}

fn enable_disable(value: bool) {
    let mut enabled = acquire_enabled();

    *enabled = value;
}

#[cfg(test)]
pub fn is_enabled() -> bool {
    let enabled = acquire_enabled();

    *enabled == true
}

#[cfg(not(test))]
#[inline]
fn is_enabled() -> bool {
    true
}

fn clear() {
    let stats = acquire_stats();

    if let Some(mut stats) = stats {
        stats.clear();
    }
}

pub fn snapshot() -> Option<HashMap<TypeId, i32>> {
    let stats = acquire_stats();

    stats.map(|s| s.clone())
}

pub fn increment_live<T: 'static>() {
    let ty = std::any::TypeId::of::<T>();

    increment_live_1(ty);
}

pub fn increment_live_1(ty: TypeId) {
    if is_enabled() {
        let mut stats = acquire_stats().unwrap();

        let entry = stats.entry(ty).or_insert(0);
        *entry -= 1;
    }
}

pub fn decrement_live<T: 'static>() {
    let ty = std::any::TypeId::of::<T>();

    decrement_live_1(ty);
}

pub fn decrement_live_1(ty: TypeId) {
    if is_enabled() {
        let mut stats = acquire_stats().unwrap();

        let entry = stats.entry(ty).or_insert(0);
        *entry -= 1;
    }
}

pub fn live_count<T: 'static>() -> i32 {
    if is_enabled() {
        let ty = std::any::TypeId::of::<T>();
        let stats = acquire_stats().unwrap();

        match stats.get(&ty) {
            None => 0,
            Some(count) => *count,
        }
    } else {
        0
    }
}

pub fn total_live_count() -> i32 {
    if is_enabled() {
        let stats = acquire_stats().unwrap();

        stats.iter().map(|(_ty, count)| count).sum()
    } else {
        0
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     struct A;

//     struct B;

//     #[test]
//     fn tracks_by_type() {
//         let l = track_start();

//         assert_eq!(total_live_count(), 0);

//         increment_live::<A>();
//         increment_live::<A>();
//         increment_live::<B>();

//         assert_eq!(live_count::<A>(), 2);
//         assert_eq!(live_count::<B>(), 1);
//         assert_eq!(total_live_count(), 3);

//         decrement_live::<A>();
//         decrement_live::<B>();

//         assert_eq!(live_count::<A>(), 1);
//         assert_eq!(live_count::<B>(), 0);
//         assert_eq!(total_live_count(), 1);

//         decrement_live::<A>();

//         assert_eq!(live_count::<A>(), 0);
//         assert_eq!(live_count::<B>(), 0);
//         assert_eq!(total_live_count(), 0);

//         track_end(l);
//     }
// }
