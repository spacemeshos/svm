///
/// 𝘛𝘴 - Vesting start time (Unix time).
/// 𝘛𝘧 - Vesting final time (Unix time).
///
/// TBL - Time Between subsequent Layers (in seconds).
/// This value is guaranteed to stay constant.
///
/// L: #Layers during the Grant period
/// L = (𝘛𝘧 - 𝘛𝘴) / TBL
///
/// 𝗖 - Grant #coins.
/// V - Vested #coins.
///
/// Vʟ: vesting-per-layer
/// Vʟ = 𝗖 / L
///
/// 𝝙L: Layers delta.
/// 𝝙L = `current_layer - last_sync_layer`
///
/// 𝝙V: Vesting delta (#coins).
/// 𝝙V = Vʟ * 𝝙L
///

pub fn compute_vested_delta() {
    todo!()
}
