mod interning;
mod snapshot;

pub use interning::{interned_type, interned_type_rev};
pub use snapshot::{
    decrement_live, decrement_live_1, increment_live, increment_live_1, live_count, live_count_1,
    svm_resource_iter_t, svm_resource_t, take_snapshot, total_live,
};

// #[must_use]
// pub fn start() -> MutexGuard<'static, ()> {
//     let lock = TEST.lock().unwrap();

//     enable();
//     clear();

//     lock
// }

// pub fn end(guard: MutexGuard<'static, ()>) {
//     disable();

//     drop(guard);
// }

// #[must_use]
// #[cfg(test)]
// pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<usize, i32>>> {
//     if is_enabled() {
//         let lock = STATS.lock().unwrap();

//         Some(lock)
//     } else {
//         None
//     }
// }

// #[must_use]
// #[cfg(not(test))]
// pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<usize, i32>>> {
//     let lock = STATS.lock().unwrap();

//     Some(lock)
// }

// #[must_use]
// pub fn acquire_enabled() -> MutexGuard<'static, bool> {
//     ENABLED.lock().unwrap()
// }

// fn enable() {
//     enable_disable(true);
// }

// fn disable() {
//     enable_disable(false);
// }

// fn enable_disable(value: bool) {
//     let mut enabled = acquire_enabled();

//     *enabled = value;
// }

// #[cfg(test)]
// pub fn is_enabled() -> bool {
//     let enabled = acquire_enabled();

//     *enabled == true
// }

// #[cfg(not(test))]
// #[inline]
// fn is_enabled() -> bool {
//     true
// }

// fn clear() {
//     let stats = acquire_stats();

//     if let Some(mut stats) = stats {
//         stats.clear();
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     struct A;

//     struct B;

//     #[test]
//     fn tracks_by_type() {
//         let l = track_start();

//         assert_eq!(total_live(), 0);

//         increment_live::<A>();
//         increment_live::<A>();
//         increment_live::<B>();

//         assert_eq!(live_count::<A>(), 2);
//         assert_eq!(live_count::<B>(), 1);
//         assert_eq!(total_live(), 3);

//         decrement_live::<A>();
//         decrement_live::<B>();

//         assert_eq!(live_count::<A>(), 1);
//         assert_eq!(live_count::<B>(), 0);
//         assert_eq!(total_live(), 1);

//         decrement_live::<A>();

//         assert_eq!(live_count::<A>(), 0);
//         assert_eq!(live_count::<B>(), 0);
//         assert_eq!(total_live(), 0);

//         track_end(l);
//     }
// }
