#![no_main]
#![no_std]
#![feature(type_ascription)]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
mod address;
pub mod constants;
mod entry_points;
mod error;
mod events;
mod helpers;
pub mod named_keys;
mod ampl;
mod swap_processor;
mod structs;
mod math_utils;
mod lock;
mod pausable;
mod owner;

use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::error::Error;
use crate::helpers::*;
use crate::lock::*;
use crate::pausable::*;
use crate::owner::*;
use alloc::{
    string::{String, ToString},
    vec::*,
};
use casper_contract::{
    contract_api::{
        runtime, storage
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, Key,
    RuntimeArgs, U128
};

#[no_mangle]
pub extern "C" fn init() {
    if get_key::<Key>(ARG_LP_TOKEN).is_some() {
        runtime::revert(Error::ContractAlreadyInitialized);
    }
    let pooled_tokens: Vec<Key> = runtime::get_named_arg(ARG_POOLED_TOKENS);
    let decimals: Vec<u8> = runtime::get_named_arg(ARG_TOKEN_DECIMALS);
    let lp_token: Key = runtime::get_named_arg(ARG_LP_TOKEN);
    let a: U128 = runtime::get_named_arg(ARG_A);
    let a = a.as_u128();
    let fee: u64 = runtime::get_named_arg(ARG_FEE);
    let admin_fee: u64 = runtime::get_named_arg(ARG_ADMIN_FEE);
    let contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);

    require(pooled_tokens.len() > 1 && pooled_tokens.len() <= 32, Error::PooledTokensLength);
    require(pooled_tokens.len() == decimals.len(), Error::PooledTokensDecimalsMismatch);

    let mut precision_multipliers: Vec<u128> = Vec::with_capacity(decimals.len());
    for i in 0..pooled_tokens.len() {
        if i > 0 {
            // Check if index is already used. Check if 0th element is a duplicate.
            let existing_index: Option<u64> = helpers::get_dictionary_value_from_key(TOKEN_INDEXES, &pooled_tokens[i].to_string());
            require(
                existing_index.is_none() &&
                pooled_tokens[0] != pooled_tokens[i],
                Error::DuplicateToken
            );
        }

        require(
            decimals[i] <= swap_processor::POOL_PRECISION_DECIMALS,
            Error::TokenDecimalsExceedMax
        );
        precision_multipliers[i] = 10u128.pow((swap_processor::POOL_PRECISION_DECIMALS - decimals[i]) as u32);
        helpers::write_dictionary_value_from_key(TOKEN_INDEXES, &pooled_tokens[i].to_string(), i as u64);
    }

    require(a < ampl::MAX_A && fee < swap_processor::MAX_SWAP_FEE as u64 && admin_fee < swap_processor::MAX_ADMIN_FEE as u64, Error::InvalidInitializedParams);

    runtime::put_key(
        CONTRACT_OWNER_KEY_NAME,
        storage::new_uref(contract_owner).into(),
    );

    runtime::put_key(
        PAUSED,
        storage::new_uref(false).into(),
    );

    runtime::put_key(
        IS_LOCKED,
        storage::new_uref(false).into(),
    );

    let swap_storage = structs::Swap {
        initial_a: a * ampl::A_PRECISION,
        future_a: a * ampl::A_PRECISION,
        initial_a_time: 0,
        future_a_time: 0,
        swap_fee: fee,
        admin_fee: admin_fee,
        lp_token: lp_token,
        pooled_tokens: pooled_tokens.clone(),
        token_precision_multipliers: precision_multipliers,
        balances: Vec::with_capacity(pooled_tokens.len())
    };
    runtime::put_key(
        SWAP_STORAGE,
        storage::new_uref(casper_serde_json_wasm::to_string(&swap_storage).unwrap()).into(),
    );
    storage::new_dictionary(TOKEN_INDEXES)
        .unwrap_or_revert_with(Error::FailedToCreateDictionary);
}

