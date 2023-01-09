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
use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::error::Error;
use crate::helpers::*;

use alloc::{
    string::{String, ToString},
    vec::*,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, ContractHash, ContractPackageHash, HashAddr, Key,
    RuntimeArgs, U256,
};
use events::StakingEvent;
use helpers::{get_immediate_caller_key, get_self_key, get_user_info_key};

// pub const u256_10_18 : U256 = U256::pow(U256::from("10"), U256::from("18"));
pub const u256_10_18: u64 = u64::pow(10, 6);

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct TokenStake {
    stake_amount: U256,
    locked_from: U256,
    locked_until: U256,
}
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct UserInfo {
    total_stake_amount: U256,
    reward_debt: U256,
    pending_rewards: U256,
    locked_until: U256,
    // stakes: Vec<TokenStake>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct PoolInfo {
    pub pool_id: u64,
    pub lp_token: Key,
    pub alloc_point: U256,
    pub last_reward_second: U256,
    pub acc_reward_per_share: U256,
    pub min_stake_duration: U256,
    pub early_withdraw_penalty_rate: U256,
    pub lp_supply: U256,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct PoolList {
    pub all_pool: Vec<PoolInfo>,
}

#[no_mangle]
pub extern "C" fn init() {
    if get_key::<Key>(CONTRACT_HASH_KEY_NAME).is_some() {
        runtime::revert(Error::ContractAlreadyInitialized);
    }
    let contract_hash: Key = runtime::get_named_arg(ARG_CONTRACT_HASH);

    let contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);

    let reward_token: Key = runtime::get_named_arg(ARG_REWARD_TOKEN);

    let reward_per_second: U256 = runtime::get_named_arg(ARG_REWARD_PER_SECOND);

    let start_second: U256 = runtime::get_named_arg(ARG_START_SECOND);

    runtime::put_key(
        CONTRACT_HASH_KEY_NAME,
        storage::new_uref(contract_hash).into(),
    );

    runtime::put_key(
        CONTRACT_OWNER_KEY_NAME,
        storage::new_uref(contract_owner).into(),
    );

    runtime::put_key(REWARD_TOKEN, storage::new_uref(reward_token).into());
    runtime::put_key(
        REWARD_PER_SECOND,
        storage::new_uref(reward_per_second as U256).into(),
    );
    runtime::put_key(START_BLOCK, storage::new_uref(start_second as U256).into());
    runtime::put_key(NUMBER_OF_POOL, storage::new_uref(0 as u64).into());
    runtime::put_key(TOTAL_ALLOC_POINT, storage::new_uref(U256::from("0")).into());

    // let init_pool_list = PoolList {
    //     all_pool: Vec::new(),
    // };
    // runtime::put_key(
    //     POOL_LIST,
    //     storage::new_uref(casper_serde_json_wasm::to_string_pretty(&init_pool_list).unwrap())
    //         .into(),
    // );
    // storage::new_dictionary(TOKEN_CONTRACT_MAP)
    //     .unwrap_or_revert_with(Error::FailedToCreateDictionary);
    // storage::new_dictionary(TOKEN_STAKE).unwrap_or_revert_with(Error::FailedToCreateDictionary);
    storage::new_dictionary(USER_INFO).unwrap_or_revert_with(Error::FailedToCreateDictionary);
    storage::new_dictionary(POOL_INFO).unwrap_or_revert_with(Error::FailedToCreateDictionary);
}

#[no_mangle]
fn call() {
    let contract_name: String = runtime::get_named_arg(STAKING_CONTRACT_NAME);
    let contract_hash_key_name = String::from(contract_name.clone());
    let contract_package_hash_key_name = String::from(contract_name.clone() + "_package_name");
    let contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);
    let reward_token: Key = runtime::get_named_arg(ARG_REWARD_TOKEN);
    let reward_per_second: U256 = runtime::get_named_arg(ARG_REWARD_PER_SECOND);
    let start_second: U256 = runtime::get_named_arg(ARG_START_SECOND);

    let (contract_package_hash, access_uref) = storage::create_contract_package_at_hash();

    let named_keys: NamedKeys = named_keys::default(
        contract_owner,
        reward_token,
        reward_per_second,
        contract_package_hash,
        start_second,
    );

    let (contract_hash, _version) = storage::add_contract_version(
        contract_package_hash: ContractPackageHash,
        entry_points::default(),
        named_keys: NamedKeys,
    );
    runtime::put_key(contract_hash_key_name.as_str(), Key::from(contract_hash));

    runtime::call_contract::<()>(
        contract_hash,
        INIT_ENTRY_POINT_NAME,
        runtime_args! {
            ARG_CONTRACT_HASH => Key::from(contract_hash),
            ARG_CONTRACT_OWNER => Key::from(contract_owner),
            ARG_REWARD_TOKEN => Key::from(reward_token),
            ARG_REWARD_PER_SECOND => reward_per_second,
            ARG_START_SECOND => start_second,
        },
    );
}

// this is userInfo

