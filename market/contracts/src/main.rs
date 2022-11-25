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
use address::Address;

use alloc::{
    string::{String, ToString},
    vec::*,
};
use casper_contract::{
    contract_api::{
        runtime, storage,
        system::{self, transfer_from_purse_to_account, transfer_from_purse_to_purse},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, ContractHash, ContractPackageHash, HashAddr, Key,
    RuntimeArgs, URef, U256,
};
use events::MarketPlaceEvent;
use helpers::{get_immediate_caller_key, get_self_key, get_token_market_key};
const FEE_DIVISOR: u64 = 10000;
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct TokenMarket {
    offeror: Option<Key>, //token seller
    minimum_offer: U256,  // min price in WCSPR
    bidder: Option<Key>,
    locked_bid: U256,
    is_active: Option<bool>,
}

#[no_mangle]
pub extern "C" fn init() {
    if get_key::<Key>(CONTRACT_HASH_KEY_NAME).is_some() {
        runtime::revert(Error::ContractAlreadyInitialized);
    }
    let contract_hash: Key = runtime::get_named_arg(ARG_CONTRACT_HASH);

    let contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);

    let contract_fee_receiver: Key = runtime::get_named_arg(ARG_MARKET_FEE_RECEIVER);

    let market_fee: U256 = runtime::get_named_arg(ARG_MARKET_FEE);

    let royalty_fee: U256 = runtime::get_named_arg(ARG_ROYALTY_FEE);

    let is_royalty: bool = runtime::get_named_arg(ARG_IS_ROYALTY);

    let wcspr_contract: Key = runtime::get_named_arg(ARG_WCSPR_CONTRACT);

    runtime::put_key(
        CONTRACT_HASH_KEY_NAME,
        storage::new_uref(contract_hash).into(),
    );

    runtime::put_key(
        CONTRACT_OWNER_KEY_NAME,
        storage::new_uref(contract_owner).into(),
    );

    runtime::put_key(
        MARKET_FEE_RECEIVER,
        storage::new_uref(contract_fee_receiver).into(),
    );

    runtime::put_key(WCSPR_CONTRACT, storage::new_uref(wcspr_contract).into());

    runtime::put_key(MARKET_FEE, storage::new_uref(market_fee as U256).into());
    runtime::put_key(ROYALTY_FEE, storage::new_uref(royalty_fee as U256).into());
    runtime::put_key(IS_ROYALTY, storage::new_uref(is_royalty as bool).into());
    storage::new_dictionary(TOKEN_CONTRACT_MAP)
        .unwrap_or_revert_with(Error::FailedToCreateDictionary);
    storage::new_dictionary(TOKEN_MARKET).unwrap_or_revert_with(Error::FailedToCreateDictionary);
}

#[no_mangle]
fn call() {
    let contract_name: String = runtime::get_named_arg(MARKET_CONTRACT_NAME);
    let contract_hash_key_name = String::from(contract_name.clone());
    let contract_package_hash_key_name = String::from(contract_name.clone() + "_package_name");
    let contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);
    let market_fee_receiver: Key = runtime::get_named_arg(ARG_MARKET_FEE_RECEIVER);
    let wcspr_contract_hash: Key = runtime::get_named_arg(ARG_WCSPR_CONTRACT);
    let market_fee: U256 = runtime::get_named_arg(ARG_MARKET_FEE);

    let royalty_fee: U256 = runtime::get_named_arg(ARG_ROYALTY_FEE);
    let is_royalty: bool = runtime::get_named_arg(ARG_IS_ROYALTY);

    let (contract_package_hash, access_uref) = storage::create_contract_package_at_hash();

    let named_keys: NamedKeys = named_keys::default(
        contract_owner,
        market_fee_receiver,
        market_fee,
        contract_package_hash,
        royalty_fee,
        None,
    );

    let (contract_hash, _version) = storage::add_contract_version(
        contract_package_hash: ContractPackageHash,
        entry_points::default(),
        named_keys: NamedKeys,
    );
    runtime::put_key(contract_hash_key_name.as_str(), Key::from(contract_hash));

    // set_key(PUNK_MARKETPLACE_KEY_NAME, Key::from(contract_hash));

    runtime::call_contract::<()>(
        contract_hash,
        INIT_ENTRY_POINT_NAME,
        runtime_args! {
            ARG_CONTRACT_HASH => Key::from(contract_hash),
            ARG_CONTRACT_OWNER => Key::from(contract_owner),
            ARG_MARKET_FEE_RECEIVER => Key::from(market_fee_receiver),
            ARG_MARKET_FEE => market_fee,
            ARG_WCSPR_CONTRACT => Key::from(wcspr_contract_hash),
            ARG_ROYALTY_FEE => royalty_fee,
            ARG_IS_ROYALTY => is_royalty,
        },
    );
}

