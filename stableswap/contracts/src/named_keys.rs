use alloc::{
    string::{ToString}
};

use casper_contract::contract_api::{storage};
use casper_types::{contracts::NamedKeys, ContractPackageHash, Key};

use crate::constants::*;
pub fn default(
    contract_owner: Key,
    contract_package_hash: ContractPackageHash
) -> NamedKeys {
    let mut named_keys = NamedKeys::new();

    // Contract 'Named keys'
    named_keys.insert(
        CONTRACT_OWNER_KEY_NAME.to_string(),
        Key::from(storage::new_uref(contract_owner)),
    );

    named_keys.insert(
        "contract_package_hash".to_string(),
        storage::new_uref(contract_package_hash).into(),
    );

    named_keys
}