fn get_user_info(pool_id: u64, user: Key) -> UserInfo {
    let user_info_key = get_user_info_key(pool_id, user);
    let user_info: UserInfo;
    // let user_info_str = get_dictionary_value_from_key::<String>(USER_INFO, &user_info_key);

    if get_dictionary_value_from_key::<String>(USER_INFO, &user_info_key).is_some() {
        let user_info_str =
            get_dictionary_value_from_key::<String>(USER_INFO, &user_info_key).unwrap_or_revert();
        user_info = casper_serde_json_wasm::from_str::<UserInfo>(&user_info_str).unwrap();

        // if casper_serde_json_wasm::from_str::<UserInfo>(&user_info_str).is_err() {
        //     runtime::revert(Error::CanNotGetUserInfo);
        // } else {
        //     let user_info_result = casper_serde_json_wasm::from_str::<UserInfo>(&user_info_str).unwrap();
        //     user_info = UserInfo {
        //         total_stake_amount: user_info_result.total_stake_amount,
        //         reward_debt: user_info_result.reward_debt,
        //         pending_rewards: user_info_result.pending_rewards,
        //         locked_until: user_info_result.locked_until,
        //         // stakes: user_info_result.stakes,
        //     }
        // };
    } else {
        user_info = UserInfo {
            total_stake_amount: U256::from("0"),
            reward_debt: U256::from("0"),
            pending_rewards: U256::from("0"),
            locked_until: U256::from("0"),
            // stakes: Vec::new(),
        }
    }
    user_info
}

// write user_info

fn write_dictionary_user_info(
    pool_id: u64,
    user: Key,
    new_user_info: UserInfo,
) -> Result<(), Error> {
    let user_info_key = get_user_info_key(pool_id, user);

    write_dictionary_value_from_key(
        USER_INFO,
        &user_info_key,
        casper_serde_json_wasm::to_string_pretty(&new_user_info).unwrap(),
    );

    Ok(())
}

// this is pool_info
fn get_pool_info(pool_id: u64) -> PoolInfo {
    let pool_info_str: String =
        helpers::get_dictionary_value_from_key(POOL_INFO, &pool_id.clone().to_string()).unwrap();
    if casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).is_err() {
        runtime::revert(Error::CanNotGetPoolList)
    }

    let pool_info = casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).unwrap();

    pool_info
}

// Add new pool for new lp_token
#[no_mangle]
pub extern "C" fn add_new_pool2() -> Result<(), Error> {
    let caller = get_immediate_caller_key();

    // let caller = helpers::get_verified_caller().unwrap_or_revert();
    let current_contract_owner = helpers::get_stored_value_with_user_errors(
        CONTRACT_OWNER_KEY_NAME,
        Error::MissingContractOwner,
        Error::InvalidContractOwner,
    );

    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }

    let lp_contract_hash: Key = runtime::get_named_arg(ARG_LP_CONTRACT_HASH);

    // let lp_contract_str_key = helpers::make_dictionary_item_key_for_key(lp_contract_hash);
    // if get_dictionary_value_from_key::<String>(POOL_INFO, &lp_contract_str_key).is_some() {
    //     runtime::revert(Error::PoolAlreadyInit);
    // }

    let alloc_point: U256 = runtime::get_named_arg(ARG_ALLOC_POINT);
    let last_reward_second: U256 = runtime::get_named_arg(ARG_LAST_REWARD_SECOND);
    let acc_reward_per_block: U256 = runtime::get_named_arg(ARG_ACC_REWARD_PER_SHARE);
    let min_stake_duration: U256 = runtime::get_named_arg(ARG_MIN_STAKE_DURATION);
    let early_withdraw_penalty_rate: U256 = runtime::get_named_arg(ARG_PENALTY_RATE);
    let new_pool = PoolInfo {
        pool_id: 0,
        lp_token: lp_contract_hash,
        alloc_point: alloc_point,
        last_reward_second: last_reward_second,
        acc_reward_per_share: acc_reward_per_block,
        min_stake_duration: min_stake_duration,
        early_withdraw_penalty_rate: early_withdraw_penalty_rate,
        lp_supply: U256::from("0"),
    };

    let current_list_str: String = helpers::get_stored_value_with_user_errors(
        POOL_LIST,
        Error::MissingPoolList,
        Error::InvalidPoolList,
    );

    // let current_list = casper_serde_json_wasm::from_str::<PoolList>(&current_list_str).unwrap();

    if casper_serde_json_wasm::from_str::<PoolList>(&current_list_str).is_err() {
        runtime::revert(Error::CanNotGetPoolList)
    }

    let current_list = casper_serde_json_wasm::from_str::<PoolList>(&current_list_str);

    let mut list: Vec<PoolInfo> = current_list.unwrap().all_pool;

    // let mut list: Vec<PoolInfo> = current_list.all_pool;
    list.push(new_pool);
    let add_pool_list = PoolList { all_pool: list };

    let total_alloc_point: U256 = helpers::get_stored_value_with_user_errors::<U256>(
        TOTAL_ALLOC_POINT,
        Error::MissingTotalAllocPoint,
        Error::InvalidTotalAllocPoint,
    );

    let new_total_alloc_point: U256 = total_alloc_point + alloc_point;

    set_key(
        POOL_LIST,
        casper_serde_json_wasm::to_string_pretty(&add_pool_list).unwrap(),
    );

    set_key(TOTAL_ALLOC_POINT, new_total_alloc_point);

    // write_dictionary_value_from_key(
    //     POOL_INFO,
    //     &lp_contract_str_key,
    //     casper_serde_json_wasm::to_string_pretty(&PoolInfo {
    //         lp_token: lp_contract_hash,
    //         total_weight: total_weight,
    //         alloc_point: alloc_point,
    //         last_reward_block: last_reward_block,
    //         acc_reward_per_block: acc_reward_per_block,
    //         min_stake_duration: min_stake_duration,
    //         early_withdraw_penalty_rate: early_withdraw_penalty_rate,
    //     })
    //     .unwrap(),
    // );

    Ok(())
}