fn get_token_market(contract_hash: &Key, token_identifier: &TokenIdentifier) -> TokenMarket {
    let token_market_key = get_token_market_key(contract_hash, token_identifier);
    let token_market_str =
        get_dictionary_value_from_key::<String>(TOKEN_MARKET, &token_market_key).unwrap_or_revert();
    let token_market = casper_serde_json_wasm::from_str::<TokenMarket>(&token_market_str).unwrap();
    token_market
}

#[no_mangle]
pub extern "C" fn offer() {
    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    // Check if nft is supported or not
    check_enabled_nft(contract_hash: Key);
    // Take token_id from runtime
    let identifier_mode = get_identifier_mode_from_runtime_args();
    let token_identifier = get_token_identifier_from_runtime_args(&identifier_mode);

    let minimum_offer: U256 = runtime::get_named_arg(ARG_MINIMUM_OFFER);

    let caller = get_immediate_caller_key();
    let owner_of = get_token_owner(&contract_hash, &identifier_mode, &token_identifier);
    if caller != owner_of {
        runtime::revert(Error::OnlyOwnerCanOffer);
    }

    if minimum_offer == U256::zero() {
        runtime::revert(Error::AskForMore);
    }

    // Check if NFT IS APPROVED or revert
    // let contract_hash_addr: HashAddr = contract_hash.into_hash().unwrap_or_revert();
    // let contract_hash_1: ContractHash = ContractHash::new(contract_hash_addr);
    // let identifier_mode = get_identifier_mode_from_runtime_args();
    // let token_identifier = get_token_identifier_from_runtime_args(&identifier_mode);
    // let mut ret: Option<Vec<Key>>;
    // // let mut ret : Key ;
    // match identifier_mode {
    //     NFTIdentifierMode::Ordinal => {
    //         ret = runtime::call_contract(
    //             contract_hash_1,
    //             GET_APPROVED_ENTRY_POINT_NAME,
    //             runtime_args! {
    //                 ARG_TOKEN_ID => token_identifier.get_index().unwrap()
    //             },
    //         );
    //     }
    //     NFTIdentifierMode::Hash => {
    //         ret = runtime::call_contract(
    //             contract_hash_1,
    //             GET_APPROVED_ENTRY_POINT_NAME,
    //             runtime_args! {
    //                 ARG_TOKEN_HASH => token_identifier.clone().get_hash().unwrap()
    //             },
    //         );
    //     }
    // }

    let self_key = get_self_key();

    // if ret.is_some() {
    //     if !ret.unwrap().contains(&self_key) {
    //         runtime::revert(Error::NftIsNotApproved);
    //     }
    // }
    // if ret !=  self_key {
    //     runtime::revert(Error::NftIsNotApproved);
    // }

    // if !ret.contains(&self_key) {
    //     runtime::revert(Error::NftIsNotApproved);
    // }
    // Do_Trade or Set_offer

    set_offer(
        &contract_hash,
        &identifier_mode,
        &token_identifier,
        Some(caller),
        minimum_offer,
    );
}

