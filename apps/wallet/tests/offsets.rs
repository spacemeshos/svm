use wallet::offset;

#[test]
pub fn test_offsets() {
    // 32 bytes
    assert_eq!(0, offset!(pub_key1));
    assert_eq!(32, offset!(pub_key2));
    assert_eq!(64, offset!(pub_key3));
    assert_eq!(96, offset!(pending_pub_key));

    // 8 bytes
    assert_eq!(128, offset!(first_layer));
    assert_eq!(136, offset!(last_run_layer));
    assert_eq!(144, offset!(period_time_sec));
    assert_eq!(152, offset!(lockup_time_sec));

    // 4 bytes
    assert_eq!(160, offset!(liquidated));
    assert_eq!(164, offset!(unliquidated));
    assert_eq!(168, offset!(balance));

    // 2 bytes
    assert_eq!(172, offset!(layer_liquidation));
    assert_eq!(174, offset!(is_multisig));
}
