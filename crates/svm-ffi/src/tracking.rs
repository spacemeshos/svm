use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref STATS: Mutex<HashMap<&'static str, i32>> = Mutex::new(HashMap::new());
}

pub fn clear() {
    let mut stats = STATS.lock().unwrap();

    stats.clear();
}

pub fn snapshot() -> HashMap<&'static str, i32> {
    let stats = STATS.lock().unwrap();

    stats.clone()
}

pub fn increment_live<T>() {
    let ty = std::any::type_name::<T>();

    let mut stats = STATS.lock().unwrap();

    let entry = stats.entry(ty).or_insert(0);

    *entry += 1;
}

pub fn decrement_live<T>() {
    let ty = std::any::type_name::<T>();

    let mut stats = STATS.lock().unwrap();

    let entry = stats.entry(ty).or_insert(0);

    *entry -= 1;
}

pub fn live_count<T>() -> i32 {
    let ty = std::any::type_name::<T>();

    let stats = STATS.lock().unwrap();

    match stats.get(ty) {
        None => 0,
        Some(count) => *count,
    }
}

pub fn total_live_count() -> i32 {
    let stats = STATS.lock().unwrap();

    stats.iter().map(|(_ty, count)| count).sum()
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     struct A;

//     struct B;

//     #[test]
//     fn tracks_by_type() {
//         clear();

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
//     }
// }
