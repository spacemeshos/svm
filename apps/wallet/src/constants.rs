//!
//!  App Storage Layout
//!  ==================
//!
//!             Page #1
//!  +--------------------------------+
//!  |  pub_key1           (32 bytes) |
//!  |--------------------------------+
//!  |  pub_key2           (32 bytes) |
//!  |--------------------------------+
//!  |  pub_key3           (32 bytes) |
//!  |--------------------------------+
//!  |  pending_pub_key    (32 bytes) |
//!  |--------------------------------+
//!  |  first_layer        (8 bytes)  |
//!  +--------------------------------+
//!  |  last_run_layer     (8 bytes)  |
//!  +--------------------------------+
//!  |  period_time_sec    (8 bytes)  |
//!  +--------------------------------+
//!  |  lockup_time_sec    (8 bytes)  |
//!  +--------------------------------+
//!  |  liquidated         (4 bytes)  |
//!  +--------------------------------+
//!  |  unliquidated       (4 bytes)  |
//!  +--------------------------------+
//!  |  balance            (4 bytes)  |
//!  +--------------------------------+
//!  |  layer_liquidation  (2 bytes)  |
//!  +--------------------------------+
//!  |  is_multisig        (1 byte)   |
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
            "lockup_time_sec" => 152,
            "liquidated" => 160,
            "unliquidated" => 164,
            "balance" => 168,
            "layer_liquidation" => 172,
            "is_multisig" => 174,
            _ => unreachable!(),
        }
    }};
}
