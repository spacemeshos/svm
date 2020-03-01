use wallet::hostctx;

#[test]
fn test_hostctx() {
    assert_eq!(0, hostctx!(addr));
    assert_eq!(1, hostctx!(pub_key));
    assert_eq!(2, hostctx!(layer));
    assert_eq!(3, hostctx!(layer_time_sec));
}
