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
/// 𝙇𝘱𝘳𝘦𝘷: The layer of the last liquidation computation (initialized with 𝘓𝘴).
///        After each liquidation refresh, we save to the app-storage:
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
///
/// time_iternval - A time interval (in seconds).
/// layer_time    - The time time between subsequent layers (in seconds).
pub fn layer_count(time_interval: u32, layer_time: u32) -> u32 {
    assert!(time_interval % layer_time == 0);

    time_interval / layer_time;
}

/// Computes the `liquidation per layer`.
///
/// `amount`      - The amount of unliquidated-yet coins.
/// `layer_count` - The #layers during the liquidation period.
///
pub fn layer_liquidation(amount: u32, layer_count: u32) -> u32 {
    assert!(amount % layer_count == 0);

    amount / layer_count
}

/// Computes the new liquidated coins between layers `layer_prev` to `layer_current`
///
/// `layer_liq`     - The liquidation per layer. (see: `layer_liquidation` above).
/// `last_layer`    - The last layer where the liquidation has been calculated.
/// `current_layer` - The current layer.
///
pub fn liquidation_delta(layer_liq: u32, last_layer: u32, current_layer: u32) -> u32 {
    assert!(current_layer >= last_layer);

    (layer_liq) * (current_layer - last_layer)
}
