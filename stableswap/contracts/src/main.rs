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
mod erc20_helpers;

use crate::constants::*;
use crate::error::Error;
use crate::helpers::*;
use crate::lock::*;
use crate::pausable::*;
use crate::owner::*;
use alloc::{
    string::{String},
    vec::*,
    vec
};
use casper_contract::{
    contract_api::{
        runtime, storage
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, Key,
    RuntimeArgs, U128, CLValue
};

#[no_mangle]
pub extern "C" fn init() {
    if get_key::<String>(SWAP_STORAGE).is_some() {
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

    storage::new_dictionary(TOKEN_INDEXES)
        .unwrap_or_revert_with(Error::FailedToCreateDictionary);
        
    let mut precision_multipliers: Vec<u128> = vec![0; decimals.len()];
    for i in 0..pooled_tokens.len() {
        let k_str = hex::encode(pooled_tokens[i].into_hash().unwrap());
        if i > 0 {
            // Check if index is already used. Check if 0th element is a duplicate.
            let existing_index: Option<u64> = helpers::get_dictionary_value_from_key(TOKEN_INDEXES, &k_str);
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
        helpers::write_dictionary_value_from_key(TOKEN_INDEXES, &k_str, i as u64);
    }

    require(a < ampl::MAX_A && fee < swap_processor::MAX_SWAP_FEE as u64 && admin_fee < swap_processor::MAX_ADMIN_FEE as u64, Error::InvalidInitializedParams);
    
    owner::init(contract_owner);
    pausable::init();

    lock::init();

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
        balances: vec![0; pooled_tokens.len()]
    };
    save_swap_storage(&swap_storage);
}

#[no_mangle]
fn call() {
    let contract_name: String = runtime::get_named_arg(STABLESWAP_CONTRACT_NAME);
    let contract_hash_key_name = String::from(contract_name.clone());
    let contract_package_hash_key_name = String::from(contract_name.clone() + "_package_name");

    let pooled_tokens: Vec<Key> = runtime::get_named_arg(ARG_POOLED_TOKENS);
    let mut decimals: Vec<u8> = vec![0; pooled_tokens.len()]; 
    // reading decimals
    for i in 0..pooled_tokens.len() {
        decimals[i] = erc20_helpers::get_decimals(pooled_tokens[i]);
    }

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
    let swap: structs::Swap = helpers::get_key(SWAP_STORAGE).unwrap();
    swap
}

fn save_swap_storage(swap: &structs::Swap) {
    helpers::set_key(SWAP_STORAGE, swap.clone());
}

fn deadline_check(deadline: u64) {
    require(helpers::current_block_timestamp() <= deadline, Error::DeadlineNotMet);
}

#[no_mangle]
pub extern "C" fn get_a() {
    let swap = read_swap_storage();
    runtime::ret(CLValue::from_t(U128::from(ampl::get_a(&swap))).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn get_a_precise() {
    let swap = read_swap_storage();
    runtime::ret(CLValue::from_t(U128::from(ampl::get_a_precise(&swap))).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn get_token() {
    let index: u64 = runtime::get_named_arg(ARG_INDEX);
    let swap = read_swap_storage();
    let r = swap.pooled_tokens[index as usize];
    runtime::ret(CLValue::from_t(r).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn get_token_index() {
    let token: Key = runtime::get_named_arg(ARG_TOKEN);
    let existing_index: Option<u64> = helpers::get_dictionary_value_from_key(TOKEN_INDEXES, &hex::encode(token.into_hash().unwrap()));
    runtime::ret(CLValue::from_t(existing_index.unwrap()).unwrap_or_revert());    
}

#[no_mangle]
pub extern  "C" fn get_token_balance() {
    let index: u64 = runtime::get_named_arg(ARG_INDEX);
    let index = index as usize;
    let swap = read_swap_storage();
    require(index < swap.pooled_tokens.len(), Error::TokenIndexOutOfRange);
    runtime::ret(CLValue::from_t(U128::from(swap.balances[index])).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn get_virtual_price() {
    let swap = read_swap_storage();
    runtime::ret(CLValue::from_t(U128::from(swap_processor::get_virtual_price(&swap))).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn calculate_swap() {
    let token_index_from: u64 = runtime::get_named_arg(ARG_TOKEN_INDEX_FROM);
    let token_index_to: u64 = runtime::get_named_arg(ARG_TOKEN_INDEX_TO);
    let dx: U128 = runtime::get_named_arg(ARG_DX);

    let swap = read_swap_storage();
    let r = swap_processor::calculate_swap(&swap, token_index_from as usize, token_index_to as usize, dx.as_u128());
    runtime::ret(CLValue::from_t(U128::from(r)).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn calculate_token_amount() {
    let amounts: Vec<U128> = runtime::get_named_arg(ARG_AMOUNTS);
    let deposit: bool = runtime::get_named_arg(ARG_DEPOSIT);
    let swap = read_swap_storage();
    let r = swap_processor::calculate_token_amount(&swap, &amounts.into_iter().map(|x| x.as_u128()).collect(), deposit);
    runtime::ret(CLValue::from_t(U128::from(r)).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn calculate_remove_liquidity() {
    let amount: U128 = runtime::get_named_arg(ARG_AMOUNT);
    let mut swap = read_swap_storage();
    let ret = swap_processor::calculate_remove_liquidity(&mut swap, amount.as_u128());
    runtime::ret(CLValue::from_t(ret.into_iter().map(|x| U128::from(x)).collect::<Vec<U128>>()).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn calculate_remove_liquidity_one_token() {
    let token_amount: U128 = runtime::get_named_arg(ARG_TOKEN_AMOUNT);
    let token_index: u64 = runtime::get_named_arg(ARG_TOKEN_INDEX);
    let swap = read_swap_storage();
    let r = swap_processor::calculate_withdraw_one_token(&swap, token_amount.as_u128(), token_index as usize);
    runtime::ret(CLValue::from_t(U128::from(r)).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn get_admin_balance() {
    let index: u64 = runtime::get_named_arg(ARG_TOKEN_INDEX);
    let swap = read_swap_storage();
    let r = swap_processor::get_admin_balance(&swap, index as usize);
    runtime::ret(CLValue::from_t(U128::from(r)).unwrap_or_revert());    
}

#[no_mangle]
pub extern "C" fn update_lp() -> Result<(), Error> {
    only_owner();
    let lp_token: Key = runtime::get_named_arg(ARG_LP_TOKEN);
    let mut swap = read_swap_storage();
    swap.lp_token = lp_token;
    save_swap_storage(&swap);
    Ok(()) 
}

#[no_mangle]
pub extern "C" fn swap() -> Result<(), Error> {
    when_not_locked();
    when_not_paused();
    lock_contract();

    let token_index_from: u64 = runtime::get_named_arg(ARG_TOKEN_INDEX_FROM);
    let token_index_to: u64 = runtime::get_named_arg(ARG_TOKEN_INDEX_TO);
    let dx: U128 = runtime::get_named_arg(ARG_DX);
    let min_dy: U128 = runtime::get_named_arg(ARG_MIN_DY);
    let deadline: u64 = runtime::get_named_arg(ARG_DEADLINE);
    deadline_check(deadline);

    let mut swap = read_swap_storage();
    swap_processor::swap(&mut swap, token_index_from as usize, token_index_to as usize, dx.as_u128(), min_dy.as_u128());
    unlock_contract();
    save_swap_storage(&swap);
    Ok(())
}

#[no_mangle]
pub extern "C" fn add_liquidity() -> Result<(), Error> {
    when_not_locked();
    when_not_paused();
    lock_contract();    
    let amounts: Vec<U128> = runtime::get_named_arg(ARG_AMOUNTS);
    let min_to_mint: U128 = runtime::get_named_arg(ARG_MIN_TO_MINT);
    let deadline: u64 = runtime::get_named_arg(ARG_DEADLINE);
    deadline_check(deadline);

    let mut swap = read_swap_storage();
    swap_processor::add_liquidity(&mut swap, amounts.into_iter().map(|x| x.as_u128()).collect(), min_to_mint.as_u128());
    unlock_contract();
    save_swap_storage(&swap);
    Ok(())
}

#[no_mangle]
pub extern "C" fn remove_liquidity() -> Result<(), Error> {
    when_not_locked();
    when_not_paused();
    lock_contract();

    let amount: U128 = runtime::get_named_arg(ARG_AMOUNT);
    let min_amounts: Vec<U128> = runtime::get_named_arg(ARG_MIN_AMOUNTS);
    let deadline: u64 = runtime::get_named_arg(ARG_DEADLINE);
    deadline_check(deadline);

    let mut swap = read_swap_storage();
    swap_processor::remove_liquidity(&mut swap, amount.as_u128(), min_amounts.into_iter().map(|x| x.as_u128()).collect());
    unlock_contract();
    save_swap_storage(&swap);
    Ok(())
}

#[no_mangle]
pub extern "C" fn remove_liquidity_one_token() -> Result<(), Error> {
    when_not_locked();
    when_not_paused();
    lock_contract();

    let token_amount: U128 = runtime::get_named_arg(ARG_TOKEN_AMOUNT);
    let token_index: u64 = runtime::get_named_arg(ARG_TOKEN_INDEX);
    let min_amount: U128 = runtime::get_named_arg(ARG_MIN_AMOUNT);
    let deadline: u64 = runtime::get_named_arg(ARG_DEADLINE);

    deadline_check(deadline);

    let mut swap = read_swap_storage();
    swap_processor::remove_liquidity_one_token(&mut swap, token_amount.as_u128(), token_index as usize, min_amount.as_u128());
    unlock_contract();
    save_swap_storage(&swap);
    Ok(())
}

#[no_mangle]
pub extern "C" fn remove_liquidity_imbalance() -> Result<(), Error> {
    when_not_locked();
    when_not_paused();
    lock_contract();

    let amounts: Vec<U128> = runtime::get_named_arg(ARG_AMOUNTS);
    let max_burn_amount: U128 = runtime::get_named_arg(ARG_MAX_BURN_AMOUNT);
    let deadline: u64 = runtime::get_named_arg(ARG_DEADLINE);

    deadline_check(deadline);

    let mut swap = read_swap_storage();
    swap_processor::remove_liquidity_imbalance(&mut swap, amounts.into_iter().map(|x| x.as_u128()).collect(), max_burn_amount.as_u128());
    unlock_contract();
    save_swap_storage(&swap);
    Ok(())
}

#[no_mangle]
pub extern "C" fn withdraw_admin_fees() -> Result<(), Error> {
    only_owner();
    let mut swap = read_swap_storage();
    swap_processor::withdraw_admin_fees(&mut swap, owner_internal());
    save_swap_storage(&swap);
    Ok(())
}

#[no_mangle]
pub extern "C" fn set_admin_fee() -> Result<(), Error> {
    only_owner();
    let new_fee: u64 = runtime::get_named_arg(ARG_ADMIN_FEE);
    let mut swap = read_swap_storage();
    swap_processor::set_admin_fee(&mut swap, new_fee);
    save_swap_storage(&swap);
    Ok(())
}

#[no_mangle]
pub extern "C" fn set_swap_fee() -> Result<(), Error> {
    only_owner();
    let new_fee: u64 = runtime::get_named_arg(ARG_SWAP_FEE);
    let mut swap = read_swap_storage();
    swap_processor::set_swap_fee(&mut swap, new_fee);
    save_swap_storage(&swap);
    Ok(())
}

#[no_mangle]
pub extern "C" fn ramp_a() -> Result<(), Error> {
    only_owner();
    let future_a: U128 = runtime::get_named_arg(ARG_FUTURE_A);
    let future_time: u64 = runtime::get_named_arg(ARG_FUTURE_TIME);
    let mut swap = read_swap_storage();
    ampl::ramp_a(&mut swap, future_a.as_u128(), future_time);
    save_swap_storage(&swap);
    Ok(())
}

#[no_mangle]
pub extern "C" fn stop_ramp_a() -> Result<(), Error> {
    only_owner();
    let mut swap = read_swap_storage();
    ampl::stop_ramp_a(&mut swap);
    save_swap_storage(&swap);
    Ok(())
}
