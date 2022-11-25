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

pub enum MarketPlaceEvent {
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
    DoTrade {
        from: Key,
        to: Key,
        value: U256,
    },
    Bid {
        token_market_key: String,
        offeror: Key,
        bidder: Key,
        value: U256,
    },

    MintFactory {
        src_purse: URef,
        owner: Key,
        // token_id: u8,
    },
    Withdrawal {
        cspr_recipient: AccountHash,
        from: Key,
        value: U256,
    },
}

impl MarketPlaceEvent {
    pub fn type_name(&self) -> String {
        match self {
            MarketPlaceEvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            MarketPlaceEvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
            MarketPlaceEvent::Offer {
                token_market_key: _,
                offeror: _,
                minimum_offer: _,
            } => "offer",

            MarketPlaceEvent::DoTrade {
                from: _,
                to: _,
                value: _,
            } => "dotrade",

            MarketPlaceEvent::Bid {
                token_market_key: _,
                offeror: _,
                bidder: _,
                value: _,
            } => "bid",

            MarketPlaceEvent::MintFactory {
                src_purse: _,
                owner: _,
                // token_id: _,
            } => "mint_factory",
            MarketPlaceEvent::Withdrawal {
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

pub(crate) fn emit(pair_event: &MarketPlaceEvent) {
    let mut events = Vec::new();
    // let package : ContractPackageHash = runtime::get_key("contract_package_hash").into_hash().unwrap_or_revert();
    let package = contract_package_hash();
    match pair_event {
        MarketPlaceEvent::Approval {
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
        MarketPlaceEvent::Transfer { from, to, value } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("value", value.to_string());
            events.push(event);
        }
        MarketPlaceEvent::Offer { token_market_key, offeror, minimum_offer } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("token_market_key", token_market_key.to_string());
            event.insert("offeror", offeror.to_string());
            event.insert("minimum_offer", minimum_offer.to_string());
            events.push(event);
        }

        MarketPlaceEvent::DoTrade { from, to, value } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("value", value.to_string());
            events.push(event);
        }
        MarketPlaceEvent::Bid { token_market_key, offeror, bidder, value } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("token_market_key", token_market_key.to_string());
            event.insert("offeror", offeror.to_string());
            event.insert("bidder", bidder.to_string());
            event.insert("value", value.to_string());
            events.push(event);
        }
        MarketPlaceEvent::MintFactory {
            src_purse,
            owner,
            // token_id,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("src_purse", src_purse.to_string());
            event.insert("owner", owner.to_string());
            // event.insert("value", token.to_string());
            events.push(event);
        }
        MarketPlaceEvent::Withdrawal {
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
