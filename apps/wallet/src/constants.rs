//!
//!  App Storage Layout
//!  ==================
//!
//!             Page #1
//!  +--------------------------------+
//!  |  pub_key1           (32 bytes) |    bytes: 0...31
//!  |--------------------------------+
//!  |  pub_key2           (32 bytes) |    bytes: 32...63
//!  |--------------------------------+
//!  |  pub_key3           (32 bytes) |    bytes: 64...95
//!  |--------------------------------+
//!  |  pending_pub_key    (32 bytes) |    bytes: 96...127
//!  |--------------------------------+
//!  |  first_layer        (8 bytes)  |    bytes: 128...135
//!  +--------------------------------+
//!  |  last_run_layer     (8 bytes)  |    bytes: 136...143
//!  +--------------------------------+
//!  |  period_time_sec    (4 bytes)  |    bytes: 144...147
//!  +--------------------------------+
//!  |  lockup_time_sec    (4 bytes)  |    bytes: 148...151
//!  +--------------------------------+
//!  |  liquidated         (4 bytes)  |    bytes: 152...155
//!  +--------------------------------+
//!  |  unliquidated       (4 bytes)  |    bytes: 156...159
//!  +--------------------------------+
//!  |  balance            (4 bytes)  |    bytes: 160...163
//!  +--------------------------------+
//!  |  layer_liquidation  (2 bytes)  |    bytes: 164...165
//!  +--------------------------------+
//!  |  is_multisig        (1 byte)   |    bytes: 166...166
//!  |--------------------------------+
//!

#[macro_export]
macro_rules! offset {
    ($field:expr) => {{
        let field = stringify!($field);

        match field {
            "pub_key1" => 0,
            "pub_key2" => 32,
            "pub_key3" => 64,
            "pending_pub_key" => 96,
            "first_layer" => 128,
            "last_run_layer" => 136,
            "period_time_sec" => 144,
            "lockup_time_sec" => 148,
            "liquidated" => 152,
            "unliquidated" => 156,
            "balance" => 160,
            "layer_liquidation" => 164,
            "is_multisig" => 166,
            _ => unreachable!(),
        }
    }};
}
