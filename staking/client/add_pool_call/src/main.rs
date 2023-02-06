#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::string::String;

use casper_contract::contract_api::{runtime};
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs};

const ENTRY_POINT_ADD_POOL: &str = "add_new_pool";

const ARG_STAKING_CONTRACT_HASH: &str = "staking_contract_hash";
const ARG_TOKEN_OWNER: &str = "token_owner";
const ARG_TOKEN_META_DATA: &str = "token_meta_data";


#[no_mangle]
pub extern "C" fn call() {
    let staking_contract_hash: ContractHash = runtime::get_named_arg::<Key>(ARG_STAKING_CONTRACT_HASH)
        .into_hash()
        .map(|hash| ContractHash::new(hash))
        .unwrap();

    let token_owner = runtime::get_named_arg::<Key>(ARG_TOKEN_OWNER);
    let token_metadata: String = runtime::get_named_arg(ARG_TOKEN_META_DATA);

    let _;() = runtime::call_contract::<()>(
        staking_contract_hash,
        ENTRY_POINT_ADD_POOL,
        runtime_args! {
            ARG_TOKEN_OWNER => token_owner,
            ARG_TOKEN_META_DATA => token_metadata,
        },
    );

    runtime::put_key(&receipt_name, owned_tokens_dictionary_key)
}