// Add new pool for new lp_token
#[no_mangle]
pub extern "C" fn add_new_pool() -> Result<(), Error> {
    let caller = get_immediate_caller_key();

    // let caller = helpers::get_verified_caller().unwrap_or_revert();
    let current_contract_owner: Key = helpers::get_stored_value_with_user_errors(
        CONTRACT_OWNER_KEY_NAME,
        Error::MissingContractOwner,
        Error::InvalidContractOwner,
    );
    let current_number_of_pool: u64 = helpers::get_stored_value_with_user_errors(
        NUMBER_OF_POOL,
        Error::MissingNumberOfPool,
        Error::InvalidNumberOfPool,
    );

    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }

    let lp_contract_hash: Key = runtime::get_named_arg(ARG_LP_CONTRACT_HASH);

    // let lp_contract_str_key = helpers::make_dictionary_item_key_for_key(lp_contract_hash);
    // if get_dictionary_value_from_key::<String>(POOL_INFO, &lp_contract_str_key).is_some() {
    //     runtime::revert(Error::PoolAlreadyInit);
    // }

    let alloc_point: U256 = runtime::get_named_arg(ARG_ALLOC_POINT);

    let last_reward_second: U256 = runtime::get_named_arg(ARG_LAST_REWARD_SECOND);
    let acc_reward_per_block: U256 = runtime::get_named_arg(ARG_ACC_REWARD_PER_SHARE);
    let min_stake_duration: U256 = runtime::get_named_arg(ARG_MIN_STAKE_DURATION);
    let early_withdraw_penalty_rate: U256 = runtime::get_named_arg(ARG_PENALTY_RATE);
    let new_pool = PoolInfo {
        pool_id: current_number_of_pool.clone(),
        lp_token: lp_contract_hash,
        alloc_point: alloc_point,
        last_reward_second: last_reward_second,
        acc_reward_per_share: acc_reward_per_block,
        min_stake_duration: min_stake_duration,
        early_withdraw_penalty_rate: early_withdraw_penalty_rate,
        lp_supply: U256::from("0"),
    };

    // let current_list_str: String = helpers::get_stored_value_with_user_errors(
    //     POOL_LIST,
    //     Error::MissingPoolList,
    //     Error::InvalidPoolList,
    // );

    // // let current_list = casper_serde_json_wasm::from_str::<PoolList>(&current_list_str).unwrap();

    // if casper_serde_json_wasm::from_str::<PoolList>(&current_list_str).is_err() {
    //     runtime::revert(Error::CanNotGetPoolList)
    // }

    // let current_list = casper_serde_json_wasm::from_str::<PoolList>(&current_list_str);

    // let mut list: Vec<PoolInfo> = current_list.unwrap().all_pool;

    // // let mut list: Vec<PoolInfo> = current_list.all_pool;
    // list.push(new_pool);
    // let add_pool_list = PoolList { all_pool: list };

    let total_alloc_point: U256 = helpers::get_stored_value_with_user_errors::<U256>(
        TOTAL_ALLOC_POINT,
        Error::MissingTotalAllocPoint,
        Error::InvalidTotalAllocPoint,
    );

    let new_total_alloc_point: U256 = total_alloc_point + alloc_point;

    // set_key(
    //     POOL_INFO,
    //     casper_serde_json_wasm::to_string_pretty(&new_pool).unwrap(),
    // );
    write_dictionary_value_from_key(
        POOL_INFO,
        &current_number_of_pool.clone().to_string(),
        casper_serde_json_wasm::to_string_pretty(&new_pool).unwrap(),
    );

    set_key(TOTAL_ALLOC_POINT, new_total_alloc_point);
    set_key(NUMBER_OF_POOL, current_number_of_pool + 1);

    // write_dictionary_value_from_key(
    //     POOL_INFO,
    //     &lp_contract_str_key,
    //     casper_serde_json_wasm::to_string_pretty(&PoolInfo {
    //         lp_token: lp_contract_hash,
    //         total_weight: total_weight,
    //         alloc_point: alloc_point,
    //         last_reward_block: last_reward_block,
    //         acc_reward_per_block: acc_reward_per_block,
    //         min_stake_duration: min_stake_duration,
    //         early_withdraw_penalty_rate: early_withdraw_penalty_rate,
    //     })
    //     .unwrap(),
    // );

    Ok(())
}

