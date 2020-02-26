///
/// 𝘓𝘛: Layer time (in seconds).
/// The time between subsequent Layers is guaranteed to stay constant.
///
/// 𝘓𝙞𝑞𝘛: Liquidation time (in seconds).
///
/// 𝘓: #Layers during the liquidation time.
/// 𝘓 = 𝘓𝙞𝑞𝘛 / 𝘓𝘛
///
/// 𝘓𝘴:    The layer the liquidation starts (given by the Host).
/// 𝘓𝘤𝘶𝘳:  The current layer (given by the Host).
///
/// 𝙇𝘱𝘳𝘦𝘷: The layer of the last vesting computation (initialized with 𝘓𝘴).
///        After each vesting refresh, we save to the app-storage:
///        𝙇𝘱𝘳𝘦𝘷 <- 𝙇𝘤𝘶𝘳
///
/// 𝗖: total #coins.
/// 𝘓𝙞𝑞ʟ: liquidation-per-layer
/// 𝘓𝙞𝑞ʟ = 𝗖 / 𝘓
///
/// 𝝙L: Layers delta.
/// 𝝙L = 𝙇𝘤𝘶𝘳 - 𝙇𝘱𝘳𝘦𝘷
///
/// 𝝙𝘓𝙞𝑞: Liquidation delta (#coins).
/// 𝝙𝘓𝙞𝑞 = 𝘓𝙞𝑞ʟ * 𝝙L
///

/// Computes the total liquidation period in layers.
pub fn liquidation_layers(liq_time_sec: u32, layer_time_sec: u32) -> u32 {
    assert!(liq_time_sec % layer_time_sec == 0);

    liq_time_sec / layer_time_sec
}

/// Computes the `liquidation per layer`.
///
/// `coins`      - The total #coins to be liquidated eventually.
/// `liq_layers` - The #layers during the liquidation period.
///
pub fn liquidation_per_layer(coins: u32, liq_layers: u32) -> u32 {
    assert!(coins % liq_layers == 0);

    coins / liq_layers
}

/// Computes the new liquidated coins between layers `layer_prev` to `layer_current`
///
/// `layer_liq`     - The liquidation per layer. (see: `liquidation_per_layer` above).
/// `layer_prev`    - The last layer where the liquidation has been calculated.
/// `layer_current` - Current layer.
///
pub fn liquidation_delta(layer_liq: u32, layer_prev: u32, layer_current: u32) -> u32 {
    assert!(layer_current >= layer_prev);

    (layer_liq) * (layer_current - layer_prev)
}