#[no_mangle]
pub extern "C" fn bid() {
    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    // Check if nft is supported or not
    check_enabled_nft(contract_hash: Key);
    // Check if NFT IS APPROVED or revert
    // let contract_hash_addr: HashAddr = contract_hash.into_hash().unwrap_or_revert();
    // let contract_hash_1: ContractHash = ContractHash::new(contract_hash_addr);
    let identifier_mode = get_identifier_mode_from_runtime_args();
    let token_identifier = get_token_identifier_from_runtime_args(&identifier_mode);

    let bidding_offer: U256 = runtime::get_named_arg(ARG_BIDDING_OFFER);

    let caller = get_immediate_caller_key();
    let caller_addr: Address =
        get_immediate_caller_address().unwrap_or_revert_with(Error::MissingKey);

    let token_market_key: String = get_token_market_key(&contract_hash, &token_identifier.clone()); // Key for TOKEN_MARKET dictionary
    let token_market_str =
        get_dictionary_value_from_key::<String>(TOKEN_MARKET, &token_market_key).unwrap_or_revert();

    let token_market = casper_serde_json_wasm::from_str::<TokenMarket>(&token_market_str);
    // let mut existing_bidder = None;
    let mut locked_bid = U256::zero();

    if token_market.is_ok() {
        let unwrap = token_market.unwrap();
        // existing_bidder = unwrap.bidder;
        locked_bid = unwrap.locked_bid;
        let minimum: U256 = unwrap.minimum_offer;
        // }

        if bidding_offer < locked_bid {
            runtime::revert(Error::BidTooLow)
        };
        if caller == unwrap.offeror.unwrap() {
            runtime::revert(Error::InvalidAccount)
        }

        //Calculate needed_amount
        let fee_portion: U256 = helpers::get_stored_value_with_user_errors(
            MARKET_FEE,
            Error::MissingFeePortion,
            Error::InvalidFeePortion,
        );
        let needed_amount: U256 =
            unwrap.minimum_offer + unwrap.minimum_offer * fee_portion / U256::from("1000");

        let wcspr_contract_hash: Key = helpers::get_stored_value_with_user_errors(
            WCSPR_CONTRACT,
            Error::MissingWcsprContract,
            Error::InvalidWcsprContract,
        );

        // get contract Key

        let contract_self_key = get_self_key();
        // let contract_self_address = get_self_address().unwrap_or_revert_with(Error::MissingKey);

        if bidding_offer >= unwrap.minimum_offer {
            do_trade(
                token_market_key: String,
                &contract_hash: &Key,
                &token_identifier: &TokenIdentifier,
                &identifier_mode: &NFTIdentifierMode,
                unwrap.offeror.unwrap(): Key,
                caller: Key,
                unwrap.minimum_offer: U256,
                wcspr_contract_hash: Key, // ERC20 token contract
            );
        } else {
            update_new_bidder(
                token_market_key: String,
                bidding_offer: U256,
                unwrap: TokenMarket,
                caller: Key,
                wcspr_contract_hash: Key, // ERC20 token contract
                contract_self_key: Key,
                // caller_addr: Address,
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn revoke_bid() {
    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    check_enabled_nft(contract_hash: Key);
    let identifier_mode = get_identifier_mode_from_runtime_args();
    let token_identifier = get_token_identifier_from_runtime_args(&identifier_mode);
    let token_market_key_to_update = get_token_market_key(&contract_hash, &token_identifier);
    let token_market = get_token_market(&contract_hash, &token_identifier);
    let caller = get_immediate_caller_key();
    if token_market.bidder.unwrap() != caller {
        runtime::revert(Error::OnlyBidderCanRevoke);
    }
    if !token_market.is_active.unwrap() || token_market.is_active.unwrap() == false {
        runtime::revert(Error::BidInactive)
    }

    let wcspr_contract_hash: Key = helpers::get_stored_value_with_user_errors(
        WCSPR_CONTRACT,
        Error::MissingWcsprContract,
        Error::InvalidWcsprContract,
    );

    let contract_self_key = get_self_key();

    let contract_hash_addr: HashAddr = wcspr_contract_hash.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    let _: () = runtime::call_contract(
        contract_hash,
        TRANSFER_ENTRY_POINT_NAME,
        runtime_args! {
            "recipient" => caller,
            "amount" => token_market.locked_bid,
        },
    );

    // When revoke-bid => token_market will be set to its origin
    write_dictionary_value_from_key(
        TOKEN_MARKET,
        &token_market_key_to_update,
        casper_serde_json_wasm::to_string_pretty(&TokenMarket {
            offeror: Some(token_market.offeror.unwrap()),
            minimum_offer: token_market.minimum_offer,
            bidder: None,
            locked_bid: U256::zero(),
            is_active: Some(true),
        })
        .unwrap(),
    );
}
#[no_mangle]
pub extern "C" fn increase_bid() {
    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    check_enabled_nft(contract_hash: Key);
    let new_offer: U256 = runtime::get_named_arg(ARG_NEW_OFFER);
    let identifier_mode = get_identifier_mode_from_runtime_args();
    let token_identifier = get_token_identifier_from_runtime_args(&identifier_mode);
    let token_market_key_to_update = get_token_market_key(&contract_hash, &token_identifier);
    let token_market = get_token_market(&contract_hash, &token_identifier);
    let caller = get_immediate_caller_key();
    if token_market.bidder.unwrap() != caller {
        runtime::revert(Error::OnlyBidderCanIncreaseBid);
    }
    if !token_market.is_active.unwrap() || token_market.is_active.unwrap() == false {
        runtime::revert(Error::BidInactive)
    }
    if new_offer <= token_market.locked_bid {
        runtime::revert(Error::AskForMore)
    }

    // Transfer the increased amount
    let increased_amount: U256 = new_offer - token_market.locked_bid;

    // let new_src_purse: URef = runtime::get_named_arg(ARG_SRC_PURSE);
    // let contract_purse_key = runtime::get_key(CONTRACT_PURSE).unwrap_or_revert();
    // let contract_purse = *contract_purse_key.as_uref().unwrap_or_revert();

    let fee_portion: U256 = helpers::get_stored_value_with_user_errors(
        MARKET_FEE,
        Error::MissingFeePortion,
        Error::InvalidFeePortion,
    );
    let total_increased_amount: U256 =
        increased_amount + increased_amount * fee_portion / U256::from("1000");
    let wcspr_contract: Key = helpers::get_stored_value_with_user_errors(
        WCSPR_CONTRACT,
        Error::MissingWcsprContract,
        Error::InvalidWcsprContract,
    );

    let contract_self_key = get_self_key();
    // let contract_self_address = get_self_address().unwrap_or_revert_with(Error::MissingKey);

    if new_offer >= token_market.minimum_offer {
        do_trade(
            token_market_key_to_update: String,
            &contract_hash: &Key,
            &token_identifier: &TokenIdentifier,
            &identifier_mode: &NFTIdentifierMode,
            token_market.offeror.unwrap(): Key,
            caller: Key,
            token_market.minimum_offer: U256,
            wcspr_contract: Key, // ERC20 token contract
        );
    } else {
        // transfer_from the increased amount from bidder to contract
        let contract_hash_addr: HashAddr = wcspr_contract.into_hash().unwrap_or_revert();
        let wcspr_contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

        let _: () = runtime::call_contract(
            wcspr_contract_hash,
            TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                "owner" => caller,
                "recipient" => contract_self_key,
                "amount" => increased_amount,
            },
        );

        write_dictionary_value_from_key(
            TOKEN_MARKET,
            &token_market_key_to_update,
            casper_serde_json_wasm::to_string_pretty(&TokenMarket {
                offeror: Some(token_market.offeror.unwrap()),
                minimum_offer: token_market.minimum_offer,
                bidder: Some(caller),
                locked_bid: new_offer,
                is_active: Some(true),
            })
            .unwrap(),
        );
        events::emit(&MarketPlaceEvent::Bid {
            token_market_key: token_market_key_to_update,
            offeror: token_market.offeror.unwrap(),
            bidder: caller,
            value: new_offer,
        });
    }
}

#[no_mangle]
pub extern "C" fn revoke_offer() {
    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);

    check_enabled_nft(contract_hash: Key);

    let identifier_mode = get_identifier_mode_from_runtime_args();
    let token_identifier = get_token_identifier_from_runtime_args(&identifier_mode);
    let token_market_key_to_update = get_token_market_key(&contract_hash, &token_identifier);
    let token_market = get_token_market(&contract_hash, &token_identifier);
    let caller = get_immediate_caller_key();
    if token_market.offeror.unwrap() != caller {
        runtime::revert(Error::OnlyOfferorCanRevoke);
    }
    if !token_market.is_active.unwrap() || token_market.is_active.unwrap() == false {
        runtime::revert(Error::OfferInactive)
    }

    let wcspr_contract: Key = helpers::get_stored_value_with_user_errors(
        WCSPR_CONTRACT,
        Error::MissingWcsprContract,
        Error::InvalidWcsprContract,
    );
    let contract_hash_addr: HashAddr = wcspr_contract.into_hash().unwrap_or_revert();
    let wspr_contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    if token_market.locked_bid > U256::zero() {
        let _: () = runtime::call_contract(
            wspr_contract_hash,
            TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                "recipient" => caller,
                "amount" => token_market.locked_bid,
            },
        );
    }

    // When revoke-offer => token_market will be set is_active to false
    write_dictionary_value_from_key(
        TOKEN_MARKET,
        &token_market_key_to_update,
        casper_serde_json_wasm::to_string_pretty(&TokenMarket {
            offeror: Some(token_market.offeror.unwrap()),
            minimum_offer: U256::zero(),
            bidder: None,
            locked_bid: U256::zero(),
            is_active: Some(false),
        })
        .unwrap(),
    );
}

#[no_mangle]
pub extern "C" fn change_offer() {
    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    let new_minimum_offer: U256 = runtime::get_named_arg(ARG_NEW_MINIMUM_OFFER);

    check_enabled_nft(contract_hash: Key);

    let identifier_mode = get_identifier_mode_from_runtime_args();
    let token_identifier = get_token_identifier_from_runtime_args(&identifier_mode);
    let token_market_key_to_update = get_token_market_key(&contract_hash, &token_identifier);
    let token_market = get_token_market(&contract_hash, &token_identifier);
    let caller = get_immediate_caller_key();
    if token_market.offeror.unwrap() != caller {
        runtime::revert(Error::OnlyOfferorCanRevoke);
    }
    if !token_market.is_active.unwrap() || token_market.is_active.unwrap() == false {
        runtime::revert(Error::OfferInactive)
    }

    let wcspr_contract: Key = helpers::get_stored_value_with_user_errors(
        WCSPR_CONTRACT,
        Error::MissingWcsprContract,
        Error::InvalidWcsprContract,
    );
    let contract_hash_addr: HashAddr = wcspr_contract.into_hash().unwrap_or_revert();
    let wspr_contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    if token_market.locked_bid > U256::zero() {
        if token_market.locked_bid >= new_minimum_offer {
            do_trade(
                token_market_key_to_update: String,
                &contract_hash: &Key,
                &token_identifier: &TokenIdentifier,
                &identifier_mode: &NFTIdentifierMode,
                token_market.offeror.unwrap(): Key,
                token_market.bidder.unwrap(): Key,
                new_minimum_offer: U256,
                wcspr_contract: Key,
            )
        } else {
            write_dictionary_value_from_key(
                TOKEN_MARKET,
                &token_market_key_to_update,
                casper_serde_json_wasm::to_string_pretty(&TokenMarket {
                    offeror: Some(token_market.offeror.unwrap()),
                    minimum_offer: new_minimum_offer,
                    bidder: Some(token_market.offeror.unwrap()),
                    locked_bid: token_market.locked_bid,
                    is_active: Some(true),
                })
                .unwrap(),
            );
        }
    } else {
        write_dictionary_value_from_key(
            TOKEN_MARKET,
            &token_market_key_to_update,
            casper_serde_json_wasm::to_string_pretty(&TokenMarket {
                offeror: Some(token_market.offeror.unwrap()),
                minimum_offer: new_minimum_offer,
                bidder: None,
                locked_bid: U256::zero(),
                is_active: Some(true),
            })
            .unwrap(),
        );
    }
}

#[no_mangle]
pub extern "C" fn set_support_token() -> Result<(), Error> {
    let nft_contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);

    let nft_contract_str_key = helpers::make_dictionary_item_key_for_key(nft_contract_hash);

    // let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();

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
    let mut token_list = get_key::<Vec<Key>>(TOKEN_CONTRACT_LIST).unwrap_or_revert();

    let nft_enabled: bool = runtime::get_named_arg(ARG_NFT_ENABLED);
    if nft_enabled {
        if !token_list.contains(&nft_contract_hash) {
            token_list.push(nft_contract_hash);
            set_key(TOKEN_CONTRACT_LIST, token_list);
        }

        write_dictionary_value_from_key(TOKEN_CONTRACT_MAP, &nft_contract_str_key, true);
    } else {
        if token_list.contains(&nft_contract_hash) {
            token_list.retain(|x| *x != nft_contract_hash);
            set_key(TOKEN_CONTRACT_LIST, token_list);
        }
        write_dictionary_value_from_key(TOKEN_CONTRACT_MAP, &nft_contract_str_key, false);

        // write_dictionary_value_from_key(TOKEN_CONTRACT_MAP, &nft_contract_hash.to_string(), false);
    }
    Ok(())
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

    // let caller = get_immediate_caller_key();
    let caller = helpers::get_verified_caller().unwrap_or_revert();

    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }
    set_key(CONTRACT_OWNER_KEY_NAME, new_contract_owner);
    Ok(())
}

