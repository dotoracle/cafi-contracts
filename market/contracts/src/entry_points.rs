use alloc::{boxed::Box, string::String, vec};

use crate::constants::*;

use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256,
};

// fn request_bridge_nft() -> EntryPoint {
//     // EntryPoint::new(
//     //     String::from(REQUEST_BRIDGE_ENTRY_POINT_NAME),
//     //     vec![
//     //         Parameter::new(ARG_TOKEN_IDS, CLType::List(Box::new(CLType::U64))),
//     //         Parameter::new(ARG_TOKEN_HASHES, CLType::List(Box::new(CLType::String))),
//     //         Parameter::new(ARG_TO_CHAINID, U256::cl_type()),
//     //         Parameter::new(ARG_IDENTIFIER_MODE, u8::cl_type()),
//     //         Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
//     //         Parameter::new(ARG_RECEIVER_ADDRESS, String::cl_type()),
//     //         Parameter::new(ARG_REQUEST_ID, String::cl_type()),
//     //     ],
//     //     CLType::String,
//     //     EntryPointAccess::Public,
//     //     EntryPointType::Contract,
//     // )
// }

fn transfer_owner() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_OWNER_ENTRY_POINT_NAME),
        vec![Parameter::new(ARG_CONTRACT_OWNER, CLType::Key)],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn change_fee() -> EntryPoint {
    EntryPoint::new(
        String::from(CHANGE_FEE_ENTRY_POINT_NAME),
        vec![Parameter::new(ARG_MARKET_FEE, CLType::U256)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn change_royalty_fee() -> EntryPoint {
    EntryPoint::new(
        String::from(CHANGE_ROYALTY_FEE_ENTRY_POINT_NAME),
        vec![Parameter::new(ARG_ROYALTY_FEE, CLType::U256)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn change_is_royalty() -> EntryPoint {
    EntryPoint::new(
        String::from(CHANGE_IS_ROYALTY_ENTRY_POINT_NAME),
        vec![Parameter::new(ARG_IS_ROYALTY, CLType::Bool)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn change_wcspr_contract() -> EntryPoint {
    EntryPoint::new(
        String::from(CHANGE_WCSPR_CONTRACT_ENTRY_POINT_NAME),
        vec![Parameter::new(ARG_WCSPR_CONTRACT, CLType::Key)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}


fn offer() -> EntryPoint {
    EntryPoint::new(
        String::from(OFFER_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_MINIMUM_OFFER, CLType::U256),
            Parameter::new(ARG_TOKEN_ID, CLType::U64),
            Parameter::new(ARG_TOKEN_HASH, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn change_offer() -> EntryPoint {
    EntryPoint::new(
        String::from(CHANGE_OFFER_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_NEW_MINIMUM_OFFER, CLType::U256),
            Parameter::new(ARG_TOKEN_ID, CLType::U64),
            Parameter::new(ARG_TOKEN_HASH, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn bid() -> EntryPoint {
    EntryPoint::new(
        String::from(BID_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_BIDDING_OFFER, CLType::U256),
            Parameter::new(ARG_TOKEN_ID, CLType::U64),
            Parameter::new(ARG_TOKEN_HASH, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn increase_bid() -> EntryPoint {
    EntryPoint::new(
        String::from(INCREASE_BID_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_NEW_OFFER, CLType::U256),
            Parameter::new(ARG_TOKEN_ID, CLType::U64),
            Parameter::new(ARG_TOKEN_HASH, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn revoke_offer() -> EntryPoint {
    EntryPoint::new(
        String::from(REVOKE_OFFER_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_TOKEN_ID, CLType::U64),
            Parameter::new(ARG_TOKEN_HASH, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn revoke_bid() -> EntryPoint {
    EntryPoint::new(
        String::from(REVOKE_BID_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_TOKEN_ID, CLType::U64),
            Parameter::new(ARG_TOKEN_HASH, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn set_support_token() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_SUPPORTED_TOKEN_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_NFT_ENABLED, CLType::Bool),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn init() -> EntryPoint {
    EntryPoint::new(
        String::from(INIT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_CONTRACT_OWNER, CLType::Key),
            Parameter::new(ARG_MARKET_FEE_RECEIVER, CLType::Key),
            Parameter::new(ARG_WCSPR_CONTRACT, CLType::Key),
            Parameter::new(ARG_MARKET_FEE, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub(crate) fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(transfer_owner());
    entry_points.add_entry_point(init());
    entry_points.add_entry_point(offer());
    entry_points.add_entry_point(change_fee());
    entry_points.add_entry_point(change_royalty_fee());
    entry_points.add_entry_point(bid());
    entry_points.add_entry_point(increase_bid());
    entry_points.add_entry_point(revoke_offer());
    entry_points.add_entry_point(change_offer());
    entry_points.add_entry_point(change_wcspr_contract());
    entry_points.add_entry_point(revoke_bid());
    entry_points.add_entry_point(set_support_token());
    entry_points.add_entry_point(change_is_royalty());
    entry_points
}
