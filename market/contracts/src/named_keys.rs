use alloc::{
    string::{String, ToString},
    vec::*,
};

use casper_contract::contract_api::storage;
use casper_types::{contracts::NamedKeys, Key, U256, ContractPackageHash};

use crate::constants::*;
pub fn default(
    contract_owner: Key,
    market_fee_receiver: Key,
    market_fee: U256,
    contract_package_hash : ContractPackageHash,
    royalty_fee : U256,
    fee_token: Option<Key>,
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


    if fee_token.is_some() {
        named_keys.insert(
            FEE_TOKEN_KEY_NAME.to_string(),
            Key::from(storage::new_uref(fee_token.unwrap())),
        );
    }

    named_keys
}
