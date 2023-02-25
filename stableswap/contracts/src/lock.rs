use crate::helpers::{self, *};
use crate::constants::*;
use crate::error::Error;

pub(crate) fn when_not_locked() {
    let locked: bool = helpers::get_key(IS_LOCKED).unwrap();
    require(!locked, Error::ContractLocked);
}

pub(crate) fn lock_contract() {
    helpers::set_key(IS_LOCKED, true);
}

pub(crate) fn unlock_contract() {
    helpers::set_key(IS_LOCKED, false);
}