// Stake LP token
#[no_mangle]
pub extern "C" fn stake() -> Result<(), Error> {
    // This is equal to pool_id
    let pool_id: u64 = runtime::get_named_arg(ARG_POOL_ID);
    let amount: U256 = runtime::get_named_arg(ARG_AMOUNT);
    if amount <= U256::from("0") {
        runtime::revert(Error::InvalidAmount);
    }
    let stake_duration: U256 = runtime::get_named_arg(ARG_STAKE_DURATION);
    let rewards_token: Key = helpers::get_stored_value_with_user_errors::<Key>(
        REWARD_TOKEN,
        Error::MissingRewardToken,
        Error::InvalidRewardToken,
    );

    // let pool_id_usize: usize = pool_id as usize;
    // update pool with pool_id
    update_pool(rewards_token, pool_id.clone());

    let caller = get_immediate_caller_key();

    // get pool_info

    let pool_info_str: String =
        helpers::get_dictionary_value_from_key(POOL_INFO, &pool_id.to_string()).unwrap();
    // if casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).is_err() {
    //     runtime::revert(Error::CanNotGetPoolList)
    // }

    let mut pool_info = casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).unwrap();

    // Check if stake_duration is
    if stake_duration < pool_info.min_stake_duration {
        runtime::revert(Error::InvalidStakeDuration);
    }
    // Todo: write to USER_INFO (userInfo)

    let this_key: Key = get_self_key();

    let mut user_info_of_this_pool = get_user_info(pool_id, caller);
    if user_info_of_this_pool.total_stake_amount > U256::from("0") {
        // pay pending rewards
        let pending_rewards: U256 = (user_info_of_this_pool.total_stake_amount.clone()
            * pool_info.acc_reward_per_share.clone()
            / U256::from(u256_10_18))
            - user_info_of_this_pool.reward_debt;
        let total_pending: U256 = user_info_of_this_pool.pending_rewards.clone() + pending_rewards;

        user_info_of_this_pool = pay_rewards(
            rewards_token,
            this_key,
            caller,
            pending_rewards,
            user_info_of_this_pool.clone(),
        );
    }
    // Call transfer_from function to transfer deposit to this contract

    let lp_token_hash = pool_info.lp_token;
    // transfer lp_token to the pool
    transfer_from_erc20_token(lp_token_hash, caller, this_key, amount.clone());

    // Recalculate  user_info and pool_info
    user_info_of_this_pool.total_stake_amount =
        user_info_of_this_pool.total_stake_amount.clone() + amount.clone();
    pool_info.lp_supply = pool_info.lp_supply.clone() + amount.clone();

    // Todo: rewards debt

    user_info_of_this_pool.reward_debt = user_info_of_this_pool.total_stake_amount.clone()
        * pool_info.acc_reward_per_share.clone()
        / U256::from(u256_10_18);

    // save
    write_dictionary_user_info(pool_id, caller, user_info_of_this_pool);

    write_dictionary_value_from_key(
        POOL_INFO,
        &pool_id.clone().to_string(),
        casper_serde_json_wasm::to_string_pretty(&pool_info).unwrap(),
    );
    // Emit event
    events::emit(&StakingEvent::UserStake {
        user: caller.clone(),
        lp_token: lp_token_hash.clone(),
        amount: amount.clone(),
        stake_duration: stake_duration.clone(),
    });

    Ok(())
}

// Stake LP token
// #[no_mangle]
// pub extern "C" fn stake2() -> Result<(), Error> {
//     // This is equal to pool_id
//     let pool_id: u64 = helpers::get_named_arg_with_user_errors(
//         ARG_POOL_ID,
//         Error::MissingPoolId,
//         Error::InvalidPoolId,
//     )
//     .unwrap_or_revert();
//     let amount: U256 = helpers::get_named_arg_with_user_errors(
//         ARG_AMOUNT,
//         Error::MissingAmount,
//         Error::InvalidAmount,
//     )
//     .unwrap_or_revert();
//     if amount <= U256::from("0") {
//         runtime::revert(Error::InvalidAmount);
//     }
//     let stake_duration: U256 = helpers::get_named_arg_with_user_errors(
//         ARG_STAKE_DURATION,
//         Error::MissingStakeDuration,
//         Error::InvalidStakeDuration,
//     )
//     .unwrap_or_revert();
//     // let pool_id_usize: usize = pool_id as usize;
//     // update pool with pool_id
//     update_pool(pool_id.clone());

//     let caller = get_immediate_caller_key();

//     // get pool_info
//     let current_list_str: String = helpers::get_stored_value_with_user_errors(
//         POOL_LIST,
//         Error::MissingPoolList,
//         Error::InvalidPoolList,
//     );
//     if casper_serde_json_wasm::from_str::<PoolList>(&current_list_str).is_err() {
//         runtime::revert(Error::CanNotGetPoolList)
//     }

//     // let current_list = casper_serde_json_wasm::from_str::<PoolList>(&current_list_str);

//     // let mut list: Vec<PoolInfo> = current_list.unwrap().all_pool;

//     // let pool_info = &mut list[pool_id_usize];

//     // let pool_info = get_pool_info(pool_id_usize);

//     // Check if stake_duration is
//     if stake_duration < pool_info.min_stake_duration {
//         runtime::revert(Error::InvalidStakeDuration);
//     }
//     // Todo: write to USER_INFO (userInfo)

