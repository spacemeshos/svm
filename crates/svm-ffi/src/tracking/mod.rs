mod interning;
mod snapshot;

pub use interning::{interned_type, interned_type_rev};
pub use snapshot::{
    decrement_live, decrement_live_1, increment_live, increment_live_1, live_count, live_count_1,
    set_tracking_off, set_tracking_on, svm_resource_iter_t, svm_resource_t, take_snapshot,
    total_live,
};

#[cfg(test)]
mod test {
    use super::*;

    use maplit::hashmap;
    use svm_types::Type;

    struct A;

    struct B;

    #[test]
    fn tracks_by_type() {
        set_tracking_on();

        assert_eq!(total_live(), 0);

        let ty_a = Type::of::<A>();
        let ty_b = Type::of::<B>();

        increment_live(ty_a);
        increment_live(ty_a);
        increment_live(ty_b);

        assert_eq!(live_count(ty_a), 2);
        assert_eq!(live_count(ty_b), 1);
        assert_eq!(total_live(), 3);

        decrement_live(ty_a);
        decrement_live(ty_b);

        assert_eq!(live_count(ty_a), 1);
        assert_eq!(live_count(ty_b), 0);
        assert_eq!(total_live(), 1);

        decrement_live(ty_a);

        assert_eq!(live_count(ty_a), 0);
        assert_eq!(live_count(ty_b), 0);
        assert_eq!(total_live(), 0);

        set_tracking_off();
    }

    #[test]
    fn snapshot_taking() {
        set_tracking_on();

        assert_eq!(total_live(), 0);

        let ty_a = Type::of::<A>();
        let ty_b = Type::of::<B>();

        increment_live(ty_a);
        increment_live(ty_a);
        increment_live(ty_b);

        let iter = take_snapshot();
        let snapshot = iter.prettify();

        set_tracking_off();

        assert_eq!(
            snapshot,
            hashmap! {
                Some(ty_a) => 2,
                Some(ty_b) => 1
            }
        )
    }

    #[test]
    fn setting_tracking_on_resets_tracking() {
        set_tracking_on();

        assert_eq!(total_live(), 0);

        let ty_a = Type::of::<A>();

        increment_live(ty_a);
        increment_live(ty_a);
        assert_eq!(total_live(), 2);

        set_tracking_off();

        set_tracking_on();
        assert_eq!(total_live(), 0);
    }
}
