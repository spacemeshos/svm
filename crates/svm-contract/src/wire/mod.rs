//!      Deploy Contract Wire Protocol Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   proto    |                |                       |
//!  |  version   |  name length   |     name (UTF-8)      |
//!  |  (4 bytes) |   (1 byte)     |                       |
//!  |____________|________________|_______________________|
//!  |                                                     |
//!  |                      author                         |
//!  |                    (32 bytes)                       |
//!  |_____________________________________________________|
//!  |             |                                       |
//!  |  #admins    |         admins addresses              |
//!  |  (2 bytes)  |          (32 bytes each)              |
//!  |_____________|_______________________________________|
//!  |           |                                         |
//!  |   #deps   |           dependencies                  |
//!  | (2 bytes) |              (TBD)                      |
//!  |___________|_________________________________________|
//!  |                |                                    |
//!  |  code length   |              code                  |
//!  |   (8 bytes)    |             (wasm)                 |
//!  |________________|____________________________________|
//!

mod error;
mod field;
mod parse;
mod validate;

pub use crate::wasm::WasmContract;

use crate::traits::ContractAddressCompute;
use crate::types::ContractTypes;
use parse::parse_contract;
use validate::validate_contract;

pub fn build_wasm_contract<CT: ContractTypes>(
    bytes: &[u8],
) -> Result<WasmContract, error::ContractError> {
    let mut contract = parse_contract(bytes)?;

    validate_contract(&contract)?;
    add_contract_address::<CT>(&mut contract);

    Ok(contract)
}

fn add_contract_address<CT: ContractTypes>(contract: &mut WasmContract) {
    let address = <CT as ContractTypes>::AddressCompute::compute(&contract);
    contract.address = Some(address);
}