//     let this_key: Key = get_self_key();

//     let mut user_info_of_this_pool = get_user_info(pool_id, caller);
//     if user_info_of_this_pool.total_stake_amount > U256::from("0") {
//         // pay pending rewards
//         let pending_rewards: U256 = (user_info_of_this_pool.total_stake_amount.clone()
//             * pool_info.acc_reward_per_share.clone()
//             / u256_10_18)
//             - user_info_of_this_pool.reward_debt.clone();
//         let total_pending: U256 = user_info_of_this_pool.pending_rewards.clone() + pending_rewards;

//         pay_rewards(
//             this_key,
//             caller,
//             pending_rewards,
//             user_info_of_this_pool.clone(),
//         );
//     }
//     // Call transfer_from function to transfer deposit to this contract

//     let lp_token_hash = pool_info.lp_token;
//     // transfer lp_token to the pool
//     transfer_from_erc20_token(lp_token_hash, caller, this_key, U256::from(amount));

//     // Recalculate  user_info and pool_info
//     user_info_of_this_pool.total_stake_amount =
//         user_info_of_this_pool.total_stake_amount.clone() + amount;
//     pool_info.lp_supply = pool_info.lp_supply.clone() + amount;

//     // Todo: rewards debt

//     user_info_of_this_pool.reward_debt = user_info_of_this_pool.total_stake_amount
//         * pool_info.acc_reward_per_share
//         / u256_10_18;

//     // save
//     write_dictionary_user_info(pool_id, caller, user_info_of_this_pool);
//     set_key(
//         POOL_LIST,
//         casper_serde_json_wasm::to_string_pretty(&list).unwrap(),
//     );

//     // Emit event
//     events::emit(&StakingEvent::UserStake {
//         user: caller.clone(),
//         lp_token: lp_token_hash.clone(),
//         amount: amount.clone(),
//         stake_duration: stake_duration.clone(),
//     });

//     Ok(())
// }

// #[no_mangle]
// pub extern "C" fn un_stake2() -> Result<(), Error> {
//     // This is equal to pool_id
//     let pool_id: u64 = helpers::get_named_arg_with_user_errors(
//         ARG_POOL_ID,
//         Error::MissingPoolId,
//         Error::InvalidPoolId,
//     )
//     .unwrap_or_revert();
//     let amount: U256 = helpers::get_named_arg_with_user_errors(
//         ARG_AMOUNT,
//         Error::MissingAmount,
//         Error::InvalidAmount,
//     )
//     .unwrap_or_revert();
//     if amount <= U256::from("0") {
//         runtime::revert(Error::InvalidAmount);
//     }
//     let pool_id_usize: usize = pool_id as usize;
//     // update pool with pool_id
//     update_pool(pool_id_usize);

//     let caller = get_immediate_caller_key();

//     // get pool_info
//     let current_list_str: String = helpers::get_stored_value_with_user_errors(
//         POOL_LIST,
//         Error::MissingPoolList,
//         Error::InvalidPoolList,
//     );

//     let current_list = casper_serde_json_wasm::from_str::<PoolList>(&current_list_str).unwrap();

//     let mut list: Vec<PoolInfo> = current_list.all_pool;

//     // this is pool_info
//     let pool_info = &mut list[pool_id_usize];

//     let this_key: Key = get_self_key();

//     // this is user_info
//     let mut user_info_of_this_pool = get_user_info(pool_id, caller);

//     if user_info_of_this_pool.total_stake_amount < amount {
//         runtime::revert(Error::InvalidAmount);
//     }

//     let pending: U256 = user_info_of_this_pool.total_stake_amount * pool_info.acc_reward_per_share
//         / u256_10_18
//         - user_info_of_this_pool.reward_debt;
//     let total_pending: U256 = user_info_of_this_pool.pending_rewards + pending;
//     user_info_of_this_pool.pending_rewards = U256::from("0");
//     user_info_of_this_pool = pay_rewards(this_key, caller, total_pending, user_info_of_this_pool);
//     user_info_of_this_pool.total_stake_amount =
//         user_info_of_this_pool.total_stake_amount.clone() - amount.clone();
//     pool_info.lp_supply = pool_info.lp_supply.clone() - amount.clone();
//     transfer_erc20_token(pool_info.lp_token.clone(), caller, amount);
//     user_info_of_this_pool.reward_debt = user_info_of_this_pool.total_stake_amount
//         * pool_info.acc_reward_per_share
//         / u256_10_18;

//     // save
//     write_dictionary_user_info(pool_id, caller, user_info_of_this_pool);
//     set_key(
//         POOL_LIST,
//         casper_serde_json_wasm::to_string_pretty(&list).unwrap(),
//     );

//     Ok(())
// }

