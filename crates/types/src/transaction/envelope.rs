use crate::{AppAddr, Gas, Layer};

#[derive(Debug, Clone, PartialEq)]
pub struct TxEnvelope {
    principal: AppAddr,

    layer: Layer,

    gas_limit: Gas,

    
}
