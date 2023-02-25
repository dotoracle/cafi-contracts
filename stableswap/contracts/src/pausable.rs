use crate::helpers::{self, *};
use crate::constants::*;
use crate::owner::*;
use crate::error::Error;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use alloc::{string::String, vec, vec::*};
use casper_types::{
    EntryPoint, EntryPointAccess, EntryPointType, CLType, Parameter, CLValue
};
#[no_mangle]
pub extern "C" fn get_paused() {
    runtime::ret(CLValue::from_t(get_paused_internal()).unwrap_or_revert());    
}

pub(crate) fn get_paused_internal() -> bool {
    let paused: bool = helpers::get_key(PAUSED).unwrap();
    paused
}

#[no_mangle]
pub extern "C" fn set_paused() -> Result<(), Error> {
    only_owner();
    let paused: bool = runtime::get_named_arg(ARG_PAUSED);
    helpers::set_key(PAUSED, paused);
    Ok(())
}

pub(crate) fn when_not_paused() {
    require(!get_paused_internal(), Error::ContractPaused);
}

pub fn entry_points() -> Vec<EntryPoint> {
    vec![EntryPoint::new(
        String::from("set_paused"),
        vec![Parameter::new(ARG_PAUSED, CLType::Bool)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )]
}