#[no_mangle]
pub extern "C" fn un_stake() -> Result<(), Error> {
    // This is equal to pool_id
    let pool_id: u64 = helpers::get_named_arg_with_user_errors(
        ARG_POOL_ID,
        Error::MissingPoolId,
        Error::InvalidPoolId,
    )
    .unwrap_or_revert();
    let amount: U256 = helpers::get_named_arg_with_user_errors(
        ARG_AMOUNT,
        Error::MissingAmount,
        Error::InvalidAmount,
    )
    .unwrap_or_revert();
    if amount <= U256::from("0") {
        runtime::revert(Error::InvalidAmount);
    }

    let rewards_token: Key = helpers::get_stored_value_with_user_errors::<Key>(
        REWARD_TOKEN,
        Error::MissingRewardToken,
        Error::InvalidRewardToken,
    );

    // let pool_id_usize: usize = pool_id.clone() as usize;
    // update pool with pool_id
    update_pool(rewards_token, pool_id.clone());

    let caller = get_immediate_caller_key();

    // get pool_info

    let pool_info_str: String =
        helpers::get_dictionary_value_from_key(POOL_INFO, &pool_id.clone().to_string()).unwrap();
    if casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).is_err() {
        runtime::revert(Error::CanNotGetPoolList)
    }

    let mut pool_info = casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).unwrap();

    let this_key: Key = get_self_key();

    // this is user_info
    let mut user_info_of_this_pool = get_user_info(pool_id, caller);

    if user_info_of_this_pool.total_stake_amount < amount {
        runtime::revert(Error::InvalidAmount);
    }

    let pending: U256 = user_info_of_this_pool.total_stake_amount * pool_info.acc_reward_per_share
        / U256::from(u256_10_18)
        - user_info_of_this_pool.reward_debt;
    let total_pending: U256 = user_info_of_this_pool.pending_rewards + pending;
    user_info_of_this_pool.pending_rewards = U256::from("0");
    user_info_of_this_pool = pay_rewards(
        rewards_token,
        this_key,
        caller,
        total_pending,
        user_info_of_this_pool,
    );
    user_info_of_this_pool.total_stake_amount =
        user_info_of_this_pool.total_stake_amount.clone() - amount.clone();
    pool_info.lp_supply = pool_info.lp_supply.clone() - amount.clone();
    transfer_erc20_token(pool_info.lp_token.clone(), caller, amount);
    user_info_of_this_pool.reward_debt = user_info_of_this_pool.total_stake_amount
        * pool_info.acc_reward_per_share
        / U256::from(u256_10_18);

    // save
    write_dictionary_user_info(pool_id, caller, user_info_of_this_pool);
    write_dictionary_value_from_key(
        POOL_INFO,
        &pool_id.clone().to_string(),
        casper_serde_json_wasm::to_string_pretty(&pool_info).unwrap(),
    );

    // Emit event
    events::emit(&StakingEvent::UnStake {
        user: caller.clone(),
        pool_id: pool_id.clone(),
        amount: amount.clone(),
    });

    Ok(())
}

fn pay_rewards(
    rewards_token: Key,
    this_contract: Key,
    user: Key,
    rewards_amount: U256,
    mut user_info_of_this_pool: UserInfo,
) -> UserInfo {
    // transfer rewards
    // let rewards_token: Key = helpers::get_stored_value_with_user_errors::<Key>(
    //     REWARD_TOKEN,
    //     Error::MissingRewardToken,
    //     Error::InvalidRewardToken,
    // );

    let reward_token_balance_of_contract: U256 =
        get_balance_erc20_token(rewards_token, this_contract);
    if reward_token_balance_of_contract < rewards_amount {
        transfer_erc20_token(rewards_token, user, reward_token_balance_of_contract);
        user_info_of_this_pool.pending_rewards =
            user_info_of_this_pool.pending_rewards - reward_token_balance_of_contract;
    } else {
        transfer_erc20_token(rewards_token, user, rewards_amount);
        user_info_of_this_pool.pending_rewards = U256::from("0");
    }
    user_info_of_this_pool
}