#[no_mangle]
pub extern "C" fn change_wcspr_contract() -> Result<(), Error> {
    let new_wcspr_contract: Key = runtime::get_named_arg(ARG_WCSPR_CONTRACT);
    // let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();
    let current_contract_owner = helpers::get_stored_value_with_user_errors(
        CONTRACT_OWNER_KEY_NAME,
        Error::MissingContractOwner,
        Error::InvalidContractOwner,
    );

    // let caller = get_immediate_caller_key();
    let caller = helpers::get_verified_caller().unwrap_or_revert();

    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }
    set_key(WCSPR_CONTRACT, new_wcspr_contract);
    Ok(())
}


#[no_mangle]
pub extern "C" fn set_fee_receiver() -> Result<(), Error> {
    let fee_receiver: Key = runtime::get_named_arg(ARG_MARKET_FEE_RECEIVER);
    let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();
    let caller = helpers::get_verified_caller().unwrap_or_revert();
    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }
    set_key(MARKET_FEE_RECEIVER, fee_receiver);
    Ok(())
}

#[no_mangle]
pub extern "C" fn change_fee() -> Result<(), Error> {
    let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();
    // let caller = get_immediate_caller_key();
    let caller = helpers::get_verified_caller().unwrap_or_revert();
    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }
    let new_fee: U256 = runtime::get_named_arg(MARKET_FEE);
    if new_fee > U256::from("20") {
        runtime::revert(Error::FeeTooHigh);
    }
    set_key(MARKET_FEE, new_fee);
    Ok(())
}

