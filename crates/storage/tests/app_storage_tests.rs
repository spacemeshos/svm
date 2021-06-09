use svm_layout::{Id, Layout};
use svm_storage::{app::AppStorage, testing};
use svm_types::Address;

macro_rules! assert_vars {
        ($app:expr, $($var_id:expr => $expected:expr), *) => {{
            $(
                let actual = $app.read_var(Id($var_id));
                assert_eq!(actual, $expected);
             )*
        }};
    }

macro_rules! write_vars {
        ($app:expr, $($var_id:expr => $value:expr), *) => {{
            $(
                $app.write_var(Id($var_id), $value.to_vec());
             )*
        }};
    }

#[test]
fn app_storage_vars_are_persisted_only_on_commit() {
    // `var #0` consumes 4 bytes (offsets: `[0..4)`)
    // `var #1` consumes 2 bytes (offsets: `[4, 6)`)
    let layout = Layout::from(vec![4, 2].as_slice());

    let addr = Address::of("my-app");
    let kv = testing::create_app_kv(addr);

    let mut app = AppStorage::new(layout.clone(), kv.clone());

    // vars are initialized with zeros
    assert_vars!(app, 0 => [0, 0, 0, 0], 1 => [0, 0]);
    write_vars!(app, 0 => [10, 20, 30, 40], 1 => [50, 60]);

    // vars latest version are in memory (uncommitted yet)
    assert_vars!(app, 0 => [10, 20, 30, 40], 1 => [50, 60]);

    // spin a new app with no in-memory dirty data
    let app2 = AppStorage::new(layout.clone(), kv.clone());

    // `app`'s' uncomitted changes are not reflected yet
    assert_vars!(app2, 0 => [0, 0, 0, 0], 1 => [0, 0]);

    // now, we'll commit the dirty changes
    let _state = app.commit();

    // we'll spin a new app with no caching
    let app3 = AppStorage::new(layout.clone(), kv.clone());

    // asserting that `commit` persisted the data
    assert_vars!(app3, 0 => [10, 20, 30, 40], 1 => [50, 60]);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn app_storage_write_var_value_should_match_layout_length() {
    // `var #0` consumes 4 bytes (i.e `length = 4`)
    let layout: Layout = vec![4].into();
    let addr = Address::of("my-app");
    let kv = testing::create_app_kv(addr);

    let mut app = AppStorage::new(layout, kv);

    // calling `write_var` with 2-byte value (expected variable's to value to be 4 bytes)
    app.write_var(Id(0), vec![0, 0]);
}