#[no_mangle]
fn call() {
    let contract_name: String = runtime::get_named_arg(STABLESWAP_CONTRACT_NAME);
    let contract_hash_key_name = String::from(contract_name.clone());
    let contract_package_hash_key_name = String::from(contract_name.clone() + "_package_name");

    let pooled_tokens: Vec<Key> = runtime::get_named_arg(ARG_POOLED_TOKENS);
    let decimals: Vec<u8> = runtime::get_named_arg(ARG_TOKEN_DECIMALS);
    let lp_token: Key = runtime::get_named_arg(ARG_LP_TOKEN);
    let a: U128 = runtime::get_named_arg(ARG_A);
    let fee: u64 = runtime::get_named_arg(ARG_FEE);
    let admin_fee: u64 = runtime::get_named_arg(ARG_ADMIN_FEE);
    let contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);

    let (contract_package_hash, _) = storage::create_contract_package_at_hash();

    let named_keys: NamedKeys = named_keys::default(
        contract_owner,
        contract_package_hash
    );

    let (contract_hash, _version) = storage::add_contract_version(
        contract_package_hash,
        entry_points::default(),
        named_keys
    );
    runtime::put_key(contract_hash_key_name.as_str(), Key::from(contract_hash));
    runtime::put_key(contract_package_hash_key_name.as_str(), Key::from(contract_package_hash));

    runtime::call_contract::<()>(
        contract_hash,
        INIT_ENTRY_POINT_NAME,
        runtime_args! {
            ARG_POOLED_TOKENS => pooled_tokens,
            ARG_TOKEN_DECIMALS => decimals,
            ARG_LP_TOKEN => lp_token,
            ARG_A => a,
            ARG_FEE => fee,
            ARG_ADMIN_FEE => admin_fee,
            ARG_CONTRACT_OWNER => contract_owner

        },
    );
}

fn read_swap_storage() -> structs::Swap {
    let str: String = helpers::get_key(SWAP_STORAGE).unwrap();
    casper_serde_json_wasm::from_str::<structs::Swap>(&str).unwrap()
}

fn deadline_check(deadline: u64) {
    require(helpers::current_block_timestamp() <= deadline, Error::DeadlineNotMet);
}

pub fn get_a() -> u128 {
    let mut swap = read_swap_storage();
    ampl::get_a(&mut swap)
}

pub fn get_a_precise() -> u128 {
    let mut swap = read_swap_storage();
    ampl::get_a_precise(&mut swap)
}

pub fn get_token(index: usize) -> Key {
    let swap = read_swap_storage();
    swap.pooled_tokens[index]
}

pub fn get_token_index(token: Key) -> u64 {
    let existing_index: Option<u64> = helpers::get_dictionary_value_from_key(TOKEN_INDEXES, &token.to_string());
    existing_index.unwrap()
}

pub fn get_token_balance(index: usize) -> u128 {
    let swap = read_swap_storage();
    require(index < swap.pooled_tokens.len(), Error::TokenIndexOutOfRange);
    swap.balances[index]
}

pub fn get_virtual_price() -> u128 {
    let mut swap = read_swap_storage();
    swap_processor::get_virtual_price(&mut swap)
}

pub fn calculate_swap(token_index_from: usize, token_index_to: usize, dx: u128) -> u128 {
    let mut swap = read_swap_storage();
    swap_processor::calculate_swap(&mut swap, token_index_from, token_index_to, dx)
}

pub fn calculate_token_amount(amounts: Vec<u128>, deposit: bool) -> u128 {
    let mut swap = read_swap_storage();
    swap_processor::calculate_token_amount(&mut swap, &amounts, deposit)
}

pub fn calculate_remove_liquidity(amount: u128) -> Vec<u128> {
    let mut swap = read_swap_storage();
    swap_processor::calculate_remove_liquidity(&mut swap, amount)
}