#[no_mangle]
pub extern "C" fn change_royalty_fee() -> Result<(), Error> {
    let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();
    // let caller = get_immediate_caller_key();
    let caller = helpers::get_verified_caller().unwrap_or_revert();
    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }
    let new_royalty_fee: U256 = runtime::get_named_arg(ARG_ROYALTY_FEE);
    if new_royalty_fee > U256::from("10") {
        runtime::revert(Error::FeeTooHigh);
    }
    set_key(ROYALTY_FEE, new_royalty_fee);
    Ok(())
}

#[no_mangle]
pub extern "C" fn change_is_royalty() -> Result<(), Error> {
    let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();

    // let caller = get_immediate_caller_key();
    let caller = helpers::get_verified_caller().unwrap_or_revert();
    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }

    let old_is_royalty: bool = helpers::get_stored_value_with_user_errors(
        IS_ROYALTY,
        Error::MissingIsRoyalty,
        Error::InvalidIsRoyalty,
    );

    let new_is_royalty: bool = runtime::get_named_arg(ARG_IS_ROYALTY);
    if new_is_royalty == old_is_royalty {
        runtime::revert(Error::SameIsRoyalty);
    }
    set_key(IS_ROYALTY, new_is_royalty);
    Ok(())
}

fn get_token_owner(
    contract_hash: &Key,
    identifier_mode: &NFTIdentifierMode,
    token_identifier: &TokenIdentifier,
) -> Key {
    let contract_hash_addr: HashAddr = contract_hash.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    match *identifier_mode {
        NFTIdentifierMode::Ordinal => runtime::call_contract::<Key>(
            contract_hash,
            ENTRY_POINT_OWNER_OF,
            runtime_args! {
                ARG_TOKEN_ID => token_identifier.clone().get_index().unwrap()
            },
        ),
        NFTIdentifierMode::Hash => runtime::call_contract::<Key>(
            contract_hash,
            ENTRY_POINT_OWNER_OF,
            runtime_args! {
                ARG_TOKEN_HASH =>
                token_identifier.clone().get_hash().unwrap()
            },
        ),
    }
}

