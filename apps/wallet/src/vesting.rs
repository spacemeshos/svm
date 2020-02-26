///
/// 𝘛𝘉𝘓 - Time Between subsequent Layers (in seconds).
/// This value is guaranteed to stay constant.
///
/// 𝘛: Grant vesting time (in seconds).
///
/// 𝘓: #Layers during the Grant period.
/// 𝘓 = 𝘛 / 𝘛𝘉𝘓

/// 𝘓𝘴:   The layer the Grant starts (given by the Host).
/// 𝘓𝘤𝘶𝘳: The current layer (given by the Host).
/// 𝙇𝘱𝘳𝘦𝘷: The layer of the last vesting computtion. (initialized with 𝘓𝘴)
/// After each vesting refresh, we save to the app-storage:
/// 𝙇𝘱𝘳𝘦𝘷 <- 𝙇𝘤𝘶𝘳
///
/// 𝗖: Grant #coins.
/// Vʟ: vesting-per-layer
/// Vʟ = 𝗖 / 𝘓
///
/// 𝝙L: Layers delta.
/// 𝝙L = 𝙇𝘤𝘶𝘳 - 𝙇𝘱𝘳𝘦𝘷
///
/// 𝝙V: Vesting delta (#coins).
/// 𝝙V = Vʟ * 𝝙L
///

pub fn compute_vesting_delta() {
    todo!()
}
