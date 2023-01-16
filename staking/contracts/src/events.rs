#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// use std::collections::BTreeMap;

extern crate alloc;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::{self, *},
};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{account::AccountHash, ContractPackageHash, HashAddr, Key, URef, U256};

use crate::helpers::*; 

pub enum StakingEvent {
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Offer {
        token_market_key: String,
        offeror: Key,
        minimum_offer: U256,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
    UnStake {
        user: Key,
        pool_id : u64,
        amount: U256,
    },
    Bid {
        token_market_key: String,
        offeror: Key,
        bidder: Key,
        value: U256,
    },

    UserStake {
        user: Key,
        lp_token: Key,
        amount: U256,
        // stake_duration: U256,
        // token_id: u8,
    },
    Withdrawal {
        cspr_recipient: AccountHash,
        from: Key,
        value: U256,
    },
}

impl StakingEvent {
    pub fn type_name(&self) -> String {
        match self {
            StakingEvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            StakingEvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
            StakingEvent::Offer {
                token_market_key: _,
                offeror: _,
                minimum_offer: _,
            } => "offer",

            StakingEvent::UnStake {
                user: _,
                pool_id: _,
                amount: _,
            } => "un_stake",

            StakingEvent::Bid {
                token_market_key: _,
                offeror: _,
                bidder: _,
                value: _,
            } => "bid",

            StakingEvent::UserStake {
                user: _,
                lp_token: _,
                amount: _,
                // stake_duration: _,
                // token_id: _,
            } => "user_stake",
            StakingEvent::Withdrawal {
                cspr_recipient: _,
                from: _,
                value: _,
            } => "withdrawal",
        }
        .to_string()
    }
}

pub fn contract_package_hash() -> ContractPackageHash {
    get_key::<ContractPackageHash>("contract_package_hash").unwrap()
}

// pub(crate) fn contract_package_hash() -> ContractPackageHash {
//     let key : Key = runtime::get_key("contract_package_hash").unwrap();
//     let contract_package_hash_addr: HashAddr = key.into_hash().unwrap();
//     let factory_package_hash: ContractPackageHash = ContractPackageHash::new(contract_package_hash_addr);
//     factory_package_hash

// }

pub(crate) fn emit(pair_event: &StakingEvent) {
    let mut events = Vec::new();
    // let package : ContractPackageHash = runtime::get_key("contract_package_hash").into_hash().unwrap_or_revert();
    let package = contract_package_hash();
    match pair_event {
        StakingEvent::Approval {
            owner,
            spender,
            value,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("owner", owner.to_string());
            event.insert("spender", spender.to_string());
            event.insert("value", value.to_string());
            events.push(event);
        }
        StakingEvent::Transfer { from, to, value } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("value", value.to_string());
            events.push(event);
        }
        StakingEvent::Offer { token_market_key, offeror, minimum_offer } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("token_market_key", token_market_key.to_string());
            event.insert("offeror", offeror.to_string());
            event.insert("minimum_offer", minimum_offer.to_string());
            events.push(event);
        }

        StakingEvent::UnStake { user, pool_id, amount } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("user", user.to_string());
            event.insert("pool_id", pool_id.to_string());
            event.insert("amount", amount.to_string());
            events.push(event);
        }
        StakingEvent::Bid { token_market_key, offeror, bidder, value } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("token_market_key", token_market_key.to_string());
            event.insert("offeror", offeror.to_string());
            event.insert("bidder", bidder.to_string());
            event.insert("value", value.to_string());
            events.push(event);
        }
        StakingEvent::UserStake {
            user,
            lp_token,
            amount,
            // stake_duration,
            // token_id,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("user", user.to_string());
            event.insert("lp_token", lp_token.to_string());
            event.insert("amount", amount.to_string());
            // event.insert("stake_duration", stake_duration.to_string());
            events.push(event);
        }
        StakingEvent::Withdrawal {
            cspr_recipient,
            from,
            value,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("cspr_recipient", cspr_recipient.to_string());
            event.insert("from", from.to_string());
            event.insert("value", value.to_string());
            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}