// this fn to get CasperPunk Creator to do royalty things
fn get_token_creator(
    contract_hash: &Key,
    identifier_mode: &NFTIdentifierMode,
    token_identifier: &TokenIdentifier,
) -> Key {
    let csp_contract_hash_addr: HashAddr = contract_hash.into_hash().unwrap_or_revert();
    let csp_contract_hash: ContractHash = ContractHash::new(csp_contract_hash_addr);

    match *identifier_mode {
        NFTIdentifierMode::Ordinal => runtime::call_contract::<Key>(
            csp_contract_hash,
            "get_token_creator",
            runtime_args! {
                ARG_TOKEN_ID => token_identifier.clone().get_index().unwrap()
            },
        ),
        NFTIdentifierMode::Hash => runtime::call_contract::<Key>(
            csp_contract_hash,
            "get_token_creator",
            runtime_args! {
                ARG_TOKEN_HASH =>
                token_identifier.clone().get_hash().unwrap()
            },
        ),
    }
}

fn get_token_metadata(
    contract_hash: &Key,
    identifier_mode: &NFTIdentifierMode,
    token_identifier: &TokenIdentifier,
) -> String {
    let contract_hash_addr: HashAddr = contract_hash.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    match *identifier_mode {
        NFTIdentifierMode::Ordinal => runtime::call_contract::<String>(
            contract_hash,
            ENTRY_POINT_METADATA,
            runtime_args! {
                ARG_TOKEN_ID => token_identifier.clone().get_index().unwrap()
            },
        ),
        NFTIdentifierMode::Hash => runtime::call_contract::<String>(
            contract_hash,
            ENTRY_POINT_METADATA,
            runtime_args! {
                ARG_TOKEN_HASH =>
                token_identifier.clone().get_hash().unwrap()
            },
        ),
    }
}

fn set_offer(
    contract_hash: &Key,
    identifier_mode: &NFTIdentifierMode,
    token_identifier: &TokenIdentifier,
    offeror: Option<Key>,
    minimum_offer: U256,
) {
    let token_market_key = get_token_market_key(contract_hash, token_identifier); // Key for TOKEN_MARKET dictionary
                                                                                  // Check if this token is already been offered
    let token_market_str = get_dictionary_value_from_key::<String>(TOKEN_MARKET, &token_market_key);
    if token_market_str.is_some() {
        let token_market =
            casper_serde_json_wasm::from_str::<TokenMarket>(&token_market_str.unwrap());
        if token_market.is_ok() {
            let token_market_unwrap = token_market.unwrap();
            if token_market_unwrap.is_active.unwrap() == true {
                runtime::revert(Error::AlreadyMakeOffer);
            }
        }
    }

    write_dictionary_value_from_key(
        TOKEN_MARKET,
        &token_market_key,
        casper_serde_json_wasm::to_string_pretty(&TokenMarket {
            offeror: offeror,
            minimum_offer: minimum_offer,
            bidder: None,
            locked_bid: U256::zero(),
            is_active: Some(true),
        })
        .unwrap(),
    );
    events::emit(&MarketPlaceEvent::Offer {
        token_market_key: token_market_key,
        offeror: offeror.unwrap(),
        minimum_offer: minimum_offer,
    });
}

