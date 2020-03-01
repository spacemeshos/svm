use wallet::offset;

#[test]
fn test_offsets() {
    // 32 bytes
    assert_eq!(0, offset!(pub_key, 0));
    assert_eq!(32, offset!(pub_key, 1));
    assert_eq!(64, offset!(pub_key, 2));
    assert_eq!(96, offset!(pending_pub_key));

    // 8 bytes
    assert_eq!(128, offset!(first_layer));
    assert_eq!(136, offset!(last_run_layer));

    // 4 bytes
    assert_eq!(144, offset!(period_sec));
    assert_eq!(148, offset!(lockup_sec));
    assert_eq!(152, offset!(liquidated));
    assert_eq!(156, offset!(unliquidated));

    // 2 bytes
    assert_eq!(160, offset!(layer_liquidation));

    // 1 bytes
    assert_eq!(162, offset!(is_multisig));
}