pub fn calculate_remove_liquidity_one_token(token_amount: u128, token_index: usize) -> u128 {
    let mut swap = read_swap_storage();
    swap_processor::calculate_withdraw_one_token(&mut swap, token_amount, token_index)
}

pub fn get_admin_balance(index: usize) -> u128 {
    let mut swap = read_swap_storage();
    swap_processor::get_admin_balance(&mut swap, index)
}

// function swap(
//     uint8 tokenIndexFrom,
//     uint8 tokenIndexTo,
//     uint256 dx,
//     uint256 minDy,
//     uint256 deadline
// )
//     external
//     payable
//     virtual
//     nonReentrant
//     whenNotPaused
//     deadlineCheck(deadline)
//     returns (uint256)
// {
//     return swapStorage.swap(tokenIndexFrom, tokenIndexTo, dx, minDy);
// }
pub fn swap(
    token_index_from: usize,
    token_index_to: usize,
    dx: u128,
    min_dy: u128,
    deadline: u64
) -> u128 {
    when_not_locked();
    when_not_paused();
    deadline_check(deadline);
    lock_contract();
    let mut swap = read_swap_storage();
    let ret = swap_processor::swap(&mut swap, token_index_from, token_index_to, dx, min_dy);
    unlock_contract();
    ret
}

pub fn add_liquidity(
    amounts: Vec<u128>,
    min_to_mint: u128,
    deadline: u64
) -> u128 {
    when_not_locked();
    when_not_paused();
    deadline_check(deadline);
    lock_contract();    
    let mut swap = read_swap_storage();
    let ret = swap_processor::add_liquidity(&mut swap, amounts, min_to_mint);
    unlock_contract();
    ret
}

pub fn remove_liquidity(
    amount: u128,
    min_amounts: Vec<u128>,
    deadline: u64
) -> Vec<u128> {
    when_not_locked();
    when_not_paused();
    deadline_check(deadline);
    lock_contract();
    let mut swap = read_swap_storage();
    let ret = swap_processor::remove_liquidity(&mut swap, amount, min_amounts);
    unlock_contract();
    ret
}

pub fn remove_liquidity_one_token(
    token_amount: u128,
    token_index: usize,
    min_amount: u128,
    deadline: u64
) -> u128 {
    when_not_locked();
    when_not_paused();
    deadline_check(deadline);
    lock_contract();
    let mut swap = read_swap_storage();
    let ret = swap_processor::remove_liquidity_one_token(&mut swap, token_amount, token_index, min_amount);
    unlock_contract();
    ret
}

pub fn remove_liquidity_imbalance(
    amounts: Vec<u128>,
    max_burn_amount: u128,
    deadline: u64
) -> u128 {
    when_not_locked();
    when_not_paused();
    deadline_check(deadline);
    lock_contract();
    let mut swap = read_swap_storage();
    let ret = swap_processor::remove_liquidity_imbalance(&mut swap, amounts, max_burn_amount);
    unlock_contract();
    ret
}

pub fn withdraw_admin_fees() {
    only_owner();
    let mut swap = read_swap_storage();
    swap_processor::withdraw_admin_fees(&mut swap, owner());
}

pub fn set_admin_fee(new_fee: u64) {
    only_owner();
    let mut swap = read_swap_storage();
    swap_processor::set_admin_fee(&mut swap, new_fee);
}

pub fn set_swap_fee(new_fee: u64) {
    only_owner();
    let mut swap = read_swap_storage();
    swap_processor::set_swap_fee(&mut swap, new_fee);
}

pub fn ramp_a(future_a: u128, future_time: u64) {
    only_owner();
    let mut swap = read_swap_storage();
    ampl::ramp_a(&mut swap, future_a, future_time);
}

pub fn stop_ramp_a() {
    only_owner();
    let mut swap = read_swap_storage();
    ampl::stop_ramp_a(&mut swap);
}
