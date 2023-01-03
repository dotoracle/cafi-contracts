use alloc::{
    string::{String, ToString},
    vec::*,
};

use casper_contract::contract_api::storage;
use casper_types::{contracts::NamedKeys, ContractPackageHash, Key, U256};

use crate::constants::*;
use serde::{Deserialize, Serialize};


pub fn default(
    contract_owner: Key,
    reward_token: Key,
    token_per_block: U256,
    contract_package_hash: ContractPackageHash,
    start_block: U256,
) -> NamedKeys {
    let mut named_keys = NamedKeys::new();

    // Contract 'Named keys'
    named_keys.insert(
        CONTRACT_OWNER_KEY_NAME.to_string(),
        Key::from(storage::new_uref(contract_owner)),
    );
    // named_keys.insert(
    //     MARKET_FEE_RECEIVER.to_string(),
    //     Key::from(storage::new_uref(market_fee_receiver)),
    // );
    // named_keys.insert(
    //     MARKET_FEE.to_string(),
    //     Key::from(storage::new_uref(U256::zero())),
    // );
    named_keys.insert(
        TOKEN_CONTRACT_LIST.to_string(),
        Key::from(storage::new_uref(Vec::<Key>::new())),
    );
    named_keys.insert(
        "contract_package_hash".to_string(),
        storage::new_uref(contract_package_hash).into(),
    );
    named_keys
}