fn do_trade(
    token_market_key: String,
    nft_contract_hash: &Key,
    token_id: &TokenIdentifier,
    identifier_mode: &NFTIdentifierMode,
    offeror: Key,
    bidder: Key,
    value: U256,         // cspr value
    wcspr_contract: Key, // ERC20 token contract
) {
    let market_fee_receiver: Key = helpers::get_stored_value_with_user_errors(
        MARKET_FEE_RECEIVER,
        Error::MissingFeeReceiver,
        Error::InvalidFeeReceiver,
    );

    let royalty_fee: U256 = helpers::get_stored_value_with_user_errors(
        ROYALTY_FEE,
        Error::MissingRoyaltyFee,
        Error::InvalidRoyaltyFee,
    );

    let is_royalty: bool = helpers::get_stored_value_with_user_errors(
        IS_ROYALTY,
        Error::MissingIsRoyalty,
        Error::InvalidIsRoyalty,
    );

    // pay FEE to contract of CSPR.

    let fee_portion: U256 = helpers::get_stored_value_with_user_errors(
        MARKET_FEE,
        Error::MissingFeePortion,
        Error::InvalidFeePortion,
    );

    let trade_fee: U256 = value * fee_portion / U256::from("1000");
    let total_contract_receive: U256 = trade_fee * U256::from("2");

    // let nft_creator: Key = get_token_creator(
    //     nft_contract_hash: &Key,
    //     identifier_mode: &NFTIdentifierMode,
    //     token_id: &TokenIdentifier,
    // );

    let royalty_amount: U256 = (value - trade_fee) * royalty_fee / U256::from("1000");
    let seller_amount: U256 = value - trade_fee - royalty_amount;

    let contract_hash_addr: HashAddr = wcspr_contract.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);
    // Transfer fee to contract
    let _: () = runtime::call_contract(
        contract_hash, // wcspr contract
        TRANSFER_FROM_ENTRY_POINT_NAME,
        runtime_args! {
            "owner" => bidder,
            "recipient" => market_fee_receiver,
            "amount" => total_contract_receive,
        },
    );

    // Check if is_royalty is true then transfer ROYALTY_FEE to CREATOR

    if is_royalty {
        // get creator of nft
        let nft_creator : Key = get_token_creator(nft_contract_hash: &Key, identifier_mode: &NFTIdentifierMode, token_id: &TokenIdentifier);

        let _: () = runtime::call_contract(
            contract_hash, // wcspr contract
            TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                "owner" => bidder,
                "recipient" => nft_creator,
                "amount" => royalty_amount,
            },
        );

        // Transfer wcspr to seller
        let _: () = runtime::call_contract(
            contract_hash, // wcspr contract
            TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                "owner" => bidder,
                "recipient" => offeror,
                "amount" => seller_amount,
            },
        );
    } else {
        // Transfer wcspr to seller
        let _: () = runtime::call_contract(
            contract_hash, // wcspr contract
            TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                "owner" => bidder,
                "recipient" => offeror,
                "amount" => value - trade_fee,
            },
        );
    }

    // let src_purse: URef = runtime::get_named_arg("src_purse");

    // let contract_purse_key = runtime::get_key("contract_purse").unwrap_or_revert();
    // let contract_purse = *contract_purse_key.as_uref().unwrap_or_revert();

    // transfer_from_purse_to_purse(
    //     src_purse,
    //     contract_purse,
    //     u256_to_u512(total_contract_receive),
    //     None,
    // )
    // .unwrap_or_revert();

    // // CSPR to NFT owner
    // let offeror_account_add = offeror.into_account().unwrap_or_revert();

    // transfer_from_purse_to_account(
    //     src_purse,
    //     offeror_account_add,
    //     u256_to_u512(value - trade_fee),
    //     None,
    // )
    // .unwrap_or_revert();

    // transfer_from nft from offeror to bidder
    let identifier_mode = get_identifier_mode_from_runtime_args();
    let token_identifier = get_token_identifier_from_runtime_args(&identifier_mode);

    cep78_transfer_from(
        nft_contract_hash: &Key,
        offeror: Key,
        bidder: Key,
        identifier_mode: NFTIdentifierMode,
        token_identifier: TokenIdentifier,
    );

    write_dictionary_value_from_key(
        TOKEN_MARKET,
        &token_market_key,
        casper_serde_json_wasm::to_string_pretty(&TokenMarket {
            offeror: None,
            minimum_offer: U256::zero(),
            bidder: None,
            locked_bid: U256::zero(),
            is_active: Some(false),
        })
        .unwrap(),
    );

    events::emit(&MarketPlaceEvent::DoTrade {
        from: offeror,
        to: bidder,
        value: value,
    });
}

