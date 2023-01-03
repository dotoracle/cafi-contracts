use alloc::{boxed::Box, string::String, vec};

use crate::constants::*;

use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256,
};

fn transfer_owner() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_OWNER_ENTRY_POINT_NAME),
        vec![Parameter::new(ARG_CONTRACT_OWNER, CLType::Key)],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn set_reward_per_second() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_REWARD_PER_SECOND),
        vec![Parameter::new(ARG_REWARD_PER_SECOND, CLType::U256)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn set_reward_token() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_REWARD_TOKEN),
        vec![
            Parameter::new(ARG_REWARD_TOKEN, CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn add_new_pool() -> EntryPoint {
    EntryPoint::new(
        String::from(ADD_NEW_POOL),
        vec![
            
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn stake() -> EntryPoint {
    EntryPoint::new(
        String::from(STAKE),
        vec![
            
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn un_stake() -> EntryPoint {
    EntryPoint::new(
        String::from(UN_STAKE),
        vec![
            
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
            Parameter::new(ARG_REWARD_TOKEN, CLType::Key),
            Parameter::new(ARG_START_SECOND, CLType::U256),
            Parameter::new(ARG_REWARD_PER_SECOND, CLType::U256),
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
    entry_points.add_entry_point(set_reward_token());
    entry_points.add_entry_point(set_reward_per_second());
    entry_points.add_entry_point(add_new_pool());
    entry_points.add_entry_point(stake());
    entry_points.add_entry_point(un_stake());
    entry_points
}
