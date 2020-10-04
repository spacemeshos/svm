#[derive(PartialEq, Copy, Clone)]
pub enum VaultType {
    Simple,
    MultiSig,
}

impl From<bool> for VaultType {
    fn from(v: bool) -> Self {
        match v {
            true => VaultType::Simple,
            false => VaultType::MultiSig,
        }
    }
}

impl From<VaultType> for bool {
    fn from(ty: VaultType) -> bool {
        match ty {
            VaultType::Simple => true,
            VaultType::MultiSig => false,
        }
    }
}
