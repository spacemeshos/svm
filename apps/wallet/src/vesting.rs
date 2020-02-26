///
/// 𝘛𝘉𝘓 - Time Between subsequent Layers (in seconds).
/// This value is guaranteed to stay constant.
///
/// 𝘛: Grant vesting time (in seconds).
///
/// 𝘓: #Layers during the Grant period.
/// 𝘓 = 𝘛 / 𝘛𝘉𝘓

/// 𝘓𝘴: The layer the Grant starts (input from the running Host).
///
/// 𝗖: Grant #coins.
///
/// Vʟ: vesting-per-layer
/// Vʟ = 𝗖 / 𝘓
///
/// 𝝙L: Layers delta.
/// 𝝙L = 𝙇𝘤𝘶𝘳- 𝙇𝘱𝘳𝘦𝘷
///
/// 𝙇𝘱𝘳𝘦𝘷 is initialized with 𝘓𝘴
/// After each vesting refresh:
/// 𝙇𝘱𝘳𝘦𝘷 <- 𝙇𝘤𝘶𝘳
///
/// 𝝙V: Vesting delta (#coins).
/// 𝝙V = Vʟ * 𝝙L
///

pub fn compute_vesting_delta() {
    todo!()
}
