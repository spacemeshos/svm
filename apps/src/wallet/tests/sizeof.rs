use wallet::sizeof;

#[test]
fn test_sizeof() {
    assert_eq!(20, sizeof!(address));
    assert_eq!(32, sizeof!(pub_key));
    assert_eq!(8, sizeof!(layer));

    assert_eq!(4, sizeof!(period_sec));
    assert_eq!(4, sizeof!(lockup_sec));
    assert_eq!(4, sizeof!(liquidated));
    assert_eq!(4, sizeof!(unliquidated));
    assert_eq!(4, sizeof!(transferred));

    assert_eq!(2, sizeof!(layer_liquidation));
    assert_eq!(1, sizeof!(is_multisig));
}
