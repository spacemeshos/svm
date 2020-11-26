use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;

lazy_static! {
    static ref STATS: Mutex<HashMap<&'static str, i32>> = Mutex::new(HashMap::new());
}

#[cfg(test)]
lazy_static! {
    static ref ENABLED: Mutex<bool> = Mutex::new(false);
}

#[cfg(test)]
lazy_static! {
    static ref TEST: Mutex<()> = Mutex::new(());
}

#[cfg(test)]
pub fn track_start() -> MutexGuard<'static, ()> {
    let lock = TEST.lock().unwrap();

    enable();
    clear();

    lock
}

#[cfg(test)]
pub fn track_end(guard: MutexGuard<'static, ()>) {
    disable();

    drop(guard);
}

#[cfg(test)]
pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<&'static str, i32>>> {
    if is_enabled() {
        let lock = STATS.lock().unwrap();

        Some(lock)
    } else {
        None
    }
}

#[cfg(not(test))]
pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<&'static str, i32>>> {
    let lock = STATS.lock().unwrap();

    Some(lock)
}

#[cfg(test)]
pub fn acquire_enabled() -> MutexGuard<'static, bool> {
    ENABLED.lock().unwrap()
}

#[cfg(test)]
fn enable() {
    enable_disable(true);
}

#[cfg(test)]
fn disable() {
    enable_disable(false);
}

#[cfg(test)]
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

#[cfg(test)]
fn clear() {
    let stats = acquire_stats();

    if let Some(mut stats) = stats {
        stats.clear();
    }
}

#[cfg(test)]
pub fn snapshot() -> Option<HashMap<&'static str, i32>> {
    let stats = acquire_stats();

    stats.map(|s| s.clone())
}

pub fn increment_live<T>() {
    if is_enabled() {
        let ty = std::any::type_name::<T>();
        let mut stats = acquire_stats().unwrap();

        let entry = stats.entry(ty).or_insert(0);
        *entry += 1;
    }
}

pub fn decrement_live<T>() {
    if is_enabled() {
        let ty = std::any::type_name::<T>();
        let mut stats = acquire_stats().unwrap();

        let entry = stats.entry(ty).or_insert(0);
        *entry -= 1;
    }
}

pub fn live_count<T>() -> i32 {
    if is_enabled() {
        let ty = std::any::type_name::<T>();
        let stats = acquire_stats().unwrap();

        match stats.get(ty) {
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

#[cfg(test)]
mod test {
    use super::*;

    struct A;

    struct B;

    #[test]
    fn tracks_by_type() {
        let l = track_start();

        assert_eq!(total_live_count(), 0);

        increment_live::<A>();
        increment_live::<A>();
        increment_live::<B>();

        assert_eq!(live_count::<A>(), 2);
        assert_eq!(live_count::<B>(), 1);
        assert_eq!(total_live_count(), 3);

        decrement_live::<A>();
        decrement_live::<B>();

        assert_eq!(live_count::<A>(), 1);
        assert_eq!(live_count::<B>(), 0);
        assert_eq!(total_live_count(), 1);

        decrement_live::<A>();

        assert_eq!(live_count::<A>(), 0);
        assert_eq!(live_count::<B>(), 0);
        assert_eq!(total_live_count(), 0);

        track_end(l);
    }
}
