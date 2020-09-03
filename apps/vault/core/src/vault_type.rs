#[derive(PartialEq, Copy, Clone)]
pub enum VaultType {
    Simple,
    MultiSig,
}

impl From<u32> for VaultType {
    fn from(v: u32) -> Self {
        match v {
            0 => VaultType::Simple,
            1 => VaultType::MultiSig,
            _ => todo!("log error"),
        }
    }
}

impl From<VaultType> for u32 {
    fn from(ty: VaultType) -> u32 {
        match ty {
            VaultType::Simple => 0,
            VaultType::MultiSig => 1,
        }
    }
}
