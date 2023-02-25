#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// use std::collections::BTreeMap;

extern crate alloc;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::{*},
};

use casper_contract::contract_api::{storage};
use casper_types::{ContractPackageHash, Key, URef};
use serde::{Deserialize, Serialize};

use crate::helpers::*;

pub enum CAFIDexEvent {
    RampA {
        old_a: u128,
        new_a: u128,
        initial_time: u64,
        future_time: u64,
    },
    StopRampA {
        current_a: u128,
        time: u64,
    },
    TokenSwap {
        buyer: Key,
        tokens_sold: u128,
        tokens_bought: u128,
        sold_id: u128,
        bought_id: u128,
    },
    AddLiquidity {
        provider: Key,
        token_amounts: Vec<u128>,
        fees: Vec<u128>,
        invariant: u128,
        lp_token_supply: u128,
    },
    RemoveLiquidity {
        provider: Key,
        token_amounts: Vec<u128>,
        lp_token_supply: u128,
    },
    RemoveLiquidityOne {
        provider: Key,
        lp_token_amount: u128,
        lp_token_supply: u128,
        bought_id: u128,
        tokens_bought: u128,
    },
    RemoveLiquidityImbalance {
        provider: Key,
        token_amounts: Vec<u128>,
        fees: Vec<u128>,
        invariant: u128,
        lp_token_supply: u128,
    },
}

impl CAFIDexEvent {
    pub fn type_name(&self) -> String {
        match self {
            CAFIDexEvent::RampA {
                old_a: _,
                new_a: _,
                initial_time: _,
                future_time: _,
            } => "rampa",
            CAFIDexEvent::StopRampA {
                current_a: _,
                time: _,
            } => "stop_ramp_a",
            CAFIDexEvent::TokenSwap {
                buyer: _,
                tokens_sold: _,
                tokens_bought: _,
                sold_id: _,
                bought_id: _,
            } => "token_swap",
            CAFIDexEvent::AddLiquidity {
                provider: _,
                token_amounts: _,
                fees: _,
                invariant: _,
                lp_token_supply: _,
            } => "add_liquidity",
            CAFIDexEvent::RemoveLiquidity {
                provider: _,
                token_amounts: _,
                lp_token_supply: _,
            } => "add_liquidity",
            CAFIDexEvent::RemoveLiquidityOne {
                provider: _,
                lp_token_amount: _,
                lp_token_supply: _,
                bought_id: _,
                tokens_bought: _,
            } => "remove_liquidity_one",
            CAFIDexEvent::RemoveLiquidityImbalance {
                provider: _,
                token_amounts: _,
                fees: _,
                invariant: _,
                lp_token_supply: _,
            } => "remove_liquidity_one",
        }
        .to_string()
    }
}

pub fn contract_package_hash() -> ContractPackageHash {
    get_key::<ContractPackageHash>("contract_package_hash").unwrap()
}

pub(crate) fn emit(pair_event: &CAFIDexEvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match pair_event {
        CAFIDexEvent::RampA {
            old_a,
            new_a,
            initial_time,
            future_time,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("old_a", old_a.to_string());
            event.insert("new_a", new_a.to_string());
            event.insert("initial_time", initial_time.to_string());
            event.insert("future_time", future_time.to_string());
            events.push(event);
        }
        CAFIDexEvent::StopRampA { current_a, time } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("current_a", current_a.to_string());
            event.insert("time", time.to_string());
            events.push(event);
        }
        CAFIDexEvent::TokenSwap {
            buyer,
            tokens_sold,
            tokens_bought,
            sold_id,
            bought_id,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("buyer", buyer.to_string());
            event.insert("tokens_sold", tokens_sold.to_string());
            event.insert("tokens_bought", tokens_bought.to_string());
            event.insert("sold_id", sold_id.to_string());
            event.insert("bought_id", bought_id.to_string());
            events.push(event);
        }
        CAFIDexEvent::AddLiquidity {
            provider,
            token_amounts,
            fees,
            invariant,
            lp_token_supply,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("provider", provider.to_string());
            event.insert("token_amounts", casper_serde_json_wasm::to_string_pretty(token_amounts).unwrap());
            event.insert("fees", casper_serde_json_wasm::to_string_pretty(fees).unwrap());
            event.insert("invariant", invariant.to_string());
            event.insert("lp_token_supply", lp_token_supply.to_string());
            events.push(event);
        }
        CAFIDexEvent::RemoveLiquidity {
            provider,
            token_amounts,
            lp_token_supply,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("provider", provider.to_string());
            event.insert("token_amounts", casper_serde_json_wasm::to_string_pretty(token_amounts).unwrap());
            event.insert("lp_token_supply", lp_token_supply.to_string());
            events.push(event);
        }
        CAFIDexEvent::RemoveLiquidityOne {
            provider,
            lp_token_amount,
            lp_token_supply,
            bought_id,
            tokens_bought,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("provider", provider.to_string());
            event.insert("lp_token_amount", lp_token_amount.to_string());
            event.insert("lp_token_supply", lp_token_supply.to_string());
            event.insert("bought_id", bought_id.to_string());
            event.insert("tokens_bought", tokens_bought.to_string());
            events.push(event);
        }
        CAFIDexEvent::RemoveLiquidityImbalance {
            provider,
            token_amounts,
            fees,
            invariant,
            lp_token_supply
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("provider", provider.to_string());
            event.insert("token_amounts", casper_serde_json_wasm::to_string_pretty(token_amounts).unwrap());
            event.insert("fees", casper_serde_json_wasm::to_string_pretty(fees).unwrap());
            event.insert("invariant", invariant.to_string());
            event.insert("lp_token_supply", lp_token_supply.to_string());
            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}
