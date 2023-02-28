use alloc::{boxed::Box, string::String, vec, vec::Vec};

use crate::constants::*;

use casper_types::{
    CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

use crate::owner;
use crate::pausable;

// fn change_fee() -> EntryPoint {
//     EntryPoint::new(
//         String::from(CHANGE_FEE_ENTRY_POINT_NAME),
//         vec![Parameter::new(ARG_MARKET_FEE, CLType::U256)],
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     )
// }

// fn sell() -> EntryPoint {
//     EntryPoint::new(
//         String::from(SELL_ENTRY_POINT_NAME),
//         vec![
//             // Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
//             // Parameter::new(ARG_MINIMUM_OFFER, CLType::U256),
//             // Parameter::new(ARG_TOKEN_ID, CLType::U64),
//             // Parameter::new(ARG_TOKEN_HASH, CLType::String),
//         ],
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     )
// }
// fn buy() -> EntryPoint {
//     EntryPoint::new(
//         String::from(BUY_ENTRY_POINT_NAME),
//         vec![
//             Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
//             Parameter::new("amount", CLType::U256),
//             Parameter::new(ARG_TOKEN_ID, CLType::String),
//             Parameter::new(ARG_BUYER, CLType::Key),
//             Parameter::new("src_purse", CLType::URef),

//         ],
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     )
// }


// fn revoke_sell() -> EntryPoint {
//     EntryPoint::new(
//         String::from(REVOKE_OFFER_ENTRY_POINT_NAME),
//         vec![
//             Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
//             Parameter::new(ARG_TOKEN_ID, CLType::String),
//             Parameter::new(ARG_TOKEN_HASH, CLType::String),
//         ],
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     )
// }

// fn set_support_token() -> EntryPoint {
//     EntryPoint::new(
//         String::from(SET_SUPPORTED_TOKEN_ENTRY_POINT_NAME),
//         vec![
//             Parameter::new(ARG_NFT_CONTRACT_HASH, CLType::Key),
//             Parameter::new(ARG_NFT_ENABLED, CLType::Bool),
//         ],
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     )
// }

// fn init() -> EntryPoint {
//     EntryPoint::new(
//         String::from(INIT_ENTRY_POINT_NAME),
//         vec![
//             Parameter::new(ARG_CONTRACT_HASH, CLType::Key),
//             Parameter::new(ARG_CONTRACT_OWNER, CLType::Key),
//             Parameter::new(ARG_MARKET_FEE_RECEIVER, CLType::Key),
//             Parameter::new(ARG_MARKET_FEE, CLType::U256),
//         ],
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     )
// }

/// Returns the default set of ERC20 token entry points.

fn add_entry_points(entry_points: &mut EntryPoints, list: &Vec<EntryPoint>) {
    for e in list {
        entry_points.add_entry_point(e.clone());
    }
}

pub(crate) fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    
    add_entry_points(&mut entry_points, &owner::entry_points());
    add_entry_points(&mut entry_points, &pausable::entry_points());
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from(INIT_ENTRY_POINT_NAME),
            vec![
                Parameter::new(ARG_POOLED_TOKENS, CLType::List(Box::new(CLType::Key))),
                Parameter::new(ARG_TOKEN_DECIMALS, CLType::List(Box::new(CLType::U8))),
                Parameter::new(ARG_LP_TOKEN, CLType::Key),
                Parameter::new(ARG_A, CLType::U128),
                Parameter::new(ARG_FEE, CLType::U64),
                Parameter::new(ARG_ADMIN_FEE, CLType::U64),
                Parameter::new(ARG_CONTRACT_OWNER, CLType::Key)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("get_a"),
            vec![],
            CLType::U128,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("get_a_precise"),
            vec![],
            CLType::U128,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("get_token"),
            vec![
                Parameter::new(ARG_INDEX, CLType::U64)
            ],
            CLType::Key,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("get_token_index"),
            vec![
                Parameter::new(ARG_TOKEN, CLType::Key)
            ],
            CLType::U64,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("get_token_balance"),
            vec![
                Parameter::new(ARG_INDEX, CLType::U64)
            ],
            CLType::U128,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("get_virtual_price"),
            vec![],
            CLType::U128,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("calculate_swap"),
            vec![
                Parameter::new(ARG_TOKEN_INDEX_FROM, CLType::U64),
                Parameter::new(ARG_TOKEN_INDEX_TO, CLType::U64),
                Parameter::new(ARG_DX, CLType::U128)
            ],
            CLType::U128,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("calculate_token_amount"),
            vec![
                Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U128))),
                Parameter::new(ARG_DEPOSIT, CLType::Bool)
            ],
            CLType::U128,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("calculate_remove_liquidity_one_token"),
            vec![
                Parameter::new(ARG_TOKEN_AMOUNT, CLType::U128),
                Parameter::new(ARG_TOKEN_INDEX, CLType::U64)
            ],
            CLType::U128,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );


    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("get_admin_balance"),
            vec![
                Parameter::new(ARG_TOKEN_INDEX, CLType::U64)
            ],
            CLType::U128,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("swap"),
            vec![
                Parameter::new(ARG_TOKEN_INDEX_FROM, CLType::U64),
                Parameter::new(ARG_TOKEN_INDEX_TO, CLType::U64),
                Parameter::new(ARG_DX, CLType::U128),
                Parameter::new(ARG_MIN_DY, CLType::U128),
                Parameter::new(ARG_DEADLINE, CLType::U64)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("add_liquidity"),
            vec![
                Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U128))),
                Parameter::new(ARG_MIN_TO_MINT, CLType::U128),
                Parameter::new(ARG_DEADLINE, CLType::U64)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("remove_liquidity"),
            vec![
                Parameter::new(ARG_AMOUNT, CLType::U128),
                Parameter::new(ARG_MIN_AMOUNTS, CLType::List(Box::new(CLType::U128))),
                Parameter::new(ARG_DEADLINE, CLType::U64)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("remove_liquidity_one_token"),
            vec![
                Parameter::new(ARG_TOKEN_AMOUNT, CLType::U128),
                Parameter::new(ARG_TOKEN_INDEX, CLType::U64),
                Parameter::new(ARG_MIN_AMOUNT, CLType::U128),
                Parameter::new(ARG_DEADLINE, CLType::U64)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("remove_liquidity_imbalance"),
            vec![
                Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U128))),
                Parameter::new(ARG_MAX_BURN_AMOUNT, CLType::U128),
                Parameter::new(ARG_DEADLINE, CLType::U64)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("withdraw_admin_fees"),
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("set_admin_fee"),
            vec![
                Parameter::new(ARG_ADMIN_FEE, CLType::U64)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("set_swap_fee"),
            vec![
                Parameter::new(ARG_SWAP_FEE, CLType::U64)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("ramp_a"),
            vec![
                Parameter::new(ARG_FUTURE_A, CLType::U128),
                Parameter::new(ARG_FUTURE_TIME, CLType::U64)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("stop_ramp_a"),
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("update_lp"),
            vec![
                Parameter::new(ARG_LP_TOKEN, CLType::Key)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );

    entry_points
}