fn get_multiplier(from: U256, to: U256) -> U256 {
    let multiplier = to - from;
    multiplier
}
fn update_pool(rewards_token: Key, pool_id: u64) {
    // get Pool_Info of this pool

    let pool_info_str: String =
        helpers::get_dictionary_value_from_key(POOL_INFO, &pool_id.to_string()).unwrap();
    // if casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).is_err() {
    //     runtime::revert(Error::CanNotGetPoolList)
    // }

    let mut this_pool = casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).unwrap();

    // get current block stamps
    let current_block_timestamps: U256 = U256::from(current_block_timestamp());

    if current_block_timestamps <= this_pool.last_reward_second {
        runtime::revert(Error::InvalidContext);
    }

    // Todo: update pool
    let multiplier: U256 = get_multiplier(this_pool.last_reward_second, current_block_timestamps);
    let total_alloc_point: U256 = helpers::get_stored_value_with_user_errors::<U256>(
        TOTAL_ALLOC_POINT,
        Error::MissingTotalAllocPoint,
        Error::InvalidTotalAllocPoint,
    );
    let reward_per_second: U256 = helpers::get_stored_value_with_user_errors::<U256>(
        REWARD_PER_SECOND,
        Error::MissingRewardPerSecond,
        Error::InvalidRewardPerSecond,
    );

    if total_alloc_point > U256::from("0") && this_pool.lp_supply != U256::from("0") {
        let reward: U256 =
            multiplier * reward_per_second * this_pool.alloc_point / total_alloc_point;
        // Todo: transfer reward to this contract

        let contract_key: Key = get_self_key();

        // mint reward

        // mint_reward_to_contract(contract_key, reward);

        // let rewards_token: Key = helpers::get_stored_value_with_user_errors::<Key>(
        //     REWARD_TOKEN,
        //     Error::MissingRewardToken,
        //     Error::InvalidRewardToken,
        // );
        let contract_hash_addr: HashAddr = rewards_token.into_hash().unwrap_or_revert();
        let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);
        let _: () = runtime::call_contract(
            contract_hash,
            MINT_ENTRY_POINT_NAME,
            runtime_args! {
                "owner" => contract_key,
                "amount" => reward,
            },
        );
        //update
        let new_acc_reward_per_share = this_pool.acc_reward_per_share.clone()
            + (reward * U256::from(u256_10_18) / this_pool.lp_supply);

        let new_this_pool = PoolInfo {
             pool_id: pool_id.clone(),
             lp_token: this_pool.lp_token,
             alloc_point: this_pool.alloc_point,
             last_reward_second: current_block_timestamps.clone(),
             acc_reward_per_share: new_acc_reward_per_share,
             min_stake_duration: this_pool.min_stake_duration,
             early_withdraw_penalty_rate: this_pool.early_withdraw_penalty_rate,
             lp_supply: this_pool.lp_supply,
        };

        // save pool_info

        write_dictionary_value_from_key(
            POOL_INFO,
            &pool_id.to_string(),
            // this_pool,
            casper_serde_json_wasm::to_string_pretty(&new_this_pool).unwrap(),
        );
    }
}
// View function to see pending rewards of user
#[no_mangle]
pub extern "C" fn get_pending_rewards2() -> U256 {
    // This is equal to pool_id
    let pool_id: u64 = helpers::get_named_arg_with_user_errors(
        ARG_POOL_ID,
        Error::MissingLpContractHash,
        Error::InvalidLpContractHash,
    )
    .unwrap_or_revert();
    let user: Key =
        helpers::get_named_arg_with_user_errors(ARG_USER, Error::MissingUser, Error::InvalidUser)
            .unwrap_or_revert();

    // let lp_token_dictionary_key: String = make_dictionary_item_key_for_key(lp_token);
    // let pool_id: String =
    //     helpers::get_dictionary_value_from_key::<String>(POOL_INFO, &lp_token_dictionary_key)
    //         .unwrap_or_revert();

    let total_alloc_point: U256 = helpers::get_stored_value_with_user_errors(
        TOTAL_ALLOC_POINT,
        Error::MissingTotalAllocPoint,
        Error::InvalidTotalAllocPoint,
    );

    let reward_per_second: U256 = helpers::get_stored_value_with_user_errors(
        REWARD_PER_SECOND,
        Error::MissingRewardPerSecond,
        Error::InvalidRewardPerSecond,
    );

    let current_list_str: String = helpers::get_stored_value_with_user_errors(
        POOL_LIST,
        Error::MissingPoolList,
        Error::InvalidPoolList,
    );

    let current_list = casper_serde_json_wasm::from_str::<PoolList>(&current_list_str).unwrap();

    let list: Vec<PoolInfo> = current_list.all_pool;
    let pool_id_usize: usize = pool_id.clone() as usize;

    let pool_info = &list[pool_id_usize];

    let user_info_of_this_pool = get_user_info(pool_id, user);
    let mut acc_reward_per_share: U256 = pool_info.acc_reward_per_share;
    let current_block_timestamp = U256::from(current_block_timestamp());
    if current_block_timestamp > pool_info.last_reward_second
        && pool_info.lp_supply != U256::from("0")
        && total_alloc_point > U256::from("0")
    {
        let multiplier: U256 =
            get_multiplier(pool_info.last_reward_second, current_block_timestamp);
        let rewards: U256 =
            (multiplier * reward_per_second * pool_info.alloc_point) / total_alloc_point;
        acc_reward_per_share =
            acc_reward_per_share.clone() + (rewards * U256::from(u256_10_18)) / pool_info.lp_supply;
    }
    let return_value = ((user_info_of_this_pool.total_stake_amount * acc_reward_per_share)
        / U256::from(u256_10_18))
        - user_info_of_this_pool.reward_debt
        + user_info_of_this_pool.pending_rewards;

    return_value
}

