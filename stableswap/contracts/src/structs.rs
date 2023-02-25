use alloc::{
    vec::*,
};

use casper_types::{
    Key
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Swap {
    pub initial_a: u128,
    pub future_a: u128,
    pub initial_a_time: u64,
    pub future_a_time: u64,
    pub swap_fee: u64,
    pub admin_fee: u64,
    pub lp_token: Key,
    pub pooled_tokens: Vec<Key>,
    pub token_precision_multipliers: Vec<u128>,
    pub balances: Vec<u128>
}

pub(crate) struct CalculateWithdrawOneTokenDYInfo {
    pub d0: u128,
    pub d1: u128,
    pub new_y: u128,
    pub fee_per_token: u128,
    pub precise_a: u128
}

pub(crate) struct ManageLiquidityInfo {
    pub d0: u128,
    pub d1: u128,
    pub d2: u128,
    pub precise_a: u128,
    pub lp_token: Key,
    pub total_supply: u128,
    pub balances: Vec<u128>,
    pub multipliers: Vec<u128>
}