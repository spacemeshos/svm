use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref STATS: Mutex<HashMap<&'static str, usize>> = Mutex::new(HashMap::new());
}

pub fn clear<T>() {
    let mut stats = STATS.lock().unwrap();

    stats.clear();

    drop(stats);
}

pub fn increment_live<T>() {
    let ty = std::any::type_name::<T>();

    let mut stats = STATS.lock().unwrap();

    let entry = stats.entry(ty).or_insert(0);
    *entry += 1;

    drop(stats);
}

pub fn decrement_live<T>() {
    let ty = std::any::type_name::<T>();

    let mut stats = STATS.lock().unwrap();

    let entry = stats.entry(ty).or_insert(0);
    assert!(*entry > 0);

    *entry -= 1;

    drop(stats);
}

pub fn live_count<T>() -> usize {
    let ty = std::any::type_name::<T>();

    let stats = STATS.lock().unwrap();

    match stats.get(ty) {
        None => 0,
        Some(count) => *count,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct A;

    struct B;

    lazy_static! {
        static ref LOCK: Mutex<()> = Mutex::new(());
    }

    fn run<F>(func: F)
    where
        F: FnOnce() + std::panic::UnwindSafe,
    {
        let lock = LOCK.lock().unwrap();

        let _ = std::panic::catch_unwind(|| {
            func();
        });

        drop(lock);
    }

    #[test]
    fn tracks_by_type() {
        run(|| {
            increment_live::<A>();
            increment_live::<A>();

            increment_live::<B>();

            assert_eq!(live_count::<A>(), 2);
            assert_eq!(live_count::<B>(), 1);

            decrement_live::<A>();
            decrement_live::<B>();

            assert_eq!(live_count::<A>(), 1);
            assert_eq!(live_count::<B>(), 0);

            decrement_live::<A>();

            assert_eq!(live_count::<A>(), 0);
            assert_eq!(live_count::<B>(), 0);
        })
    }

    #[test]
    fn live_count_must_be_non_negative() {
        run(|| {
            increment_live::<A>();
            assert_eq!(live_count::<A>(), 1);

            decrement_live::<A>();
            assert_eq!(live_count::<A>(), 0);

            let result = std::panic::catch_unwind(|| {
                decrement_live::<A>();
            });

            assert!(result.is_err());
        });
    }
}