fn cep78_transfer_from(
    contract_hash: &Key,
    source: Key,
    target: Key,
    identifier_mode: NFTIdentifierMode,
    token_identifier: TokenIdentifier,
) {
    let contract_hash_addr: HashAddr = contract_hash.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);
    match identifier_mode {
        NFTIdentifierMode::Ordinal => {
            let _: (String, Key) = runtime::call_contract(
                contract_hash,
                TRANSFER_ENTRY_POINT_NAME,
                runtime_args! {
                    ARG_SOURCE_KEY => source,
                    ARG_TARGET_KEY => target,
                    ARG_TOKEN_ID => token_identifier.get_index().unwrap()
                },
            );
        }
        NFTIdentifierMode::Hash => {
            let _: (String, Key) = runtime::call_contract(
                contract_hash,
                TRANSFER_ENTRY_POINT_NAME,
                runtime_args! {
                    ARG_SOURCE_KEY => source,
                    ARG_TARGET_KEY => target,
                    ARG_TOKEN_HASH => token_identifier.get_hash().unwrap()
                },
            );
        }
    }
}
fn update_new_bidder(
    token_market_key: String,
    bidding_offer: U256,
    token_market: TokenMarket,
    new_bidder: Key,
    wcspr_contract: Key, // ERC20 token contract
    contract_self_key: Key,
    // new_bidder_addr : Address
) {
    let market_fee_receiver: Key = helpers::get_stored_value_with_user_errors(
        MARKET_FEE_RECEIVER,
        Error::MissingFeeReceiver,
        Error::InvalidFeeReceiver,
    );

    let fee_portion: U256 = helpers::get_stored_value_with_user_errors(
        MARKET_FEE,
        Error::MissingFeePortion,
        Error::InvalidFeePortion,
    );
    let needed_amount: U256 = bidding_offer + bidding_offer * fee_portion / U256::from("1000");

    // Transfer CSPR
    // let new_src_purse: URef = runtime::get_named_arg(ARG_SRC_PURSE);
    // let contract_purse_key = runtime::get_key(CONTRACT_PURSE).unwrap_or_revert();
    // let contract_purse = *contract_purse_key.as_uref().unwrap_or_revert();
    // // Transfer_from CSPR from new_bidder to contract_purse.
    // transfer_from_purse_to_purse(
    //     new_src_purse,
    //     contract_purse,
    //     u256_to_u512(needed_amount),
    //     None,
    // )
    // .unwrap_or_revert();

    // // Transfer_from CSPR from contract_purse to old_bidder.

    // if token_market.bidder.is_some() && (token_market.locked_bid != U256::zero()) {
    //     let older_bidder_account: Key = token_market.bidder.unwrap();
    //     let older_bidder_account_add = older_bidder_account.into_account().unwrap_or_revert();

    //     transfer_from_purse_to_account(
    //         contract_purse,
    //         older_bidder_account_add,
    //         u256_to_u512(token_market.locked_bid),
    //         None,
    //     )
    //     .unwrap_or_revert();
    // };
    let offeror: Key = token_market
        .offeror
        .unwrap_or_revert_with(Error::MissingOfferer);

    let contract_hash_addr: HashAddr = wcspr_contract.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    let _: () = runtime::call_contract(
        contract_hash,
        TRANSFER_FROM_ENTRY_POINT_NAME,
        runtime_args! {
            "owner" => new_bidder,
            "recipient" => contract_self_key,
            "amount" => bidding_offer,
        },
    );

    if token_market.bidder.is_some() && (token_market.locked_bid != U256::zero()) {
        let older_bidder_account: Key = token_market.bidder.unwrap();
        let _: () = runtime::call_contract(
            contract_hash,
            TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                "recipient" => older_bidder_account,
                "amount" => token_market.locked_bid,
            },
        );
    };

    //Update dictionary for token_maket_key

    write_dictionary_value_from_key(
        TOKEN_MARKET,
        &token_market_key,
        casper_serde_json_wasm::to_string_pretty(&TokenMarket {
            offeror: token_market.offeror,
            minimum_offer: token_market.minimum_offer,
            bidder: Some(new_bidder),
            locked_bid: bidding_offer,
            is_active: Some(true),
        })
        .unwrap(),
    );

    events::emit(&MarketPlaceEvent::Bid {
        token_market_key: token_market_key,
        offeror: token_market.offeror.unwrap(),
        bidder: new_bidder,
        value: bidding_offer,
    });
}
fn check_enabled_nft(contract_hash: Key) {
    let nft_contract_str_key = helpers::make_dictionary_item_key_for_key(contract_hash);

    let enabled = get_dictionary_value_from_key::<bool>(
        TOKEN_CONTRACT_MAP,
        &nft_contract_str_key.to_string(),
    )
    .unwrap();
    if !enabled {
        runtime::revert(Error::UnsupportedToken);
    }
}