// View function to see pending rewards of user
#[no_mangle]
pub extern "C" fn get_pending_rewards() -> U256 {
    // This is equal to pool_id
    let pool_id: u64 = helpers::get_named_arg_with_user_errors(
        ARG_POOL_ID,
        Error::MissingLpContractHash,
        Error::InvalidLpContractHash,
    )
    .unwrap_or_revert();
    let user: Key =
        helpers::get_named_arg_with_user_errors(ARG_USER, Error::MissingUser, Error::InvalidUser)
            .unwrap_or_revert();

    // let lp_token_dictionary_key: String = make_dictionary_item_key_for_key(lp_token);
    // let pool_id: String =
    //     helpers::get_dictionary_value_from_key::<String>(POOL_INFO, &lp_token_dictionary_key)
    //         .unwrap_or_revert();

    let total_alloc_point: U256 = helpers::get_stored_value_with_user_errors(
        TOTAL_ALLOC_POINT,
        Error::MissingTotalAllocPoint,
        Error::InvalidTotalAllocPoint,
    );

    let reward_per_second: U256 = helpers::get_stored_value_with_user_errors(
        REWARD_PER_SECOND,
        Error::MissingRewardPerSecond,
        Error::InvalidRewardPerSecond,
    );

    let pool_info_str: String =
        helpers::get_dictionary_value_from_key(POOL_INFO, &pool_id.clone().to_string()).unwrap();
    // if casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).is_err() {
    //     runtime::revert(Error::CanNotGetPoolList)
    // }

    let pool_info = casper_serde_json_wasm::from_str::<PoolInfo>(&pool_info_str).unwrap();

    let user_info_of_this_pool = get_user_info(pool_id, user);
    let mut acc_reward_per_share: U256 = pool_info.acc_reward_per_share;
    let current_block_timestamp = U256::from(current_block_timestamp());
    if current_block_timestamp > pool_info.last_reward_second
        && pool_info.lp_supply != U256::from("0")
        && total_alloc_point > U256::from("0")
    {
        let multiplier: U256 =
            get_multiplier(pool_info.last_reward_second, current_block_timestamp);
        let rewards: U256 =
            (multiplier * reward_per_second * pool_info.alloc_point) / total_alloc_point;
        acc_reward_per_share =
            acc_reward_per_share.clone() + (rewards * U256::from(u256_10_18)) / pool_info.lp_supply;
    }
    let return_value = ((user_info_of_this_pool.total_stake_amount * acc_reward_per_share)
        / U256::from(u256_10_18))
        - user_info_of_this_pool.reward_debt
        + user_info_of_this_pool.pending_rewards;

    return_value
}

#[no_mangle]
pub extern "C" fn transfer_owner() -> Result<(), Error> {
    let new_contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);
    // let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();
    let current_contract_owner = helpers::get_stored_value_with_user_errors(
        CONTRACT_OWNER_KEY_NAME,
        Error::MissingContractOwner,
        Error::InvalidContractOwner,
    );

    let caller = get_immediate_caller_key();
    // let caller = helpers::get_verified_caller().unwrap_or_revert();

    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }
    set_key(CONTRACT_OWNER_KEY_NAME, new_contract_owner);
    Ok(())
}

#[no_mangle]
pub extern "C" fn set_reward_token() -> Result<(), Error> {
    let reward_token: Key = runtime::get_named_arg(ARG_REWARD_TOKEN);
    // let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();
    let current_contract_owner = helpers::get_stored_value_with_user_errors(
        CONTRACT_OWNER_KEY_NAME,
        Error::MissingContractOwner,
        Error::InvalidContractOwner,
    );
    let caller = get_immediate_caller_key();

    // let caller = helpers::get_verified_caller().unwrap_or_revert();
    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }
    set_key(REWARD_TOKEN, reward_token);
    Ok(())
}

#[no_mangle]
pub extern "C" fn set_reward_per_second() -> Result<(), Error> {
    // let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();

    let current_contract_owner = helpers::get_stored_value_with_user_errors(
        CONTRACT_OWNER_KEY_NAME,
        Error::MissingContractOwner,
        Error::InvalidContractOwner,
    );

    let caller = get_immediate_caller_key();
    // let caller = helpers::get_verified_caller().unwrap_or_revert();
    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }
    let new_reward_per_second: U256 = runtime::get_named_arg(ARG_REWARD_PER_SECOND);
    set_key(REWARD_PER_SECOND, new_reward_per_second);
    Ok(())
}

fn transfer_from_erc20_token(
    token: Key,
    source: Key,
    target: Key,
    amount: U256,
) -> Result<(), Error> {
    let contract_hash_addr: HashAddr = token.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    let _: () = runtime::call_contract(
        contract_hash,
        TRANSFER_FROM_ENTRY_POINT_NAME,
        runtime_args! {
            "owner" => source,
            "recipient" => target,
            "amount" => amount,
        },
    );
    Ok(())
}

fn transfer_erc20_token(token: Key, target: Key, amount: U256) -> Result<(), Error> {
    let contract_hash_addr: HashAddr = token.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    let _: () = runtime::call_contract(
        contract_hash,
        TRANSFER_ENTRY_POINT_NAME,
        runtime_args! {
            "recipient" => target,
            "amount" => amount,
        },
    );
    Ok(())
}

fn get_balance_erc20_token(token: Key, owner: Key) -> U256 {
    let contract_hash_addr: HashAddr = token.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    runtime::call_contract::<U256>(
        contract_hash,
        BALANCE_OF_ENTRY_POINT_NAME,
        runtime_args! {
            "address" => owner,
        },
    )
}
fn mint_reward_to_contract(target: Key, amount: U256) -> Result<(), Error> {
    let rewards_token: Key = helpers::get_stored_value_with_user_errors::<Key>(
        REWARD_TOKEN,
        Error::MissingRewardToken,
        Error::InvalidRewardToken,
    );

    let contract_hash_addr: HashAddr = rewards_token.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    let _: () = runtime::call_contract(
        contract_hash,
        MINT_ENTRY_POINT_NAME,
        runtime_args! {
            "owner" => target,
            "amount" => amount,
        },
    );
    Ok(())
}
