///
///  App Storage Layout
///  ==================
///
///  +--------------------------------+
///  |  pub_key1           (32 bytes) |    bytes: 0...31
///  |--------------------------------+
///  |  pub_key2           (32 bytes) |    bytes: 32...63
///  |--------------------------------+
///  |  pub_key3           (32 bytes) |    bytes: 64...95
///  |--------------------------------+
///  |  pending_pub_key    (32 bytes) |    bytes: 96...127
///  |--------------------------------+
///  |  first_layer        (8 bytes)  |    bytes: 128...135
///  +--------------------------------+
///  |  last_run_layer     (8 bytes)  |    bytes: 136...143
///  +--------------------------------+
///  |  period_sec         (4 bytes)  |    bytes: 144...147
///  +--------------------------------+
///  |  lockup_sec         (4 bytes)  |    bytes: 148...151
///  +--------------------------------+
///  |  liquidated         (4 bytes)  |    bytes: 152...155
///  +--------------------------------+
///  |  unliquidated       (4 bytes)  |    bytes: 156...159
///  +--------------------------------+
///  |  transferred        (4 bytes)  |    bytes: 160...163
///  +--------------------------------+
///  |  layer_liquidation  (2 bytes)  |    bytes: 164...165
///  +--------------------------------+
///  |  is_multisig        (1 byte)   |    bytes: 166...166
///  |--------------------------------+
///

#[macro_export]
macro_rules! offset {
    ($field:expr, $index:expr) => {{
        let field = stringify!($field);

        match (field, $index) {
            ("pub_key", 0) => 0,
            ("pub_key", 1) => 32,
            ("pub_key", 2) => 64,
            ("pub_key", 3) => 96,
            _ => unreachable!(),
        }
    }};
    ($field:expr) => {{
        let field = stringify!($field);

        match field {
            "pending_pub_key" => offset!(pub_key, 3),
            "first_layer" => 128,
            "last_run_layer" => 136,
            "period_sec" => 144,
            "lockup_sec" => 148,
            "liquidated" => 152,
            "unliquidated" => 156,
            "transferred" => 160,
            "layer_liquidation" => 164,
            "is_multisig" => 166,
            _ => unreachable!(),
        }
    }};
}

#[macro_export]
macro_rules! sizeof {
    ($field:expr) => {{
        let field = stringify!($field);

        match field {
            "address" => 20,
            "pub_key" => 32,
            "layer" => 8,
            "period_sec" => 4,
            "lockup_sec" => 4,
            "liquidated" => 4,
            "unliquidated" => 4,
            "transferred" => 4,
            "layer_liquidation" => 2,
            "is_multisig" => 1,
            _ => unreachable!(),
        }
    }};
}

#[macro_export]
macro_rules! hostctx {
    ($field:expr) => {{
        let field = stringify!($field);

        match field {
            "addr" => 0,
            "pub_key" => 1,
            "layer" => 2,
            "layer_time_sec" => 3,
            _ => unreachable!(),
        }
    }};
}
